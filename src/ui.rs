use crate::app::{MaterialSpaceApp, Page};
use crate::material::{Material, Property};
use crate::metrics::{
    aerospace_suitability_score, generate_observations, rank_materials, specific_stiffness,
    specific_strength, thermal_diffusivity, RankingMetric,
};
use crate::physics;

/// Master rendering function called once per frame.
pub fn render(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    // Left navigation sidebar panel
    egui::Panel::left("navigation_sidebar")
        .resizable(false)
        .default_size(240.0)
        .show(ui, |ui| {
            sidebar_ui(app, ui);
        });

    // Top header panel
    egui::Panel::top("top_header_panel")
        .resizable(false)
        .default_size(60.0)
        .show(ui, |ui| {
            topbar_ui(app, ui);
        });

    // Central workspace panel rendering the active page
    egui::CentralPanel::default().show(ui, |ui| {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| match app.page {
                Page::Dashboard => dashboard_ui(app, ui),
                Page::Materials => materials_ui(app, ui),
                Page::Compare => compare_ui(app, ui),
                Page::MaterialDetails => details_ui(app, ui),
                Page::Rankings => rankings_ui(app, ui),
                Page::Calculator => calculator_ui(app, ui),
                Page::About => about_ui(ui),
            });
    });
}

// ----------------------------------------------------------------------------
// Sidebar & Topbar UI
// ----------------------------------------------------------------------------

fn sidebar_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(16.0);

    ui.heading("🔬 MaterialSpace");
    ui.label("Materials Intelligence & Physics");

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label("NAVIGATION");
    ui.add_space(6.0);

    nav_item(app, ui, Page::Dashboard, "📊  Dashboard");
    nav_item(app, ui, Page::Materials, "📚  Materials Database");
    nav_item(app, ui, Page::Compare, "⚖️  Compare Materials");
    nav_item(app, ui, Page::MaterialDetails, "🔍  Material Details");
    nav_item(app, ui, Page::Rankings, "🏆  Engineering Rankings");
    nav_item(app, ui, Page::Calculator, "🧮  Physics Calculator");

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(10.0);

    nav_item(app, ui, Page::About, "ℹ️  About & Methodology");

    ui.add_space(24.0);
    ui.separator();
    ui.add_space(12.0);

    // Database Status Summary
    ui.label("DATABASE STATUS");
    ui.add_space(4.0);
    ui.label(format!("• Materials: {}", app.database.materials.len()));
    ui.label(format!("• Categories: {}", app.database.categories().len()));
    ui.label(format!("• Verified Sources: {}", app.database.sources.len()));

    if let Some(err) = &app.status_message {
        ui.add_space(10.0);
        ui.colored_label(egui::Color32::from_rgb(255, 180, 50), err);
    }
}

fn nav_item(app: &mut MaterialSpaceApp, ui: &mut egui::Ui, page: Page, title: &str) {
    let is_selected = app.page == page;
    let button = egui::Button::new(title)
        .min_size(egui::vec2(ui.available_width(), 32.0))
        .selected(is_selected);

    if ui.add(button).clicked() {
        app.page = page;
    }
    ui.add_space(2.0);
}

fn topbar_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.horizontal_centered(|ui| {
        let (title, subtitle) = match app.page {
            Page::Dashboard => (
                "Engineering Dashboard",
                "Overview of materials datasets, thermophysical records, and quick tools",
            ),
            Page::Materials => (
                "Materials Explorer",
                "Browse, search, and filter normalized engineering materials",
            ),
            Page::Compare => (
                "Material Comparison",
                "Direct side-by-side comparison of mechanical, thermal, and derived properties",
            ),
            Page::MaterialDetails => (
                "Material Inspection",
                "Comprehensive material datasheet with provenance, testing conditions, and heuristics",
            ),
            Page::Rankings => (
                "Engineering Rankings",
                "Transparent criteria-based ranking across specific engineering metrics",
            ),
            Page::Calculator => (
                "Engineering Physics Calculator",
                "Interactive physics calculators for thermal expansion, heat transfer, and thermal stress",
            ),
            Page::About => (
                "About MaterialSpace",
                "Methodology, data provenance, scientific honesty, and architectural overview",
            ),
        };

        ui.vertical(|ui| {
            ui.heading(title);
            ui.label(subtitle);
        });

        // Search input shortcut directly on Materials page
        if app.page == Page::Materials {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut app.search_query)
                        .hint_text("🔍 Search by name, ID, composition...")
                        .desired_width(280.0),
                );
            });
        }
    });
}

// ----------------------------------------------------------------------------
// 1. Dashboard Page
// ----------------------------------------------------------------------------

fn dashboard_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(10.0);

    // Welcome Header
    ui.group(|ui| {
        ui.heading("Welcome to MaterialSpace");
        ui.add_space(6.0);
        ui.label(
            "MaterialSpace is a materials engineering exploration, analysis, and comparison platform \
             built in Rust. It enables students and engineers to inspect verified materials data, calculate \
             physical responses, evaluate derived engineering figures of merit, and understand materials selection tradeoffs.",
        );
    });

    ui.add_space(16.0);

    // High-Level Statistics Cards
    ui.label("SYSTEM OVERVIEW");
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        stat_card(
            ui,
            "Total Materials",
            &app.database.materials.len().to_string(),
            "Normalized records",
        );
        stat_card(
            ui,
            "Categories",
            &app.database.categories().len().to_string(),
            "Metals, ceramics, polymers...",
        );
        stat_card(
            ui,
            "Thermal Records",
            &app.database.count_with_thermal_data().to_string(),
            "k, Cp, α, melting point",
        );
        stat_card(
            ui,
            "Mechanical Records",
            &app.database.count_with_mechanical_data().to_string(),
            "E, yield, tensile strength",
        );
    });

    ui.add_space(24.0);

    // Quick Action Cards
    ui.label("QUICK WORKFLOWS");
    ui.add_space(6.0);

    ui.columns(3, |columns| {
        columns[0].group(|ui| {
            ui.heading("📚 Explore Database");
            ui.add_space(4.0);
            ui.label("Search across 50+ aerospace alloys, refractory ceramics, and structural polymers.");
            ui.add_space(8.0);
            if ui.button("Open Materials →").clicked() {
                app.page = Page::Materials;
            }
        });

        columns[1].group(|ui| {
            ui.heading("⚖️ Compare Materials");
            ui.add_space(4.0);
            ui.label("Side-by-side property comparison with relative performance visualization bars.");
            ui.add_space(8.0);
            if ui.button("Start Comparison →").clicked() {
                app.page = Page::Compare;
            }
        });

        columns[2].group(|ui| {
            ui.heading("🧮 Physics Calculator");
            ui.add_space(4.0);
            ui.label("Calculate thermal expansion, required heat energy, and fully constrained thermal stress.");
            ui.add_space(8.0);
            if ui.button("Launch Calculator →").clicked() {
                app.page = Page::Calculator;
            }
        });
    });

    ui.add_space(24.0);

    // Verified Sources Overview
    ui.group(|ui| {
        ui.heading("Authoritative Data Provenance");
        ui.add_space(6.0);
        ui.label(
            "Every normalized property in MaterialSpace is traceable to an established scientific database:",
        );
        ui.add_space(8.0);

        for (id, source) in &app.database.sources {
            ui.horizontal(|ui| {
                ui.strong(format!("• {} ({})", source.organization, id));
                ui.label(format!("— {}", source.title));
            });
        }
    });
}

fn stat_card(ui: &mut egui::Ui, title: &str, value: &str, subtitle: &str) {
    ui.group(|ui| {
        ui.set_min_width(175.0);
        ui.vertical(|ui| {
            ui.label(title);
            ui.add_space(2.0);
            ui.heading(value);
            ui.add_space(2.0);
            ui.label(subtitle);
        });
    });
}

// ----------------------------------------------------------------------------
// 2. Materials Database Explorer Page
// ----------------------------------------------------------------------------

fn materials_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(8.0);

    // Filter Controls Bar
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Category Filter:");

            egui::ComboBox::from_id_salt("materials_category_filter")
                .selected_text(&app.category_filter)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(app.category_filter == "All", "All Categories").clicked() {
                        app.category_filter = "All".to_string();
                    }
                    for cat in app.database.categories() {
                        if ui.selectable_label(app.category_filter == cat, &cat).clicked() {
                            app.category_filter = cat;
                        }
                    }
                });

            ui.add_space(20.0);

            if !app.search_query.is_empty() || app.category_filter != "All" {
                if ui.button("Reset Filters").clicked() {
                    app.search_query.clear();
                    app.category_filter = "All".to_string();
                }
            }
        });
    });

    ui.add_space(10.0);

    /*
     * We clone the filtered material search items here to avoid borrow conflicts
     * between rendering closures and modifying app navigation state.
     */
    let results: Vec<(usize, Material)> = app
        .filtered_materials()
        .into_iter()
        .map(|(idx, mat)| (idx, mat.clone()))
        .collect();

    ui.horizontal(|ui| {
        ui.strong(format!("Showing {} materials", results.len()));
        if !app.search_query.is_empty() {
            ui.label(format!("matching \"{}\"", app.search_query));
        }
    });

    ui.add_space(10.0);

    // Material Cards List
    for (index, material) in results.iter() {
        material_card(app, ui, *index, material);
        ui.add_space(6.0);
    }
}

fn material_card(
    app: &mut MaterialSpaceApp,
    ui: &mut egui::Ui,
    index: usize,
    material: &Material,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            // Material Identity
            ui.vertical(|ui| {
                ui.set_width(260.0);
                ui.heading(&material.name);
                ui.horizontal(|ui| {
                    ui.label(format!("ID: {}", material.id));
                    ui.label("•");
                    ui.strong(&material.category);
                });
                if let Some(comp) = &material.composition {
                    ui.label(format!("Comp: {}", comp));
                }
            });

            ui.separator();

            // Quick Properties Grid
            ui.vertical(|ui| {
                ui.set_width(380.0);
                ui.horizontal(|ui| {
                    property_badge(ui, "Density", material.density_val(), "kg/m³", "{:.1}");
                    property_badge(
                        ui,
                        "Conductivity",
                        material.thermal_conductivity_val(),
                        "W/(m·K)",
                        "{:.1}",
                    );
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let strength = material
                        .tensile_strength_val()
                        .or_else(|| material.yield_strength_val())
                        .map(|v| v / 1.0e6);
                    property_badge(ui, "Strength", strength, "MPa", "{:.0}");
                    property_badge(ui, "Melting Pt", material.melting_point_val(), "K", "{:.0}");
                });
            });

            ui.separator();

            // Action Buttons
            ui.vertical_centered(|ui| {
                if ui.button("🔍 Inspect Details").clicked() {
                    app.inspect_material(index);
                }
                ui.add_space(4.0);
                if ui.button("⚖️ Compare").clicked() {
                    app.comparison_a = Some(index);
                    app.page = Page::Compare;
                }
            });
        });
    });
}

fn property_badge(
    ui: &mut egui::Ui,
    label: &str,
    value: Option<f64>,
    unit: &str,
    format_str: &str,
) {
    ui.vertical(|ui| {
        ui.label(label);
        match value {
            Some(v) => {
                let formatted = if format_str == "{:.0}" {
                    format!("{:.0}", v)
                } else if format_str == "{:.1}" {
                    format!("{:.1}", v)
                } else {
                    format!("{:.2}", v)
                };
                ui.strong(format!("{} {}", formatted, unit));
            }
            None => {
                ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "N/A");
            }
        }
    });
}

// ----------------------------------------------------------------------------
// 3. Side-by-Side Comparison Page
// ----------------------------------------------------------------------------

fn compare_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Material A:");
            comparison_selector(app, ui, true);

            ui.add_space(30.0);

            ui.label("Material B:");
            comparison_selector(app, ui, false);
        });
    });

    ui.add_space(16.0);

    let mat_a = app.material(app.comparison_a).cloned();
    let mat_b = app.material(app.comparison_b).cloned();

    match (mat_a, mat_b) {
        (Some(a), Some(b)) => {
            render_comparison_table(ui, &a, &b);
        }
        _ => {
            ui.group(|ui| {
                ui.label("Please select two materials from the dropdowns above to compare.");
            });
        }
    }
}

fn comparison_selector(app: &mut MaterialSpaceApp, ui: &mut egui::Ui, is_material_a: bool) {
    let current_index = if is_material_a {
        app.comparison_a
    } else {
        app.comparison_b
    };

    let selected_label = current_index
        .and_then(|idx| app.database.get(idx))
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "Select Material".to_string());

    let salt_id = if is_material_a { "comp_sel_a" } else { "comp_sel_b" };

    egui::ComboBox::from_id_salt(salt_id)
        .selected_text(selected_label)
        .width(280.0)
        .show_ui(ui, |ui| {
            for (idx, mat) in app.database.materials.iter().enumerate() {
                let is_sel = current_index == Some(idx);
                if ui.selectable_label(is_sel, &mat.name).clicked() {
                    if is_material_a {
                        app.comparison_a = Some(idx);
                    } else {
                        app.comparison_b = Some(idx);
                    }
                }
            }
        });
}

fn render_comparison_table(ui: &mut egui::Ui, a: &Material, b: &Material) {
    // Header Overview
    ui.group(|ui| {
        ui.columns(2, |cols| {
            cols[0].heading(format!("Material A: {}", a.name));
            cols[0].label(format!("Category: {} | Grade: {}", a.category, a.grade.as_deref().unwrap_or("Standard")));

            cols[1].heading(format!("Material B: {}", b.name));
            cols[1].label(format!("Category: {} | Grade: {}", b.category, b.grade.as_deref().unwrap_or("Standard")));
        });
    });

    ui.add_space(16.0);

    // Mechanical Properties Comparison
    ui.heading("Mechanical Properties");
    ui.add_space(6.0);
    comparison_grid(ui, "mech_comp", |ui| {
        compare_property_row(ui, "Density", a.density_val(), b.density_val(), "kg/m³", false);
        compare_property_row(
            ui,
            "Young's Modulus",
            a.youngs_modulus_val().map(|v| v / 1.0e9),
            b.youngs_modulus_val().map(|v| v / 1.0e9),
            "GPa",
            true,
        );
        compare_property_row(
            ui,
            "Yield Strength",
            a.yield_strength_val().map(|v| v / 1.0e6),
            b.yield_strength_val().map(|v| v / 1.0e6),
            "MPa",
            true,
        );
        compare_property_row(
            ui,
            "Tensile Strength",
            a.tensile_strength_val().map(|v| v / 1.0e6),
            b.tensile_strength_val().map(|v| v / 1.0e6),
            "MPa",
            true,
        );
    });

    ui.add_space(16.0);

    // Thermal Properties Comparison
    ui.heading("Thermal & Thermophysical Properties");
    ui.add_space(6.0);
    comparison_grid(ui, "therm_comp", |ui| {
        compare_property_row(
            ui,
            "Thermal Conductivity",
            a.thermal_conductivity_val(),
            b.thermal_conductivity_val(),
            "W/(m·K)",
            true,
        );
        compare_property_row(
            ui,
            "Specific Heat Capacity",
            a.specific_heat_val(),
            b.specific_heat_val(),
            "J/(kg·K)",
            true,
        );
        compare_property_row(
            ui,
            "Thermal Expansion (CTE)",
            a.thermal_expansion_val().map(|v| v * 1.0e6),
            b.thermal_expansion_val().map(|v| v * 1.0e6),
            "µm/(m·K)",
            false,
        );
        compare_property_row(
            ui,
            "Melting / Solidus Point",
            a.melting_point_val(),
            b.melting_point_val(),
            "K",
            true,
        );
    });

    ui.add_space(16.0);

    // Derived Metrics Comparison
    ui.heading("Derived Engineering Figures of Merit");
    ui.add_space(6.0);
    comparison_grid(ui, "derived_comp", |ui| {
        compare_property_row(
            ui,
            "Specific Strength (σ/ρ)",
            specific_strength(a).map(|v| v / 1000.0),
            specific_strength(b).map(|v| v / 1000.0),
            "kN·m/kg",
            true,
        );
        compare_property_row(
            ui,
            "Specific Stiffness (E/ρ)",
            specific_stiffness(a).map(|v| v / 1.0e6),
            specific_stiffness(b).map(|v| v / 1.0e6),
            "GPa/(g/cm³)",
            true,
        );
        compare_property_row(
            ui,
            "Thermal Diffusivity (α)",
            thermal_diffusivity(a).map(|v| v * 1.0e6),
            thermal_diffusivity(b).map(|v| v * 1.0e6),
            "mm²/s",
            true,
        );
        compare_property_row(
            ui,
            "Aerospace Score (Heuristic)",
            aerospace_suitability_score(a),
            aerospace_suitability_score(b),
            "/ 100",
            true,
        );
    });
}

fn comparison_grid<F>(ui: &mut egui::Ui, id: &str, add_rows: F)
where
    F: FnOnce(&mut egui::Ui),
{
    egui::Grid::new(id)
        .striped(true)
        .min_col_width(180.0)
        .spacing([30.0, 10.0])
        .show(ui, |ui| {
            ui.strong("Property");
            ui.strong("Material A");
            ui.strong("Material B");
            ui.strong("Relative Comparison Bar (A vs B)");
            ui.end_row();

            add_rows(ui);
        });
}

fn compare_property_row(
    ui: &mut egui::Ui,
    name: &str,
    val_a: Option<f64>,
    val_b: Option<f64>,
    unit: &str,
    _higher_is_better: bool,
) {
    ui.label(name);

    // Val A
    match val_a {
        Some(v) => ui.label(format!("{:.3} {}", v, unit)),
        None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
    };

    // Val B
    match val_b {
        Some(v) => ui.label(format!("{:.3} {}", v, unit)),
        None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
    };

    // Visual relative comparison bar
    match (val_a, val_b) {
        (Some(va), Some(vb)) if va > 0.0 && vb > 0.0 => {
            let max = va.max(vb);
            let frac_a = (va / max) as f32;
            let frac_b = (vb / max) as f32;

            ui.horizontal(|ui| {
                ui.label("A:");
                ui.add(egui::ProgressBar::new(frac_a).desired_width(70.0));
                ui.label("B:");
                ui.add(egui::ProgressBar::new(frac_b).desired_width(70.0));
            });
        }
        _ => {
            ui.label("—");
        }
    }

    ui.end_row();
}

// ----------------------------------------------------------------------------
// 4. Material Details Page
// ----------------------------------------------------------------------------

fn details_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    let index = match app.selected_material {
        Some(idx) => idx,
        None => {
            ui.group(|ui| {
                ui.label("No material selected. Browse the Materials page and select 'Inspect Details'.");
            });
            return;
        }
    };

    let material = match app.database.get(index) {
        Some(m) => m.clone(),
        None => {
            ui.label("Selected material not found in database.");
            return;
        }
    };

    ui.add_space(8.0);

    // Header Card
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(&material.name);
                ui.label(format!("ID: {} | Category: {}", material.id, material.category));
                if let Some(grade) = &material.grade {
                    ui.label(format!("Grade / Condition: {}", grade));
                }
                if let Some(comp) = &material.composition {
                    ui.label(format!("Nominal Composition: {}", comp));
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚖️ Compare Material").clicked() {
                    app.comparison_a = Some(index);
                    app.page = Page::Compare;
                }
                ui.add_space(8.0);
                if ui.button("🧮 Use in Calculator").clicked() {
                    app.calc_expansion_material = Some(index);
                    app.calc_heat_material = Some(index);
                    app.calc_stress_material = Some(index);
                    app.page = Page::Calculator;
                }
            });
        });

        if let Some(desc) = &material.description {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);
            ui.label(desc);
        }
    });

    ui.add_space(16.0);

    // Mechanical Properties
    ui.heading("Mechanical Properties");
    ui.add_space(6.0);
    property_detail_table(
        ui,
        "detail_mech_table",
        &[
            ("Density", &material.density),
            ("Young's Modulus (Elasticity)", &material.youngs_modulus),
            ("Yield Strength (0.2% offset)", &material.yield_strength),
            ("Tensile Strength (Ultimate)", &material.tensile_strength),
        ],
    );

    ui.add_space(16.0);

    // Thermal Properties
    ui.heading("Thermal & Thermophysical Properties");
    ui.add_space(6.0);
    property_detail_table(
        ui,
        "detail_therm_table",
        &[
            ("Thermal Conductivity", &material.thermal_conductivity),
            ("Specific Heat Capacity", &material.specific_heat),
            ("Thermal Expansion Coefficient", &material.thermal_expansion),
            ("Thermal Emissivity", &material.emissivity),
            ("Melting / Solidus Temperature", &material.melting_point),
            ("Boiling Temperature", &material.boiling_point),
        ],
    );

    ui.add_space(16.0);

    // Derived Figures of Merit
    ui.heading("Derived Engineering Figures of Merit");
    ui.add_space(6.0);
    ui.group(|ui| {
        egui::Grid::new("detail_derived_grid")
            .striped(true)
            .min_col_width(200.0)
            .spacing([30.0, 8.0])
            .show(ui, |ui| {
                ui.strong("Metric");
                ui.strong("Derived Value");
                ui.strong("Engineering Formula / Interpretation");
                ui.end_row();

                // Specific Strength
                ui.label("Specific Strength");
                match specific_strength(&material) {
                    Some(v) => ui.label(format!("{:.2} kN·m/kg", v / 1000.0)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("σ / ρ (Strength per unit mass)");
                ui.end_row();

                // Specific Stiffness
                ui.label("Specific Stiffness (Modulus)");
                match specific_stiffness(&material) {
                    Some(v) => ui.label(format!("{:.2} GPa/(g/cm³)", v / 1.0e6)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("E / ρ (Deflection resistance per unit mass)");
                ui.end_row();

                // Thermal Diffusivity
                ui.label("Thermal Diffusivity (α)");
                match thermal_diffusivity(&material) {
                    Some(v) => ui.label(format!("{:.3} mm²/s", v * 1.0e6)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("k / (ρ · Cp) (Rate of thermal propagation)");
                ui.end_row();

                // Aerospace Heuristic Score
                ui.label("Aerospace Suitability Score");
                match aerospace_suitability_score(&material) {
                    Some(v) => ui.strong(format!("{:.1} / 100", v)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Insufficient data"),
                };
                ui.label("Heuristic multi-factor index (Not an industrial standard)");
                ui.end_row();
            });
    });

    ui.add_space(16.0);

    // Rule-Based Engineering Observations
    ui.heading("Engineering Observations");
    ui.add_space(6.0);
    ui.group(|ui| {
        ui.label(
            "The following rule-based observations are generated from known engineering property thresholds \
             for exploratory screening and should not replace detailed structural simulation:",
        );
        ui.add_space(6.0);

        let observations = generate_observations(&material);
        for obs in observations {
            ui.label(format!("• {}", obs));
        }
    });

    ui.add_space(16.0);

    // Provenance & Sources
    ui.heading("Data Provenance & Scientific Sources");
    ui.add_space(6.0);
    ui.group(|ui| {
        if material.source_ids.is_empty() {
            ui.label("No source IDs recorded for this material.");
        } else {
            for source_id in &material.source_ids {
                if let Some(source_info) = app.database.get_source(source_id) {
                    ui.vertical(|ui| {
                        ui.strong(format!("• {} ({})", source_info.organization, source_info.source_id));
                        ui.label(format!("  Title: {}", source_info.title));
                        ui.label(format!("  Notes: {}", source_info.notes));
                        ui.label(format!("  Accessed: {}", source_info.accessed));
                        ui.label(format!("  URL: {}", source_info.url));
                        ui.add_space(4.0);
                    });
                } else {
                    ui.label(format!("• {}", source_id));
                }
            }
        }
    });
}

fn property_detail_table(ui: &mut egui::Ui, id: &str, rows: &[(&str, &Option<Property>)]) {
    egui::Grid::new(id)
        .striped(true)
        .min_col_width(180.0)
        .spacing([30.0, 8.0])
        .show(ui, |ui| {
            ui.strong("Property Name");
            ui.strong("Normalized SI Value");
            ui.strong("Condition / State");
            ui.strong("Original Data / Source");
            ui.end_row();

            for (name, prop_opt) in rows {
                ui.label(*name);

                match prop_opt {
                    Some(prop) => {
                        let unit = prop.unit.as_deref().unwrap_or("");
                        ui.strong(format!("{:.5} {}", prop.value, unit));

                        let condition = prop.condition.as_deref().unwrap_or("Standard / Ambient");
                        ui.label(condition);

                        let source = prop.source_id.as_deref().unwrap_or("Normalized Dataset");
                        if let Some(orig_val) = prop.original_value {
                            let orig_unit = prop.original_unit.as_deref().unwrap_or("");
                            ui.label(format!("{} (Orig: {} {})", source, orig_val, orig_unit));
                        } else {
                            ui.label(source);
                        }

                        if let Some(warning) = &prop.normalization_warning {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 180, 50),
                                format!("⚠️ {}", warning),
                            );
                        }
                    }
                    None => {
                        ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable");
                        ui.label("—");
                        ui.label("—");
                    }
                }

                ui.end_row();
            }
        });
}

// ----------------------------------------------------------------------------
// 5. Engineering Rankings Page
// ----------------------------------------------------------------------------

fn rankings_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(8.0);

    // Metric Selection Header
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Ranking Criterion:");

            egui::ComboBox::from_id_salt("ranking_metric_selector")
                .selected_text(app.ranking_metric.name())
                .width(320.0)
                .show_ui(ui, |ui| {
                    for metric in RankingMetric::all() {
                        if ui
                            .selectable_label(app.ranking_metric == *metric, metric.name())
                            .clicked()
                        {
                            app.ranking_metric = *metric;
                        }
                    }
                });
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        ui.label(app.ranking_metric.description());

        if app.ranking_metric == RankingMetric::AerospaceScore {
            ui.add_space(4.0);
            ui.colored_label(
                egui::Color32::from_rgb(255, 200, 80),
                "⚠️ Note: The aerospace score is a transparent heuristic figure of merit, NOT an official aerospace engineering standard.",
            );
        }
    });

    ui.add_space(16.0);

    // Evaluate and sort materials
    let ranked_items = rank_materials(&app.database.materials, app.ranking_metric);

    ui.horizontal(|ui| {
        ui.strong(format!(
            "Ranked {} materials with valid property data",
            ranked_items.len()
        ));
        let missing = app.database.materials.len().saturating_sub(ranked_items.len());
        if missing > 0 {
            ui.label(format!("({} materials excluded due to missing data)", missing));
        }
    });

    ui.add_space(10.0);

    // Rankings Table
    egui::Grid::new("rankings_table")
        .striped(true)
        .min_col_width(120.0)
        .spacing([24.0, 10.0])
        .show(ui, |ui| {
            ui.strong("Rank");
            ui.strong("Material Name");
            ui.strong("Category");
            ui.strong("Metric Value");
            ui.strong("Relative Scale");
            ui.strong("Action");
            ui.end_row();

            let max_val = ranked_items.first().map(|item| item.sort_value).unwrap_or(1.0);
            let min_val = ranked_items.last().map(|item| item.sort_value).unwrap_or(0.0);
            let range = (max_val - min_val).max(1e-6);

            for (pos, item) in ranked_items.iter().enumerate() {
                // Rank #
                let rank_str = match pos + 1 {
                    1 => "🥇 1".to_string(),
                    2 => "🥈 2".to_string(),
                    3 => "🥉 3".to_string(),
                    n => format!("   {}", n),
                };
                ui.label(rank_str);

                // Material Name
                ui.strong(&item.name);

                // Category
                ui.label(&item.category);

                // Metric Formatted Value
                ui.strong(&item.formatted_value);

                // Relative bar
                let frac = ((item.sort_value - min_val) / range).clamp(0.05, 1.0) as f32;
                ui.add(egui::ProgressBar::new(frac).desired_width(120.0));

                // Inspect button
                if ui.button("Inspect →").clicked() {
                    app.inspect_material(item.material_index);
                }

                ui.end_row();
            }
        });
}

// ----------------------------------------------------------------------------
// 6. Physics Calculator Page
// ----------------------------------------------------------------------------

fn calculator_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(8.0);

    ui.label(
        "Explore fundamental materials physics equations. Select materials from the database \
         or modify inputs to compute physical thermal and mechanical responses.",
    );

    ui.add_space(16.0);

    // 1. Linear Thermal Expansion Calculator
    ui.group(|ui| {
        ui.heading("1. Linear Thermal Expansion (ΔL = α · L₀ · ΔT)");
        ui.add_space(6.0);

        ui.label(
            "Predicts the change in length of a solid bar when subjected to a uniform temperature change.",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Material:");
            calc_material_combobox(app, ui, 0);
        });

        ui.add_space(8.0);

        let selected_mat = app.material(app.calc_expansion_material).cloned();

        ui.horizontal(|ui| {
            ui.label("Initial Length L₀ (m):");
            ui.add(egui::DragValue::new(&mut app.initial_length).speed(0.05).range(0.001..=1000.0));

            ui.add_space(20.0);

            ui.label("Temperature Change ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.delta_temperature).speed(1.0).range(-500.0..=3000.0));
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(6.0);

        let exp_result = selected_mat.as_ref().and_then(|m| physics::thermal_expansion(m, app.initial_length, app.delta_temperature));

        match exp_result {
            Some(delta_l) => {
                let alpha = selected_mat.as_ref().and_then(|m| m.thermal_expansion_val()).unwrap_or(0.0);
                let final_l = app.initial_length + delta_l;

                ui.horizontal(|ui| {
                    ui.label(format!("Coefficient α: {:.2e} 1/K", alpha));
                    ui.label("•");
                    ui.strong(format!("Length Change ΔL: {:.6} m  ({:.3} mm / {:.1} µm)", delta_l, delta_l * 1000.0, delta_l * 1.0e6));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("Final Length L_final: {:.6} m", final_l));
                });
            }
            None => {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 180, 50),
                    "Thermal expansion coefficient α is unavailable for this material.",
                );
            }
        }

        ui.add_space(6.0);
        ui.label("Assumptions: Isotropic linear expansion, unconstrained boundaries, constant α over ΔT.");
    });

    ui.add_space(16.0);

    // 2. Heat Energy Transfer Calculator
    ui.group(|ui| {
        ui.heading("2. Thermal Heat Energy Transfer (Q = m · c · ΔT)");
        ui.add_space(6.0);

        ui.label("Calculates sensible heat energy required to change an object's temperature without a phase change.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Material:");
            calc_material_combobox(app, ui, 1);
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Mass m (kg):");
            ui.add(egui::DragValue::new(&mut app.calculator_mass).speed(0.1).range(0.0001..=100_000.0));

            ui.add_space(20.0);

            ui.label("Specific Heat c (J/(kg·K)):");
            ui.add(egui::DragValue::new(&mut app.calculator_specific_heat).speed(10.0).range(1.0..=10_000.0));

            ui.add_space(20.0);

            ui.label("ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.heat_delta_temperature).speed(1.0).range(-500.0..=3000.0));
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(6.0);

        let q_joules = physics::heat_energy(app.calculator_mass, app.calculator_specific_heat, app.heat_delta_temperature);
        ui.horizontal(|ui| {
            ui.strong(format!("Required Heat Energy Q: {:.2} J  ({:.3} kJ / {:.4} MJ)", q_joules, q_joules / 1000.0, q_joules / 1.0e6));
        });

        ui.add_space(6.0);
        ui.label("Assumptions: Constant specific heat capacity over interval, no phase change, zero heat loss.");
    });

    ui.add_space(16.0);

    // 3. Thermal Stress Approximation Calculator
    ui.group(|ui| {
        ui.heading("3. Fully Constrained Thermal Stress (σ = E · α · ΔT)");
        ui.add_space(6.0);

        ui.label(
            "Estimates upper-bound compressive/tensile stress induced in a 1D bar with rigid, fixed boundary constraints.",
        );
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Material:");
            calc_material_combobox(app, ui, 2);
        });

        ui.add_space(8.0);

        let selected_mat = app.material(app.calc_stress_material).cloned();

        ui.horizontal(|ui| {
            ui.label("Temperature Change ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.stress_delta_temperature).speed(1.0).range(-500.0..=3000.0));
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(6.0);

        let stress_result = selected_mat.as_ref().and_then(|m| physics::thermal_stress(m, app.stress_delta_temperature));

        match (selected_mat.as_ref(), stress_result) {
            (Some(mat), Some(stress_pa)) => {
                let e = mat.youngs_modulus_val().unwrap_or(0.0);
                let alpha = mat.thermal_expansion_val().unwrap_or(0.0);
                let stress_mpa = stress_pa / 1.0e6;

                ui.horizontal(|ui| {
                    ui.label(format!("E = {:.1} GPa, α = {:.2e} 1/K", e / 1.0e9, alpha));
                    ui.label("•");
                    ui.strong(format!("Thermal Stress σ: {:.2} MPa ({:.4} GPa)", stress_mpa, stress_pa / 1.0e9));
                });

                if let Some(sy) = mat.yield_strength_val() {
                    let sy_mpa = sy / 1.0e6;
                    let ratio = (stress_mpa.abs() / sy_mpa) * 100.0;
                    ui.horizontal(|ui| {
                        ui.label(format!("Yield Strength: {:.1} MPa", sy_mpa));
                        ui.label("•");
                        if ratio > 100.0 {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 100, 100),
                                format!("⚠️ Exceeds yield strength! ({:.1}% of yield) — Plastic deformation or fracture expected.", ratio),
                            );
                        } else {
                            ui.colored_label(
                                egui::Color32::from_rgb(100, 220, 120),
                                format!("Within elastic limit ({:.1}% of yield strength).", ratio),
                            );
                        }
                    });
                }
            }
            _ => {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 180, 50),
                    "Either Young's Modulus E or Thermal Expansion α is unavailable for this material.",
                );
            }
        }

        ui.add_space(6.0);
        ui.label("Assumptions: 1D bar rigidly constrained at both ends, linear elastic behavior, no structural buckling.");
    });
}

fn calc_material_combobox(app: &mut MaterialSpaceApp, ui: &mut egui::Ui, calc_id: u8) {
    let current_idx = match calc_id {
        0 => app.calc_expansion_material,
        1 => app.calc_heat_material,
        _ => app.calc_stress_material,
    };

    let selected_name = current_idx
        .and_then(|idx| app.database.get(idx))
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "Select Material".to_string());

    let salt_id = format!("calc_mat_combo_{}", calc_id);

    egui::ComboBox::from_id_salt(salt_id)
        .selected_text(selected_name)
        .width(300.0)
        .show_ui(ui, |ui| {
            for (idx, mat) in app.database.materials.iter().enumerate() {
                let is_sel = current_idx == Some(idx);
                if ui.selectable_label(is_sel, &mat.name).clicked() {
                    match calc_id {
                        0 => app.calc_expansion_material = Some(idx),
                        1 => {
                            app.calc_heat_material = Some(idx);
                            if let Some(cp) = mat.specific_heat_val() {
                                app.calculator_specific_heat = cp;
                            }
                        }
                        _ => app.calc_stress_material = Some(idx),
                    }
                }
            }
        });
}

// ----------------------------------------------------------------------------
// 7. About & Methodology Page
// ----------------------------------------------------------------------------

fn about_ui(ui: &mut egui::Ui) {
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.heading("What is MaterialSpace?");
        ui.add_space(6.0);
        ui.label(
            "MaterialSpace is an exploratory engineering application built entirely in Rust. \
             It is designed to bridge raw materials science databases with practical engineering decision-making, \
             providing transparent figures of merit, direct comparative analyses, and physics calculations.",
        );
    });

    ui.add_space(16.0);

    ui.group(|ui| {
        ui.heading("Scientific Data Provenance");
        ui.add_space(6.0);
        ui.label("Material properties in this application originate from normalized datasets sourced from:");
        ui.add_space(6.0);
        ui.label("• NASA Ames Research Center — TPSX (Thermal Protection Systems Expert Database)");
        ui.label("• NIST (National Institute of Standards and Technology) — Chemistry WebBook (SRD 69)");
        ui.label("• MatWeb — Online Materials Information Resource");
        ui.label("• ASM International — ASM Handbook Series (Vols 1, 2, 21)");
    });

    ui.add_space(16.0);

    ui.group(|ui| {
        ui.heading("Physics & Derived Metrics Formulas");
        ui.add_space(6.0);
        ui.label("1. Specific Strength: σ_strength / ρ (N·m/kg or kN·m/kg)");
        ui.label("2. Specific Stiffness: E / ρ (Resistance to elastic deflection per mass)");
        ui.label("3. Thermal Diffusivity: α = k / (ρ · Cp) (Rate of heat propagation)");
        ui.label("4. Linear Thermal Expansion: ΔL = α · L₀ · ΔT");
        ui.label("5. Heat Energy: Q = m · c · ΔT");
        ui.label("6. Constrained Thermal Stress: σ = E · α · ΔT");
    });

    ui.add_space(16.0);

    ui.group(|ui| {
        ui.heading("Engineering Disclaimers & Limitations");
        ui.add_space(6.0);
        ui.label(
            "• Missing data is preserved as 'Data unavailable' and is never treated as zero.\n\
             • The Aerospace Suitability Score is a transparent exploratory heuristic index, not an official design standard.\n\
             • Melting point represents a thermodynamic phase change, not a continuous certified operating limit.\n\
             • 1D thermal calculations assume idealized isotropic, homogeneous, and unbuckled conditions.",
        );
    });
}