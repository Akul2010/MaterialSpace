use crate::material::{Material, format_scientific};

/// User-defined constraints for multi-criteria materials screening.
#[derive(Debug, Clone, Default)]
pub struct Constraints {
    pub max_density: Option<f64>,
    pub min_thermal_conductivity: Option<f64>,
    pub max_thermal_expansion: Option<f64>,
    pub min_tensile_strength: Option<f64>,
    pub min_yield_strength: Option<f64>,
    pub min_flexural_strength: Option<f64>,
    pub min_youngs_modulus: Option<f64>,
    pub min_melting_point: Option<f64>,
    pub category: Option<String>,
}

impl Constraints {
    pub fn is_empty(&self) -> bool {
        self.max_density.is_none()
            && self.min_thermal_conductivity.is_none()
            && self.max_thermal_expansion.is_none()
            && self.min_tensile_strength.is_none()
            && self.min_yield_strength.is_none()
            && self.min_flexural_strength.is_none()
            && self.min_youngs_modulus.is_none()
            && self.min_melting_point.is_none()
            && self.category.is_none()
    }
}

/// Status of a material evaluated against screening constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreeningStatus {
    /// Satisfies all active constraints with documented data.
    Passed,
    /// Demonstrably violates one or more active constraints.
    Failed,
    /// Lacks required data for one or more active constraints (data unavailable).
    InsufficientData,
}

/// Detailed evaluation record for a screened material.
#[derive(Debug, Clone)]
pub struct ScreeningResult {
    pub material: Material,
    pub status: ScreeningStatus,
    /// Specific checks that passed (with actual vs required values).
    pub passed_reasons: Vec<String>,
    /// Specific constraints that were demonstrably violated.
    pub failed_reasons: Vec<String>,
    /// Active constraints that could not be evaluated due to missing property data.
    pub missing_reasons: Vec<String>,
}

/// Pre-configured engineering screening profiles representing typical application criteria.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreeningProfilePreset {
    AerospaceAirframe,
    ThermalProtection,
    HeatExchanger,
    DimensionalStability,
    HighTempRefractory,
}

impl ScreeningProfilePreset {
    pub fn all() -> &'static [ScreeningProfilePreset] {
        &[
            ScreeningProfilePreset::AerospaceAirframe,
            ScreeningProfilePreset::ThermalProtection,
            ScreeningProfilePreset::HeatExchanger,
            ScreeningProfilePreset::DimensionalStability,
            ScreeningProfilePreset::HighTempRefractory,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ScreeningProfilePreset::AerospaceAirframe => "Aerospace Airframe Structural",
            ScreeningProfilePreset::ThermalProtection => "Thermal Protection System (TPS)",
            ScreeningProfilePreset::HeatExchanger => "High-Efficiency Heat Exchanger",
            ScreeningProfilePreset::DimensionalStability => "Dimensional Stability / Space Optical",
            ScreeningProfilePreset::HighTempRefractory => "High-Temperature Refractory",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ScreeningProfilePreset::AerospaceAirframe => {
                "Screens for lightweight, high-strength structural materials: Density ≤ 4,500 kg/m³, \
                 Tensile Strength ≥ 300 MPa, Young's Modulus ≥ 60 GPa."
            }
            ScreeningProfilePreset::ThermalProtection => {
                "Screens for passive thermal insulation and re-entry heat shields: \
                 Thermal Conductivity ≤ 5.0 W/(m·K), Melting Point ≥ 1,600 K, Density ≤ 3,000 kg/m³."
            }
            ScreeningProfilePreset::HeatExchanger => {
                "Screens for high-flux thermal management components: \
                 Thermal Conductivity ≥ 80 W/(m·K), Melting Point ≥ 800 K."
            }
            ScreeningProfilePreset::DimensionalStability => {
                "Screens for precision satellite optical benches and laser platforms: \
                 Thermal Expansion (CTE) ≤ 5.0 µm/(m·K), Young's Modulus ≥ 50 GPa."
            }
            ScreeningProfilePreset::HighTempRefractory => {
                "Screens for extreme-temperature propulsion and furnace hardware: \
                 Melting Point ≥ 2,000 K, Young's Modulus ≥ 100 GPa."
            }
        }
    }

    /// Generates editable default constraints for this preset.
    pub fn default_constraints(&self) -> Constraints {
        match self {
            ScreeningProfilePreset::AerospaceAirframe => Constraints {
                max_density: Some(4500.0),
                min_tensile_strength: Some(300.0e6),
                min_youngs_modulus: Some(60.0e9),
                ..Default::default()
            },
            ScreeningProfilePreset::ThermalProtection => Constraints {
                max_density: Some(3000.0),
                min_thermal_conductivity: None,
                max_thermal_expansion: None,
                min_tensile_strength: None,
                min_yield_strength: None,
                min_flexural_strength: None,
                min_youngs_modulus: None,
                min_melting_point: Some(1600.0),
                category: None,
            },
            ScreeningProfilePreset::HeatExchanger => Constraints {
                min_thermal_conductivity: Some(80.0),
                min_melting_point: Some(800.0),
                ..Default::default()
            },
            ScreeningProfilePreset::DimensionalStability => Constraints {
                max_thermal_expansion: Some(5.0e-6),
                min_youngs_modulus: Some(50.0e9),
                ..Default::default()
            },
            ScreeningProfilePreset::HighTempRefractory => Constraints {
                min_melting_point: Some(2000.0),
                min_youngs_modulus: Some(100.0e9),
                ..Default::default()
            },
        }
    }
}

/// Evaluates a single material against the specified constraints.
///
/// **Scientific Honesty**: A missing property is NEVER treated as satisfying a constraint.
/// If required data is missing, the material receives `ScreeningStatus::InsufficientData`.
pub fn evaluate_material(material: &Material, constraints: &Constraints) -> ScreeningResult {
    let mut passed_reasons = Vec::new();
    let mut failed_reasons = Vec::new();
    let mut missing_reasons = Vec::new();

    // 1. Category Constraint
    if let Some(cat) = &constraints.category {
        if cat != "All" && !cat.is_empty() {
            if material.category.eq_ignore_ascii_case(cat) {
                passed_reasons.push(format!("Category matches '{}'", cat));
            } else {
                failed_reasons.push(format!(
                    "Category '{}' does not match '{}'",
                    material.category, cat
                ));
            }
        }
    }

    // 2. Maximum Density
    if let Some(max_rho) = constraints.max_density {
        match material.density_val() {
            Some(rho) => {
                if rho <= max_rho {
                    passed_reasons.push(format!(
                        "Density {} kg/m³ ≤ {} kg/m³ [PASS]",
                        format_scientific(rho),
                        format_scientific(max_rho)
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Density {} kg/m³ exceeds maximum {} kg/m³ [VIOLATION]",
                        format_scientific(rho),
                        format_scientific(max_rho)
                    ));
                }
            }
            None => {
                missing_reasons.push("Density data unavailable".into());
            }
        }
    }

    // 3. Minimum Thermal Conductivity
    if let Some(min_k) = constraints.min_thermal_conductivity {
        match material.thermal_conductivity_val() {
            Some(k) => {
                if k >= min_k {
                    passed_reasons.push(format!(
                        "Thermal conductivity {:.2} W/(m·K) ≥ {:.2} [PASS]",
                        k, min_k
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Thermal conductivity {:.2} W/(m·K) below minimum {:.2} [VIOLATION]",
                        k, min_k
                    ));
                }
            }
            None => {
                missing_reasons.push("Thermal conductivity data unavailable".into());
            }
        }
    }

    // 4. Maximum Thermal Expansion (CTE)
    if let Some(max_alpha) = constraints.max_thermal_expansion {
        match material.thermal_expansion_val() {
            Some(alpha) => {
                if alpha <= max_alpha {
                    passed_reasons.push(format!(
                        "Thermal expansion {} 1/K ≤ {} [PASS]",
                        format_scientific(alpha),
                        format_scientific(max_alpha)
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Thermal expansion {} 1/K exceeds maximum {} [VIOLATION]",
                        format_scientific(alpha),
                        format_scientific(max_alpha)
                    ));
                }
            }
            None => {
                missing_reasons.push("Thermal expansion data unavailable".into());
            }
        }
    }

    // 5. Minimum Tensile Strength (Explicitly Tensile)
    if let Some(min_ts) = constraints.min_tensile_strength {
        match material.tensile_strength_val() {
            Some(ts) => {
                let ts_mpa = ts / 1.0e6;
                let min_mpa = min_ts / 1.0e6;
                if ts >= min_ts {
                    passed_reasons.push(format!(
                        "Tensile strength {:.1} MPa ≥ {:.1} MPa [PASS]",
                        ts_mpa, min_mpa
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Tensile strength {:.1} MPa below minimum {:.1} MPa [VIOLATION]",
                        ts_mpa, min_mpa
                    ));
                }
            }
            None => {
                missing_reasons.push(
                    "Tensile strength data unavailable (flexural/yield data not substituted)"
                        .into(),
                );
            }
        }
    }

    // 6. Minimum Yield Strength (Explicitly Yield)
    if let Some(min_ys) = constraints.min_yield_strength {
        match material.yield_strength_val() {
            Some(ys) => {
                let ys_mpa = ys / 1.0e6;
                let min_mpa = min_ys / 1.0e6;
                if ys >= min_ys {
                    passed_reasons.push(format!(
                        "Yield strength {:.1} MPa ≥ {:.1} MPa [PASS]",
                        ys_mpa, min_mpa
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Yield strength {:.1} MPa below minimum {:.1} MPa [VIOLATION]",
                        ys_mpa, min_mpa
                    ));
                }
            }
            None => {
                missing_reasons.push("Yield strength data unavailable".into());
            }
        }
    }

    // 7. Minimum Flexural Strength (Explicitly Flexural)
    if let Some(min_fs) = constraints.min_flexural_strength {
        match material.flexural_strength_val() {
            Some(fs) => {
                let fs_mpa = fs / 1.0e6;
                let min_mpa = min_fs / 1.0e6;
                if fs >= min_fs {
                    passed_reasons.push(format!(
                        "Flexural strength {:.1} MPa ≥ {:.1} MPa [PASS]",
                        fs_mpa, min_mpa
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Flexural strength {:.1} MPa below minimum {:.1} MPa [VIOLATION]",
                        fs_mpa, min_mpa
                    ));
                }
            }
            None => {
                missing_reasons.push("Flexural strength data unavailable".into());
            }
        }
    }

    // 8. Minimum Young's Modulus
    if let Some(min_e) = constraints.min_youngs_modulus {
        match material.youngs_modulus_val() {
            Some(e) => {
                let e_gpa = e / 1.0e9;
                let min_gpa = min_e / 1.0e9;
                if e >= min_e {
                    passed_reasons.push(format!(
                        "Young's modulus {:.1} GPa ≥ {:.1} GPa [PASS]",
                        e_gpa, min_gpa
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Young's modulus {:.1} GPa below minimum {:.1} GPa [VIOLATION]",
                        e_gpa, min_gpa
                    ));
                }
            }
            None => {
                missing_reasons.push("Young's modulus data unavailable".into());
            }
        }
    }

    // 9. Minimum Melting Point
    if let Some(min_tm) = constraints.min_melting_point {
        match material.melting_point_val() {
            Some(tm) => {
                if tm >= min_tm {
                    passed_reasons.push(format!(
                        "Melting point {:.0} K ({:.0} °C) ≥ {:.0} K [PASS]",
                        tm,
                        tm - 273.15,
                        min_tm
                    ));
                } else {
                    failed_reasons.push(format!(
                        "Melting point {:.0} K ({:.0} °C) below minimum {:.0} K [VIOLATION]",
                        tm,
                        tm - 273.15,
                        min_tm
                    ));
                }
            }
            None => {
                missing_reasons.push("Melting point data unavailable".into());
            }
        }
    }

    // Determine final status
    let status = if !failed_reasons.is_empty() {
        ScreeningStatus::Failed
    } else if !missing_reasons.is_empty() {
        ScreeningStatus::InsufficientData
    } else {
        ScreeningStatus::Passed
    };

    ScreeningResult {
        material: material.clone(),
        status,
        passed_reasons,
        failed_reasons,
        missing_reasons,
    }
}

/// Screens all materials in the database against the specified constraints.
pub fn screen_materials(materials: &[Material], constraints: &Constraints) -> Vec<ScreeningResult> {
    materials
        .iter()
        .map(|mat| evaluate_material(mat, constraints))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{Material, Property};

    fn make_test_material(density: f64, tensile_strength: Option<f64>) -> Material {
        Material {
            id: "mat_a".into(),
            name: "Material A".into(),
            category: "metal".into(),
            composition: None,
            grade: None,
            description: None,
            source_ids: vec![],
            aliases: vec![],
            density: Some(Property {
                value: density,
                unit: Some("kg/m^3".into()),
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
            youngs_modulus: None,
            yield_strength: None,
            tensile_strength: tensile_strength.map(|ts| Property {
                value: ts,
                unit: Some("Pa".into()),
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
            flexural_strength: None,
            thermal_conductivity: None,
            specific_heat: None,
            thermal_expansion: None,
            emissivity: None,
            melting_point: None,
            boiling_point: None,
        }
    }

    #[test]
    fn test_screening_passed_and_failed() {
        let mat_passing = make_test_material(2700.0, Some(310.0e6));
        let mat_heavy = make_test_material(8000.0, Some(500.0e6));

        let constraints = Constraints {
            max_density: Some(3000.0),
            min_tensile_strength: Some(300.0e6),
            ..Default::default()
        };

        let res_passing = evaluate_material(&mat_passing, &constraints);
        assert_eq!(res_passing.status, ScreeningStatus::Passed);
        assert_eq!(res_passing.passed_reasons.len(), 2);
        assert!(res_passing.failed_reasons.is_empty());

        let res_heavy = evaluate_material(&mat_heavy, &constraints);
        assert_eq!(res_heavy.status, ScreeningStatus::Failed);
        assert_eq!(res_heavy.failed_reasons.len(), 1);
        assert!(res_heavy.failed_reasons[0].contains("exceeds maximum"));
    }

    #[test]
    fn test_screening_insufficient_data() {
        // Material has density but missing tensile strength
        let mat_incomplete = make_test_material(2700.0, None);

        let constraints = Constraints {
            max_density: Some(3000.0),
            min_tensile_strength: Some(300.0e6),
            ..Default::default()
        };

        let res = evaluate_material(&mat_incomplete, &constraints);
        assert_eq!(res.status, ScreeningStatus::InsufficientData);
        assert_eq!(res.missing_reasons.len(), 1);
        assert!(res.failed_reasons.is_empty());
    }

    #[test]
    fn test_constraints_is_empty() {
        let empty = Constraints::default();
        assert!(empty.is_empty());

        let non_empty = Constraints {
            max_density: Some(3000.0),
            ..Default::default()
        };
        assert!(!non_empty.is_empty());
    }
}
