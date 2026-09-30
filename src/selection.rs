use crate::material::Material;

#[derive(Clone, Debug)]
pub struct Constraints {
    pub max_density: Option<f64>,
    pub min_melting_point: Option<f64>,
    pub min_thermal_conductivity: Option<f64>,
    pub min_tensile_strength: Option<f64>,
    pub min_youngs_modulus: Option<f64>,
    pub max_thermal_expansion: Option<f64>,
}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            max_density: None,
            min_melting_point: None,
            min_thermal_conductivity: None,
            min_tensile_strength: None,
            min_youngs_modulus: None,
            max_thermal_expansion: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaterialScore {
    pub material: Material,
    pub score: f64,
    pub passed: bool,
    pub reasons: Vec<String>,
}

pub fn evaluate(
    material: &Material,
    constraints: &Constraints,
) -> MaterialScore {
    let mut passed = true;
    let mut reasons = Vec::new();

    if let Some(max) = constraints.max_density {
        match material.density {
            Some(value) if value <= max => {}
            Some(value) => {
                passed = false;

                reasons.push(format!(
                    "Density {:.2} > {:.2} g/cm³",
                    value, max
                ));
            }
            None => {
                passed = false;

                reasons.push(
                    "Density data unavailable".into()
                );
            }
        }
    }

    if let Some(min) = constraints.min_melting_point {
        match material.melting_point {
            Some(value) if value >= min => {}
            Some(value) => {
                passed = false;

                reasons.push(format!(
                    "Melting point {:.0} < {:.0} °C",
                    value, min
                ));
            }
            None => {
                passed = false;

                reasons.push(
                    "Melting point data unavailable".into()
                );
            }
        }
    }

    if let Some(min) = constraints.min_thermal_conductivity {
        match material.thermal_conductivity {
            Some(value) if value >= min => {}
            Some(value) => {
                passed = false;

                reasons.push(format!(
                    "Thermal conductivity {:.2} < {:.2}",
                    value, min
                ));
            }
            None => {
                passed = false;

                reasons.push(
                    "Thermal conductivity unavailable".into()
                );
            }
        }
    }

    if let Some(min) = constraints.min_tensile_strength {
        match material.tensile_strength {
            Some(value) if value >= min => {}
            Some(value) => {
                passed = false;

                reasons.push(format!(
                    "Tensile strength {:.0} < {:.0} MPa",
                    value, min
                ));
            }
            None => {
                passed = false;

                reasons.push(
                    "Tensile strength unavailable".into()
                );
            }
        }
    }

    if let Some(min) = constraints.min_youngs_modulus {
        match material.youngs_modulus {
            Some(value) if value >= min => {}
            Some(value) => {
                passed = false;

                reasons.push(format!(
                    "Young's modulus {:.0} < {:.0} GPa",
                    value, min
                ));
            }
            None => {
                passed = false;

                reasons.push(
                    "Young's modulus unavailable".into()
                );
            }
        }
    }

    if let Some(max) = constraints.max_thermal_expansion {
        match material.thermal_expansion {
            Some(value) if value <= max => {}
            Some(value) => {
                passed = false;

                reasons.push(format!(
                    "Thermal expansion {:.2} > {:.2}",
                    value, max
                ));
            }
            None => {
                passed = false;

                reasons.push(
                    "Thermal expansion unavailable".into()
                );
            }
        }
    }

    let score = calculate_score(
        material,
        constraints,
    );

    MaterialScore {
        material: material.clone(),
        score,
        passed,
        reasons,
    }
}

fn calculate_score(
    material: &Material,
    constraints: &Constraints,
) -> f64 {
    let mut total = 0.0;
    let mut factors = 0.0;

    if let Some(max) = constraints.max_density {
        if let Some(value) = material.density {
            if value > 0.0 && max > 0.0 {
                total += (max / value).min(1.0);
                factors += 1.0;
            }
        }
    }

    if let Some(min) = constraints.min_melting_point {
        if let Some(value) = material.melting_point {
            if min > 0.0 {
                total += (value / min).min(2.0) / 2.0;
                factors += 1.0;
            }
        }
    }

    if let Some(min) = constraints.min_thermal_conductivity {
        if let Some(value) = material.thermal_conductivity {
            if min > 0.0 {
                total += (value / min).min(2.0) / 2.0;
                factors += 1.0;
            }
        }
    }

    if let Some(min) = constraints.min_tensile_strength {
        if let Some(value) = material.tensile_strength {
            if min > 0.0 {
                total += (value / min).min(2.0) / 2.0;
                factors += 1.0;
            }
        }
    }

    if let Some(min) = constraints.min_youngs_modulus {
        if let Some(value) = material.youngs_modulus {
            if min > 0.0 {
                total += (value / min).min(2.0) / 2.0;
                factors += 1.0;
            }
        }
    }

    if let Some(max) = constraints.max_thermal_expansion {
        if let Some(value) = material.thermal_expansion {
            if value > 0.0 && max > 0.0 {
                total += (max / value).min(1.0);
                factors += 1.0;
            }
        }
    }

    if factors == 0.0 {
        0.0
    } else {
        total / factors
    }
}

pub fn rank_materials(
    materials: &[Material],
    constraints: &Constraints,
) -> Vec<MaterialScore> {
    let mut results: Vec<MaterialScore> = materials
        .iter()
        .map(|material| {
            evaluate(material, constraints)
        })
        .collect();

    results.sort_by(|a, b| {
        b.passed
            .cmp(&a.passed)
            .then_with(|| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(
                        std::cmp::Ordering::Equal
                    )
            })
    });

    results
}