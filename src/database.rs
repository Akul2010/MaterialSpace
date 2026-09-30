use crate::material::{Material, RawMaterial};
use std::fs;

pub struct Database {
    pub materials: Vec<Material>,
}

impl Database {
    pub fn load() -> Result<Self, String> {
        let path = "data/materials_normalized.json";

        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Could not read {}: {}", path, e))?;

        let raw_materials: Vec<RawMaterial> =
            serde_json::from_str(&contents)
                .map_err(|e| {
                    format!(
                        "Could not parse {}: {}",
                        path, e
                    )
                })?;

        let materials =
            raw_materials
                .into_iter()
                .map(Material::from)
                .collect();

        Ok(Self { materials })
    }

    pub fn search(
        &self,
        query: &str,
        category: Option<&str>,
    ) -> Vec<&Material> {
        let query = query.trim().to_lowercase();

        self.materials
            .iter()
            .filter(|material| {
                let category_matches =
                    match category {
                        Some(category)
                            if category != "All" =>
                        {
                            material.category
                                .eq_ignore_ascii_case(category)
                        }

                        _ => true,
                    };

                if !category_matches {
                    return false;
                }

                if query.is_empty() {
                    return true;
                }

                material.name.to_lowercase().contains(&query)
                    || material
                        .id
                        .to_lowercase()
                        .contains(&query)
                    || material
                        .composition
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
                    || material
                        .description
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
            })
            .collect()
    }

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

    pub fn get(&self, index: usize) -> Option<&Material> {
        self.materials.get(index)
    }
}