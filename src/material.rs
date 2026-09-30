use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub category: String,
    pub composition: Option<String>,
    pub grade: Option<String>,
    pub description: Option<String>,
    pub source_ids: Vec<String>,

    pub density: Option<f64>,
    pub thermal_conductivity: Option<f64>,
    pub specific_heat: Option<f64>,
    pub thermal_expansion: Option<f64>,
    pub youngs_modulus: Option<f64>,
    pub yield_strength: Option<f64>,
    pub tensile_strength: Option<f64>,
    pub emissivity: Option<f64>,
    pub melting_point: Option<f64>,
    pub boiling_point: Option<f64>,
}

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
    pub density: Option<PropertyValue>,
    pub thermal_conductivity: Option<PropertyValue>,
    pub specific_heat: Option<PropertyValue>,
    pub thermal_expansion: Option<PropertyValue>,
    pub youngs_modulus: Option<PropertyValue>,
    pub yield_strength: Option<PropertyValue>,
    pub tensile_strength: Option<PropertyValue>,
    pub emissivity: Option<PropertyValue>,
    pub melting_point: Option<PropertyValue>,
    pub boiling_point: Option<PropertyValue>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyValue {
    pub value: f64,

    #[serde(default)]
    pub unit: Option<String>,

    #[serde(default)]
    pub source_id: Option<String>,
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

            density: property_value(&raw.properties.density),
            thermal_conductivity:
                property_value(&raw.properties.thermal_conductivity),
            specific_heat:
                property_value(&raw.properties.specific_heat),
            thermal_expansion:
                property_value(&raw.properties.thermal_expansion),
            youngs_modulus:
                property_value(&raw.properties.youngs_modulus),
            yield_strength:
                property_value(&raw.properties.yield_strength),
            tensile_strength:
                property_value(&raw.properties.tensile_strength),
            emissivity:
                property_value(&raw.properties.emissivity),
            melting_point:
                property_value(&raw.properties.melting_point),
            boiling_point:
                property_value(&raw.properties.boiling_point),
        }
    }
}

fn property_value(property: &Option<PropertyValue>) -> Option<f64> {
    property.as_ref().map(|p| p.value)
}