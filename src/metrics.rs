use crate::material::{Material, format_scientific};

/// Tensile strength per unit of density.
///
/// This uses actual tensile-strength data: flexural strength isn't a stand-in
/// for a tensile test, so brittle materials without tensile data return `None`.
///
pub fn specific_tensile_strength(material: &Material) -> Option<f64> {
    let tensile_strength = material.tensile_strength_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(tensile_strength / density)
}

/// Yield strength per unit of density.
///
/// This is handy for comparing materials when permanent deformation matters.
pub fn specific_yield_strength(material: &Material) -> Option<f64> {
    let yield_strength = material.yield_strength_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(yield_strength / density)
}

/// Flexural strength per unit of density.
///
/// This is based on bending tests, which are often used for ceramics and glass.
pub fn specific_flexural_strength(material: &Material) -> Option<f64> {
    let flexural_strength = material.flexural_strength_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(flexural_strength / density)
}

/// Stiffness per unit of density.
///
/// It compares how stiff a material is for its weight.
pub fn specific_stiffness(material: &Material) -> Option<f64> {
    let youngs_modulus = material.youngs_modulus_val()?;
    let density = material.density_val()?;

    if density <= 0.0 {
        return None;
    }

    Some(youngs_modulus / density)
}

/// Works out how quickly a temperature change spreads through a material.
///
/// α_diff = k / (ρ · c_p)
///
/// Where:
/// - k = Thermal conductivity (W/(m·K))
/// - ρ = Density (kg/m³)
/// - c_p = Specific heat capacity (J/(kg·K))
///
/// Quantifies the rate at which temperature transients diffuse through a material.
/// Expressed in m²/s (often displayed in mm²/s by multiplying by 10⁶).
pub fn thermal_diffusivity(material: &Material) -> Option<f64> {
    let k = material.thermal_conductivity_val()?;
    let rho = material.density_val()?;
    let cp = material.specific_heat_val()?;

    if k < 0.0 || rho <= 0.0 || cp <= 0.0 {
        return None;
    }

    let diffusivity = k / (rho * cp);
    Some(diffusivity)
}

/// The score and a few details about how it was worked out.
#[derive(Debug, Clone)]
pub struct AerospaceScoreResult {
    /// Final heuristic score on a 0.0 to 100.0 scale.
    pub score: f64,
    /// Fraction of evaluable criteria that had valid data (0.0 to 1.0).
    pub data_completeness: f32,
    /// Number of properties that contributed to the score.
    pub properties_used: usize,
    /// Total properties evaluated by the heuristic model.
    pub total_properties: usize,
    /// Specific engineering notes or caveats (e.g. regarding brittleness).
    pub caveats: Vec<String>,
}

/// A rough score for comparing materials against a few aerospace-related properties.
/// It isn't a qualification or a substitute for engineering review.
///
/// It combines specific strength (40%), specific stiffness (35%), melting point
/// (15%), and thermal conductivity (10%). The score uses the same ranges as before:
/// 20-220 kN·m/kg for strength, 5-35 million m²/s² for stiffness, 500-2500 K
/// for melting point, and 0-200 W/(m·K) for conductivity. Missing strength or
/// density means there's no score.
pub fn aerospace_suitability_score(material: &Material) -> Option<AerospaceScoreResult> {
    let density = material.density_val()?;
    // Prefers tensile strength, transparently notes when using yield strength
    let (strength, is_yield) = match material.tensile_strength_val() {
        Some(ts) => (ts, false),
        None => match material.yield_strength_val() {
            Some(ys) => (ys, true),
            None => return None,
        },
    };

    if density <= 0.0 {
        return None;
    }

    let mut score = 0.0;
    let mut total_weight = 0.0;
    let mut properties_used = 0;
    let total_properties = 4;
    let mut caveats = Vec::new();

    if is_yield {
        caveats.push("Tensile strength unavailable; score utilizes yield strength.".into());
    }

    // 1. Specific Strength (Weight = 40.0)
    let spec_str = strength / density;
    let norm_spec_str = ((spec_str - 20_000.0) / (220_000.0 - 20_000.0)).clamp(0.0, 1.0);
    score += norm_spec_str * 40.0;
    total_weight += 40.0;
    properties_used += 1;

    // 2. Specific Stiffness (Weight = 35.0)
    if let Some(e) = material.youngs_modulus_val() {
        let spec_stiff = e / density;
        let norm_stiff = ((spec_stiff - 5.0e6) / (35.0e6 - 5.0e6)).clamp(0.0, 1.0);
        score += norm_stiff * 35.0;
        total_weight += 35.0;
        properties_used += 1;
    } else {
        caveats.push("Young's modulus unavailable for specific stiffness evaluation.".into());
    }

    // 3. Melting Point (Weight = 15.0)
    if let Some(tm) = material.melting_point_val() {
        let norm_temp = ((tm - 500.0) / (2500.0 - 500.0)).clamp(0.0, 1.0);
        score += norm_temp * 15.0;
        total_weight += 15.0;
        properties_used += 1;
    } else {
        caveats.push("Melting point unavailable for thermal capability evaluation.".into());
    }

    // 4. Thermal Conductivity (Weight = 10.0)
    if let Some(k) = material.thermal_conductivity_val() {
        let norm_k = (k / 200.0).clamp(0.0, 1.0);
        score += norm_k * 10.0;
        total_weight += 10.0;
        properties_used += 1;
    } else {
        caveats.push("Thermal conductivity unavailable.".into());
    }

    // Special engineering notes for specific materials
    if material.id == "diamond_cvd" {
        caveats.push(
            "Note: CVD diamond exhibits extraordinary stiffness and thermal conductivity, \
             yielding a very high heuristic score; however, it has near-zero fracture toughness \
             and oxidizes above ~650 °C in air, limiting macro-structural aerospace viability."
                .into(),
        );
    }

    if total_weight == 0.0 {
        return None;
    }

    let final_score = (score / total_weight) * 100.0;
    let data_completeness = properties_used as f32 / total_properties as f32;

    Some(AerospaceScoreResult {
        score: final_score.clamp(0.0, 100.0),
        data_completeness,
        properties_used,
        total_properties,
        caveats,
    })
}

/// Generates engineering observations using thresholds applied to recorded properties.
pub fn generate_observations(material: &Material) -> Vec<String> {
    let mut observations = Vec::new();

    // Density
    if let Some(rho) = material.density_val() {
        if rho < 2000.0 {
            observations.push(format!(
                "Ultralight structural candidate (Density: {} kg/m³)",
                format_scientific(rho)
            ));
        } else if rho < 3000.0 {
            observations.push(format!(
                "Relatively low density ({} kg/m³) — attractive for weight-critical structural design",
                format_scientific(rho)
            ));
        } else if rho > 8000.0 {
            observations.push(format!(
                "High density ({} kg/m³) — heavy structural alloy or ballast candidate",
                format_scientific(rho)
            ));
        }
    }

    // Specific tensile strength
    if let Some(spec_str) = specific_tensile_strength(material) {
        let spec_str_kn = spec_str / 1000.0;
        if spec_str_kn > 150.0 {
            observations.push(format!(
                "Exceptional tensile strength-to-weight ratio ({:.1} kN·m/kg) — high structural efficiency",
                spec_str_kn
            ));
        } else if spec_str_kn > 80.0 {
            observations.push(format!(
                "High tensile strength-to-weight potential ({:.1} kN·m/kg)",
                spec_str_kn
            ));
        }
    }

    // Flexural strength
    if let Some(flex) = material.flexural_strength_val() {
        let flex_mpa = flex / 1.0e6;
        observations.push(format!(
            "Characterized by flexural strength / modulus of rupture ({:.1} MPa) rather than uniaxial tension",
            flex_mpa
        ));
    }

    // Specific stiffness
    if let Some(spec_stiff) = specific_stiffness(material) {
        let spec_stiff_m = spec_stiff / 1.0e6;
        if spec_stiff_m > 25.0 {
            observations.push(format!(
                "High specific stiffness ({:.1} GPa/(g/cm³)) — excellent resistance to elastic deflection and aeroelastic flutter",
                spec_stiff_m
            ));
        }
    }

    // Thermal conductivity
    if let Some(k) = material.thermal_conductivity_val() {
        if k > 100.0 {
            observations.push(format!(
                "High thermal conductivity ({:.1} W/(m·K)) — suitable for heat sinks, spreaders, and thermal management",
                k
            ));
        } else if k < 2.0 {
            observations.push(format!(
                "Very low thermal conductivity ({:.2} W/(m·K)) — effective thermal insulator or barrier",
                k
            ));
        }
    }

    // Thermal expansion
    if let Some(alpha) = material.thermal_expansion_val() {
        let alpha_ppm = alpha * 1.0e6;
        if alpha.abs() < 2.0e-6 {
            observations.push(format!(
                "Near-zero thermal expansion ({:.2} µm/(m·K)) — exceptional dimensional stability over thermal cycling",
                alpha_ppm
            ));
        } else if alpha < 0.0 {
            observations.push(format!(
                "Negative coefficient of thermal expansion ({:.2} µm/(m·K)) — contracts along axis when heated",
                alpha_ppm
            ));
        } else if alpha_ppm > 20.0 {
            observations.push(format!(
                "High thermal expansion ({:.1} µm/(m·K)) — vulnerable to thermal strain under temperature gradients",
                alpha_ppm
            ));
        }
    }

    // Melting Point
    if let Some(tm) = material.melting_point_val() {
        let tm_c = tm - 273.15;
        if tm > 2500.0 {
            observations.push(format!(
                "Ultra-high temperature refractory capability (Melting point: {:.0} K / {:.0} °C). Note: Continuous operating limit is lower due to creep and oxidation.",
                tm, tm_c
            ));
        } else if tm > 1600.0 {
            observations.push(format!(
                "High temperature phase stability (Melting point: {:.0} K / {:.0} °C)",
                tm, tm_c
            ));
        }
    }

    if observations.is_empty() {
        observations.push(
            "Limited property data available to derive engineering observations.".to_string(),
        );
    }

    observations
}

/// Supported criteria for material rankings.
///
/// Every strength criterion is explicit; tensile and yield strengths are never silently conflated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingMetric {
    AerospaceScore,
    SpecificTensileStrength,
    SpecificYieldStrength,
    SpecificFlexuralStrength,
    TensileStrength,
    YieldStrength,
    FlexuralStrength,
    SpecificStiffness,
    ThermalConductivity,
    ThermalDiffusivity,
    LowestDensity,
    MeltingPoint,
}

impl RankingMetric {
    pub fn all() -> &'static [RankingMetric] {
        &[
            RankingMetric::AerospaceScore,
            RankingMetric::SpecificTensileStrength,
            RankingMetric::SpecificYieldStrength,
            RankingMetric::SpecificFlexuralStrength,
            RankingMetric::TensileStrength,
            RankingMetric::YieldStrength,
            RankingMetric::FlexuralStrength,
            RankingMetric::SpecificStiffness,
            RankingMetric::ThermalConductivity,
            RankingMetric::ThermalDiffusivity,
            RankingMetric::LowestDensity,
            RankingMetric::MeltingPoint,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            RankingMetric::AerospaceScore => "Aerospace Suitability (Heuristic Index)",
            RankingMetric::SpecificTensileStrength => "Specific Tensile Strength (UTS / Density)",
            RankingMetric::SpecificYieldStrength => "Specific Yield Strength (Yield / Density)",
            RankingMetric::SpecificFlexuralStrength => {
                "Specific Flexural Strength (Flex / Density)"
            }
            RankingMetric::TensileStrength => "Ultimate Tensile Strength (UTS)",
            RankingMetric::YieldStrength => "Yield Strength (σ_y)",
            RankingMetric::FlexuralStrength => "Flexural Strength (Modulus of Rupture)",
            RankingMetric::SpecificStiffness => "Specific Stiffness (E / Density)",
            RankingMetric::ThermalConductivity => "Thermal Conductivity (k)",
            RankingMetric::ThermalDiffusivity => "Thermal Diffusivity (α_diff)",
            RankingMetric::LowestDensity => "Lowest Density (Lightweight)",
            RankingMetric::MeltingPoint => "Melting / Solidus Point (Phase Boundary)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RankingMetric::AerospaceScore => {
                "Multi-criteria heuristic index (0-100) weighting specific tensile strength (40%), \
                 specific stiffness (35%), melting point (15%), and thermal conductivity (10%). (Not an official standard)."
            }
            RankingMetric::SpecificTensileStrength => {
                "Ultimate tensile strength per unit mass (σ_uts / ρ). Higher values denote superior structural load efficiency. \
                 Materials without genuine tensile data are excluded."
            }
            RankingMetric::SpecificYieldStrength => {
                "Yield strength per unit mass (σ_y / ρ). Measures capacity to resist plastic deformation per unit weight."
            }
            RankingMetric::SpecificFlexuralStrength => {
                "Flexural / bending strength per unit mass (σ_flex / ρ). Crucial for brittle ceramics and glasses in bending."
            }
            RankingMetric::TensileStrength => {
                "Maximum nominal tensile stress sustained before fracture (Pa / MPa)."
            }
            RankingMetric::YieldStrength => {
                "Stress at which onset of permanent plastic deformation occurs (Pa / MPa)."
            }
            RankingMetric::FlexuralStrength => {
                "Maximum stress sustained in 3-point or 4-point bending test before fracture (Pa / MPa)."
            }
            RankingMetric::SpecificStiffness => {
                "Elastic modulus per unit weight (E / ρ). Governs resistance to elastic deflection and aeroelastic flutter."
            }
            RankingMetric::ThermalConductivity => {
                "Rate of steady-state heat conduction (W/(m·K)). Higher values indicate heat spreaders; lower denote insulation."
            }
            RankingMetric::ThermalDiffusivity => {
                "Speed of temperature equalization across a material: α = k / (ρ · c_p) (mm²/s)."
            }
            RankingMetric::LowestDensity => {
                "Material mass per unit volume (kg/m³), ranked from lowest to highest mass."
            }
            RankingMetric::MeltingPoint => {
                "Melting / solidus phase change temperature in Kelvin (K). Denotes the thermodynamic upper boundary."
            }
        }
    }

    /// Evaluates the ranking value and human-readable string for a material.
    ///
    /// Returns `Some((sort_value, formatted_value, note))` or `None` if the material
    /// lacks the required property for this metric.
    pub fn evaluate(&self, material: &Material) -> Option<(f64, String, Option<String>)> {
        match self {
            RankingMetric::AerospaceScore => {
                let res = aerospace_suitability_score(material)?;
                let note = if res.data_completeness < 1.0 {
                    Some(format!("{}/4 properties", res.properties_used))
                } else {
                    None
                };
                Some((res.score, format!("{:.1} / 100", res.score), note))
            }
            RankingMetric::SpecificTensileStrength => {
                let val = specific_tensile_strength(material)?;
                let kn = val / 1000.0;
                Some((val, format!("{:.2} kN·m/kg", kn), None))
            }
            RankingMetric::SpecificYieldStrength => {
                let val = specific_yield_strength(material)?;
                let kn = val / 1000.0;
                Some((val, format!("{:.2} kN·m/kg", kn), None))
            }
            RankingMetric::SpecificFlexuralStrength => {
                let val = specific_flexural_strength(material)?;
                let kn = val / 1000.0;
                Some((val, format!("{:.2} kN·m/kg", kn), None))
            }
            RankingMetric::TensileStrength => {
                let val = material.tensile_strength_val()?;
                let mpa = val / 1.0e6;
                Some((val, format!("{:.1} MPa", mpa), None))
            }
            RankingMetric::YieldStrength => {
                let val = material.yield_strength_val()?;
                let mpa = val / 1.0e6;
                Some((val, format!("{:.1} MPa", mpa), None))
            }
            RankingMetric::FlexuralStrength => {
                let val = material.flexural_strength_val()?;
                let mpa = val / 1.0e6;
                Some((val, format!("{:.1} MPa", mpa), None))
            }
            RankingMetric::SpecificStiffness => {
                let val = specific_stiffness(material)?;
                let spec_stiff_m = val / 1.0e6;
                Some((val, format!("{:.2} GPa/(g/cm³)", spec_stiff_m), None))
            }
            RankingMetric::ThermalConductivity => {
                let val = material.thermal_conductivity_val()?;
                Some((val, format!("{:.2} W/(m·K)", val), None))
            }
            RankingMetric::ThermalDiffusivity => {
                let val = thermal_diffusivity(material)?;
                let val_mm2_s = val * 1.0e6;
                Some((val, format!("{:.3} mm²/s", val_mm2_s), None))
            }
            RankingMetric::LowestDensity => {
                let val = material.density_val()?;
                // Negated for sorting so lowest density ranks #1
                Some((-val, format!("{:.1} kg/m³", val), None))
            }
            RankingMetric::MeltingPoint => {
                let val = material.melting_point_val()?;
                Some((val, format!("{:.0} K ({:.0} °C)", val, val - 273.15), None))
            }
        }
    }
}

/// A ranked material entry for the UI and exports.
#[derive(Debug, Clone)]
pub struct RankedMaterialItem {
    pub material_index: usize,
    pub name: String,
    pub category: String,
    pub sort_value: f64,
    pub formatted_value: String,
    pub note: Option<String>,
}

/// Evaluates and ranks materials according to the chosen metric.
///
/// Ineligible candidates (materials lacking required properties) are excluded,
/// and never treated as zero.
pub fn rank_materials(materials: &[Material], metric: RankingMetric) -> Vec<RankedMaterialItem> {
    let mut ranked = Vec::new();

    for (index, material) in materials.iter().enumerate() {
        if let Some((sort_value, formatted_value, note)) = metric.evaluate(material) {
            ranked.push(RankedMaterialItem {
                material_index: index,
                name: material.name.clone(),
                category: material.category_display().to_string(),
                sort_value,
                formatted_value,
                note,
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

    fn make_test_alloy() -> Material {
        Material {
            id: "test_alloy".to_string(),
            name: "Test Alloy".to_string(),
            category: "metal".to_string(),
            composition: None,
            grade: None,
            description: None,
            source_ids: vec!["matweb".to_string()],
            aliases: vec![],
            density: Some(Property {
                value: 2700.0,
                unit: Some("kg/m^3".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(2700.0),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            youngs_modulus: Some(Property {
                value: 70.0e9,
                unit: Some("Pa".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(70.0e9),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            yield_strength: Some(Property {
                value: 280.0e6,
                unit: Some("Pa".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(280.0e6),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            tensile_strength: Some(Property {
                value: 310.0e6,
                unit: Some("Pa".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(310.0e6),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            flexural_strength: None,
            thermal_conductivity: Some(Property {
                value: 150.0,
                unit: Some("W/(m*K)".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(150.0),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            specific_heat: Some(Property {
                value: 900.0,
                unit: Some("J/(kg*K)".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(900.0),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            thermal_expansion: Some(Property {
                value: 23.0e-6,
                unit: Some("1/K".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(23.0e-6),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            emissivity: None,
            melting_point: Some(Property {
                value: 933.0,
                unit: Some("K".to_string()),
                source_id: Some("matweb".to_string()),
                condition: None,
                test_method: None,
                temperature: None,
                original_value: Some(933.0),
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            boiling_point: None,
        }
    }

    fn make_test_ceramic() -> Material {
        Material {
            id: "alumina_test".to_string(),
            name: "Alumina Test".to_string(),
            category: "ceramic".to_string(),
            composition: None,
            grade: None,
            description: None,
            source_ids: vec!["matweb".to_string()],
            aliases: vec![],
            density: Some(Property {
                value: 3900.0,
                unit: Some("kg/m^3".to_string()),
                source_id: None,
                condition: None,
                test_method: None,
                temperature: None,
                original_value: None,
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            youngs_modulus: Some(Property {
                value: 370.0e9,
                unit: Some("Pa".to_string()),
                source_id: None,
                condition: None,
                test_method: None,
                temperature: None,
                original_value: None,
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            yield_strength: None,
            tensile_strength: None, // No tensile strength!
            flexural_strength: Some(Property {
                value: 350.0e6, // 350 MPa flexural
                unit: Some("Pa".to_string()),
                source_id: None,
                condition: Some("3-point bend".into()),
                test_method: None,
                temperature: None,
                original_value: None,
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            thermal_conductivity: Some(Property {
                value: 30.0,
                unit: Some("W/(m*K)".to_string()),
                source_id: None,
                condition: None,
                test_method: None,
                temperature: None,
                original_value: None,
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            specific_heat: None,
            thermal_expansion: None,
            emissivity: None,
            melting_point: Some(Property {
                value: 2345.0,
                unit: Some("K".to_string()),
                source_id: None,
                condition: None,
                test_method: None,
                temperature: None,
                original_value: None,
                original_unit: None,
                uncertainty: None,
                notes: None,
                normalization_warning: None,
            }),
            boiling_point: None,
        }
    }

    #[test]
    fn test_brittle_material_not_in_tensile_ranking() {
        let ceramic = make_test_ceramic();
        // Specific tensile strength MUST be None because tensile strength is None!
        assert!(specific_tensile_strength(&ceramic).is_none());

        // But specific flexural strength should be valid
        let spec_flex =
            specific_flexural_strength(&ceramic).expect("should compute specific flexural");
        let expected = 350.0e6 / 3900.0;
        assert!((spec_flex - expected).abs() < 1e-4);

        // In ranking under SpecificTensileStrength, the ceramic must be excluded
        let materials = vec![ceramic];
        let ranked = rank_materials(&materials, RankingMetric::SpecificTensileStrength);
        assert_eq!(
            ranked.len(),
            0,
            "Brittle ceramic must not appear in tensile strength ranking"
        );

        // In ranking under SpecificFlexuralStrength, it must be ranked
        let ranked_flex = rank_materials(&materials, RankingMetric::SpecificFlexuralStrength);
        assert_eq!(ranked_flex.len(), 1);
    }

    #[test]
    fn test_specific_strength_known_case() {
        let mat = make_test_alloy();
        // Tensile strength = 310 MPa, Density = 2700 kg/m³
        // Specific strength = 310,000,000 / 2700 ≈ 114,814.81 (N*m)/kg
        let spec_str = specific_tensile_strength(&mat).expect("should compute specific tensile");
        let expected = 310.0e6 / 2700.0;
        assert!((spec_str - expected).abs() < 0.1);
    }

    #[test]
    fn test_thermal_diffusivity_known_case() {
        let mat = make_test_alloy();
        // k = 150 W/(m*K), rho = 2700 kg/m³, cp = 900 J/(kg*K)
        // alpha = 150 / (2700 * 900) = 6.172839e-5 m²/s
        let diff = thermal_diffusivity(&mat).expect("should compute thermal diffusivity");
        let expected = 150.0 / (2700.0 * 900.0);
        assert!((diff - expected).abs() < 1e-9);
    }

    #[test]
    fn test_aerospace_suitability_score_valid_and_bounds() {
        let mat = make_test_alloy();
        let res =
            aerospace_suitability_score(&mat).expect("should produce score for valid material");
        assert!(
            res.score >= 0.0 && res.score <= 100.0,
            "Score {} must be between 0 and 100",
            res.score
        );
        assert_eq!(res.properties_used, 4);
        assert_eq!(res.data_completeness, 1.0);
    }

    #[test]
    fn test_aerospace_suitability_score_missing_data() {
        let mut mat = make_test_alloy();
        mat.density = None;
        assert!(aerospace_suitability_score(&mat).is_none());

        let mut mat2 = make_test_alloy();
        mat2.tensile_strength = None;
        mat2.yield_strength = None;
        assert!(aerospace_suitability_score(&mat2).is_none());
    }
}
