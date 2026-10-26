use crate::database::Database;
use crate::material::Material;
use crate::metrics::{RankedMaterialItem, RankingMetric, rank_materials};
use crate::screening::{Constraints, ScreeningProfilePreset, ScreeningResult, screen_materials};

/// Which page we're looking at right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Materials,
    Compare,
    MaterialDetails,
    Screening,
    Rankings,
    Calculator,
    About,
}

/// Tab selector for the Screening page results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreeningTab {
    Passed,
    Failed,
    InsufficientData,
    All,
}

/// The app's current selections, inputs, and loaded data.
pub struct MaterialSpaceApp {
    /// The materials and source details we've loaded.
    pub database: Database,

    /// The page we're showing.
    pub page: Page,

    /// What we're searching for.
    pub search_query: String,

    /// The selected category, or all of them.
    pub category_filter: String,

    /// Index of the material inspected in Material Details.
    pub selected_material: Option<usize>,

    /// Index of first material in side-by-side comparison.
    pub comparison_a: Option<usize>,

    /// Index of second material in side-by-side comparison.
    pub comparison_b: Option<usize>,

    // --- Screening State ---
    /// The limits we're screening materials against.
    pub screening_constraints: Constraints,
    /// Currently loaded screening profile preset, if any.
    pub selected_preset: Option<ScreeningProfilePreset>,
    /// Active tab in screening results view.
    pub screening_tab: ScreeningTab,
    /// Saved screening results until something changes.
    pub screening_cache: Option<Vec<ScreeningResult>>,

    // --- Rankings State ---
    /// What we're using to sort the rankings.
    pub ranking_metric: RankingMetric,
    /// Cached ranked material items.
    pub ranking_cache: Option<Vec<RankedMaterialItem>>,

    // --- Calculator State ---
    /// The starting temperature for the calculators.
    pub ambient_temperature_k: f64,

    // 1. Thermal Expansion
    pub calc_expansion_material: Option<usize>,
    pub initial_length: f64,
    pub delta_temperature: f64,

    // 2. Heat Energy Transfer
    pub calc_heat_material: Option<usize>,
    pub calculator_mass: f64,
    /// Specific heat capacity c (J/(kg·K)). `None` if material lacks specific heat.
    pub calculator_specific_heat: Option<f64>,
    pub heat_delta_temperature: f64,

    // 3. Thermal Stress
    pub calc_stress_material: Option<usize>,
    pub stress_delta_temperature: f64,

    // 4. Heat Conduction
    pub calc_conduction_material: Option<usize>,
    pub conduction_area: f64,
    pub conduction_thickness: f64,
    pub conduction_delta_t: f64,

    // 5. Thermal Diffusion Timescale
    pub calc_diffusion_material: Option<usize>,
    pub diffusion_thickness: f64,

    // --- Feedback & Notifications ---
    pub status_message: Option<String>,
    pub export_notice: Option<String>,
}

impl MaterialSpaceApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (database, status_message) = match Database::load() {
            Ok(db) => {
                let note = if db.is_from_embedded_fallback {
                    Some(
                        "Operating in self-contained mode using compiled-in baseline dataset."
                            .into(),
                    )
                } else {
                    None
                };
                (db, note)
            }
            Err(err) => {
                eprintln!("Database load error: {}", err);
                (
                    Database {
                        materials: Vec::new(),
                        sources: std::collections::HashMap::new(),
                        cache_manager: crate::retrieval::LocalCacheManager::new("."),
                        is_from_embedded_fallback: false,
                    },
                    Some(format!("Database Initialization Warning: {}", err)),
                )
            }
        };

        let default_first = if !database.materials.is_empty() {
            Some(0)
        } else {
            None
        };
        let default_second = if database.materials.len() > 1 {
            Some(1)
        } else {
            default_first
        };

        let mut app = Self {
            database,
            page: Page::Dashboard,
            search_query: String::new(),
            category_filter: "All".to_string(),
            selected_material: default_first,
            comparison_a: default_first,
            comparison_b: default_second,

            screening_constraints: Constraints::default(),
            selected_preset: None,
            screening_tab: ScreeningTab::Passed,
            screening_cache: None,

            ranking_metric: RankingMetric::AerospaceScore,
            ranking_cache: None,

            ambient_temperature_k: 293.15,

            calc_expansion_material: default_first,
            initial_length: 1.0,
            delta_temperature: 100.0,

            calc_heat_material: default_first,
            calculator_mass: 1.0,
            calculator_specific_heat: None,
            heat_delta_temperature: 50.0,

            calc_stress_material: default_first,
            stress_delta_temperature: 100.0,

            calc_conduction_material: default_first,
            conduction_area: 1.0,
            conduction_thickness: 0.02,
            conduction_delta_t: 50.0,

            calc_diffusion_material: default_first,
            diffusion_thickness: 0.01,

            status_message,
            export_notice: None,
        };

        // When the app starts, we pick a default material and make sure all the calculator boxes
        // are using that same material. This keeps the UI from showing mismatched or stale values.
        app.sync_calculator_material(default_first);
        app
    }

    /// Access material reference by index.
    pub fn material(&self, index: Option<usize>) -> Option<&Material> {
        index.and_then(|i| self.database.get(i))
    }

    /// Returns filtered materials matching search query and category filter.
    pub fn filtered_materials(&self) -> Vec<(usize, &Material)> {
        self.database
            .search(&self.search_query, &self.category_filter)
    }

    /// Navigates to Material Details page for the specified material index.
    pub fn inspect_material(&mut self, index: usize) {
        self.selected_material = Some(index);
        self.page = Page::MaterialDetails;
    }

    /// Updates the calculator inputs when the selected material changes.
    ///
    /// Clear values that the new material doesn't have, so old inputs don't stick around.
    pub fn sync_calculator_material(&mut self, material_index: Option<usize>) {
        self.calc_expansion_material = material_index;
        self.calc_heat_material = material_index;
        self.calc_stress_material = material_index;
        self.calc_conduction_material = material_index;
        self.calc_diffusion_material = material_index;

        if let Some(idx) = material_index {
            if let Some(mat) = self.database.get(idx) {
                self.calculator_specific_heat = mat.specific_heat_val();
            } else {
                self.calculator_specific_heat = None;
            }
        } else {
            self.calculator_specific_heat = None;
        }
    }

    /// Re-evaluates materials screening against current constraints.
    pub fn run_screening(&mut self) {
        self.screening_cache = Some(screen_materials(
            &self.database.materials,
            &self.screening_constraints,
        ));
    }

    /// Returns screening results, computing them if not already cached.
    pub fn get_screening_results(&mut self) -> &[ScreeningResult] {
        if self.screening_cache.is_none() {
            self.run_screening();
        }
        self.screening_cache.as_ref().unwrap()
    }

    /// Re-computes material rankings for the current metric.
    pub fn run_rankings(&mut self) {
        self.ranking_cache = Some(rank_materials(
            &self.database.materials,
            self.ranking_metric,
        ));
    }

    /// Returns ranked items, computing them if not already cached.
    pub fn get_rankings(&mut self) -> &[RankedMaterialItem] {
        if self.ranking_cache.is_none() {
            self.run_rankings();
        }
        self.ranking_cache.as_ref().unwrap()
    }
}

impl eframe::App for MaterialSpaceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::ui::render(self, ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Property;

    fn make_mock_app() -> MaterialSpaceApp {
        let mat_with_cp = Material {
            id: "mat_cp".into(),
            name: "Material With Cp".into(),
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
            specific_heat: Some(Property {
                value: 900.0,
                unit: Some("J/(kg*K)".into()),
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
            thermal_expansion: None,
            emissivity: None,
            melting_point: None,
            boiling_point: None,
        };

        let mat_without_cp = Material {
            id: "mat_nocp".into(),
            name: "Material Without Cp".into(),
            category: "ceramic".into(),
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
            specific_heat: None, // Missing!
            thermal_expansion: None,
            emissivity: None,
            melting_point: None,
            boiling_point: None,
        };

        let db = Database {
            materials: vec![mat_with_cp, mat_without_cp],
            sources: std::collections::HashMap::new(),
            cache_manager: crate::retrieval::LocalCacheManager::new("."),
            is_from_embedded_fallback: true,
        };

        MaterialSpaceApp {
            database: db,
            page: Page::Dashboard,
            search_query: String::new(),
            category_filter: "All".into(),
            selected_material: None,
            comparison_a: None,
            comparison_b: None,
            screening_constraints: Constraints::default(),
            selected_preset: None,
            screening_tab: ScreeningTab::Passed,
            screening_cache: None,
            ranking_metric: RankingMetric::AerospaceScore,
            ranking_cache: None,
            ambient_temperature_k: 293.15,
            calc_expansion_material: None,
            initial_length: 1.0,
            delta_temperature: 100.0,
            calc_heat_material: None,
            calculator_mass: 1.0,
            calculator_specific_heat: None,
            heat_delta_temperature: 50.0,
            calc_stress_material: None,
            stress_delta_temperature: 100.0,
            calc_conduction_material: None,
            conduction_area: 1.0,
            conduction_thickness: 0.02,
            conduction_delta_t: 50.0,
            calc_diffusion_material: None,
            diffusion_thickness: 0.01,
            status_message: None,
            export_notice: None,
        }
    }

    #[test]
    fn test_calculator_synchronization_regression() {
        let mut app = make_mock_app();

        // 1. Select material with specific heat
        app.sync_calculator_material(Some(0));
        assert_eq!(
            app.calculator_specific_heat,
            Some(900.0),
            "Calculator specific heat must synchronize to 900.0"
        );

        // 2. Select material WITHOUT specific heat
        app.sync_calculator_material(Some(1));
        assert_eq!(
            app.calculator_specific_heat, None,
            "Calculator specific heat must be cleared to None to prevent using stale properties"
        );

        // 3. Clear material selection
        app.sync_calculator_material(None);
        assert_eq!(app.calculator_specific_heat, None);
    }
}
