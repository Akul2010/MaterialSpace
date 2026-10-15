use crate::material::Material;

/// Warning produced by physics calculation sanity checks.
#[derive(Debug, Clone, PartialEq)]
pub enum PhysicsWarning {
    /// The final temperature exceeds the material's melting or decomposition point.
    MeltingPointExceeded {
        final_temperature_k: f64,
        melting_point_k: f64,
    },
    /// The temperature delta exceeds the documented measurement range of the property.
    ExtrapolationWarning {
        condition: String,
        delta_temperature_k: f64,
    },
    /// The induced thermal stress exceeds the material's yield strength.
    YieldStressExceeded {
        thermal_stress_mpa: f64,
        yield_strength_mpa: f64,
        ratio_percent: f64,
    },
}

impl PhysicsWarning {
    pub fn message(&self) -> String {
        match self {
            PhysicsWarning::MeltingPointExceeded {
                final_temperature_k,
                melting_point_k,
            } => {
                format!(
                    "⚠️ Physical Sanity Warning: Implied temperature ({:.1} K / {:.1} °C) exceeds material melting point ({:.1} K / {:.1} °C)! Melting or thermal decomposition will occur.",
                    final_temperature_k,
                    final_temperature_k - 273.15,
                    melting_point_k,
                    melting_point_k - 273.15
                )
            }
            PhysicsWarning::ExtrapolationWarning {
                condition,
                delta_temperature_k,
            } => {
                format!(
                    "⚠️ Extrapolation Warning: Property coefficient was documented under condition '{}', but calculation applies ΔT = {:.1} K. Coefficients may vary nonlinearly at elevated temperatures.",
                    condition, delta_temperature_k
                )
            }
            PhysicsWarning::YieldStressExceeded {
                thermal_stress_mpa,
                yield_strength_mpa,
                ratio_percent,
            } => {
                format!(
                    "⚠️ Yield Exceedance Warning: Constrained thermal stress ({:.1} MPa) exceeds yield strength ({:.1} MPa) by {:.1}%! Plastic deformation or fracture expected.",
                    thermal_stress_mpa, yield_strength_mpa, ratio_percent
                )
            }
        }
    }
}

// ----------------------------------------------------------------------------
// 1. Linear Thermal Expansion
// ----------------------------------------------------------------------------

/// Calculates linear thermal expansion:
///
/// ΔL = α · L₀ · ΔT
///
/// Where:
/// - α (alpha) = Linear coefficient of thermal expansion (1/K)
/// - L₀ = Initial length (meters)
/// - ΔT (delta T) = Change in temperature (Kelvin or °C)
///
/// Returns: Change in length ΔL in meters (m), or `None` if α is unavailable.
///
/// ### Engineering Assumptions:
/// 1. Unconstrained, free expansion in 1D.
/// 2. Isotropic material behavior (expansion is identical in all directions).
/// 3. Constant coefficient of thermal expansion (α) over the specified ΔT interval.
pub fn thermal_expansion(
    material: &Material,
    initial_length: f64,
    delta_temperature: f64,
) -> Option<f64> {
    let alpha = material.thermal_expansion_val()?;
    Some(thermal_expansion_raw(
        alpha,
        initial_length,
        delta_temperature,
    ))
}

/// Raw calculation of linear thermal expansion: ΔL = α · L₀ · ΔT
pub fn thermal_expansion_raw(alpha: f64, initial_length: f64, delta_temperature: f64) -> f64 {
    alpha * initial_length * delta_temperature
}

// ----------------------------------------------------------------------------
// 2. Sensible Heat Energy
// ----------------------------------------------------------------------------

/// Calculates thermal heat energy transferred:
///
/// Q = m · c · ΔT
///
/// Where:
/// - m = Mass of the object (kg)
/// - c = Specific heat capacity (J/(kg·K))
/// - ΔT = Temperature change (Kelvin or °C)
///
/// Returns: Heat energy Q in Joules (J).
///
/// ### Engineering Assumptions:
/// 1. Constant specific heat capacity over the temperature interval.
/// 2. No latent heat / phase changes (solid-liquid-gas) occur.
/// 3. Negligible heat loss to surrounding environment during the process.
pub fn heat_energy(mass: f64, specific_heat: f64, delta_temperature: f64) -> f64 {
    mass * specific_heat * delta_temperature
}

// ----------------------------------------------------------------------------
// 3. Fully Constrained Thermal Stress
// ----------------------------------------------------------------------------

/// Calculates 1D thermal stress under fully constrained boundary conditions:
///
/// σ = E · α · ΔT
///
/// Where:
/// - E = Young's modulus of elasticity (Pa)
/// - α = Linear coefficient of thermal expansion (1/K)
/// - ΔT = Temperature change (K)
///
/// Returns: Thermal stress σ in Pascals (Pa), or `None` if E or α is missing.
///
/// ### Engineering Assumptions:
/// 1. Fully constrained uniaxial bar (rigid ends, axial strain ε = 0).
/// 2. Linear elastic behavior (Hooke's law: σ = E · ε_thermal).
/// 3. Homogeneous and isotropic material.
/// 4. No structural buckling occurs under compressive thermal load.
pub fn thermal_stress(material: &Material, delta_temperature: f64) -> Option<f64> {
    let youngs_modulus = material.youngs_modulus_val()?;
    let alpha = material.thermal_expansion_val()?;
    Some(thermal_stress_raw(youngs_modulus, alpha, delta_temperature))
}

/// Raw calculation of 1D fully constrained thermal stress: σ = E · α · ΔT
pub fn thermal_stress_raw(youngs_modulus: f64, alpha: f64, delta_temperature: f64) -> f64 {
    youngs_modulus * alpha * delta_temperature
}

// ----------------------------------------------------------------------------
// 4. Steady-State 1D Heat Conduction (Fourier's Law)
// ----------------------------------------------------------------------------

/// Calculates 1D steady-state heat conduction across a plane wall:
///
/// q = (k · A · ΔT) / L
/// q'' = q / A = (k · ΔT) / L
///
/// Where:
/// - k = Thermal conductivity (W/(m·K))
/// - A = Cross-sectional heat transfer area (m²)
/// - L = Thickness / conduction path length (m)
/// - ΔT = Temperature difference across wall (K)
///
/// Returns: `Some((q_watts, heat_flux_w_m2))` or `None` if k is missing or L <= 0.
///
/// ### Engineering Assumptions:
/// 1. One-dimensional, steady-state heat transfer.
/// 2. Constant thermal conductivity (k) across the thickness.
/// 3. Isothermal plane surfaces with uniform boundary temperatures.
/// 4. No internal volumetric heat generation.
pub fn heat_conduction(
    material: &Material,
    area_m2: f64,
    thickness_m: f64,
    delta_temperature_k: f64,
) -> Option<(f64, f64)> {
    let k = material.thermal_conductivity_val()?;
    heat_conduction_raw(k, area_m2, thickness_m, delta_temperature_k)
}

/// Raw calculation of 1D steady-state heat conduction: returns `(q_watts, heat_flux_w_m2)`
pub fn heat_conduction_raw(
    k: f64,
    area_m2: f64,
    thickness_m: f64,
    delta_temperature_k: f64,
) -> Option<(f64, f64)> {
    if thickness_m <= 0.0 || area_m2 <= 0.0 {
        return None;
    }
    let heat_flux = (k * delta_temperature_k) / thickness_m;
    let total_heat_rate = heat_flux * area_m2;
    Some((total_heat_rate, heat_flux))
}

// ----------------------------------------------------------------------------
// 5. Thermal Diffusion Timescale Estimate
// ----------------------------------------------------------------------------

/// Estimates the characteristic timescale for transient thermal diffusion
/// across a characteristic thickness L:
///
/// τ ≈ L² / α_diff = (L² · ρ · c_p) / k
///
/// Where:
/// - L = Characteristic diffusion length / thickness (m)
/// - α_diff = Thermal diffusivity = k / (ρ · c_p) (m²/s)
/// - k = Thermal conductivity (W/(m·K))
/// - ρ = Density (kg/m³)
/// - c_p = Specific heat capacity (J/(kg·K))
///
/// Returns: Characteristic timescale τ in seconds (s), or `None` if required data is missing.
///
/// ### Engineering Significance:
/// This order-of-magnitude timescale indicates how rapidly thermal equilibrium
/// is approached across a component (e.g. TPS heat shield or heat sink wall).
pub fn thermal_diffusion_timescale(material: &Material, thickness_m: f64) -> Option<f64> {
    let k = material.thermal_conductivity_val()?;
    let rho = material.density_val()?;
    let cp = material.specific_heat_val()?;

    thermal_diffusion_timescale_raw(k, rho, cp, thickness_m)
}

/// Raw calculation of characteristic thermal diffusion timescale τ ≈ (L² · ρ · c_p) / k
pub fn thermal_diffusion_timescale_raw(k: f64, rho: f64, cp: f64, thickness_m: f64) -> Option<f64> {
    if k <= 0.0 || rho <= 0.0 || cp <= 0.0 || thickness_m <= 0.0 {
        return None;
    }
    let alpha_diff = k / (rho * cp);
    Some((thickness_m * thickness_m) / alpha_diff)
}

// ----------------------------------------------------------------------------
// Sanity Check Evaluation
// ----------------------------------------------------------------------------

/// Checks a calculation for physical sanity against material properties.
pub fn evaluate_sanity_checks(
    material: &Material,
    delta_temp: f64,
    ambient_temp_k: f64,
    thermal_stress_pa: Option<f64>,
) -> Vec<PhysicsWarning> {
    let mut warnings = Vec::new();
    let final_temp = ambient_temp_k + delta_temp;

    // 1. Melting point check
    if let Some(tm) = material.melting_point_val() {
        if final_temp > tm {
            warnings.push(PhysicsWarning::MeltingPointExceeded {
                final_temperature_k: final_temp,
                melting_point_k: tm,
            });
        }
    }

    // 2. Extrapolation warning based on condition string
    if let Some(prop) = &material.thermal_expansion {
        if let Some(cond) = &prop.condition {
            if delta_temp.abs() > 300.0 {
                warnings.push(PhysicsWarning::ExtrapolationWarning {
                    condition: cond.clone(),
                    delta_temperature_k: delta_temp,
                });
            }
        }
    }

    // 3. Thermal stress vs yield strength check
    if let (Some(stress_pa), Some(sy_pa)) = (thermal_stress_pa, material.yield_strength_val()) {
        let stress_mpa = stress_pa.abs() / 1.0e6;
        let sy_mpa = sy_pa / 1.0e6;
        if sy_mpa > 0.0 && stress_mpa > sy_mpa {
            let ratio = (stress_mpa / sy_mpa) * 100.0;
            warnings.push(PhysicsWarning::YieldStressExceeded {
                thermal_stress_mpa: stress_mpa,
                yield_strength_mpa: sy_mpa,
                ratio_percent: ratio,
            });
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_expansion_known_case() {
        // Aluminum 6061: alpha = 23.6e-6 (1/K), L0 = 2.0 m, delta T = 50 K
        // Expected ΔL = 23.6e-6 * 2.0 * 50 = 0.00236 m (2.36 mm)
        let delta_l = thermal_expansion_raw(23.6e-6, 2.0, 50.0);
        assert!((delta_l - 0.00236).abs() < 1e-7);
    }

    #[test]
    fn test_heat_energy_known_case() {
        // Mass = 5.0 kg, c = 900 J/(kg·K), delta T = 40 K
        // Expected Q = 5 * 900 * 40 = 180,000 J (180 kJ)
        let q = heat_energy(5.0, 900.0, 40.0);
        assert_eq!(q, 180_000.0);
    }

    #[test]
    fn test_thermal_stress_known_case() {
        // Steel: E = 200 GPa (200e9 Pa), alpha = 12e-6 (1/K), delta T = 100 K
        // Expected stress = 200e9 * 12e-6 * 100 = 240,000,000 Pa (240 MPa)
        let stress = thermal_stress_raw(200.0e9, 12.0e-6, 100.0);
        assert!((stress - 240.0e6).abs() < 1.0);
    }

    #[test]
    fn test_heat_conduction_known_case() {
        // Copper: k = 400 W/(m·K), Area = 0.5 m², Thickness = 0.02 m (2 cm), delta T = 10 K
        // Heat flux q'' = 400 * 10 / 0.02 = 200,000 W/m²
        // Total heat rate q = 200,000 * 0.5 = 100,000 W (100 kW)
        let result = heat_conduction_raw(400.0, 0.5, 0.02, 10.0);
        assert!(result.is_some());
        let (q, flux) = result.unwrap();
        assert_eq!(q, 100_000.0);
        assert_eq!(flux, 200_000.0);
    }

    #[test]
    fn test_thermal_diffusion_timescale_known_case() {
        // Material with k = 100 W/(m·K), rho = 2500 kg/m³, cp = 1000 J/(kg·K)
        // alpha_diff = 100 / (2500 * 1000) = 4.0e-5 m²/s
        // Thickness L = 0.01 m (1 cm)
        // tau = (0.01)² / 4.0e-5 = 0.0001 / 0.00004 = 2.5 seconds
        let tau = thermal_diffusion_timescale_raw(100.0, 2500.0, 1000.0, 0.01);
        assert!(tau.is_some());
        assert!((tau.unwrap() - 2.5).abs() < 1e-6);
    }

    #[test]
    fn test_sanity_check_melting_point_warning() {
        use crate::material::Property;
        let mat = Material {
            id: "lead".into(),
            name: "Lead".into(),
            category: "metal".into(),
            composition: None,
            grade: None,
            description: None,
            source_ids: vec![],
            aliases: vec![],
            density: None,
            youngs_modulus: None,
            yield_strength: None,
            tensile_strength: None,
            flexural_strength: None,
            thermal_conductivity: None,
            specific_heat: None,
            thermal_expansion: None,
            emissivity: None,
            melting_point: Some(Property {
                value: 600.0, // 600 K
                unit: Some("K".into()),
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
        };

        // Ambient 300 K + Delta 400 K = 700 K > 600 K melting point
        let warnings = evaluate_sanity_checks(&mat, 400.0, 300.0, None);
        assert_eq!(warnings.len(), 1);
        match &warnings[0] {
            PhysicsWarning::MeltingPointExceeded {
                final_temperature_k,
                melting_point_k,
            } => {
                assert_eq!(*final_temperature_k, 700.0);
                assert_eq!(*melting_point_k, 600.0);
            }
            _ => panic!("Expected MeltingPointExceeded warning"),
        }
    }
}
