use crate::material::{Material, RawMaterial, SourceInfo, SourcesMap};
use crate::retrieval::{LocalCacheManager, is_conservative_match};
use std::fs;
use std::path::Path;

/// The baseline data bundled into the app, so it can run offline.
const BUNDLED_MATERIALS_JSON: &str = include_str!("../data/materials_normalized.json");
const BUNDLED_SOURCES_JSON: &str = include_str!("../data/sources.json");

/// Holds the materials and source info that the app has loaded.
pub struct Database {
    pub materials: Vec<Material>,
    pub sources: SourcesMap,
    pub cache_manager: LocalCacheManager,
    pub is_from_embedded_fallback: bool,
}

impl Database {
    /// Loads local data when available, then fills in from the bundled copy.
    ///
    /// The idea is simple
    /// 1. Try to read the real data files from the project folder.
    /// 2. If the files are missing or broken, fall back to the built-in default data.
    /// 3. Add any cached records we saved earlier.
    /// 4. Sort everything alphabetically by material name.
    pub fn load() -> Result<Self, String> {
        let materials_path = Path::new("data/materials_normalized.json");
        let sources_path = Path::new("data/sources.json");
        let cache_manager = LocalCacheManager::new(".");

        let mut used_embedded_data = false;

        // Read the source info first. If the file is not there, just use the built-in list.
        let sources: SourcesMap = if sources_path.exists() {
            match fs::read_to_string(sources_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                    eprintln!(
                        "Warning: Failed to parse external sources.json: {}. Using embedded fallback.",
                        e
                    );
                    serde_json::from_str(BUNDLED_SOURCES_JSON).unwrap_or_default()
                }),
                Err(_) => serde_json::from_str(BUNDLED_SOURCES_JSON).unwrap_or_default(),
            }
        } else {
            serde_json::from_str(BUNDLED_SOURCES_JSON).unwrap_or_default()
        };

        // Try to read the real material list. If that file is missing or invalid, the fallback data is used.
        let raw_materials: Vec<RawMaterial> = if materials_path.exists() {
            match fs::read_to_string(materials_path) {
                Ok(content) => match serde_json::from_str::<Vec<RawMaterial>>(&content) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        return Err(format!(
                            "Error parsing {}: {}. Database file exists but contains invalid JSON schema.",
                            materials_path.display(),
                            e
                        ));
                    }
                },
                Err(e) => {
                    return Err(format!(
                        "Could not read {}: {}",
                        materials_path.display(),
                        e
                    ));
                }
            }
        } else {
            used_embedded_data = true;
            serde_json::from_str(BUNDLED_MATERIALS_JSON)
                .map_err(|e| format!("Corrupt embedded baseline materials JSON: {}", e))?
        };

        let mut all_materials: Vec<Material> = raw_materials.into_iter().map(Material::from).collect();

        // Add cached materials, but do not overwrite an already-known record if it is the same material.
        let cached_records = cache_manager.load_cached_materials();
        for cached_material in cached_records {
            let already_in_list = all_materials
                .iter()
                .any(|material| is_conservative_match(material, &cached_material.id, &cached_material.name));
            if !already_in_list {
                all_materials.push(cached_material);
            }
        }

        // Keep the list easy to browse in the app by sorting it alphabetically by name.
        all_materials.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));

        Ok(Self {
            materials: all_materials,
            sources,
            cache_manager,
            is_from_embedded_fallback: used_embedded_data,
        })
    }

    /// Access material by index.
    pub fn get(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }

    /// Finds material index by unique canonical ID string.
    pub fn find_index_by_id(&self, id: &str) -> Option<usize> {
        self.materials.iter().position(|m| m.id == id)
    }

    /// Retrieves metadata for a specific cited source ID (e.g. "nasa_tpsx", "matweb").
    pub fn get_source(&self, source_id: &str) -> Option<&SourceInfo> {
        self.sources.get(source_id)
    }

    /// Returns a deterministically sorted list of all cited sources.
    pub fn sorted_sources(&self) -> Vec<(&String, &SourceInfo)> {
        let mut list: Vec<(&String, &SourceInfo)> = self.sources.iter().collect();
        list.sort_by(|a, b| a.1.organization.cmp(&b.1.organization));
        list
    }

    /// Returns a sorted, deduplicated list of all material categories present in database.
    pub fn categories(&self) -> Vec<String> {
        let mut categories: Vec<String> =
            self.materials.iter().map(|m| m.category.clone()).collect();

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

    /// Counts materials containing at least one mechanical property (tensile, yield, or modulus).
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

    /// Counts materials containing flexural strength data (technical ceramics and glasses).
    pub fn count_with_flexural_data(&self) -> usize {
        self.materials
            .iter()
            .filter(|m| m.flexural_strength.is_some())
            .count()
    }

    /// Searches database across canonical Name, Aliases, ID, Composition, Grade, and Description.
    pub fn search(&self, query: &str, category: &str) -> Vec<(usize, &Material)> {
        let query_lower = query.trim().to_lowercase();
        let filter_all = category == "All" || category.is_empty();

        self.materials
            .iter()
            .enumerate()
            .filter(|(_, material)| {
                // Category filter check
                let category_matches =
                    filter_all || material.category.eq_ignore_ascii_case(category);
                if !category_matches {
                    return false;
                }

                // Empty query matches all in category
                if query_lower.is_empty() {
                    return true;
                }

                // Check Name and ID
                if material.name.to_lowercase().contains(&query_lower)
                    || material.id.to_lowercase().contains(&query_lower)
                {
                    return true;
                }

                // Check Aliases
                for alias in &material.aliases {
                    if alias.to_lowercase().contains(&query_lower) {
                        return true;
                    }
                }

                // Check Composition
                if let Some(comp) = &material.composition {
                    if comp.to_lowercase().contains(&query_lower) {
                        return true;
                    }
                }

                // Check Grade
                if let Some(grade) = &material.grade {
                    if grade.to_lowercase().contains(&query_lower) {
                        return true;
                    }
                }

                // Check Description
                if let Some(desc) = &material.description {
                    if desc.to_lowercase().contains(&query_lower) {
                        return true;
                    }
                }

                false
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_load_baseline_dataset() {
        let db = Database::load()
            .expect("Database must load successfully from disk or embedded baseline");
        // Verify real material count
        assert!(
            db.materials.len() >= 50,
            "Expected at least 50 materials in normalized database, found {}",
            db.materials.len()
        );

        // Verify all IDs are non-empty and unique
        let mut ids = HashSet::new();
        for m in &db.materials {
            assert!(!m.id.is_empty(), "Material ID must not be empty");
            assert!(!m.name.is_empty(), "Material name must not be empty");
            assert!(
                ids.insert(m.id.clone()),
                "Duplicate material ID detected: {}",
                m.id
            );
        }

        // Verify cited sources are loaded
        assert!(db.sources.len() >= 4, "Expected at least 4 cited sources");
        assert!(db.sources.contains_key("nasa_tpsx"));
        assert!(db.sources.contains_key("matweb"));
        assert!(db.sources.contains_key("nist_webbook"));
        assert!(db.sources.contains_key("asm_handbook"));
    }

    #[test]
    fn test_search_functionality() {
        let db = Database::load().unwrap();

        // Canonical name search
        let results = db.search("Titanium", "All");
        assert!(
            !results.is_empty(),
            "Search for 'Titanium' should return materials"
        );
        assert!(results.iter().any(|(_, m)| m.id.contains("ti6al4v")));

        // Case-insensitivity test
        let lower = db.search("inconel", "All");
        let upper = db.search("INCONEL", "All");
        assert_eq!(lower.len(), upper.len());

        // Category filter test
        let ceramics = db.search("", "ceramic");
        for (_, m) in &ceramics {
            assert_eq!(m.category, "ceramic");
        }

        // Unmatched search returns empty
        let empty = db.search("nonexistent_element_xyz_123", "All");
        assert!(empty.is_empty());
    }

    #[test]
    fn test_tensile_and_flexural_separation() {
        let db = Database::load().unwrap();

        // Find silicon carbide
        let sic = db
            .materials
            .iter()
            .find(|m| m.id == "silicon_carbide_sintered");
        assert!(sic.is_some(), "Silicon carbide must exist in database");
        let sic = sic.unwrap();

        // Must have flexural strength
        assert!(
            sic.flexural_strength.is_some(),
            "Silicon carbide must have flexural_strength recorded"
        );
        // Must NOT have tensile strength
        assert!(
            sic.tensile_strength.is_none(),
            "Silicon carbide must NOT have tensile_strength recorded"
        );

        // Find aluminum 6061
        let al = db
            .materials
            .iter()
            .find(|m| m.id == "aluminum_6061_t6")
            .unwrap();
        assert!(
            al.tensile_strength.is_some(),
            "Aluminum 6061 must have tensile strength"
        );
        assert!(
            al.flexural_strength.is_none(),
            "Aluminum 6061 must NOT have flexural strength"
        );
    }
}
