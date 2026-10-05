use crate::material::Material;

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
/// 1. Isotropic material behavior (expansion is equal in all directions).
/// 2. Constant coefficient of thermal expansion over the given temperature range.
/// 3. Free expansion without external mechanical constraints.
pub fn thermal_expansion(material: &Material, initial_length: f64, delta_temperature: f64) -> Option<f64> {
    let alpha = material.thermal_expansion_val()?;
    Some(thermal_expansion_raw(alpha, initial_length, delta_temperature))
}

/// Raw calculation of linear thermal expansion ΔL = α · L₀ · ΔT
pub fn thermal_expansion_raw(alpha: f64, initial_length: f64, delta_temperature: f64) -> f64 {
    alpha * initial_length * delta_temperature
}

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
/// 1. Fully constrained uniaxial bar (ends cannot move, strain = 0).
/// 2. Linear elastic behavior (Hooke's law applies: σ = E · ε_thermal).
/// 3. Homogeneous and isotropic material.
/// 4. No buckling occurs under compressive stress.
///
/// > **Note**: This is an idealized upper-bound estimate. In real structures,
/// > compliance in joints, 3D Poisson effects, and plastic deformation reduce stress.
pub fn thermal_stress(material: &Material, delta_temperature: f64) -> Option<f64> {
    let youngs_modulus = material.youngs_modulus_val()?;
    let alpha = material.thermal_expansion_val()?;
    Some(thermal_stress_raw(youngs_modulus, alpha, delta_temperature))
}

/// Raw calculation of 1D fully constrained thermal stress σ = E · α · ΔT
pub fn thermal_stress_raw(youngs_modulus: f64, alpha: f64, delta_temperature: f64) -> f64 {
    youngs_modulus * alpha * delta_temperature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_expansion_known_case() {
        // Aluminum 6061: alpha ≈ 23.6e-6 (1/K)
        // Length = 2.0 meters, delta T = 50 K
        // Expected ΔL = 23.6e-6 * 2.0 * 50 = 0.00236 m (2.36 mm)
        let alpha = 23.6e-6;
        let l0 = 2.0;
        let dt = 50.0;
        let delta_l = thermal_expansion_raw(alpha, l0, dt);

        assert!((delta_l - 0.00236).abs() < 1e-7);
    }

    #[test]
    fn test_heat_energy_known_case() {
        // Water / Aluminum heating:
        // Mass = 5.0 kg, c = 900 J/(kg*K), delta T = 40 K
        // Expected Q = 5 * 900 * 40 = 180,000 J (180 kJ)
        let mass = 5.0;
        let cp = 900.0;
        let dt = 40.0;
        let q = heat_energy(mass, cp, dt);

        assert_eq!(q, 180_000.0);
    }

    #[test]
    fn test_thermal_stress_known_case() {
        // Structural steel: E = 200 GPa (200e9 Pa), alpha = 12e-6 (1/K)
        // delta T = 100 K
        // Expected stress σ = 200e9 * 12e-6 * 100 = 240,000,000 Pa (240 MPa)
        let e = 200.0e9;
        let alpha = 12.0e-6;
        let dt = 100.0;
        let stress = thermal_stress_raw(e, alpha, dt);

        assert!((stress - 240.0e6).abs() < 1.0);
    }
}