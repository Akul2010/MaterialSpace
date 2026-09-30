use crate::database::Database;
use crate::material::Material;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    Materials,
    Compare,
    Selection,
    Calculator,
    About,
}

pub struct MaterialSpaceApp {
    pub database: Database,

    pub page: Page,

    pub search_query: String,
    pub category_filter: String,

    pub selected_material: Option<usize>,

    pub comparison_a: Option<usize>,
    pub comparison_b: Option<usize>,

    pub calculator_material: Option<usize>,

    pub initial_length: f64,
    pub delta_temperature: f64,

    pub calculator_mass: f64,
    pub calculator_specific_heat: f64,
}

impl MaterialSpaceApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
    ) -> Self {
        let database = match Database::load() {
            Ok(database) => database,

            Err(error) => {
                eprintln!("Database error: {}", error);

                Database {
                    materials: Vec::new(),
                }
            }
        };

        Self {
            database,

            page: Page::Dashboard,

            search_query: String::new(),
            category_filter: "All".to_string(),

            selected_material: None,

            comparison_a: None,
            comparison_b: None,

            calculator_material: None,

            initial_length: 1.0,
            delta_temperature: 100.0,

            calculator_mass: 1.0,
            calculator_specific_heat: 1000.0,
        }
    }

    pub fn material(
        &self,
        index: Option<usize>,
    ) -> Option<&Material> {
        index.and_then(|i| self.database.get(i))
    }

    pub fn filtered_materials(
        &self,
    ) -> Vec<(usize, &Material)> {
        let query = self
            .search_query
            .trim()
            .to_lowercase();

        self.database
            .materials
            .iter()
            .enumerate()
            .filter(|(_, material)| {
                let category_matches =
                    self.category_filter == "All"
                        || material
                            .category
                            .eq_ignore_ascii_case(
                                &self.category_filter,
                            );

                if !category_matches {
                    return false;
                }

                if query.is_empty() {
                    return true;
                }

                material
                    .name
                    .to_lowercase()
                    .contains(&query)
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
            })
            .collect()
    }
}

impl eframe::App for MaterialSpaceApp {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        crate::ui::render(self, ui);
    }
}