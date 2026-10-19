use crate::app::{MaterialSpaceApp, Page, ScreeningTab};
use crate::export::{
    export_calculation_csv, export_comparison_csv, export_database_csv, export_material_csv,
    export_rankings_csv, export_screening_csv, save_export_file,
};
use crate::material::{Material, Property, format_scientific};
use crate::metrics::{
    RankingMetric, aerospace_suitability_score, generate_observations, specific_flexural_strength,
    specific_stiffness, specific_tensile_strength, specific_yield_strength, thermal_diffusivity,
};
use crate::physics::{self, evaluate_sanity_checks};
use crate::retrieval::official_source_statuses;
use crate::screening::{Constraints, ScreeningProfilePreset, ScreeningResult, ScreeningStatus};

/// Master rendering function called on every egui frame.
pub fn render(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    // Left navigation sidebar
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
        // Display export notification banner if recently triggered
        if let Some(notice) = &app.export_notice {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(80, 200, 120), "✓ Export Generated:");
                    ui.label(notice);
                });
            });
            ui.add_space(8.0);
        }

        // Display database loading error banner if present
        if let Some(err) = &app.status_message {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "⚠️ Notice:");
                    ui.label(err);
                });
            });
            ui.add_space(8.0);
        }

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| match app.page {
                Page::Dashboard => dashboard_ui(app, ui),
                Page::Materials => materials_ui(app, ui),
                Page::Compare => compare_ui(app, ui),
                Page::MaterialDetails => details_ui(app, ui),
                Page::Screening => screening_ui(app, ui),
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
    ui.add_space(14.0);

    ui.heading("🔬 MaterialSpace");
    ui.label("Materials Screening & Engineering Physics");

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(10.0);

    ui.label("MAIN NAVIGATION");
    ui.add_space(4.0);

    nav_item(app, ui, Page::Dashboard, "📊  Dashboard");
    nav_item(app, ui, Page::Materials, "📚  Materials Database");
    nav_item(app, ui, Page::Compare, "⚖️  Compare Materials");
    nav_item(app, ui, Page::MaterialDetails, "🔍  Material Details");
    nav_item(app, ui, Page::Screening, "🎯  Requirements Screening");
    nav_item(app, ui, Page::Rankings, "🏆  Engineering Rankings");
    nav_item(app, ui, Page::Calculator, "🧮  Physics Calculator");

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    nav_item(app, ui, Page::About, "ℹ️  About & Methodology");

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(10.0);

    // Database & Cache Status
    ui.label("SYSTEM STATUS");
    ui.add_space(4.0);
    ui.label(format!("• Materials: {}", app.database.materials.len()));
    ui.label(format!("• Categories: {}", app.database.categories().len()));
    ui.label(format!("• Cited Sources: {}", app.database.sources.len()));

    let (cache_count, cache_status_text) = app.database.cache_manager.cache_status();
    if cache_count > 0 {
        ui.label(format!("• Cache: {}", cache_status_text));
    } else {
        ui.label("• Mode: Offline Baseline");
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(10.0);

    if ui.button("📥 Export Database (CSV)").clicked() {
        let csv = export_database_csv(&app.database.materials);
        let filename = format!(
            "database_export_{}.csv",
            crate::export::get_timestamp_utc().replace(':', "-")
        );
        match save_export_file(&filename, &csv) {
            Ok(p) => app.export_notice = Some(format!("Saved to {}", p.display())),
            Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
        }
    }
}

fn nav_item(app: &mut MaterialSpaceApp, ui: &mut egui::Ui, page: Page, title: &str) {
    let is_selected = app.page == page;
    let button = egui::Button::new(title)
        .min_size(egui::vec2(ui.available_width(), 30.0))
        .selected(is_selected);

    if ui.add(button).clicked() {
        app.page = page;
        app.export_notice = None;
    }
    ui.add_space(2.0);
}

fn topbar_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.horizontal_centered(|ui| {
        let (title, subtitle) = match app.page {
            Page::Dashboard => (
                "Engineering Dashboard",
                "Overview of materials datasets, thermophysical records, and screening tools",
            ),
            Page::Materials => (
                "Materials Explorer",
                "Browse, search, and filter normalized engineering materials",
            ),
            Page::Compare => (
                "Material Comparison",
                "Side-by-side comparison of mechanical, thermal, and derived properties",
            ),
            Page::MaterialDetails => (
                "Material Inspection",
                "Detailed material datasheet with source data, testing conditions, and heuristics",
            ),
            Page::Screening => (
                "Requirements Screening Engine",
                "Screen candidates against user-defined engineering constraints and application profiles",
            ),
            Page::Rankings => (
                "Engineering Rankings",
                "Transparent criteria-based ranking across explicit mechanical and thermal metrics",
            ),
            Page::Calculator => (
                "Engineering Physics Calculator",
                "Thermal expansion, sensible heat, constrained thermal stress, conduction, and diffusion",
            ),
            Page::About => (
                "About MaterialSpace",
                "Methodology, data sources, scientific honesty, and architectural overview",
            ),
        };

        ui.vertical(|ui| {
            ui.heading(title);
            ui.label(subtitle);
        });

        // Search shortcut on Materials page
        if app.page == Page::Materials {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut app.search_query)
                        .hint_text("🔍 Search by name, ID, composition, grade...")
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
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.heading("MaterialSpace — Research-Grade Materials Intelligence");
        ui.add_space(6.0);
        ui.label(
            "MaterialSpace is a materials engineering platform designed for researchers, students, \
             and engineers. It provides traceable property data, first-principles physics calculations, \
             requirements-based candidate screening, and transparent multi-criteria evaluations without \
             obscuring missing data or conflating incompatible test methods.",
        );
    });

    ui.add_space(14.0);

    ui.label("DATABASE SUMMARY");
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
            "Tensile, yield, modulus",
        );
        stat_card(
            ui,
            "Flexural Records",
            &app.database.count_with_flexural_data().to_string(),
            "Ceramics & glasses (MOR)",
        );
    });

    ui.add_space(20.0);

    ui.label("CORE WORKFLOWS");
    ui.add_space(6.0);

    ui.columns(4, |columns| {
        columns[0].group(|ui| {
            ui.heading("📚 Database");
            ui.add_space(4.0);
            ui.label("Browse 50+ aerospace alloys, ceramics, and structural polymers.");
            ui.add_space(8.0);
            if ui.button("Open Materials →").clicked() {
                app.page = Page::Materials;
            }
        });

        columns[1].group(|ui| {
            ui.heading("⚖️ Compare");
            ui.add_space(4.0);
            ui.label("Side-by-side property comparison with relative performance bars.");
            ui.add_space(8.0);
            if ui.button("Compare Materials →").clicked() {
                app.page = Page::Compare;
            }
        });

        columns[2].group(|ui| {
            ui.heading("🎯 Screening");
            ui.add_space(4.0);
            ui.label("Screen candidates against engineering limits and application profiles.");
            ui.add_space(8.0);
            if ui.button("Open Screening →").clicked() {
                app.page = Page::Screening;
            }
        });

        columns[3].group(|ui| {
            ui.heading("🧮 Calculator");
            ui.add_space(4.0);
            ui.label("Compute thermal expansion, heat energy, stress, and conduction.");
            ui.add_space(8.0);
            if ui.button("Open Calculator →").clicked() {
                app.page = Page::Calculator;
            }
        });
    });

    ui.add_space(20.0);

    // Authoritative Data Sources Overview
    ui.group(|ui| {
        ui.heading("Cited Scientific Data Sources");
        ui.add_space(6.0);
        ui.label(
            "Every normalized property in MaterialSpace is traceable to an established, cited scientific reference:",
        );
        ui.add_space(8.0);

        for (id, source) in app.database.sorted_sources() {
            ui.horizontal(|ui| {
                ui.strong(format!("• {} ({})", source.organization, id));
                ui.label(format!("— {}", source.title));
                if !source.url.is_empty() {
                    ui.hyperlink_to("🔗 Link", &source.url);
                }
            });
            if let Some(spot) = &source.spot_check_verified {
                ui.label(format!("    Spot-Check: {}", spot));
            }
        }
    });

    ui.add_space(16.0);

    // External Access Integration Status
    ui.group(|ui| {
        ui.heading("External Data Access & Integration Architecture");
        ui.add_space(6.0);
        ui.label(
            "MaterialSpace adheres to a strict scientific data retrieval policy: no website scraping or HTML \
             parsing is performed. Access methods are documented below:",
        );
        ui.add_space(8.0);

        for src in official_source_statuses() {
            ui.strong(&src.name);
            ui.label(format!("Method: {}", src.access_method));
            ui.label(&src.access_notes);
            ui.add_space(4.0);
        }
    });
}

fn stat_card(ui: &mut egui::Ui, title: &str, value: &str, subtitle: &str) {
    ui.group(|ui| {
        ui.set_min_width(150.0);
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
    ui.add_space(6.0);

    // Filter Controls Bar
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Category Filter:");

            egui::ComboBox::from_id_salt("materials_category_filter")
                .selected_text(match app.category_filter.as_str() {
                    "All" => "All Categories",
                    "metal" => "Metals & Alloys",
                    "ceramic" => "Technical Ceramics",
                    "polymer" => "Polymers & Plastics",
                    "composite" => "Composites",
                    "glass" => "Glasses & Glass-Ceramics",
                    "pure_substance" => "Elements & Pure Substances",
                    _ => &app.category_filter,
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(app.category_filter == "All", "All Categories")
                        .clicked()
                    {
                        app.category_filter = "All".to_string();
                    }
                    for cat in app.database.categories() {
                        let label = match cat.as_str() {
                            "metal" => "Metals & Alloys",
                            "ceramic" => "Technical Ceramics",
                            "polymer" => "Polymers & Plastics",
                            "composite" => "Composites",
                            "glass" => "Glasses & Glass-Ceramics",
                            "pure_substance" => "Elements & Pure Substances",
                            _ => &cat,
                        };
                        if ui
                            .selectable_label(app.category_filter == cat, label)
                            .clicked()
                        {
                            app.category_filter = cat;
                        }
                    }
                });

            ui.add_space(16.0);

            if !app.search_query.is_empty() || app.category_filter != "All" {
                if ui.button("Reset Filters").clicked() {
                    app.search_query.clear();
                    app.category_filter = "All".to_string();
                }
            }
        });
    });

    ui.add_space(10.0);

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

    ui.add_space(8.0);

    // Material Cards List
    for (index, material) in results.iter() {
        material_card(app, ui, *index, material);
        ui.add_space(6.0);
    }
}

fn material_card(app: &mut MaterialSpaceApp, ui: &mut egui::Ui, index: usize, material: &Material) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            // Material Identity
            ui.vertical(|ui| {
                ui.set_width(260.0);
                ui.heading(&material.name);
                ui.horizontal(|ui| {
                    ui.label(format!("ID: {}", material.id));
                    ui.label("•");
                    ui.strong(material.category_display());
                });
                if let Some(comp) = &material.composition {
                    ui.label(format!("Comp: {}", comp));
                }
                if let Some(grade) = &material.grade {
                    ui.label(format!("Grade: {}", grade));
                }
            });

            ui.separator();

            // Properties Summary
            ui.vertical(|ui| {
                ui.set_width(380.0);
                ui.horizontal(|ui| {
                    property_badge(ui, "Density", material.density_val(), "kg/m³");
                    property_badge(
                        ui,
                        "Thermal Conductivity",
                        material.thermal_conductivity_val(),
                        "W/(m·K)",
                    );
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if let Some(ts) = material.tensile_strength_val() {
                        property_badge(ui, "Tensile (UTS)", Some(ts / 1.0e6), "MPa");
                    } else if let Some(fs) = material.flexural_strength_val() {
                        property_badge(ui, "Flexural (MOR)", Some(fs / 1.0e6), "MPa");
                    } else if let Some(ys) = material.yield_strength_val() {
                        property_badge(ui, "Yield Strength", Some(ys / 1.0e6), "MPa");
                    } else {
                        property_badge(ui, "Strength", None, "MPa");
                    }

                    property_badge(ui, "Melting Point", material.melting_point_val(), "K");
                });
            });

            ui.separator();

            // Actions
            ui.vertical_centered(|ui| {
                if ui.button("🔍 Inspect Details").clicked() {
                    app.inspect_material(index);
                }
                ui.add_space(4.0);
                if ui.button("⚖️ Compare").clicked() {
                    app.comparison_a = Some(index);
                    app.page = Page::Compare;
                }
                ui.add_space(4.0);
                if ui.button("🧮 Calculator").clicked() {
                    app.sync_calculator_material(Some(index));
                    app.page = Page::Calculator;
                }
            });
        });
    });
}

fn property_badge(ui: &mut egui::Ui, label: &str, value: Option<f64>, unit: &str) {
    ui.vertical(|ui| {
        ui.label(label);
        match value {
            Some(v) => {
                ui.strong(format!("{} {}", format_scientific(v), unit));
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
    ui.add_space(6.0);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Material A:");
            comparison_selector(app, ui, true);

            ui.add_space(30.0);

            ui.label("Material B:");
            comparison_selector(app, ui, false);

            ui.add_space(20.0);

            if let (Some(idx_a), Some(idx_b)) = (app.comparison_a, app.comparison_b) {
                if let (Some(a), Some(b)) = (app.database.get(idx_a), app.database.get(idx_b)) {
                    if ui.button("📥 Export Comparison (CSV)").clicked() {
                        let csv = export_comparison_csv(a, b);
                        let filename = format!("compare_{}_vs_{}.csv", a.id, b.id);
                        match save_export_file(&filename, &csv) {
                            Ok(p) => app.export_notice = Some(format!("Saved to {}", p.display())),
                            Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
                        }
                    }
                }
            }
        });
    });

    ui.add_space(14.0);

    let mat_a = app.material(app.comparison_a).cloned();
    let mat_b = app.material(app.comparison_b).cloned();

    match (mat_a, mat_b) {
        (Some(a), Some(b)) => {
            render_comparison_table(ui, &a, &b);
        }
        _ => {
            ui.group(|ui| {
                ui.label("Select two materials from the dropdowns above to display side-by-side comparison.");
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

    let salt_id = if is_material_a {
        "comp_sel_a"
    } else {
        "comp_sel_b"
    };

    egui::ComboBox::from_id_salt(salt_id)
        .selected_text(selected_label)
        .width(260.0)
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
    ui.group(|ui| {
        ui.columns(2, |cols| {
            cols[0].heading(format!("Material A: {}", a.name));
            cols[0].label(format!(
                "Category: {} | Grade: {}",
                a.category_display(),
                a.grade.as_deref().unwrap_or("Not recorded")
            ));

            cols[1].heading(format!("Material B: {}", b.name));
            cols[1].label(format!(
                "Category: {} | Grade: {}",
                b.category_display(),
                b.grade.as_deref().unwrap_or("Not recorded")
            ));
        });
    });

    ui.add_space(14.0);

    // Mechanical Properties
    ui.heading("Mechanical Properties");
    ui.add_space(6.0);
    comparison_grid(ui, "mech_comp", |ui| {
        compare_property_row(
            ui,
            "Density",
            a.density.as_ref(),
            b.density.as_ref(),
            "kg/m³",
        );
        compare_property_row(
            ui,
            "Young's Modulus",
            a.youngs_modulus.as_ref(),
            b.youngs_modulus.as_ref(),
            "Pa",
        );
        compare_property_row(
            ui,
            "Yield Strength (σ_y)",
            a.yield_strength.as_ref(),
            b.yield_strength.as_ref(),
            "Pa",
        );
        compare_property_row(
            ui,
            "Tensile Strength (UTS)",
            a.tensile_strength.as_ref(),
            b.tensile_strength.as_ref(),
            "Pa",
        );
        compare_property_row(
            ui,
            "Flexural Strength (MOR)",
            a.flexural_strength.as_ref(),
            b.flexural_strength.as_ref(),
            "Pa",
        );
    });

    ui.add_space(14.0);

    // Thermal Properties
    ui.heading("Thermal & Thermophysical Properties");
    ui.add_space(6.0);
    comparison_grid(ui, "therm_comp", |ui| {
        compare_property_row(
            ui,
            "Thermal Conductivity",
            a.thermal_conductivity.as_ref(),
            b.thermal_conductivity.as_ref(),
            "W/(m·K)",
        );
        compare_property_row(
            ui,
            "Specific Heat Capacity",
            a.specific_heat.as_ref(),
            b.specific_heat.as_ref(),
            "J/(kg·K)",
        );
        compare_property_row(
            ui,
            "Thermal Expansion (CTE)",
            a.thermal_expansion.as_ref(),
            b.thermal_expansion.as_ref(),
            "1/K",
        );
        compare_property_row(
            ui,
            "Melting / Solidus Point",
            a.melting_point.as_ref(),
            b.melting_point.as_ref(),
            "K",
        );
    });

    ui.add_space(14.0);

    // Derived Figures of Merit
    ui.heading("Derived Engineering Figures of Merit");
    ui.add_space(6.0);
    egui::Grid::new("derived_comp")
        .striped(true)
        .min_col_width(170.0)
        .spacing([24.0, 8.0])
        .show(ui, |ui| {
            ui.strong("Derived Metric");
            ui.strong("Material A");
            ui.strong("Material B");
            ui.strong("Relative Comparison Bar");
            ui.end_row();

            compare_derived_row(
                ui,
                "Specific Tensile Strength (σ_uts / ρ)",
                specific_tensile_strength(a).map(|v| v / 1000.0),
                specific_tensile_strength(b).map(|v| v / 1000.0),
                "kN·m/kg",
            );

            compare_derived_row(
                ui,
                "Specific Yield Strength (σ_y / ρ)",
                specific_yield_strength(a).map(|v| v / 1000.0),
                specific_yield_strength(b).map(|v| v / 1000.0),
                "kN·m/kg",
            );

            compare_derived_row(
                ui,
                "Specific Flexural Strength (σ_flex / ρ)",
                specific_flexural_strength(a).map(|v| v / 1000.0),
                specific_flexural_strength(b).map(|v| v / 1000.0),
                "kN·m/kg",
            );

            compare_derived_row(
                ui,
                "Specific Stiffness (E / ρ)",
                specific_stiffness(a).map(|v| v / 1.0e6),
                specific_stiffness(b).map(|v| v / 1.0e6),
                "GPa/(g/cm³)",
            );

            compare_derived_row(
                ui,
                "Thermal Diffusivity (α_diff)",
                thermal_diffusivity(a).map(|v| v * 1.0e6),
                thermal_diffusivity(b).map(|v| v * 1.0e6),
                "mm²/s",
            );

            let score_a = aerospace_suitability_score(a).map(|res| res.score);
            let score_b = aerospace_suitability_score(b).map(|res| res.score);
            compare_derived_row(ui, "Aerospace Heuristic Score", score_a, score_b, "/ 100");
        });
}

fn comparison_grid<F>(ui: &mut egui::Ui, id: &str, add_rows: F)
where
    F: FnOnce(&mut egui::Ui),
{
    egui::Grid::new(id)
        .striped(true)
        .min_col_width(170.0)
        .spacing([24.0, 8.0])
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
    prop_a: Option<&Property>,
    prop_b: Option<&Property>,
    unit: &str,
) {
    ui.label(name);

    match prop_a {
        Some(p) => ui.label(format!(
            "{} {} ({})",
            format_scientific(p.value),
            unit,
            p.condition_display()
        )),
        None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
    };

    match prop_b {
        Some(p) => ui.label(format!(
            "{} {} ({})",
            format_scientific(p.value),
            unit,
            p.condition_display()
        )),
        None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
    };

    match (prop_a, prop_b) {
        (Some(pa), Some(pb)) if pa.value > 0.0 && pb.value > 0.0 => {
            let max = pa.value.max(pb.value);
            let frac_a = (pa.value / max) as f32;
            let frac_b = (pb.value / max) as f32;

            ui.horizontal(|ui| {
                ui.label("A:");
                ui.add(egui::ProgressBar::new(frac_a).desired_width(60.0));
                ui.label("B:");
                ui.add(egui::ProgressBar::new(frac_b).desired_width(60.0));
            });
        }
        _ => {
            ui.label("—");
        }
    }

    ui.end_row();
}

fn compare_derived_row(
    ui: &mut egui::Ui,
    name: &str,
    val_a: Option<f64>,
    val_b: Option<f64>,
    unit: &str,
) {
    ui.label(name);

    match val_a {
        Some(v) => ui.label(format!("{} {}", format_scientific(v), unit)),
        None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
    };

    match val_b {
        Some(v) => ui.label(format!("{} {}", format_scientific(v), unit)),
        None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
    };

    match (val_a, val_b) {
        (Some(va), Some(vb)) if va > 0.0 && vb > 0.0 => {
            let max = va.max(vb);
            let frac_a = (va / max) as f32;
            let frac_b = (vb / max) as f32;

            ui.horizontal(|ui| {
                ui.label("A:");
                ui.add(egui::ProgressBar::new(frac_a).desired_width(60.0));
                ui.label("B:");
                ui.add(egui::ProgressBar::new(frac_b).desired_width(60.0));
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
                ui.label(
                    "No material selected. Browse the Materials page and click 'Inspect Details'.",
                );
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

    ui.add_space(6.0);

    // Overview Header Card
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(&material.name);
                ui.label(format!(
                    "Canonical ID: {} | Category: {}",
                    material.id,
                    material.category_display()
                ));
                if let Some(grade) = &material.grade {
                    ui.label(format!("Grade / Condition: {}", grade));
                }
                if let Some(comp) = &material.composition {
                    ui.label(format!("Nominal Composition: {}", comp));
                }
                if !material.aliases.is_empty() {
                    ui.label(format!("Known Aliases: {}", material.aliases.join(", ")));
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("📥 Export Datasheet (CSV)").clicked() {
                    let csv = export_material_csv(&material);
                    let filename = format!("datasheet_{}.csv", material.id);
                    match save_export_file(&filename, &csv) {
                        Ok(p) => app.export_notice = Some(format!("Saved to {}", p.display())),
                        Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
                    }
                }
                ui.add_space(8.0);
                if ui.button("⚖️ Compare").clicked() {
                    app.comparison_a = Some(index);
                    app.page = Page::Compare;
                }
                ui.add_space(8.0);
                if ui.button("🧮 Use in Calculator").clicked() {
                    app.sync_calculator_material(Some(index));
                    app.page = Page::Calculator;
                }
            });
        });

        if let Some(desc) = &material.description {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(4.0);
            ui.label(desc);
        }
    });

    ui.add_space(14.0);

    // Mechanical Properties Table
    ui.heading("Mechanical Properties");
    ui.add_space(6.0);
    property_detail_table(
        ui,
        "detail_mech_table",
        &[
            ("Density", &material.density),
            ("Young's Modulus (Elasticity)", &material.youngs_modulus),
            ("Yield Strength (σ_y)", &material.yield_strength),
            ("Tensile Strength (UTS)", &material.tensile_strength),
            (
                "Flexural Strength (Modulus of Rupture)",
                &material.flexural_strength,
            ),
        ],
    );

    ui.add_space(14.0);

    // Thermal Properties Table
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

    ui.add_space(14.0);

    // Derived Figures of Merit
    ui.heading("Derived Engineering Figures of Merit");
    ui.add_space(6.0);
    ui.group(|ui| {
        egui::Grid::new("detail_derived_grid")
            .striped(true)
            .min_col_width(200.0)
            .spacing([24.0, 8.0])
            .show(ui, |ui| {
                ui.strong("Metric");
                ui.strong("Derived Value");
                ui.strong("Engineering Formula / Interpretation");
                ui.end_row();

                // Specific Tensile Strength
                ui.label("Specific Tensile Strength");
                match specific_tensile_strength(&material) {
                    Some(v) => ui.label(format!("{:.2} kN·m/kg", v / 1000.0)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable (no tensile data)"),
                };
                ui.label("σ_uts / ρ (Tensile load capacity per unit mass)");
                ui.end_row();

                // Specific Yield Strength
                ui.label("Specific Yield Strength");
                match specific_yield_strength(&material) {
                    Some(v) => ui.label(format!("{:.2} kN·m/kg", v / 1000.0)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("σ_yield / ρ (Elastic load capacity per unit mass)");
                ui.end_row();

                // Specific Flexural Strength
                ui.label("Specific Flexural Strength");
                match specific_flexural_strength(&material) {
                    Some(v) => ui.label(format!("{:.2} kN·m/kg", v / 1000.0)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("σ_flex / ρ (Bending capacity per unit mass)");
                ui.end_row();

                // Specific Stiffness
                ui.label("Specific Stiffness (Modulus)");
                match specific_stiffness(&material) {
                    Some(v) => ui.label(format!("{:.2} GPa/(g/cm³)", v / 1.0e6)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("E / ρ (Deflection & flutter resistance per mass)");
                ui.end_row();

                // Thermal Diffusivity
                ui.label("Thermal Diffusivity (α_diff)");
                match thermal_diffusivity(&material) {
                    Some(v) => ui.label(format!("{:.3} mm²/s", v * 1.0e6)),
                    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
                };
                ui.label("k / (ρ · Cp) (Rate of thermal equilibrium diffusion)");
                ui.end_row();

                // Aerospace Heuristic Score
                ui.label("Aerospace Suitability Score");
                match aerospace_suitability_score(&material) {
                    Some(res) => {
                        ui.vertical(|ui| {
                            ui.strong(format!("{:.1} / 100", res.score));
                            ui.label(format!("Data Completeness: {:.0}% ({}/{} properties)", res.data_completeness * 100.0, res.properties_used, res.total_properties));
                            for caveat in &res.caveats {
                                ui.colored_label(egui::Color32::from_rgb(230, 160, 40), format!("Note: {}", caveat));
                            }
                        });
                    }
                    None => {
                        ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Insufficient data (strength or density missing)");
                    }
                };
                ui.label("Heuristic multi-criteria index (Exploratory only; not an aerospace specification)");
                ui.end_row();
            });
    });

    ui.add_space(14.0);

    // Rule-Based Observations
    ui.heading("Engineering Observations");
    ui.add_space(6.0);
    ui.group(|ui| {
        ui.label(
            "Rule-based engineering observations derived from verified property thresholds for exploratory screening:",
        );
        ui.add_space(6.0);

        let observations = generate_observations(&material);
        for obs in observations {
            ui.label(format!("• {}", obs));
        }
    });

    ui.add_space(14.0);

    // Cited Sources List
    ui.heading("Data Sources & Scientific Citations");
    ui.add_space(6.0);
    ui.group(|ui| {
        if material.source_ids.is_empty() {
            ui.label("No source IDs cited for this material.");
        } else {
            for source_id in &material.source_ids {
                if let Some(source_info) = app.database.get_source(source_id) {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.strong(format!(
                                "• {} ({})",
                                source_info.organization, source_info.source_id
                            ));
                            if !source_info.url.is_empty() {
                                ui.hyperlink_to("🔗 Reference Link", &source_info.url);
                            }
                        });
                        ui.label(format!("  Title: {}", source_info.title));
                        ui.label(format!("  Notes: {}", source_info.notes));
                        ui.add_space(4.0);
                    });
                } else {
                    ui.label(format!("• Source ID: {}", source_id));
                }
            }
        }
    });
}

/// Renders a cleanly aligned 4-column property detail table.
///
/// **Bug Fix**: Places normalization warnings cleanly inside the 4th column
/// rather than creating a 5th column that shifts grid cells out of alignment.
fn property_detail_table(ui: &mut egui::Ui, id: &str, rows: &[(&str, &Option<Property>)]) {
    egui::Grid::new(id)
        .striped(true)
        .min_col_width(170.0)
        .spacing([24.0, 8.0])
        .show(ui, |ui| {
            ui.strong("Property Name");
            ui.strong("Normalized SI Value");
            ui.strong("Condition / Test Method");
            ui.strong("Original Data / Source");
            ui.end_row();

            for (name, prop_opt) in rows {
                ui.label(*name);

                match prop_opt {
                    Some(prop) => {
                        let unit = prop.unit.as_deref().unwrap_or("");
                        ui.strong(format!("{} {}", format_scientific(prop.value), unit));

                        // Condition display: Never shows "Standard / Ambient" unless explicitly in data
                        ui.label(prop.condition_display());

                        // Column 4: Source + Original Data + Warnings cleanly contained in single column
                        ui.vertical(|ui| {
                            let source = prop.source_id.as_deref().unwrap_or("Normalized Baseline");
                            if let Some(orig_val) = prop.original_value {
                                let orig_unit = prop.original_unit.as_deref().unwrap_or("");
                                ui.label(format!(
                                    "{} (Orig: {} {})",
                                    source,
                                    format_scientific(orig_val),
                                    orig_unit
                                ));
                            } else {
                                ui.label(source);
                            }

                            if let Some(warning) = &prop.normalization_warning {
                                ui.colored_label(
                                    egui::Color32::from_rgb(255, 180, 50),
                                    format!("⚠️ {}", warning),
                                );
                            }
                        });
                    }
                    None => {
                        ui.colored_label(
                            egui::Color32::from_rgb(140, 140, 140),
                            "Data unavailable",
                        );
                        ui.label("Not recorded");
                        ui.label("—");
                    }
                }

                ui.end_row();
            }
        });
}

// ----------------------------------------------------------------------------
// 5. Materials Screening Page
// ----------------------------------------------------------------------------

fn screening_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(6.0);

    // Profile Presets & Constraint Controls
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Screening Profile Preset:");

            let preset_name = app
                .selected_preset
                .map(|p| p.name())
                .unwrap_or("Custom Constraints");
            egui::ComboBox::from_id_salt("screening_preset_selector")
                .selected_text(preset_name)
                .width(280.0)
                .show_ui(ui, |ui| {
                    for preset in ScreeningProfilePreset::all() {
                        if ui
                            .selectable_label(app.selected_preset == Some(*preset), preset.name())
                            .clicked()
                        {
                            app.selected_preset = Some(*preset);
                            app.screening_constraints = preset.default_constraints();
                            app.run_screening();
                        }
                    }
                });

            ui.add_space(16.0);

            if ui.button("Apply Screening").clicked() {
                app.run_screening();
            }

            if ui.button("Reset Constraints").clicked() {
                app.screening_constraints = Constraints::default();
                app.selected_preset = None;
                app.run_screening();
            }

            if ui.button("📥 Export Results (CSV)").clicked() {
                let results = app.get_screening_results().to_vec();
                let csv = export_screening_csv(&results, &app.screening_constraints);
                let filename = format!(
                    "screening_results_{}.csv",
                    crate::export::get_timestamp_utc().replace(':', "-")
                );
                match save_export_file(&filename, &csv) {
                    Ok(p) => app.export_notice = Some(format!("Saved to {}", p.display())),
                    Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
                }
            }
        });

        if let Some(preset) = app.selected_preset {
            ui.add_space(6.0);
            ui.label(preset.description());
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(6.0);

        // Editable Constraints Grid
        egui::Grid::new("screening_constraints_inputs")
            .num_columns(4)
            .spacing([24.0, 8.0])
            .show(ui, |ui| {
                // Max Density
                ui.label("Max Density (kg/m³):");
                let mut rho_active = app.screening_constraints.max_density.is_some();
                if ui.checkbox(&mut rho_active, "").changed() {
                    app.screening_constraints.max_density =
                        if rho_active { Some(4000.0) } else { None };
                }
                if let Some(val) = &mut app.screening_constraints.max_density {
                    ui.add(egui::DragValue::new(val).speed(50.0).range(100.0..=25000.0));
                } else {
                    ui.label("No constraint");
                }
                ui.end_row();

                // Min Thermal Conductivity
                ui.label("Min Thermal Conductivity (W/(m·K)):");
                let mut k_active = app.screening_constraints.min_thermal_conductivity.is_some();
                if ui.checkbox(&mut k_active, "").changed() {
                    app.screening_constraints.min_thermal_conductivity =
                        if k_active { Some(50.0) } else { None };
                }
                if let Some(val) = &mut app.screening_constraints.min_thermal_conductivity {
                    ui.add(egui::DragValue::new(val).speed(5.0).range(0.1..=2500.0));
                } else {
                    ui.label("No constraint");
                }
                ui.end_row();

                // Max Thermal Expansion
                ui.label("Max Thermal Expansion (1/K):");
                let mut cte_active = app.screening_constraints.max_thermal_expansion.is_some();
                if ui.checkbox(&mut cte_active, "").changed() {
                    app.screening_constraints.max_thermal_expansion =
                        if cte_active { Some(1.0e-5) } else { None };
                }
                if let Some(val) = &mut app.screening_constraints.max_thermal_expansion {
                    ui.add(egui::DragValue::new(val).speed(1e-6).range(-1e-5..=5e-5));
                } else {
                    ui.label("No constraint");
                }
                ui.end_row();

                // Min Tensile Strength
                ui.label("Min Tensile Strength (MPa):");
                let mut ts_active = app.screening_constraints.min_tensile_strength.is_some();
                if ui.checkbox(&mut ts_active, "").changed() {
                    app.screening_constraints.min_tensile_strength =
                        if ts_active { Some(300.0e6) } else { None };
                }
                if let Some(val) = &mut app.screening_constraints.min_tensile_strength {
                    let mut mpa = *val / 1.0e6;
                    if ui
                        .add(
                            egui::DragValue::new(&mut mpa)
                                .speed(10.0)
                                .range(1.0..=3500.0),
                        )
                        .changed()
                    {
                        *val = mpa * 1.0e6;
                    }
                } else {
                    ui.label("No constraint");
                }
                ui.end_row();

                // Min Young's Modulus
                ui.label("Min Young's Modulus (GPa):");
                let mut e_active = app.screening_constraints.min_youngs_modulus.is_some();
                if ui.checkbox(&mut e_active, "").changed() {
                    app.screening_constraints.min_youngs_modulus =
                        if e_active { Some(70.0e9) } else { None };
                }
                if let Some(val) = &mut app.screening_constraints.min_youngs_modulus {
                    let mut gpa = *val / 1.0e9;
                    if ui
                        .add(
                            egui::DragValue::new(&mut gpa)
                                .speed(5.0)
                                .range(1.0..=1200.0),
                        )
                        .changed()
                    {
                        *val = gpa * 1.0e9;
                    }
                } else {
                    ui.label("No constraint");
                }
                ui.end_row();

                // Min Melting Point
                ui.label("Min Melting Point (K):");
                let mut tm_active = app.screening_constraints.min_melting_point.is_some();
                if ui.checkbox(&mut tm_active, "").changed() {
                    app.screening_constraints.min_melting_point =
                        if tm_active { Some(1000.0) } else { None };
                }
                if let Some(val) = &mut app.screening_constraints.min_melting_point {
                    ui.add(egui::DragValue::new(val).speed(25.0).range(200.0..=4000.0));
                } else {
                    ui.label("No constraint");
                }
                ui.end_row();
            });
    });

    ui.add_space(12.0);

    if app.screening_constraints.is_empty() {
        ui.colored_label(
            egui::Color32::from_rgb(100, 180, 255),
            "ℹ️ No constraints currently active. All materials are listed. Select a screening profile preset above or configure minimum/maximum property thresholds to filter.",
        );
        ui.add_space(6.0);
    }

    // Results Tab Selector
    let results = app.get_screening_results().to_vec();
    let passed_count = results
        .iter()
        .filter(|r| r.status == ScreeningStatus::Passed)
        .count();
    let failed_count = results
        .iter()
        .filter(|r| r.status == ScreeningStatus::Failed)
        .count();
    let incomplete_count = results
        .iter()
        .filter(|r| r.status == ScreeningStatus::InsufficientData)
        .count();

    ui.horizontal(|ui| {
        if ui
            .selectable_label(
                app.screening_tab == ScreeningTab::Passed,
                format!("✓ Passed ({})", passed_count),
            )
            .clicked()
        {
            app.screening_tab = ScreeningTab::Passed;
        }
        if ui
            .selectable_label(
                app.screening_tab == ScreeningTab::Failed,
                format!("✗ Violated ({})", failed_count),
            )
            .clicked()
        {
            app.screening_tab = ScreeningTab::Failed;
        }
        if ui
            .selectable_label(
                app.screening_tab == ScreeningTab::InsufficientData,
                format!("? Insufficient Data ({})", incomplete_count),
            )
            .clicked()
        {
            app.screening_tab = ScreeningTab::InsufficientData;
        }
        if ui
            .selectable_label(
                app.screening_tab == ScreeningTab::All,
                format!("All Candidates ({})", results.len()),
            )
            .clicked()
        {
            app.screening_tab = ScreeningTab::All;
        }
    });

    ui.add_space(8.0);

    // Filter results according to active tab
    let display_results: Vec<&ScreeningResult> = results
        .iter()
        .filter(|r| match app.screening_tab {
            ScreeningTab::Passed => r.status == ScreeningStatus::Passed,
            ScreeningTab::Failed => r.status == ScreeningStatus::Failed,
            ScreeningTab::InsufficientData => r.status == ScreeningStatus::InsufficientData,
            ScreeningTab::All => true,
        })
        .collect();

    for r in display_results {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let badge = match r.status {
                    ScreeningStatus::Passed => ("✓ PASSED", egui::Color32::from_rgb(80, 200, 120)),
                    ScreeningStatus::Failed => {
                        ("✗ VIOLATION", egui::Color32::from_rgb(240, 90, 90))
                    }
                    ScreeningStatus::InsufficientData => {
                        ("? INSUFFICIENT DATA", egui::Color32::from_rgb(255, 180, 50))
                    }
                };

                ui.colored_label(badge.1, badge.0);
                ui.strong(&r.material.name);
                ui.label(format!("({})", r.material.category_display()));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(idx) = app.database.find_index_by_id(&r.material.id) {
                        if ui.button("Inspect Details →").clicked() {
                            app.inspect_material(idx);
                        }
                    }
                });
            });

            // Display check details
            if !r.failed_reasons.is_empty() {
                for reason in &r.failed_reasons {
                    ui.colored_label(
                        egui::Color32::from_rgb(240, 100, 100),
                        format!("  • {}", reason),
                    );
                }
            }
            if !r.missing_reasons.is_empty() {
                for reason in &r.missing_reasons {
                    ui.colored_label(
                        egui::Color32::from_rgb(230, 180, 60),
                        format!("  • {}", reason),
                    );
                }
            }
            if !r.passed_reasons.is_empty()
                && (r.status == ScreeningStatus::Passed || app.screening_tab == ScreeningTab::All)
            {
                for reason in &r.passed_reasons {
                    ui.label(format!("  • {}", reason));
                }
            }
        });
        ui.add_space(4.0);
    }
}

// ----------------------------------------------------------------------------
// 6. Engineering Rankings Page
// ----------------------------------------------------------------------------

fn rankings_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(6.0);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Ranking Metric:");

            egui::ComboBox::from_id_salt("ranking_metric_selector")
                .selected_text(app.ranking_metric.name())
                .width(320.0)
                .show_ui(ui, |ui| {
                    for metric in RankingMetric::all() {
                        if ui.selectable_label(app.ranking_metric == *metric, metric.name()).clicked() {
                            app.ranking_metric = *metric;
                            app.run_rankings();
                        }
                    }
                });

            ui.add_space(16.0);

            if ui.button("📥 Export Rankings (CSV)").clicked() {
                let items = app.get_rankings().to_vec();
                let csv = export_rankings_csv(&items, app.ranking_metric);
                let filename = format!("rankings_{}.csv", crate::export::get_timestamp_utc().replace(':', "-"));
                match save_export_file(&filename, &csv) {
                    Ok(p) => app.export_notice = Some(format!("Saved to {}", p.display())),
                    Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
                }
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        ui.label(app.ranking_metric.description());

        if app.ranking_metric == RankingMetric::AerospaceScore {
            ui.add_space(4.0);
            ui.colored_label(
                egui::Color32::from_rgb(255, 200, 80),
                "⚠️ Note: The aerospace score is a transparent heuristic figure of merit, NOT an official aerospace engineering qualification standard.",
            );
        }
    });

    ui.add_space(12.0);

    let ranked_items = app.get_rankings().to_vec();

    ui.horizontal(|ui| {
        ui.strong(format!("Ranked {} eligible materials", ranked_items.len()));
        let missing = app
            .database
            .materials
            .len()
            .saturating_sub(ranked_items.len());
        if missing > 0 {
            ui.colored_label(
                egui::Color32::from_rgb(140, 140, 140),
                format!(
                    "({} materials excluded due to missing property data)",
                    missing
                ),
            );
        }
    });

    ui.add_space(8.0);

    // Rankings Table
    egui::Grid::new("rankings_table")
        .striped(true)
        .min_col_width(120.0)
        .spacing([24.0, 8.0])
        .show(ui, |ui| {
            ui.strong("Rank");
            ui.strong("Material Name");
            ui.strong("Category");
            ui.strong("Metric Value");
            ui.strong("Relative Scale");
            ui.strong("Notes / Data Completeness");
            ui.strong("Action");
            ui.end_row();

            let max_val = ranked_items
                .first()
                .map(|item| item.sort_value)
                .unwrap_or(1.0);
            let min_val = ranked_items
                .last()
                .map(|item| item.sort_value)
                .unwrap_or(0.0);
            let range = (max_val - min_val).max(1e-6);

            for (pos, item) in ranked_items.iter().enumerate() {
                let rank_str = match pos + 1 {
                    1 => "🥇 1".to_string(),
                    2 => "🥈 2".to_string(),
                    3 => "🥉 3".to_string(),
                    n => format!("   {}", n),
                };
                ui.label(rank_str);

                ui.strong(&item.name);
                ui.label(&item.category);
                ui.strong(&item.formatted_value);

                let frac = ((item.sort_value - min_val) / range).clamp(0.05, 1.0) as f32;
                ui.add(egui::ProgressBar::new(frac).desired_width(110.0));

                ui.label(item.note.as_deref().unwrap_or("Complete"));

                if ui.button("Inspect →").clicked() {
                    app.inspect_material(item.material_index);
                }

                ui.end_row();
            }
        });
}

// ----------------------------------------------------------------------------
// 7. Physics Calculator Page
// ----------------------------------------------------------------------------

fn calculator_ui(app: &mut MaterialSpaceApp, ui: &mut egui::Ui) {
    ui.add_space(6.0);

    ui.label(
        "Interactive first-principles physics calculators with automatic sanity checking against \
         material property limits (melting point, measurement ranges, and yield strength).",
    );

    ui.add_space(14.0);

    // 1. Linear Thermal Expansion
    ui.group(|ui| {
        ui.heading("1. Linear Thermal Expansion (ΔL = α · L₀ · ΔT)");
        ui.add_space(4.0);
        ui.label("Predicts dimensional change of a solid unconstrained bar under uniform temperature variation.");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Selected Material:");
            calc_material_combobox(app, ui, 0);
        });

        let selected_mat = app.material(app.calc_expansion_material).cloned();

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Initial Length L₀ (m):");
            ui.add(egui::DragValue::new(&mut app.initial_length).speed(0.05).range(0.001..=1000.0));

            ui.add_space(16.0);
            ui.label("Temperature Change ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.delta_temperature).speed(1.0).range(-500.0..=3000.0));
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let exp_result = selected_mat.as_ref().and_then(|m| physics::thermal_expansion(m, app.initial_length, app.delta_temperature));

        match (selected_mat.as_ref(), exp_result) {
            (Some(mat), Some(delta_l)) => {
                let alpha = mat.thermal_expansion_val().unwrap_or(0.0);
                let final_l = app.initial_length + delta_l;

                ui.horizontal(|ui| {
                    ui.label(format!("Coefficient α: {} 1/K", format_scientific(alpha)));
                    ui.label("•");
                    ui.strong(format!("ΔL: {} m  ({:.3} mm)", format_scientific(delta_l), delta_l * 1000.0));
                    ui.label("•");
                    ui.label(format!("Final Length: {} m", format_scientific(final_l)));
                });

                // Sanity checks
                let warnings = evaluate_sanity_checks(mat, app.delta_temperature, app.ambient_temperature_k, None);
                for w in &warnings {
                    ui.colored_label(egui::Color32::from_rgb(255, 160, 40), w.message());
                }

                ui.add_space(6.0);
                if ui.button("📥 Export Expansion Calculation (CSV)").clicked() {
                    let inputs = [
                        ("Initial Length L0 (m)", format_scientific(app.initial_length)),
                        ("Temperature Change Delta T (K)", format_scientific(app.delta_temperature)),
                        ("Thermal Expansion Coefficient alpha (1/K)", format_scientific(alpha)),
                    ];
                    let outputs = [
                        ("Length Change Delta L (m)", format_scientific(delta_l)),
                        ("Final Length (m)", format_scientific(final_l)),
                    ];
                    let assumptions = [
                        "1D unconstrained free linear expansion",
                        "Isotropic material behavior",
                        "Constant alpha over temperature interval",
                    ];
                    let warn_msgs: Vec<String> = warnings.iter().map(|w| w.message()).collect();

                    let csv = export_calculation_csv(
                        "Linear Thermal Expansion",
                        "Delta L = alpha * L0 * Delta T",
                        &mat.name,
                        &inputs.iter().map(|(k, v)| (*k, v.as_str())).collect::<Vec<_>>(),
                        &outputs.iter().map(|(k, v)| (*k, v.as_str())).collect::<Vec<_>>(),
                        &assumptions,
                        &warn_msgs,
                    );
                    let filename = format!("calc_expansion_{}.csv", mat.id);
                    match save_export_file(&filename, &csv) {
                        Ok(p) => app.export_notice = Some(format!("Saved to {}", p.display())),
                        Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
                    }
                }
            }
            _ => {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "Thermal expansion coefficient α is unavailable for this material.");
            }
        }
    });

    ui.add_space(14.0);

    // 2. Heat Energy Transfer
    ui.group(|ui| {
        ui.heading("2. Sensible Heat Energy Transfer (Q = m · c · ΔT)");
        ui.add_space(4.0);
        ui.label("Calculates sensible heat energy required to change an object's temperature without a phase change.");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Selected Material:");
            calc_material_combobox(app, ui, 1);
        });

        let selected_mat = app.material(app.calc_heat_material).cloned();

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Mass m (kg):");
            ui.add(egui::DragValue::new(&mut app.calculator_mass).speed(0.1).range(0.0001..=100_000.0));

            ui.add_space(16.0);
            ui.label("Specific Heat c (J/(kg·K)):");
            match &mut app.calculator_specific_heat {
                Some(cp_val) => {
                    ui.add(egui::DragValue::new(cp_val).speed(10.0).range(1.0..=10_000.0));
                }
                None => {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "Unavailable in dataset");
                }
            }

            ui.add_space(16.0);
            ui.label("ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.heat_delta_temperature).speed(1.0).range(-500.0..=3000.0));
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        match (selected_mat.as_ref(), app.calculator_specific_heat) {
            (Some(mat), Some(cp)) => {
                let q_joules = physics::heat_energy(app.calculator_mass, cp, app.heat_delta_temperature);
                ui.horizontal(|ui| {
                    ui.strong(format!("Required Heat Energy Q: {} J  ({:.3} kJ / {:.4} MJ)", format_scientific(q_joules), q_joules / 1000.0, q_joules / 1.0e6));
                });

                let warnings = evaluate_sanity_checks(mat, app.heat_delta_temperature, app.ambient_temperature_k, None);
                for w in &warnings {
                    ui.colored_label(egui::Color32::from_rgb(255, 160, 40), w.message());
                }
            }
            _ => {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 180, 50),
                    "Specific heat capacity is unavailable for this material. Select a material with specific heat data.",
                );
            }
        }
    });

    ui.add_space(14.0);

    // 3. Fully Constrained Thermal Stress
    ui.group(|ui| {
        ui.heading("3. Fully Constrained Thermal Stress (σ = E · α · ΔT)");
        ui.add_space(4.0);
        ui.label("Estimates upper-bound uniaxial stress induced in a bar with rigid, fixed boundary constraints.");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Selected Material:");
            calc_material_combobox(app, ui, 2);
        });

        let selected_mat = app.material(app.calc_stress_material).cloned();

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Temperature Change ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.stress_delta_temperature).speed(1.0).range(-500.0..=3000.0));
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let stress_result = selected_mat.as_ref().and_then(|m| physics::thermal_stress(m, app.stress_delta_temperature));

        match (selected_mat.as_ref(), stress_result) {
            (Some(mat), Some(stress_pa)) => {
                let e = mat.youngs_modulus_val().unwrap_or(0.0);
                let alpha = mat.thermal_expansion_val().unwrap_or(0.0);
                let stress_mpa = stress_pa / 1.0e6;

                ui.horizontal(|ui| {
                    ui.label(format!("E = {:.1} GPa, α = {} 1/K", e / 1.0e9, format_scientific(alpha)));
                    ui.label("•");
                    ui.strong(format!("Thermal Stress σ: {:.2} MPa ({:.4} GPa)", stress_mpa, stress_pa / 1.0e9));
                });

                let warnings = evaluate_sanity_checks(mat, app.stress_delta_temperature, app.ambient_temperature_k, Some(stress_pa));
                for w in &warnings {
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), w.message());
                }
            }
            _ => {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "Young's Modulus E or Thermal Expansion α is unavailable for this material.");
            }
        }
    });

    ui.add_space(14.0);

    // 4. Steady-State 1D Heat Conduction (Fourier's Law)
    ui.group(|ui| {
        ui.heading("4. Steady-State 1D Heat Conduction (q = k · A · ΔT / L)");
        ui.add_space(4.0);
        ui.label("Calculates 1D steady-state conductive heat transfer rate and surface heat flux across a plane wall.");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Selected Material:");
            calc_material_combobox(app, ui, 3);
        });

        let selected_mat = app.material(app.calc_conduction_material).cloned();

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Area A (m²):");
            ui.add(egui::DragValue::new(&mut app.conduction_area).speed(0.1).range(0.001..=1000.0));

            ui.add_space(16.0);
            ui.label("Thickness L (m):");
            ui.add(egui::DragValue::new(&mut app.conduction_thickness).speed(0.005).range(0.0001..=10.0));

            ui.add_space(16.0);
            ui.label("Temperature Difference ΔT (K):");
            ui.add(egui::DragValue::new(&mut app.conduction_delta_t).speed(1.0).range(0.1..=3000.0));
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let cond_result = selected_mat.as_ref().and_then(|m| {
            physics::heat_conduction(m, app.conduction_area, app.conduction_thickness, app.conduction_delta_t)
        });

        match (selected_mat.as_ref(), cond_result) {
            (Some(_), Some((q_watts, heat_flux))) => {
                ui.horizontal(|ui| {
                    ui.strong(format!("Total Heat Rate q: {} W ({:.3} kW)", format_scientific(q_watts), q_watts / 1000.0));
                    ui.label("•");
                    ui.strong(format!("Heat Flux q'': {} W/m² ({:.2} kW/m²)", format_scientific(heat_flux), heat_flux / 1000.0));
                });
            }
            _ => {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "Thermal conductivity k is unavailable for this material.");
            }
        }
    });

    ui.add_space(14.0);

    // 5. Thermal Diffusion Timescale Estimate
    ui.group(|ui| {
        ui.heading("5. Thermal Diffusion Timescale Estimate (τ ≈ L² · ρ · c_p / k)");
        ui.add_space(4.0);
        ui.label("Estimates the characteristic transient timescale for thermal equilibrium to propagate through thickness L.");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Selected Material:");
            calc_material_combobox(app, ui, 4);
        });

        let selected_mat = app.material(app.calc_diffusion_material).cloned();

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Characteristic Thickness L (m):");
            ui.add(egui::DragValue::new(&mut app.diffusion_thickness).speed(0.002).range(0.0001..=5.0));
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let diff_result = selected_mat.as_ref().and_then(|m| physics::thermal_diffusion_timescale(m, app.diffusion_thickness));

        match (selected_mat.as_ref(), diff_result) {
            (Some(_), Some(tau_seconds)) => {
                ui.horizontal(|ui| {
                    let formatted_time = if tau_seconds < 60.0 {
                        format!("{:.2} seconds", tau_seconds)
                    } else if tau_seconds < 3600.0 {
                        format!("{:.2} minutes ({:.1} s)", tau_seconds / 60.0, tau_seconds)
                    } else {
                        format!("{:.2} hours ({:.1} s)", tau_seconds / 3600.0, tau_seconds)
                    };
                    ui.strong(format!("Characteristic Diffusion Timescale τ: {}", formatted_time));
                });
            }
            _ => {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 180, 50),
                    "Requires Thermal Conductivity, Density, and Specific Heat. One or more properties are unavailable for this material.",
                );
            }
        }
    });
}

fn calc_material_combobox(app: &mut MaterialSpaceApp, ui: &mut egui::Ui, calc_id: u8) {
    let current_idx = match calc_id {
        0 => app.calc_expansion_material,
        1 => app.calc_heat_material,
        2 => app.calc_stress_material,
        3 => app.calc_conduction_material,
        _ => app.calc_diffusion_material,
    };

    let selected_name = current_idx
        .and_then(|idx| app.database.get(idx))
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "Select Material".to_string());

    let salt_id = format!("calc_mat_selector_{}", calc_id);

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
                            app.calculator_specific_heat = mat.specific_heat_val();
                        }
                        2 => app.calc_stress_material = Some(idx),
                        3 => app.calc_conduction_material = Some(idx),
                        _ => app.calc_diffusion_material = Some(idx),
                    }
                }
            }
        });
}

// ----------------------------------------------------------------------------
// 8. About & Methodology Page
// ----------------------------------------------------------------------------

fn about_ui(ui: &mut egui::Ui) {
    ui.add_space(6.0);

    ui.group(|ui| {
        ui.heading("What is MaterialSpace?");
        ui.add_space(6.0);
        ui.label(
            "MaterialSpace is an exploratory materials engineering exploration, analysis, and screening platform \
             built in Rust. It was developed to provide an open, transparent, and reproducible alternative to \
             black-box commercial materials databases, enabling researchers and engineering students to inspect \
             normalized data, evaluate candidates against explicit application requirements, and calculate \
             physical responses with full transparency.",
        );
    });

    ui.add_space(14.0);

    ui.group(|ui| {
        ui.heading("Scientific Data History & Cited References");
        ui.add_space(6.0);
        ui.label("Properties in MaterialSpace originate from normalized datasets cited from four authoritative sources:");
        ui.add_space(6.0);
        ui.label("1. NASA Ames Research Center — TPSX (Thermal Protection Systems Expert Materials Database)");
        ui.label("2. NIST (National Institute of Standards and Technology) — Chemistry WebBook (SRD 69)");
        ui.label("3. MatWeb LLC — Online Materials Information Resource");
        ui.label("4. ASM International — ASM Handbook Series (Vols 1, 2, 21)");
        ui.add_space(6.0);
        ui.label("Spot-Checks Performed: Targeted properties (Al 6061-T6 tensile strength, Ti-6Al-4V conductivity, \
                  Zerodur CTE, Alumina flexural strength) have been verified against original cited handbooks.");
    });

    ui.add_space(14.0);

    ui.group(|ui| {
        ui.heading("Physics & Engineering Metric Equations");
        ui.add_space(6.0);
        ui.label("• Linear Thermal Expansion: ΔL = α · L₀ · ΔT (m)");
        ui.label("• Sensible Heat Energy: Q = m · c_p · ΔT (J)");
        ui.label("• 1D Constrained Thermal Stress: σ = E · α · ΔT (Pa)");
        ui.label("• 1D Steady-State Heat Conduction: q = k · A · ΔT / L (W), q'' = q / A (W/m²)");
        ui.label("• Thermal Diffusion Timescale: τ ≈ (L² · ρ · c_p) / k (seconds)");
        ui.label("• Specific Tensile Strength: σ_uts / ρ (kN·m/kg) — only genuine tensile tests");
        ui.label("• Specific Flexural Strength: σ_flex / ρ (kN·m/kg) — technical ceramics & glasses in bending");
        ui.label("• Specific Stiffness: E / ρ (GPa/(g/cm³)) — elastic deflection resistance");
        ui.label("• Thermal Diffusivity: α_diff = k / (ρ · c_p) (mm²/s)");
    });

    ui.add_space(14.0);

    ui.group(|ui| {
        ui.heading("Scientific Honesty, Assumptions & Limitations");
        ui.add_space(6.0);
        ui.label(
            "1. Missing Data Principle: Missing properties are preserved as 'Data unavailable' and are never \
             silently replaced with zero. In screening, missing properties result in 'Insufficient Data' rather than passing.\n\
             2. Strength Separation: Uniaxial tensile strength is strictly distinguished from flexural strength (modulus of rupture).\n\
             3. Phase Change vs Service Temperature: Melting point indicates a thermodynamic phase boundary, not a continuous certified operating limit.\n\
             4. Aerospace Score: A transparent heuristic index weighting specific strength (40%), stiffness (35%), temperature (15%), and thermal conductivity (10%). It does not represent an industrial flight qualification standard.\n\
             5. 1D Physics Idealizations: Thermal stress assumes rigid boundary constraints and no structural buckling. Heat conduction assumes 1D steady-state planar flow.",
        );
    });
}
