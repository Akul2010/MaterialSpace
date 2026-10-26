use crate::material::{Material, RawMaterial};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// A few details about a data source and how we can use it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub source_id: String,
    pub name: String,
    pub access_method: String,
    pub is_available: bool,
    pub access_notes: String,
}

/// The access notes we show in the app. These explain what each source actually allows.
pub fn official_source_statuses() -> Vec<SourceMetadata> {
    vec![
        SourceMetadata {
            source_id: "nasa_tpsx".into(),
            name: "NASA TPSX (Thermal Protection Systems Expert)".into(),
            access_method: "Documented Baseline Dataset (Direct Machine-Readable Export)".into(),
            is_available: true,
            access_notes: "NASA TPSX is an authoritative aerospace database operated by NASA Ames. \
                           Live dynamic API access requires NASA credentialed access and an authorized \
                           government agency agreement; unauthorized web scraping is strictly prohibited. \
                           MaterialSpace bundles documented, verified TPSX material records.".into(),
        },
        SourceMetadata {
            source_id: "nist_webbook".into(),
            name: "NIST Chemistry WebBook (SRD 69)".into(),
            access_method: "Standard Reference Data (SRD) Endpoints".into(),
            is_available: true,
            access_notes: "NIST SRD provides thermochemical data for pure substances and chemical elements \
                           via public reference tables. NIST WebBook does not maintain an open REST API \
                           for structural engineering alloys (e.g. Inconel 718, Ti-6Al-4V).".into(),
        },
        SourceMetadata {
            source_id: "matweb".into(),
            name: "MatWeb Online Materials Information".into(),
            access_method: "Licensed Enterprise Machine-Readable Feeds".into(),
            is_available: true,
            access_notes: "MatWeb is a proprietary commercial database. Dynamic machine-readable API access \
                           requires an enterprise data license; website scraping is strictly prohibited by \
                           terms of service. MaterialSpace incorporates cited MatWeb entries from verified \
                           manufacturer datasheets.".into(),
        },
        SourceMetadata {
            source_id: "custom_feed".into(),
            name: "Local Structured Research Endpoint / Cache Feed".into(),
            access_method: "JSON File & REST Endpoint Adapter".into(),
            is_available: true,
            access_notes: "Permitted machine-readable channel for importing lab-measured datasets \
                           and structured JSON materials data into the research cache.".into(),
        },
    ]
}
/// Handles the local cache, so the app can reuse data while offline.
pub struct LocalCacheManager {
    cache_dir: PathBuf,
}

impl LocalCacheManager {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        let cache_dir = base_dir.as_ref().join("data").join("cache");
        Self { cache_dir }
    }

    /// Makes the cache folder if it isn't there yet.
    #[allow(dead_code)]
    fn ensure_cache_dir(&self) -> Result<&PathBuf, String> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir).map_err(|e| {
                format!(
                    "Could not create cache directory {:?}: {}",
                    self.cache_dir, e
                )
            })?;
        }
        Ok(&self.cache_dir)
    }

    /// Reads any materials we've already cached.
    pub fn load_cached_materials(&self) -> Vec<Material> {
        let mut materials = Vec::new();
        if !self.cache_dir.exists() {
            return materials;
        }

        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json")
                    && path
                        .file_name()
                        .map_or(false, |name| name != "manifest.json")
                {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(raw) = serde_json::from_str::<RawMaterial>(&content) {
                            materials.push(Material::from(raw));
                        }
                    }
                }
            }
        }

        materials
    }

    /// Saves a newly retrieved or imported material to the local cache.
    #[allow(dead_code)]
    pub fn cache_material(&self, material: &Material) -> Result<(), String> {
        self.ensure_cache_dir()?;
        let file_name = format!("{}.json", material.id);
        let target_path = self.cache_dir.join(file_name);

        let json_content = serde_json::to_string_pretty(material)
            .map_err(|e| format!("Serialization error for {}: {}", material.id, e))?;

        fs::write(&target_path, json_content)
            .map_err(|e| format!("Failed to write cache file {:?}: {}", target_path, e))?;

        Ok(())
    }

    /// Returns cache status summary: (cached_count, last_modified_description).
    pub fn cache_status(&self) -> (usize, String) {
        if !self.cache_dir.exists() {
            return (0, "Cache empty (offline baseline active)".into());
        }

        let mut count = 0;
        let mut latest_time = SystemTime::UNIX_EPOCH;

        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json")
                    && path
                        .file_name()
                        .map_or(false, |name| name != "manifest.json")
                {
                    count += 1;
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if modified > latest_time {
                                latest_time = modified;
                            }
                        }
                    }
                }
            }
        }

        if count == 0 {
            (0, "Cache empty (offline baseline active)".into())
        } else {
            (count, format!("{} cached records active", count))
        }
    }
}

/// Conservative Material Identity Matcher.
///
/// This is a very careful "are these the same material?" check.
/// We do not merge records just because the names look a little similar.
/// The function only says "yes" when the IDs, names, or known aliases are an exact match.
pub fn is_conservative_match(existing: &Material, incoming_id: &str, incoming_name: &str) -> bool {
    // 1) Exact ID match: same canonical material ID, so this is almost certainly the same item.
    if existing.id == incoming_id {
        return true;
    }

    // 2) Exact case-insensitive canonical name match.
    if existing.name.eq_ignore_ascii_case(incoming_name) {
        return true;
    }

    // 3) Known exact alias match.
    for alias in &existing.aliases {
        if alias.eq_ignore_ascii_case(incoming_name) || alias.eq_ignore_ascii_case(incoming_id) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservative_matching() {
        let existing = Material {
            id: "alumina_99_5".into(),
            name: "Alumina Ceramic (Al2O3 99.5%)".into(),
            category: "ceramic".into(),
            composition: Some("Al2O3 99.5%".into()),
            grade: Some("High Purity Technical".into()),
            description: None,
            source_ids: vec!["matweb".into()],
            aliases: vec!["Alumina 99.5%".into(), "Al2O3 99.5%".into()],
            density: None,
            youngs_modulus: None,
            yield_strength: None,
            tensile_strength: None,
            flexural_strength: None,
            thermal_conductivity: None,
            specific_heat: None,
            thermal_expansion: None,
            emissivity: None,
            melting_point: None,
            boiling_point: None,
        };

        // Exact ID matches
        assert!(is_conservative_match(
            &existing,
            "alumina_99_5",
            "Other Name"
        ));

        // Exact name matches
        assert!(is_conservative_match(
            &existing,
            "other_id",
            "Alumina Ceramic (Al2O3 99.5%)"
        ));

        // Exact alias matches
        assert!(is_conservative_match(
            &existing,
            "other_id",
            "Alumina 99.5%"
        ));

        // Vague / different purity does NOT match!
        assert!(!is_conservative_match(
            &existing,
            "alumina_96_0",
            "Alumina 96%"
        ));
        assert!(!is_conservative_match(
            &existing,
            "pure_aluminum",
            "Aluminum"
        ));
    }

    #[test]
    fn test_cache_manager_save_and_load() {
        let temp_dir = std::env::temp_dir().join("materialspace_test_cache");
        let _ = fs::remove_dir_all(&temp_dir);

        let manager = LocalCacheManager::new(&temp_dir);
        let test_mat = Material {
            id: "test_aerospace_alloy".into(),
            name: "Test Aerospace Alloy".into(),
            category: "metal".into(),
            composition: Some("Ti-6Al-4V".into()),
            grade: Some("Grade 5".into()),
            description: None,
            source_ids: vec!["matweb".into()],
            aliases: vec!["Ti64".into()],
            density: None,
            youngs_modulus: None,
            yield_strength: None,
            tensile_strength: None,
            flexural_strength: None,
            thermal_conductivity: None,
            specific_heat: None,
            thermal_expansion: None,
            emissivity: None,
            melting_point: None,
            boiling_point: None,
        };

        // Cache the material
        let save_res = manager.cache_material(&test_mat);
        assert!(save_res.is_ok());

        // Check cache status
        let (count, _) = manager.cache_status();
        assert_eq!(count, 1);

        // Load cached materials
        let loaded = manager.load_cached_materials();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, "test_aerospace_alloy");

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
