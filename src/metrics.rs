use crate::material::Material;

/// Specific strength (Strength-to-weight ratio).
///
/// In aerospace and structural engineering, specific strength measures how much load
/// a material can withstand per unit mass:
///
/// Specific Strength = σ / ρ
///
/// Units:
/// - Strength σ: Pa (N/m²)
/// - Density ρ: kg/m³
/// - Result: (N·m)/kg or m²/s² (often expressed in kN·m/kg for readability)
///
/// Returns specific ultimate tensile strength in (N·m)/kg, or `None` if either property is missing or density <= 0.
pub fn specific_tensile_strength(material: &Material) -> Option<f64> {
    let tensile_strength = material.tensile_strength_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(tensile_strength / density)
}

/// Specific yield strength:
///
/// Specific Yield Strength = σ_yield / ρ
///
/// Returns yield specific strength in (N·m)/kg.
pub fn specific_yield_strength(material: &Material) -> Option<f64> {
    let yield_strength = material.yield_strength_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(yield_strength / density)
}

/// General specific strength using whichever strength metric is available
/// (prefers tensile strength, falls back to yield strength).
pub fn specific_strength(material: &Material) -> Option<f64> {
    specific_tensile_strength(material).or_else(|| specific_yield_strength(material))
}

/// Specific stiffness (Specific modulus):
///
/// Specific Modulus = E / ρ
///
/// Where:
/// - E = Young's modulus (Pa)
/// - ρ = Density (kg/m³)
///
/// Measures resistance to elastic deformation per unit weight.
/// Crucial for aerospace structures prone to buckling and aeroelastic flutter.
pub fn specific_stiffness(material: &Material) -> Option<f64> {
    let youngs_modulus = material.youngs_modulus_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(youngs_modulus / density)
}

/// Thermal diffusivity:
///
/// α = k / (ρ · c_p)
///
/// Where:
/// - k = Thermal conductivity (W/(m·K))
/// - ρ = Density (kg/m³)
/// - c_p = Specific heat capacity (J/(kg·K))
///
/// Measures the rate at which heat transfers through a material relative to its
/// heat storage capacity. Units: m²/s (often multiplied by 10⁶ to give mm²/s).
pub fn thermal_diffusivity(material: &Material) -> Option<f64> {
    let k = material.thermal_conductivity_val()?;
    let rho = material.density_val()?;
    let cp = material.specific_heat_val()?;

    if rho <= 0.0 || cp <= 0.0 {
        return None;
    }

    Some(k / (rho * cp))
}

/// Temperature capability:
///
/// Uses the explicitly recorded melting point (or solidus temperature) in Kelvin.
///
/// > **Scientific Note**: Melting point represents a thermodynamic phase change threshold,
/// > NOT a certified continuous operating temperature. Service limits are generally much lower
/// > due to creep, oxidation, and loss of tensile strength.
pub fn temperature_capability(material: &Material) -> Option<f64> {
    material.melting_point_val()
}

/// Transparent Aerospace Suitability Heuristic Score (0.0 to 100.0).
///
/// > **DISCLAIMER**: This is a transparent heuristic engineering metric for exploration
/// > and comparison. It is NOT an aerospace specification or industrial qualification standard.
///
/// ### Formula & Weights:
/// 1. **Specific Strength (35% weight)**: Normalized from 20 kN·m/kg (0.0) to 200 kN·m/kg (1.0).
/// 2. **Specific Stiffness (25% weight)**: Normalized from 5e6 m²/s² (0.0) to 35e6 m²/s² (1.0).
/// 3. **Temperature Capability (20% weight)**: Normalized from 500 K (0.0) to 2500 K (1.0).
/// 4. **Low Density Bonus (10% weight)**: Normalized from 8000 kg/m³ (0.0) to 1200 kg/m³ (1.0).
/// 5. **Thermal Management (10% weight)**: Stability / diffusivity index.
///
/// Missing Data Policy:
/// If fundamental structural properties (density and tensile/yield strength) are missing,
/// this function returns `None` rather than assuming zero.
pub fn aerospace_suitability_score(material: &Material) -> Option<f64> {
    let density = material.density_val()?;
    let strength = material
        .tensile_strength_val()
        .or_else(|| material.yield_strength_val())?;

    if density <= 0.0 {
        return None;
    }

    let mut score = 0.0;
    let mut total_weight = 0.0;

    // 1. Specific Strength (Weight = 35)
    let spec_strength = strength / density; // J/kg or (N*m)/kg
    let norm_spec_strength = ((spec_strength - 20_000.0) / (200_000.0 - 20_000.0)).clamp(0.0, 1.0);
    score += norm_spec_strength * 35.0;
    total_weight += 35.0;

    // 2. Specific Stiffness (Weight = 25)
    if let Some(e) = material.youngs_modulus_val() {
        let spec_stiffness = e / density;
        let norm_stiffness = ((spec_stiffness - 5.0e6) / (35.0e6 - 5.0e6)).clamp(0.0, 1.0);
        score += norm_stiffness * 25.0;
        total_weight += 25.0;
    }

    // 3. Temperature Capability (Melting Point) (Weight = 20)
    if let Some(tm) = material.melting_point_val() {
        let norm_temp = ((tm - 500.0) / (2500.0 - 500.0)).clamp(0.0, 1.0);
        score += norm_temp * 20.0;
        total_weight += 20.0;
    }

    // 4. Low Density Bonus (Weight = 10)
    let norm_density = ((8000.0 - density) / (8000.0 - 1200.0)).clamp(0.0, 1.0);
    score += norm_density * 10.0;
    total_weight += 10.0;

    // 5. Thermal Conductivity / Management (Weight = 10)
    if let Some(k) = material.thermal_conductivity_val() {
        // In aerospace, depending on role, both high conductivity (heat spreaders) and low (TPS) are valued.
        // We give an intermediate credit for well-characterized thermal behavior.
        let norm_k = (k / 200.0).clamp(0.0, 1.0);
        score += norm_k * 10.0;
        total_weight += 10.0;
    }

    if total_weight == 0.0 {
        return None;
    }

    // Rescale to 0-100 based on available property weights
    let final_score = (score / total_weight) * 100.0;
    Some(final_score.clamp(0.0, 100.0))
}

/// Generates rule-based engineering observations for a material.
///
/// These observations highlight notable physical characteristics based on engineering threshold heuristics.
pub fn generate_observations(material: &Material) -> Vec<String> {
    let mut observations = Vec::new();

    // Density observations
    if let Some(rho) = material.density_val() {
        if rho < 2000.0 {
            observations.push("Ultralight structural potential (Density < 2,000 kg/m³)".to_string());
        } else if rho < 3000.0 {
            observations.push("Relatively low density (< 3,000 kg/m³) — lightweight structural candidate".to_string());
        } else if rho > 8000.0 {
            observations.push("High density (> 8,000 kg/m³) — heavy structural or ballast material".to_string());
        }
    }

    // Specific strength observations
    if let Some(spec_str) = specific_strength(material) {
        let spec_str_kn = spec_str / 1000.0; // kN*m/kg
        if spec_str_kn > 150.0 {
            observations.push(format!(
                "Exceptional strength-to-weight ratio ({:.1} kN·m/kg) — high structural efficiency",
                spec_str_kn
            ));
        } else if spec_str_kn > 80.0 {
            observations.push(format!(
                "High strength-to-weight potential ({:.1} kN·m/kg)",
                spec_str_kn
            ));
        }
    }

    // Specific stiffness observations
    if let Some(spec_stiff) = specific_stiffness(material) {
        let spec_stiff_m = spec_stiff / 1.0e6; // GPa / (g/cm^3)
        if spec_stiff_m > 25.0 {
            observations.push(format!(
                "High specific stiffness ({:.1} GPa/(g/cm³)) — excellent resistance to structural deflection",
                spec_stiff_m
            ));
        }
    }

    // Thermal conductivity observations
    if let Some(k) = material.thermal_conductivity_val() {
        if k > 100.0 {
            observations.push(format!(
                "High thermal conductivity ({:.1} W/(m·K)) — effective for heat sinks and thermal dissipation",
                k
            ));
        } else if k < 2.0 {
            observations.push(format!(
                "Very low thermal conductivity ({:.2} W/(m·K)) — functions as a thermal insulator or barrier",
                k
            ));
        }
    }

    // Thermal expansion observations
    if let Some(alpha) = material.thermal_expansion_val() {
        let alpha_ppm = alpha * 1.0e6;
        if alpha_ppm < 5.0 {
            observations.push(format!(
                "Very low thermal expansion coefficient ({:.2} µm/(m·K)) — high dimensional stability over temperature changes",
                alpha_ppm
            ));
        } else if alpha_ppm > 20.0 {
            observations.push(format!(
                "High thermal expansion ({:.1} µm/(m·K)) — susceptible to thermal strain under temperature gradients",
                alpha_ppm
            ));
        }
    }

    // Temperature capability observations
    if let Some(tm) = material.melting_point_val() {
        let tm_c = tm - 273.15;
        if tm > 2000.0 {
            observations.push(format!(
                "Refractory high-temperature capability (Melting point: {:.0} K / {:.0} °C)",
                tm, tm_c
            ));
        } else if tm > 1200.0 {
            observations.push(format!(
                "Good elevated-temperature capability (Melting point: {:.0} K / {:.0} °C)",
                tm, tm_c
            ));
        }
    }

    if observations.is_empty() {
        observations.push("Limited property data available to derive engineering observations.".to_string());
    }

    observations
}

/// Ranking metric choices available on the Rankings page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingMetric {
    AerospaceScore,
    SpecificStrength,
    ThermalConductivity,
    TemperatureCapability,
    LowestDensity,
    SpecificStiffness,
    ThermalDiffusivity,
}

impl RankingMetric {
    pub fn all() -> &'static [RankingMetric] {
        &[
            RankingMetric::AerospaceScore,
            RankingMetric::SpecificStrength,
            RankingMetric::ThermalConductivity,
            RankingMetric::TemperatureCapability,
            RankingMetric::LowestDensity,
            RankingMetric::SpecificStiffness,
            RankingMetric::ThermalDiffusivity,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            RankingMetric::AerospaceScore => "Aerospace Suitability (Heuristic)",
            RankingMetric::SpecificStrength => "Specific Strength (Strength / Density)",
            RankingMetric::ThermalConductivity => "Thermal Conductivity (k)",
            RankingMetric::TemperatureCapability => "Temperature Capability (Melting Point)",
            RankingMetric::LowestDensity => "Lowest Density (Lightweight)",
            RankingMetric::SpecificStiffness => "Specific Stiffness (E / Density)",
            RankingMetric::ThermalDiffusivity => "Thermal Diffusivity (k / (ρ · c_p))",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RankingMetric::AerospaceScore => {
                "Multi-factor heuristic index (0-100) combining specific strength, specific stiffness, \
                 temperature capability, density bonus, and thermal properties. (Not an industrial standard)."
            }
            RankingMetric::SpecificStrength => {
                "Load-bearing capacity per unit mass (σ / ρ). Higher values denote superior structural efficiency \
                 for weight-critical airframe, space, and automotive components."
            }
            RankingMetric::ThermalConductivity => {
                "Rate of steady-state heat conduction (W/(m·K)). Higher values indicate efficient heat spreaders \
                 and cooling elements; lower values denote thermal insulation."
            }
            RankingMetric::TemperatureCapability => {
                "Melting or solidus temperature in Kelvin (K). Denotes the thermodynamic upper phase boundary of the material."
            }
            RankingMetric::LowestDensity => {
                "Material mass per unit volume (kg/m³). Ranked from lowest to highest mass for lightweight design."
            }
            RankingMetric::SpecificStiffness => {
                "Resistance to elastic deflection per unit weight (E / ρ). Vital for aeroelastic flutter resistance and structural rigidity."
            }
            RankingMetric::ThermalDiffusivity => {
                "Speed of temperature propagation (α = k / (ρ · c_p)) in mm²/s. Higher values equalize thermal gradients rapidly."
            }
        }
    }

    /// Calculates the ranking numeric value and formatting for a material.
    pub fn evaluate(&self, material: &Material) -> Option<(f64, String)> {
        match self {
            RankingMetric::AerospaceScore => {
                let val = aerospace_suitability_score(material)?;
                Some((val, format!("{:.1} / 100", val)))
            }
            RankingMetric::SpecificStrength => {
                let val = specific_strength(material)?;
                let kn = val / 1000.0;
                Some((val, format!("{:.2} kN·m/kg", kn)))
            }
            RankingMetric::ThermalConductivity => {
                let val = material.thermal_conductivity_val()?;
                Some((val, format!("{:.2} W/(m·K)", val)))
            }
            RankingMetric::TemperatureCapability => {
                let val = temperature_capability(material)?;
                Some((val, format!("{:.0} K ({:.0} °C)", val, val - 273.15)))
            }
            RankingMetric::LowestDensity => {
                let val = material.density_val()?;
                // We return negative value for sorting so lower density ranks higher
                Some((-val, format!("{:.1} kg/m³", val)))
            }
            RankingMetric::SpecificStiffness => {
                let val = specific_stiffness(material)?;
                let spec_stiff_m = val / 1.0e6;
                Some((val, format!("{:.2} GPa/(g/cm³)", spec_stiff_m)))
            }
            RankingMetric::ThermalDiffusivity => {
                let val = thermal_diffusivity(material)?;
                let val_mm2_s = val * 1.0e6;
                Some((val, format!("{:.3} mm²/s", val_mm2_s)))
            }
        }
    }
}

/// Ranked entry representation for the UI.
#[derive(Debug, Clone)]
pub struct RankedMaterialItem {
    pub material_index: usize,
    pub name: String,
    pub category: String,
    pub sort_value: f64,
    pub formatted_value: String,
    #[allow(dead_code)]
    pub raw_material: Material,
}

/// Evaluates and ranks all materials in the database according to the chosen metric.
pub fn rank_materials(materials: &[Material], metric: RankingMetric) -> Vec<RankedMaterialItem> {
    let mut ranked = Vec::new();

    for (index, material) in materials.iter().enumerate() {
        if let Some((sort_value, formatted_value)) = metric.evaluate(material) {
            ranked.push(RankedMaterialItem {
                material_index: index,
                name: material.name.clone(),
                category: material.category.clone(),
                sort_value,
                formatted_value,
                raw_material: material.clone(),
            });
        }
    }

    // Sort descending by sort_value
    ranked.sort_by(|a, b| {
        b.sort_value
            .partial_cmp(&a.sort_value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    ranked
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{Material, Property};

    fn make_test_material() -> Material {
        Material {
            id: "test_alloy".to_string(),
            name: "Test Alloy".to_string(),
            category: "metal".to_string(),
            composition: None,
            grade: None,
            description: None,
            source_ids: vec!["matweb".to_string()],
            density: Some(Property {
                value: 2700.0,
                unit: Some("kg/m^3".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(2700.0),
                original_unit: None,
                normalization_warning: None,
            }),
            youngs_modulus: Some(Property {
                value: 70.0e9,
                unit: Some("Pa".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(70.0e9),
                original_unit: None,
                normalization_warning: None,
            }),
            yield_strength: Some(Property {
                value: 280.0e6,
                unit: Some("Pa".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(280.0e6),
                original_unit: None,
                normalization_warning: None,
            }),
            tensile_strength: Some(Property {
                value: 310.0e6,
                unit: Some("Pa".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(310.0e6),
                original_unit: None,
                normalization_warning: None,
            }),
            thermal_conductivity: Some(Property {
                value: 150.0,
                unit: Some("W/(m*K)".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(150.0),
                original_unit: None,
                normalization_warning: None,
            }),
            specific_heat: Some(Property {
                value: 900.0,
                unit: Some("J/(kg*K)".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(900.0),
                original_unit: None,
                normalization_warning: None,
            }),
            thermal_expansion: Some(Property {
                value: 23.0e-6,
                unit: Some("1/K".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(23.0e-6),
                original_unit: None,
                normalization_warning: None,
            }),
            emissivity: None,
            melting_point: Some(Property {
                value: 933.0,
                unit: Some("K".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                original_value: Some(933.0),
                original_unit: None,
                normalization_warning: None,
            }),
            boiling_point: None,
        }
    }

    #[test]
    fn test_specific_strength_known_case() {
        let mat = make_test_material();
        // Tensile strength = 310,000,000 Pa, Density = 2700 kg/m3
        // Specific strength = 310,000,000 / 2700 ≈ 114,814.81 (N*m)/kg
        let spec_str = specific_strength(&mat).expect("should compute specific strength");
        let expected = 310.0e6 / 2700.0;
        assert!((spec_str - expected).abs() < 0.1);
    }

    #[test]
    fn test_thermal_diffusivity_known_case() {
        let mat = make_test_material();
        // k = 150 W/(m*K), rho = 2700 kg/m3, cp = 900 J/(kg*K)
        // alpha = 150 / (2700 * 900) = 150 / 2,430,000 = 6.172839e-5 m2/s
        let diff = thermal_diffusivity(&mat).expect("should compute thermal diffusivity");
        let expected = 150.0 / (2700.0 * 900.0);
        assert!((diff - expected).abs() < 1e-9);
    }

    #[test]
    fn test_aerospace_suitability_score_valid_and_bounds() {
        let mat = make_test_material();
        let score = aerospace_suitability_score(&mat).expect("should produce score for valid material");
        assert!(score >= 0.0 && score <= 100.0, "Score {} must be between 0 and 100", score);
    }

    #[test]
    fn test_aerospace_suitability_score_missing_data() {
        let mut mat = make_test_material();
        mat.density = None;
        assert!(aerospace_suitability_score(&mat).is_none(), "Score must be None when density is missing");

        let mut mat2 = make_test_material();
        mat2.tensile_strength = None;
        mat2.yield_strength = None;
        assert!(aerospace_suitability_score(&mat2).is_none(), "Score must be None when strength is missing");
    }
}
