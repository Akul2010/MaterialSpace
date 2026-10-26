use crate::material::{Material, format_scientific};
use crate::metrics::{RankedMaterialItem, RankingMetric};
use crate::screening::{Constraints, ScreeningResult, ScreeningStatus};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Makes a timestamp string for the export filename.
pub fn get_timestamp_utc() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let total_secs = now.as_secs();
    let days = total_secs / 86400;
    let rem_secs = total_secs % 86400;
    let hours = rem_secs / 3600;
    let minutes = (rem_secs % 3600) / 60;
    let seconds = rem_secs % 60;

    // Approximate calendar conversion from epoch day (1970-01-01)
    let (year, month, day) = days_to_ymd(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let mut d = days;
    let mut year = 1970;
    loop {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if is_leap { 366 } else { 365 };
        if d < days_in_year {
            break;
        }
        d -= days_in_year;
        year += 1;
    }
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let month_days = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1;
    for &md in &month_days {
        if d < md {
            break;
        }
        d -= md;
        month += 1;
    }
    (year, month, d + 1)
}

fn escape_csv(val: &str) -> String {
    if val.contains(',') || val.contains('"') || val.contains('\n') || val.contains('\r') {
        format!("\"{}\"", val.replace('"', "\"\""))
    } else {
        val.to_string()
    }
}

/// Saves an export under `exports/` and gives back its path.
pub fn save_export_file(file_name: &str, content: &str) -> Result<PathBuf, String> {
    let export_dir = Path::new("exports");
    if !export_dir.exists() {
        fs::create_dir_all(export_dir)
            .map_err(|e| format!("Could not create exports directory: {}", e))?;
    }

    let target_path = export_dir.join(file_name);
    fs::write(&target_path, content)
        .map_err(|e| format!("Could not write export file {:?}: {}", target_path, e))?;

    Ok(target_path)
}

// ----------------------------------------------------------------------------
// 1. Database & Material Export
// ----------------------------------------------------------------------------

/// Turns one material's details into CSV.
pub fn export_material_csv(material: &Material) -> String {
    let mut out = String::new();
    out.push_str("# MaterialSpace Material Datasheet Export\n");
    out.push_str(&format!(
        "# Export Timestamp UTC: {}\n",
        get_timestamp_utc()
    ));
    out.push_str("# Application Version: MaterialSpace v0.1.0 (Research Edition)\n");
    out.push_str(&format!("# Material ID: {}\n", material.id));
    out.push_str(&format!("# Canonical Name: {}\n", material.name));
    out.push_str(&format!("# Category: {}\n", material.category_display()));
    out.push_str(&format!(
        "# Composition: {}\n",
        material.composition.as_deref().unwrap_or("Not recorded")
    ));
    out.push_str(&format!(
        "# Grade: {}\n",
        material.grade.as_deref().unwrap_or("Not recorded")
    ));
    out.push_str("\n");
    out.push_str("Property,Normalized Value,SI Unit,Condition,Test Method,Original Value,Original Unit,Source ID\n");

    let props = [
        ("Density", &material.density),
        ("Young's Modulus", &material.youngs_modulus),
        ("Yield Strength", &material.yield_strength),
        ("Tensile Strength", &material.tensile_strength),
        ("Flexural Strength", &material.flexural_strength),
        ("Thermal Conductivity", &material.thermal_conductivity),
        ("Specific Heat", &material.specific_heat),
        ("Thermal Expansion", &material.thermal_expansion),
        ("Thermal Emissivity", &material.emissivity),
        ("Melting Point", &material.melting_point),
        ("Boiling Point", &material.boiling_point),
    ];

    for (name, prop_opt) in props {
        if let Some(prop) = prop_opt {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                escape_csv(name),
                format_scientific(prop.value),
                escape_csv(prop.unit.as_deref().unwrap_or("")),
                escape_csv(prop.condition_display()),
                escape_csv(prop.test_method.as_deref().unwrap_or("Not recorded")),
                prop.original_value
                    .map(|v| format_scientific(v))
                    .unwrap_or_else(|| "—".into()),
                escape_csv(prop.original_unit.as_deref().unwrap_or("—")),
                escape_csv(prop.source_id.as_deref().unwrap_or("—"))
            ));
        } else {
            out.push_str(&format!(
                "{},Data unavailable,N/A,N/A,N/A,N/A,N/A,N/A\n",
                escape_csv(name)
            ));
        }
    }

    out
}

/// Turns the whole materials list into CSV.
pub fn export_database_csv(materials: &[Material]) -> String {
    let mut out = String::new();
    out.push_str("ID,Name,Category,Composition,Grade,Density (kg/m3),Young's Modulus (Pa),Yield Strength (Pa),Tensile Strength (Pa),Flexural Strength (Pa),Thermal Conductivity (W/(m*K)),Specific Heat (J/(kg*K)),Thermal Expansion (1/K),Melting Point (K),Sources\n");

    for m in materials {
        let sources = m.source_ids.join("; ");
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            escape_csv(&m.id),
            escape_csv(&m.name),
            escape_csv(m.category_display()),
            escape_csv(m.composition.as_deref().unwrap_or("")),
            escape_csv(m.grade.as_deref().unwrap_or("")),
            m.density_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.youngs_modulus_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.yield_strength_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.tensile_strength_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.flexural_strength_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.thermal_conductivity_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.specific_heat_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.thermal_expansion_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            m.melting_point_val()
                .map(|v| format_scientific(v))
                .unwrap_or_else(|| "N/A".into()),
            escape_csv(&sources)
        ));
    }

    out
}

// ----------------------------------------------------------------------------
// 2. Material Comparison Export
// ----------------------------------------------------------------------------

/// Puts a two-material comparison into CSV, including units and test conditions.
pub fn export_comparison_csv(a: &Material, b: &Material) -> String {
    let mut out = String::new();
    out.push_str("# MaterialSpace Side-by-Side Comparison Export\n");
    out.push_str(&format!(
        "# Export Timestamp UTC: {}\n",
        get_timestamp_utc()
    ));
    out.push_str(&format!("# Material A: {} (ID: {})\n", a.name, a.id));
    out.push_str(&format!("# Material B: {} (ID: {})\n\n", b.name, b.id));
    out.push_str("Property,Material A Value,Material A Condition,Material B Value,Material B Condition,SI Unit\n");

    let props = [
        ("Density", a.density.as_ref(), b.density.as_ref(), "kg/m^3"),
        (
            "Young's Modulus",
            a.youngs_modulus.as_ref(),
            b.youngs_modulus.as_ref(),
            "Pa",
        ),
        (
            "Yield Strength",
            a.yield_strength.as_ref(),
            b.yield_strength.as_ref(),
            "Pa",
        ),
        (
            "Tensile Strength",
            a.tensile_strength.as_ref(),
            b.tensile_strength.as_ref(),
            "Pa",
        ),
        (
            "Flexural Strength",
            a.flexural_strength.as_ref(),
            b.flexural_strength.as_ref(),
            "Pa",
        ),
        (
            "Thermal Conductivity",
            a.thermal_conductivity.as_ref(),
            b.thermal_conductivity.as_ref(),
            "W/(m*K)",
        ),
        (
            "Specific Heat",
            a.specific_heat.as_ref(),
            b.specific_heat.as_ref(),
            "J/(kg*K)",
        ),
        (
            "Thermal Expansion",
            a.thermal_expansion.as_ref(),
            b.thermal_expansion.as_ref(),
            "1/K",
        ),
        (
            "Melting Point",
            a.melting_point.as_ref(),
            b.melting_point.as_ref(),
            "K",
        ),
    ];

    for (name, prop_a, prop_b, unit) in props {
        let val_a = prop_a
            .map(|p| format_scientific(p.value))
            .unwrap_or_else(|| "Data unavailable".into());
        let cond_a = prop_a
            .map(|p| p.condition_display().to_string())
            .unwrap_or_else(|| "N/A".into());
        let val_b = prop_b
            .map(|p| format_scientific(p.value))
            .unwrap_or_else(|| "Data unavailable".into());
        let cond_b = prop_b
            .map(|p| p.condition_display().to_string())
            .unwrap_or_else(|| "N/A".into());

        out.push_str(&format!(
            "{},{},{},{},{},{}\n",
            escape_csv(name),
            escape_csv(&val_a),
            escape_csv(&cond_a),
            escape_csv(&val_b),
            escape_csv(&cond_b),
            escape_csv(unit)
        ));
    }

    out
}

// ----------------------------------------------------------------------------
// 3. Screening Results Export
// ----------------------------------------------------------------------------

/// Puts screening results into CSV, including passes, failures, and missing data.
pub fn export_screening_csv(results: &[ScreeningResult], constraints: &Constraints) -> String {
    let mut out = String::new();
    out.push_str("# MaterialSpace Requirements Screening Export\n");
    out.push_str(&format!(
        "# Export Timestamp UTC: {}\n",
        get_timestamp_utc()
    ));
    out.push_str("# Active Screening Constraints:\n");
    if let Some(v) = constraints.max_density {
        out.push_str(&format!(
            "# - Max Density: {} kg/m3\n",
            format_scientific(v)
        ));
    }
    if let Some(v) = constraints.min_thermal_conductivity {
        out.push_str(&format!("# - Min Thermal Conductivity: {:.2} W/(m*K)\n", v));
    }
    if let Some(v) = constraints.max_thermal_expansion {
        out.push_str(&format!(
            "# - Max Thermal Expansion: {} 1/K\n",
            format_scientific(v)
        ));
    }
    if let Some(v) = constraints.min_tensile_strength {
        out.push_str(&format!("# - Min Tensile Strength: {:.1} MPa\n", v / 1.0e6));
    }
    if let Some(v) = constraints.min_yield_strength {
        out.push_str(&format!("# - Min Yield Strength: {:.1} MPa\n", v / 1.0e6));
    }
    if let Some(v) = constraints.min_flexural_strength {
        out.push_str(&format!(
            "# - Min Flexural Strength: {:.1} MPa\n",
            v / 1.0e6
        ));
    }
    if let Some(v) = constraints.min_youngs_modulus {
        out.push_str(&format!("# - Min Young's Modulus: {:.1} GPa\n", v / 1.0e9));
    }
    if let Some(v) = constraints.min_melting_point {
        out.push_str(&format!("# - Min Melting Point: {:.0} K\n", v));
    }
    out.push_str("\n");
    out.push_str("Status,Material ID,Material Name,Category,Passed Checks,Failed Constraints,Missing Data Checks\n");

    for r in results {
        let status_str = match r.status {
            ScreeningStatus::Passed => "PASSED",
            ScreeningStatus::Failed => "FAILED",
            ScreeningStatus::InsufficientData => "INSUFFICIENT_DATA",
        };

        out.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            status_str,
            escape_csv(&r.material.id),
            escape_csv(&r.material.name),
            escape_csv(r.material.category_display()),
            escape_csv(&r.passed_reasons.join(" | ")),
            escape_csv(&r.failed_reasons.join(" | ")),
            escape_csv(&r.missing_reasons.join(" | "))
        ));
    }

    out
}

// ----------------------------------------------------------------------------
// 4. Rankings Export
// ----------------------------------------------------------------------------

/// Puts the rankings into CSV.
pub fn export_rankings_csv(items: &[RankedMaterialItem], metric: RankingMetric) -> String {
    let mut out = String::new();
    out.push_str("# MaterialSpace Engineering Rankings Export\n");
    out.push_str(&format!(
        "# Export Timestamp UTC: {}\n",
        get_timestamp_utc()
    ));
    out.push_str(&format!("# Ranking Metric: {}\n", metric.name()));
    out.push_str(&format!("# Description: {}\n\n", metric.description()));
    out.push_str("Rank,Material Name,Category,Formatted Value,Sort Value,Completeness Notes\n");

    for (pos, item) in items.iter().enumerate() {
        out.push_str(&format!(
            "{},{},{},{},{},{}\n",
            pos + 1,
            escape_csv(&item.name),
            escape_csv(&item.category),
            escape_csv(&item.formatted_value),
            format_scientific(item.sort_value),
            escape_csv(item.note.as_deref().unwrap_or("Complete"))
        ));
    }

    out
}

// ----------------------------------------------------------------------------
// 5. Physics Calculation Export
// ----------------------------------------------------------------------------

/// Puts a calculation into CSV along with its inputs, results, and warnings.
pub fn export_calculation_csv(
    calc_name: &str,
    formula: &str,
    material_name: &str,
    inputs: &[(&str, &str)],
    outputs: &[(&str, &str)],
    assumptions: &[&str],
    warnings: &[String],
) -> String {
    let mut out = String::new();
    out.push_str("# MaterialSpace Physics Calculation Record\n");
    out.push_str(&format!(
        "# Export Timestamp UTC: {}\n",
        get_timestamp_utc()
    ));
    out.push_str(&format!("# Calculation: {}\n", calc_name));
    out.push_str(&format!("# Formula: {}\n", formula));
    out.push_str(&format!("# Evaluated Material: {}\n\n", material_name));

    out.push_str("Section,Parameter,Value\n");
    for (param, val) in inputs {
        out.push_str(&format!(
            "Input,{},{}\n",
            escape_csv(param),
            escape_csv(val)
        ));
    }
    for (param, val) in outputs {
        out.push_str(&format!(
            "Output,{},{}\n",
            escape_csv(param),
            escape_csv(val)
        ));
    }
    for a in assumptions {
        out.push_str(&format!(
            "Assumption,Boundary Condition,{}\n",
            escape_csv(a)
        ));
    }
    for w in warnings {
        out.push_str(&format!("Warning,Physical Check,{}\n", escape_csv(w)));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_csv() {
        assert_eq!(escape_csv("simple"), "simple");
        assert_eq!(escape_csv("has,comma"), "\"has,comma\"");
        assert_eq!(escape_csv("has \"quotes\""), "\"has \"\"quotes\"\"\"");
    }

    #[test]
    fn test_timestamp_utc_format() {
        let ts = get_timestamp_utc();
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 20); // YYYY-MM-DDTHH:MM:SSZ
    }
}
