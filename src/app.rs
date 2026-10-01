use crate::database::Database;
use crate::material::Material;
use crate::metrics::RankingMetric;

/// The primary navigation pages in MaterialSpace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Materials,
    Compare,
    MaterialDetails,
    Rankings,
    Calculator,
    About,
}

/// Central application state for MaterialSpace.
///
/// Holds the loaded material database, current navigation view,
/// active selections, search filters, calculator parameters, and ranking configurations.
pub struct MaterialSpaceApp {
    /// In-memory materials and sources database.
    pub database: Database,

    /// Currently active view page.
    pub page: Page,

    /// Material search query string (matches name, id, composition, grade, description).
    pub search_query: String,

    /// Selected category filter (e.g. "All", "metal", "ceramic", "polymer", "composite").
    pub category_filter: String,

    /// Index of the material currently inspected in the Material Details page.
    pub selected_material: Option<usize>,

    /// Index of the first material in the side-by-side comparison view.
    pub comparison_a: Option<usize>,

    /// Index of the second material in the side-by-side comparison view.
    pub comparison_b: Option<usize>,

    /// Selected metric for material rankings.
    pub ranking_metric: RankingMetric,

    // --- Calculator State ---
    /// Selected material for thermal expansion calculator.
    pub calc_expansion_material: Option<usize>,
    /// Initial length L₀ in meters for thermal expansion calculation.
    pub initial_length: f64,
    /// Temperature change ΔT in Kelvin for thermal expansion calculation.
    pub delta_temperature: f64,

    /// Selected material for heat energy calculator (auto-populates specific heat).
    pub calc_heat_material: Option<usize>,
    /// Mass in kilograms for heat energy calculation.
    pub calculator_mass: f64,
    /// Specific heat capacity c in J/(kg·K) for heat energy calculation.
    pub calculator_specific_heat: f64,
    /// Temperature change ΔT in Kelvin for heat energy calculation.
    pub heat_delta_temperature: f64,

    /// Selected material for thermal stress calculator.
    pub calc_stress_material: Option<usize>,
    /// Temperature change ΔT in Kelvin for thermal stress calculation.
    pub stress_delta_temperature: f64,

    /// Any startup error or notification message to display.
    pub status_message: Option<String>,
}

impl MaterialSpaceApp {
    /// Initializes application state, loads normalized datasets, and configures defaults.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (database, status_message) = match Database::load() {
            Ok(db) => (db, None),
            Err(err) => {
                eprintln!("Database load error: {}", err);
                (
                    Database {
                        materials: Vec::new(),
                        sources: std::collections::HashMap::new(),
                    },
                    Some(format!("Database initialization warning: {}", err)),
                )
            }
        };

        // Pick default initial selections if materials are loaded
        let default_first = if !database.materials.is_empty() { Some(0) } else { None };
        let default_second = if database.materials.len() > 1 { Some(1) } else { default_first };

        Self {
            database,
            page: Page::Dashboard,
            search_query: String::new(),
            category_filter: "All".to_string(),
            selected_material: default_first,
            comparison_a: default_first,
            comparison_b: default_second,
            ranking_metric: RankingMetric::AerospaceScore,

            calc_expansion_material: default_first,
            initial_length: 1.0,
            delta_temperature: 100.0,

            calc_heat_material: default_first,
            calculator_mass: 1.0,
            calculator_specific_heat: 900.0,
            heat_delta_temperature: 50.0,

            calc_stress_material: default_first,
            stress_delta_temperature: 100.0,

            status_message,
        }
    }

    /// Convenience helper to access a material reference by its database index.
    pub fn material(&self, index: Option<usize>) -> Option<&Material> {
        index.and_then(|i| self.database.get(i))
    }

    /// Returns the filtered list of materials matching current search and category criteria.
    pub fn filtered_materials(&self) -> Vec<(usize, &Material)> {
        self.database.search(&self.search_query, &self.category_filter)
    }

    /// Helper to navigate directly to the Material Details page for a specific material index.
    pub fn inspect_material(&mut self, index: usize) {
        self.selected_material = Some(index);
        self.page = Page::MaterialDetails;
    }
}

impl eframe::App for MaterialSpaceApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::ui::render(self, ui);
    }
}