use crate::material::{Material, RawMaterial, SourceInfo, SourcesMap};
use std::collections::HashMap;
use std::fs;

/// MaterialSpace in-memory database holding all normalized materials and verified scientific source metadata.
pub struct Database {
    pub materials: Vec<Material>,
    pub sources: SourcesMap,
}

impl Database {
    /// Loads the materials database from `data/materials_normalized.json` and source metadata from `data/sources.json`.
    pub fn load() -> Result<Self, String> {
        let materials_path = "data/materials_normalized.json";
        let sources_path = "data/sources.json";

        // 1. Load normalized materials
        let materials_content = fs::read_to_string(materials_path)
            .map_err(|e| format!("Could not read {}: {}", materials_path, e))?;

        let raw_materials: Vec<RawMaterial> = serde_json::from_str(&materials_content)
            .map_err(|e| format!("Could not parse {}: {}", materials_path, e))?;

        let materials: Vec<Material> = raw_materials
            .into_iter()
            .map(Material::from)
            .collect();

        // 2. Load source metadata (optional fallback to empty map if file missing)
        let sources: SourcesMap = match fs::read_to_string(sources_path) {
            Ok(sources_content) => {
                serde_json::from_str(&sources_content).unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to parse sources.json: {}", e);
                    HashMap::new()
                })
            }
            Err(e) => {
                eprintln!("Warning: Could not read sources.json: {}", e);
                HashMap::new()
            }
        };

        Ok(Self { materials, sources })
    }

    /// Retrieves a material reference by index.
    pub fn get(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }

    /// Finds material index by its unique ID string.
    #[allow(dead_code)]
    pub fn find_index_by_id(&self, id: &str) -> Option<usize> {
        self.materials.iter().position(|m| m.id == id)
    }

    /// Retrieves metadata for a specific source ID (e.g., "nasa_tpsx", "matweb").
    pub fn get_source(&self, source_id: &str) -> Option<&SourceInfo> {
        self.sources.get(source_id)
    }

    /// Returns a sorted, deduplicated list of all material categories present in the database.
    pub fn categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .materials
            .iter()
            .map(|m| m.category.clone())
            .collect();

        categories.sort();
        categories.dedup();
        categories
    }

    /// Counts materials containing at least one thermal property.
    pub fn count_with_thermal_data(&self) -> usize {
        self.materials
            .iter()
            .filter(|m| {
                m.thermal_conductivity.is_some()
                    || m.specific_heat.is_some()
                    || m.thermal_expansion.is_some()
                    || m.melting_point.is_some()
            })
            .count()
    }

    /// Counts materials containing at least one mechanical property.
    pub fn count_with_mechanical_data(&self) -> usize {
        self.materials
            .iter()
            .filter(|m| {
                m.youngs_modulus.is_some()
                    || m.yield_strength.is_some()
                    || m.tensile_strength.is_some()
            })
            .count()
    }

    /// Performs case-insensitive search and category filtering across the database.
    ///
    /// Searches across material Name, ID, Composition, Grade, and Description.
    pub fn search(&self, query: &str, category: &str) -> Vec<(usize, &Material)> {
        let query_lower = query.trim().to_lowercase();
        let filter_all = category == "All" || category.is_empty();

        self.materials
            .iter()
            .enumerate()
            .filter(|(_, material)| {
                // Category filter check
                let category_matches = filter_all || material.category.eq_ignore_ascii_case(category);
                if !category_matches {
                    return false;
                }

                // Query search check
                if query_lower.is_empty() {
                    return true;
                }

                material.name.to_lowercase().contains(&query_lower)
                    || material.id.to_lowercase().contains(&query_lower)
                    || material
                        .composition
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
                    || material
                        .grade
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
                    || material
                        .description
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .collect()
    }
}