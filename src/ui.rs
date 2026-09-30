use crate::app::{MaterialSpaceApp, Page};
use crate::material::Material;
use crate::physics;

pub fn render(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {

    egui::Panel::left("sidebar")
        .resizable(false)
        .default_size(220.0)
        .show(ui, |ui| {
            sidebar(app, ui);
        });

    egui::Panel::top("topbar")
        .resizable(false)
        .default_size(65.0)
        .show(ui, |ui| {
            topbar(app, ui);
        });

    egui::CentralPanel::default().show(ui, |ui| {
        match app.page {
            Page::Dashboard => dashboard(app, ui),
            Page::Materials => materials(app, ui),
            Page::Compare => comparison(app, ui),
            Page::Selection => selection(app, ui),
            Page::Calculator => calculator(app, ui),
            Page::About => about(ui),
        }
    });
}

fn sidebar(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(20.0);

    ui.heading("MaterialSpace");

    ui.label("Materials intelligence");

    ui.add_space(20.0);

    nav_button(app, ui, Page::Dashboard, "Dashboard");
    nav_button(app, ui, Page::Materials, "Materials");
    nav_button(app, ui, Page::Compare, "Compare");
    nav_button(app, ui, Page::Selection, "Material Details");
    nav_button(app, ui, Page::Calculator, "Calculator");

    ui.add_space(15.0);

    ui.separator();

    ui.add_space(10.0);

    nav_button(app, ui, Page::About, "About");

    ui.add_space(20.0);

    ui.separator();

    ui.label(format!(
        "{} materials loaded",
        app.database.materials.len()
    ));
}

fn nav_button(
    app: &mut MaterialSpaceApp,
    ui: &mut egui::Ui,
    page: Page,
    text: &str,
) {
    if ui.selectable_label(app.page == page, text).clicked() {
        app.page = page;
    }
}

fn topbar(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let title = match app.page {
            Page::Dashboard => "Dashboard",
            Page::Materials => "Materials Database",
            Page::Compare => "Material Comparison",
            Page::Selection => "Material Details",
            Page::Calculator => "Engineering Calculator",
            Page::About => "About MaterialSpace",
        };

        ui.heading(title);

        if app.page == Page::Materials {
            ui.add_space(30.0);

            ui.add(
                egui::TextEdit::singleline(&mut app.search_query)
                    .hint_text("Search materials")
                    .desired_width(300.0),
            );
        }
    });
}

fn dashboard(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.heading("MaterialSpace");

    ui.add_space(5.0);

    ui.label(
        "Explore, compare, and analyze engineering materials.",
    );

    ui.add_space(25.0);

    ui.horizontal(|ui| {
        stat_card(
            ui,
            "Materials",
            app.database.materials.len(),
        );

        stat_card(
            ui,
            "Categories",
            app.database.categories().len(),
        );

        let thermal_count = app
            .database
            .materials
            .iter()
            .filter(|material| material.thermal_conductivity.is_some())
            .count();

        stat_card(ui, "Thermal Data", thermal_count);
    });

    ui.add_space(25.0);

    ui.group(|ui| {
        ui.heading("What is MaterialSpace?");

        ui.add_space(8.0);

        ui.label(
            "MaterialSpace is a materials engineering platform \
             for exploring normalized materials data, comparing \
             engineering properties, and performing physics-based \
             calculations.",
        );

        ui.add_space(8.0);

        ui.label(
            "Use Materials to search the database, Compare to \
             examine two materials side-by-side, and Calculator \
             to perform basic thermal calculations.",
        );
    });
}

fn stat_card(ui: &mut egui::Ui, title: &str, value: usize) {
    ui.group(|ui| {
        ui.set_min_width(170.0);

        ui.label(title);

        ui.add_space(5.0);

        ui.heading(value.to_string());
    });
}

fn materials(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.heading("Materials Database");

    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Category:");

        egui::ComboBox::from_id_salt("category_filter")
            .selected_text(app.category_filter.clone())
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        app.category_filter == "All",
                        "All",
                    )
                    .clicked()
                {
                    app.category_filter = "All".to_string();
                }

                let categories = app.database.categories();

                for category in categories {
                    if ui
                        .selectable_label(
                            app.category_filter == category,
                            &category,
                        )
                        .clicked()
                    {
                        app.category_filter = category;
                    }
                }
            });
    });

    ui.add_space(15.0);

    /*
     * We clone the material indexes and the material data here.
     *
     * This is intentional. `material_row()` can then mutably
     * borrow `app` without simultaneously holding references
     * into `app.database`.
     */
    let results: Vec<(usize, Material)> = app
        .filtered_materials()
        .into_iter()
        .map(|(index, material)| {
            (index, material.clone())
        })
        .collect();

    ui.label(format!(
        "{} materials found",
        results.len()
    ));

    ui.add_space(10.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (index, material) in results.iter() {
            material_row(
                app,
                ui,
                *index,
                material,
            );
        }
    });
}

fn material_row(
    app: &mut MaterialSpaceApp,
    ui: &mut egui::Ui,
    index: usize,
    material: &Material,
) {
    let name = material.name.clone();
    let category = material.category.clone();
    let density = material.density;
    let conductivity = material.thermal_conductivity;

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.strong(name);
                ui.label(category);
            });

            ui.separator();

            property_label(ui, "Density", density, "kg/m³");

            property_label(
                ui,
                "Conductivity",
                conductivity,
                "W/(m·K)",
            );

            ui.add_space(15.0);

            if ui.button("View").clicked() {
                app.selected_material = Some(index);
                app.page = Page::Selection;
            }
        });
    });

    ui.add_space(5.0);
}

fn property_label(
    ui: &mut egui::Ui,
    name: &str,
    value: Option<f64>,
    unit: &str,
) {
    ui.vertical(|ui| {
        ui.label(name);

        match value {
            Some(value) => {
                ui.label(format!("{:.4} {}", value, unit));
            }
            None => {
                ui.label("N/A");
            }
        }
    });
}

fn selection(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.heading("Material Details");

    ui.add_space(15.0);

    let index = match app.selected_material {
        Some(index) => index,
        None => {
            ui.label("Select a material from the Materials page.");
            return;
        }
    };

    let material = match app.database.get(index) {
        Some(material) => material,
        None => {
            ui.label("Material not found.");
            return;
        }
    };

    ui.heading(&material.name);

    ui.label(&material.id);

    ui.add_space(15.0);

    if let Some(description) = &material.description {
        ui.group(|ui| {
            ui.label(description);
        });

        ui.add_space(15.0);
    }

    egui::Grid::new("material_information")
        .num_columns(2)
        .spacing([30.0, 8.0])
        .show(ui, |ui| {
            detail_row(ui, "Category", &material.category);

            detail_row(
                ui,
                "Composition",
                material.composition.as_deref().unwrap_or("N/A"),
            );

            detail_row(
                ui,
                "Grade",
                material.grade.as_deref().unwrap_or("N/A"),
            );
        });

    ui.add_space(20.0);

    ui.heading("Properties");

    ui.add_space(10.0);

    property_grid(ui, material);

    ui.add_space(20.0);

    ui.heading("Sources");

    for source in &material.source_ids {
        ui.label(format!("• {}", source));
    }
}

fn detail_row(ui: &mut egui::Ui, name: &str, value: &str) {
    ui.strong(name);
    ui.label(value);
    ui.end_row();
}

fn property_grid(ui: &mut egui::Ui, material: &Material) {
    egui::Grid::new("property_grid")
        .striped(true)
        .spacing([35.0, 8.0])
        .show(ui, |ui| {
            property_grid_row(
                ui,
                "Density",
                material.density,
                "kg/m³",
            );

            property_grid_row(
                ui,
                "Thermal Conductivity",
                material.thermal_conductivity,
                "W/(m·K)",
            );

            property_grid_row(
                ui,
                "Specific Heat",
                material.specific_heat,
                "J/(kg·K)",
            );

            property_grid_row(
                ui,
                "Thermal Expansion",
                material.thermal_expansion,
                "1/K",
            );

            property_grid_row(
                ui,
                "Young's Modulus",
                material.youngs_modulus,
                "Pa",
            );

            property_grid_row(
                ui,
                "Yield Strength",
                material.yield_strength,
                "Pa",
            );

            property_grid_row(
                ui,
                "Tensile Strength",
                material.tensile_strength,
                "Pa",
            );

            property_grid_row(
                ui,
                "Emissivity",
                material.emissivity,
                "",
            );

            property_grid_row(
                ui,
                "Melting Point",
                material.melting_point,
                "K",
            );

            property_grid_row(
                ui,
                "Boiling Point",
                material.boiling_point,
                "K",
            );
        });
}

fn property_grid_row(
    ui: &mut egui::Ui,
    name: &str,
    value: Option<f64>,
    unit: &str,
) {
    ui.label(name);

    match value {
        Some(value) => {
            ui.label(format!("{:.6} {}", value, unit));
        }
        None => {
            ui.label("Data unavailable");
        }
    }

    ui.end_row();
}

fn comparison(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.heading("Material Comparison");

    ui.add_space(15.0);

    material_selector(app, ui, "Material A", true);

    ui.add_space(10.0);

    material_selector(app, ui, "Material B", false);

    ui.add_space(20.0);

    let material_a = app
        .comparison_a
        .and_then(|index| app.database.get(index))
        .cloned();

    let material_b = app
        .comparison_b
        .and_then(|index| app.database.get(index))
        .cloned();

    match (material_a, material_b) {
        (Some(a), Some(b)) => {
            comparison_table(ui, &a, &b);
        }

        _ => {
            ui.label("Select two materials to compare.");
        }
    }
}

fn material_selector(
    app: &mut MaterialSpaceApp,
    ui: &mut egui::Ui,
    label: &str,
    first: bool,
) {
    let current = if first {
        app.comparison_a
    } else {
        app.comparison_b
    };

    let current_name = match current {
        Some(index) => match app.database.get(index) {
            Some(material) => material.name.clone(),
            None => "Select material".to_string(),
        },
        None => "Select material".to_string(),
    };

    ui.horizontal(|ui| {
        ui.label(label);

        egui::ComboBox::from_id_salt(label)
            .selected_text(current_name)
            .width(300.0)
            .show_ui(ui, |ui| {
                for index in 0..app.database.materials.len() {
                    let name =
                        app.database.materials[index].name.clone();

                    if ui
                        .selectable_label(
                            current == Some(index),
                            name,
                        )
                        .clicked()
                    {
                        if first {
                            app.comparison_a = Some(index);
                        } else {
                            app.comparison_b = Some(index);
                        }
                    }
                }
            });
    });
}

fn comparison_table(
    ui: &mut egui::Ui,
    a: &Material,
    b: &Material,
) {
    egui::Grid::new("comparison_table")
        .striped(true)
        .spacing([30.0, 10.0])
        .show(ui, |ui| {
            ui.strong("Property");
            ui.strong(&a.name);
            ui.strong(&b.name);
            ui.end_row();

            compare_row(
                ui,
                "Density",
                a.density,
                b.density,
                "kg/m³",
            );

            compare_row(
                ui,
                "Thermal Conductivity",
                a.thermal_conductivity,
                b.thermal_conductivity,
                "W/(m·K)",
            );

            compare_row(
                ui,
                "Specific Heat",
                a.specific_heat,
                b.specific_heat,
                "J/(kg·K)",
            );

            compare_row(
                ui,
                "Thermal Expansion",
                a.thermal_expansion,
                b.thermal_expansion,
                "1/K",
            );

            compare_row(
                ui,
                "Young's Modulus",
                a.youngs_modulus,
                b.youngs_modulus,
                "Pa",
            );

            compare_row(
                ui,
                "Yield Strength",
                a.yield_strength,
                b.yield_strength,
                "Pa",
            );

            compare_row(
                ui,
                "Tensile Strength",
                a.tensile_strength,
                b.tensile_strength,
                "Pa",
            );

            compare_row(
                ui,
                "Melting Point",
                a.melting_point,
                b.melting_point,
                "K",
            );
        });
}

fn compare_row(
    ui: &mut egui::Ui,
    name: &str,
    a: Option<f64>,
    b: Option<f64>,
    unit: &str,
) {
    ui.label(name);

    value_cell(ui, a, unit);
    value_cell(ui, b, unit);

    ui.end_row();
}

fn value_cell(
    ui: &mut egui::Ui,
    value: Option<f64>,
    unit: &str,
) {
    match value {
        Some(value) => {
            ui.label(format!("{:.5} {}", value, unit));
        }
        None => {
            ui.label("N/A");
        }
    }
}

fn calculator(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.heading("Engineering Calculator");

    ui.add_space(10.0);

    ui.label(
        "Basic thermal and materials calculations.",
    );

    ui.add_space(20.0);

    ui.group(|ui| {
        ui.heading("Thermal Expansion");

        ui.add_space(10.0);

        calculator_material_selector(app, ui);

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Initial length (m):");

            ui.add(
                egui::DragValue::new(
                    &mut app.initial_length,
                )
                .speed(0.01),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Temperature change (K):");

            ui.add(
                egui::DragValue::new(
                    &mut app.delta_temperature,
                )
                .speed(1.0),
            );
        });

        ui.add_space(10.0);

        match app.calculator_material {
            Some(index) => {
                match app.database.get(index) {
                    Some(material) => {
                        match physics::thermal_expansion(
                            material,
                            app.initial_length,
                            app.delta_temperature,
                        ) {
                            Some(delta_length) => {
                                ui.label(format!(
                                    "Change in length: {:.8} m",
                                    delta_length
                                ));
                            }

                            None => {
                                ui.label(
                                    "Thermal expansion data unavailable.",
                                );
                            }
                        }
                    }

                    None => {
                        ui.label("Material not found.");
                    }
                }
            }

            None => {
                ui.label("Select a material.");
            }
        }
    });

    ui.add_space(20.0);

    ui.group(|ui| {
        ui.heading("Heat Energy");

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Mass (kg):");

            ui.add(
                egui::DragValue::new(
                    &mut app.calculator_mass,
                )
                .speed(0.1),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Specific heat (J/kg·K):");

            ui.add(
                egui::DragValue::new(
                    &mut app.calculator_specific_heat,
                )
                .speed(10.0),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Temperature change (K):");

            ui.add(
                egui::DragValue::new(
                    &mut app.delta_temperature,
                )
                .speed(1.0),
            );
        });

        let energy = physics::heat_energy(
            app.calculator_mass,
            app.calculator_specific_heat,
            app.delta_temperature,
        );

        ui.add_space(10.0);

        ui.label(format!(
            "Required energy: {:.3} J",
            energy
        ));
    });
}

fn calculator_material_selector(
    app: &mut MaterialSpaceApp,
    ui: &mut egui::Ui,
) {
    let current_name = match app.calculator_material {
        Some(index) => match app.database.get(index) {
            Some(material) => material.name.clone(),
            None => "Select material".to_string(),
        },
        None => "Select material".to_string(),
    };

    ui.horizontal(|ui| {
        ui.label("Material:");

        egui::ComboBox::from_id_salt(
            "calculator_material",
        )
        .selected_text(current_name)
        .width(300.0)
        .show_ui(ui, |ui| {
            for index in 0..app.database.materials.len() {
                let name =
                    app.database.materials[index]
                        .name
                        .clone();

                if ui
                    .selectable_label(
                        app.calculator_material == Some(index),
                        name,
                    )
                    .clicked()
                {
                    app.calculator_material = Some(index);
                }
            }
        });
    });
}

fn about(ui: &mut egui::Ui) {
    ui.heading("About MaterialSpace");

    ui.add_space(15.0);

    ui.label(
        "MaterialSpace is a materials engineering \
         exploration and analysis application.",
    );

    ui.add_space(15.0);

    ui.group(|ui| {
        ui.heading("Data");

        ui.add_space(5.0);

        ui.label(
            "MaterialSpace uses normalized material \
             records derived from the project's source databases.",
        );

        ui.add_space(8.0);

        ui.label(
            "Sources represented in the project include \
             NASA TPSX, NIST Chemistry WebBook, MatWeb, \
             and ASM Handbook.",
        );
    });

    ui.add_space(15.0);

    ui.group(|ui| {
        ui.heading("Current capabilities");

        ui.add_space(5.0);

        ui.label("• Material database");
        ui.label("• Material search");
        ui.label("• Category filtering");
        ui.label("• Material property inspection");
        ui.label("• Side-by-side comparison");
        ui.label("• Thermal expansion calculation");
        ui.label("• Heat-energy calculation");
    });

    ui.add_space(15.0);

    ui.label(
        "MaterialSpace is designed to make materials \
         engineering data easier to explore and use.",
    );
}