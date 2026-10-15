use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Formats a physical quantity using scientific notation for very small (< 0.001)
/// or very large (>= 10,000) values, while keeping ordinary numbers easily readable.
///
/// Preserves negative signs (e.g. negative thermal expansion coefficients in CFRP)
/// and handles zero without emitting redundant exponents.
pub fn format_scientific(value: f64) -> String {
    if value == 0.0 {
        return "0.0".to_string();
    }

    let abs_val = value.abs();

    if abs_val >= 10_000.0 || abs_val < 0.001 {
        // e.g. 2.360e-5, -4.000e-7, 5.000e-8, 1.250e+09
        format!("{:.3e}", value)
    } else if abs_val >= 100.0 {
        format!("{:.1}", value)
    } else if abs_val >= 1.0 {
        format!("{:.2}", value)
    } else {
        format!("{:.4}", value)
    }
}

/// Metadata for an authoritative scientific data source (e.g. NASA TPSX, NIST, MatWeb, ASM Handbook).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub source_id: String,
    pub organization: String,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub accessed: String,
    #[serde(default)]
    pub notes: String,
    /// Records transparent notes from targeted spot-checks against the cited literature.
    #[serde(default)]
    pub spot_check_verified: Option<String>,
}

/// A material property value with accompanying physical unit, source provenance,
/// measurement condition, and test method metadata.
///
/// In materials engineering, a measurement without units, testing temperature,
/// method, or source cannot be scientifically validated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// Normalized value in standard SI units (e.g. kg/m³, Pa, W/(m·K), J/(kg·K), 1/K, K).
    pub value: f64,

    /// Standard SI unit string (e.g. "kg/m^3", "Pa", "W/(m*K)", "J/(kg*K)", "1/K", "K").
    #[serde(default)]
    pub unit: Option<String>,

    /// Identifier of the scientific or engineering source providing this specific property.
    #[serde(default)]
    pub source_id: Option<String>,

    /// Measurement condition (e.g. "25 °C", "0.2% offset", "Solidus", "Through-thickness").
    #[serde(default)]
    pub condition: Option<String>,

    /// Specific test standard or method (e.g. "ASTM E8", "ASTM C1161", "3-point bend").
    #[serde(default)]
    pub test_method: Option<String>,

    /// Measurement temperature in Kelvin, if explicitly recorded.
    #[serde(default)]
    pub temperature: Option<f64>,

    /// Original numerical value before unit normalization.
    #[serde(default)]
    pub original_value: Option<f64>,

    /// Original unit string before unit normalization.
    #[serde(default)]
    pub original_unit: Option<String>,

    /// Uncertainty or tolerance range reported by the source (e.g. "±5%", "250-280 MPa").
    #[serde(default)]
    pub uncertainty: Option<String>,

    /// Additional scientific notes or caveats regarding this property.
    #[serde(default)]
    pub notes: Option<String>,

    /// Any warning generated during normalization pipeline.
    #[serde(default)]
    pub normalization_warning: Option<String>,
}

impl Property {
    /// Returns the measurement condition or "Not recorded" if none was provided.
    ///
    /// Adheres to scientific honesty: never displays "Standard / Ambient" when no
    /// ambient test condition was documented in the source dataset.
    pub fn condition_display(&self) -> &str {
        match self.condition.as_deref() {
            Some(cond) if !cond.trim().is_empty() => cond,
            _ => "Not recorded",
        }
    }
}

/// Primary data structure representing an engineering material in MaterialSpace.
///
/// All properties use `Option<Property>` to be scientifically honest:
/// a missing property represents an unknown or unmeasured value, NOT zero.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
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
    pub aliases: Vec<String>,

    // Physical Properties
    pub density: Option<Property>,

    // Mechanical Properties
    pub youngs_modulus: Option<Property>,
    pub yield_strength: Option<Property>,
    pub tensile_strength: Option<Property>,
    /// Separate flexural strength (bending strength / modulus of rupture).
    /// Brittle ceramics and glasses report flexural strength, which must NEVER
    /// be conflated with uniaxial tensile strength.
    pub flexural_strength: Option<Property>,

    // Thermal & Thermophysical Properties
    pub thermal_conductivity: Option<Property>,
    pub specific_heat: Option<Property>,
    pub thermal_expansion: Option<Property>,
    pub emissivity: Option<Property>,
    pub melting_point: Option<Property>,
    pub boiling_point: Option<Property>,
}

impl Material {
    // --- Convenient numeric value getters (return Option<f64> in SI units) ---

    /// Density in kg/m³
    pub fn density_val(&self) -> Option<f64> {
        self.density.as_ref().map(|p| p.value)
    }

    /// Young's modulus of elasticity in Pa
    pub fn youngs_modulus_val(&self) -> Option<f64> {
        self.youngs_modulus.as_ref().map(|p| p.value)
    }

    /// Yield strength in Pa
    pub fn yield_strength_val(&self) -> Option<f64> {
        self.yield_strength.as_ref().map(|p| p.value)
    }

    /// Ultimate tensile strength in Pa (only genuine tensile tests)
    pub fn tensile_strength_val(&self) -> Option<f64> {
        self.tensile_strength.as_ref().map(|p| p.value)
    }

    /// Flexural strength (modulus of rupture / bending strength) in Pa
    pub fn flexural_strength_val(&self) -> Option<f64> {
        self.flexural_strength.as_ref().map(|p| p.value)
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

    /// Melting / solidus temperature in Kelvin (K)
    pub fn melting_point_val(&self) -> Option<f64> {
        self.melting_point.as_ref().map(|p| p.value)
    }

    /// Returns human-readable category label (e.g. "metal" -> "Metals & Alloys").
    pub fn category_display(&self) -> &'static str {
        match self.category.to_lowercase().as_str() {
            "metal" => "Metals & Alloys",
            "ceramic" => "Technical Ceramics",
            "polymer" => "Polymers & Plastics",
            "composite" => "Composites",
            "glass" => "Glasses & Glass-Ceramics",
            "pure_substance" => "Elements & Pure Substances",
            _ => "Other Engineering Materials",
        }
    }
}

/// Raw deserialization representation from the JSON database.
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
    pub aliases: Vec<String>,

    #[serde(default)]
    pub properties: RawProperties,
}

#[derive(Debug, Default, Deserialize)]
pub struct RawProperties {
    pub density: Option<Property>,
    pub youngs_modulus: Option<Property>,
    pub yield_strength: Option<Property>,
    pub tensile_strength: Option<Property>,
    pub flexural_strength: Option<Property>,
    pub thermal_conductivity: Option<Property>,
    pub specific_heat: Option<Property>,
    pub thermal_expansion: Option<Property>,
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
            aliases: raw.aliases,

            density: raw.properties.density,
            youngs_modulus: raw.properties.youngs_modulus,
            yield_strength: raw.properties.yield_strength,
            tensile_strength: raw.properties.tensile_strength,
            flexural_strength: raw.properties.flexural_strength,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_scientific_representative_values() {
        // Aluminum expansion: ~2.36e-5
        let al_cte = format_scientific(2.36e-5);
        assert_eq!(al_cte, "2.360e-5");

        // Zerodur expansion: ~5e-8
        let zerodur_cte = format_scientific(5e-8);
        assert_eq!(zerodur_cte, "5.000e-8");

        // Carbon-fiber composite negative expansion: ~-4e-7
        let cfrp_cte = format_scientific(-4e-7);
        assert_eq!(cfrp_cte, "-4.000e-7");

        // Zero
        assert_eq!(format_scientific(0.0), "0.0");

        // Ordinary readable numbers
        assert_eq!(format_scientific(2700.0), "2700.0");
        assert_eq!(format_scientific(167.0), "167.0");
        assert_eq!(format_scientific(0.85), "0.8500");

        // Large number >= 10,000
        assert_eq!(format_scientific(70_000_000_000.0), "7.000e10");

        // Negative ordinary number
        assert_eq!(format_scientific(-12.5), "-12.50");
    }

    #[test]
    fn test_condition_display_not_recorded() {
        let prop_no_cond = Property {
            value: 100.0,
            unit: Some("Pa".into()),
            source_id: Some("matweb".into()),
            condition: None,
            test_method: None,
            temperature: None,
            original_value: None,
            original_unit: None,
            uncertainty: None,
            notes: None,
            normalization_warning: None,
        };
        assert_eq!(prop_no_cond.condition_display(), "Not recorded");

        let prop_with_cond = Property {
            condition: Some("0.2% offset".into()),
            ..prop_no_cond
        };
        assert_eq!(prop_with_cond.condition_display(), "0.2% offset");
    }
}
