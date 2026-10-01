use serde::Deserialize;
use std::collections::HashMap;

/// Represents metadata for a scientific data source (e.g., NASA TPSX, NIST, MatWeb, ASM Handbook).
#[derive(Debug, Clone, Deserialize)]
pub struct SourceInfo {
    pub source_id: String,
    pub organization: String,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub accessed: String,
    #[serde(default)]
    pub notes: String,
}

/// A material property value with accompanying physical unit, source provenance, and measurement condition.
///
/// In materials engineering, values without units, testing conditions, or sources cannot be verified.
#[derive(Debug, Clone, Deserialize)]
pub struct Property {
    /// Normalized value in standard SI units (e.g., kg/m³, W/(m·K), Pa, K).
    pub value: f64,

    /// Standard SI unit string (e.g., "kg/m^3", "W/(m*K)", "Pa", "K").
    #[serde(default)]
    pub unit: Option<String>,

    /// Identifier of the scientific or engineering source providing this specific property.
    #[serde(default)]
    pub source_id: Option<String>,

    /// Measurement or test condition (e.g., "Room Temperature", "25 °C", "0.2% offset").
    #[serde(default)]
    pub condition: Option<String>,

    /// Original numerical value before unit normalization.
    #[serde(default)]
    pub original_value: Option<f64>,

    /// Original unit string before unit normalization.
    #[serde(default)]
    pub original_unit: Option<String>,

    /// Any warning generated during normalization pipeline.
    #[serde(default)]
    pub normalization_warning: Option<String>,
}

/// Primary data structure representing an engineering material in MaterialSpace.
///
/// All properties use `Option<Property>` to be scientifically honest:
/// a missing property represents an unknown or unmeasured value, NOT zero.
#[derive(Debug, Clone)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub category: String,
    pub composition: Option<String>,
    pub grade: Option<String>,
    pub description: Option<String>,
    pub source_ids: Vec<String>,

    // Physical & Mechanical Properties
    pub density: Option<Property>,
    pub youngs_modulus: Option<Property>,
    pub yield_strength: Option<Property>,
    pub tensile_strength: Option<Property>,

    // Thermal & Thermophysical Properties
    pub thermal_conductivity: Option<Property>,
    pub specific_heat: Option<Property>,
    pub thermal_expansion: Option<Property>,
    pub emissivity: Option<Property>,
    pub melting_point: Option<Property>,
    pub boiling_point: Option<Property>,
}

impl Material {
    // --- Convenient numeric value getters ---
    // These return Option<f64> for straightforward calculation in physics and metrics modules.

    /// Density in kg/m³
    pub fn density_val(&self) -> Option<f64> {
        self.density.as_ref().map(|p| p.value)
    }

    /// Thermal conductivity in W/(m·K)
    pub fn thermal_conductivity_val(&self) -> Option<f64> {
        self.thermal_conductivity.as_ref().map(|p| p.value)
    }

    /// Specific heat capacity in J/(kg·K)
    pub fn specific_heat_val(&self) -> Option<f64> {
        self.specific_heat.as_ref().map(|p| p.value)
    }

    /// Linear coefficient of thermal expansion in 1/K
    pub fn thermal_expansion_val(&self) -> Option<f64> {
        self.thermal_expansion.as_ref().map(|p| p.value)
    }

    /// Young's modulus of elasticity in Pa
    pub fn youngs_modulus_val(&self) -> Option<f64> {
        self.youngs_modulus.as_ref().map(|p| p.value)
    }

    /// Yield strength in Pa
    pub fn yield_strength_val(&self) -> Option<f64> {
        self.yield_strength.as_ref().map(|p| p.value)
    }

    /// Ultimate tensile strength in Pa
    pub fn tensile_strength_val(&self) -> Option<f64> {
        self.tensile_strength.as_ref().map(|p| p.value)
    }

    /// Emissivity (dimensionless, 0.0 to 1.0)
    #[allow(dead_code)]
    pub fn emissivity_val(&self) -> Option<f64> {
        self.emissivity.as_ref().map(|p| p.value)
    }

    /// Melting / solidus temperature in Kelvin (K)
    pub fn melting_point_val(&self) -> Option<f64> {
        self.melting_point.as_ref().map(|p| p.value)
    }

    /// Boiling point in Kelvin (K)
    #[allow(dead_code)]
    pub fn boiling_point_val(&self) -> Option<f64> {
        self.boiling_point.as_ref().map(|p| p.value)
    }
}

/// Raw deserialization representation from JSON database.
#[derive(Debug, Deserialize)]
pub struct RawMaterial {
    pub id: String,
    pub name: String,
    pub category: String,

    #[serde(default)]
    pub composition: Option<String>,

    #[serde(default)]
    pub grade: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub source_ids: Vec<String>,

    #[serde(default)]
    pub properties: RawProperties,
}

#[derive(Debug, Default, Deserialize)]
pub struct RawProperties {
    pub density: Option<Property>,
    pub thermal_conductivity: Option<Property>,
    pub specific_heat: Option<Property>,
    pub thermal_expansion: Option<Property>,
    pub youngs_modulus: Option<Property>,
    pub yield_strength: Option<Property>,
    pub tensile_strength: Option<Property>,
    pub emissivity: Option<Property>,
    pub melting_point: Option<Property>,
    pub boiling_point: Option<Property>,
}

impl From<RawMaterial> for Material {
    fn from(raw: RawMaterial) -> Self {
        Self {
            id: raw.id,
            name: raw.name,
            category: raw.category,
            composition: raw.composition,
            grade: raw.grade,
            description: raw.description,
            source_ids: raw.source_ids,

            density: raw.properties.density,
            youngs_modulus: raw.properties.youngs_modulus,
            yield_strength: raw.properties.yield_strength,
            tensile_strength: raw.properties.tensile_strength,

            thermal_conductivity: raw.properties.thermal_conductivity,
            specific_heat: raw.properties.specific_heat,
            thermal_expansion: raw.properties.thermal_expansion,
            emissivity: raw.properties.emissivity,
            melting_point: raw.properties.melting_point,
            boiling_point: raw.properties.boiling_point,
        }
    }
}

/// Helper type for mapping source IDs to detailed SourceInfo.
pub type SourcesMap = HashMap<String, SourceInfo>;