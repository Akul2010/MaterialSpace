use crate::material::Material;

/// ΔL = α L₀ ΔT
///
/// thermal_expansion is already stored in 1/K.
pub fn thermal_expansion(
    material: &Material,
    initial_length: f64,
    delta_temperature: f64,
) -> Option<f64> {
    let alpha = material.thermal_expansion?;

    Some(
        alpha
            * initial_length
            * delta_temperature,
    )
}

/// Q = mcΔT
pub fn heat_energy(
    mass: f64,
    specific_heat: f64,
    delta_temperature: f64,
) -> f64 {
    mass
        * specific_heat
        * delta_temperature
}

/// k / density
pub fn thermal_efficiency(
    material: &Material,
) -> Option<f64> {
    let conductivity =
        material.thermal_conductivity?;

    let density =
        material.density?;

    if density <= 0.0 {
        return None;
    }

    Some(conductivity / density)
}

/// tensile strength / density
pub fn specific_strength(
    material: &Material,
) -> Option<f64> {
    let strength =
        material.tensile_strength?;

    let density =
        material.density?;

    if density <= 0.0 {
        return None;
    }

    Some(strength / density)
}

/// σ = E α ΔT
pub fn thermal_stress(
    material: &Material,
    delta_temperature: f64,
) -> Option<f64> {
    let youngs_modulus =
        material.youngs_modulus?;

    let alpha =
        material.thermal_expansion?;

    Some(
        youngs_modulus
            * alpha
            * delta_temperature,
    )
}