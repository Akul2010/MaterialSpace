# 🏛️ MaterialSpace: System Architecture & Engineering Design

This document details the architectural design, module structure, scientific methodology, data flow, and beginner-friendly Rust patterns implemented in **MaterialSpace**.

---

## 1. High-Level Architecture: The 5-Layer System

MaterialSpace is organized into five strictly decoupled layers. This ensures that scientific computing, data access, normalization, and UI rendering can be audited, tested, and maintained independently.

```mermaid
graph TD
    subgraph Layer 1: Data Retrieval
        TPSX[NASA TPSX Structured Ingestion]
        NIST[NIST SRD Thermophysical Tables]
        MatWeb[MatWeb Export Ingestion]
        CacheMgr[retrieval.rs: LocalCacheManager]
    end

    subgraph Layer 2: Material Data & Baseline
        Embedded[database.rs: Embedded Baseline (include_str!)]
        Disk[data/materials_normalized.json]
        Model[material.rs: Material & Property Models]
    end

    subgraph Layer 3: Normalization & Validation
        NormScript[scripts/normalize_db.py]
        ValScript[scripts/validate_db.py]
        Sanity[physics.rs: Sanity Check Engine]
    end

    subgraph Layer 4: Research Engine
        Physics[physics.rs: First-Principles Physics]
        Metrics[metrics.rs: Figures of Merit & Rankings]
        Screening[screening.rs: Multi-Criteria Screening Engine]
    end

    subgraph Layer 5: User Interface & Export
        App[app.rs: MaterialSpaceApp State & Routing]
        UI[ui.rs: Immediate-Mode egui Layout]
        Export[export.rs: RFC 4180 CSV Engine]
    end

    TPSX -.-> CacheMgr
    NIST -.-> CacheMgr
    MatWeb -.-> CacheMgr
    CacheMgr --> Model
    Embedded --> Model
    Disk --> Model

    Model --> Physics
    Model --> Metrics
    Model --> Screening

    Physics --> Sanity
    Sanity --> UI
    Physics --> App
    Metrics --> App
    Screening --> App

    App --> UI
    App --> Export
```

---

## 2. Layer-by-Layer Specifications

### Layer 1: Data Retrieval Layer (`src/retrieval.rs`)
* **Strict No-Scraping Policy**: Direct website scraping, HTML parsing, and headless browser automation are completely prohibited. All external data ingestion must use authorized machine-readable access methods:
  * **NASA TPSX**: Processed via official downloadable structured datasets.
  * **NIST Chemistry WebBook (SRD 69)**: Accessed via documented thermophysical property tables for fluid/gas species and select elements; clearly documented as non-universal for structural alloys.
  * **MatWeb**: Handled via authorized user export dumps (CSV/XML), adhering to MatWeb's Terms of Service.
* **Conservative Material Identity Matching**: To avoid merging non-identical materials (e.g. Alumina 99.5% vs Alumina 96%), `is_conservative_match()` enforces strict identity matching based on exact canonical ID, exact name, or explicit documented aliases.
* **Local Caching System**: Managed by `LocalCacheManager`. Stores newly acquired materials in `data/cache/` without overwriting the curated baseline.

### Layer 2: Material Data Layer (`src/material.rs`, `src/database.rs`)
* **Provenance-Aware Property Model**: Every property is represented as `Option<Property>`, where `Property` encapsulates:
  ```rust
  pub struct Property {
      pub value: f64,                       // Normalized SI numeric value
      pub unit: String,                     // Standard SI unit (e.g. "kg/m³", "MPa")
      pub original_value: Option<f64>,      // Raw value before normalization
      pub original_unit: Option<String>,    // Raw input unit (e.g. "g/cc", "ksi")
      pub condition: Option<String>,        // Measurement condition (e.g. "293 K", "0.2% offset")
      pub test_method: Option<String>,      // ASTM / ISO standard (e.g. "ASTM E8", "ASTM C1161")
      pub temperature: Option<f64>,         // Test temperature (K)
      pub uncertainty: Option<String>,      // Measurement uncertainty (e.g. "± 5%")
      pub source_id: Option<String>,        // Citation key mapping to sources.json
      pub notes: Option<String>,            // Caveats or microstructural notes
  }
  ```
* **Separation of Tensile vs. Flexural Strength**:
  * Brittle materials (ceramics and glasses like SiC, $\text{Si}_3\text{N}_4$, $\text{Al}_2\text{O}_3$, Zirconia, Pyrex, Zerodur, Macor) store their strength measurements in `flexural_strength` (Modulus of Rupture).
  * `tensile_strength` is set to `None` for these materials. They are excluded from tensile specific strength rankings, preserving scientific rigor.
* **Robust Dual-Mode Database Loading**:
  * Bundled fallback embedded directly into the binary via `include_str!("../data/materials_normalized.json")`. The application runs seamlessly even if executed outside the project root or without disk access.
  * Checks for disk updates in `data/materials_normalized.json` and loads cached materials from `data/cache/` dynamically.

### Layer 3: Normalization and Validation Layer (`scripts/`, `src/physics.rs`)
* **Python Normalization Pipeline (`scripts/normalize_db.py`)**:
  * Converts pressure/strength units (`psi`, `ksi`, `kPa`, `GPa` $\to$ `MPa`).
  * Converts density (`g/cm³`, `lb/in³` $\to$ `kg/m³`).
  * Converts thermal conductivity (`cal/(s·cm·K)`, `BTU/(hr·ft·°F)` $\to$ `W/(m·K)`).
  * Converts thermal expansion (`1/°C`, `1/°F` $\to$ `1/K`).
  * Converts temperature (`°C`, `°F` $\to$ `K`).
* **Python Validation Suite (`scripts/validate_db.py`)**:
  * Verifies unique material IDs, valid source citations, schema adherence, and fails the build if any unexpected `normalization_warning` entries appear.
* **Automated Physical Sanity Checker (`physics::evaluate_sanity_checks`)**:
  * Alerts if temperature inputs exceed documented material melting points ($T > T_{\text{melt}}$).
  * Alerts if constrained thermal stress exceeds yield strength ($\sigma_{\text{th}} \ge \sigma_y$).
  * Alerts when extrapolating room-temperature properties across large $\Delta T > 300\ \text{K}$.

### Layer 4: Research Engine (`src/physics.rs`, `src/metrics.rs`, `src/screening.rs`)
* **First-Principles Physics**:
  1. *Linear Thermal Expansion*: $\Delta L = \alpha \cdot L_0 \cdot \Delta T$
  2. *Sensible Heat Transfer*: $Q = m \cdot c_p \cdot \Delta T$
  3. *1D Constrained Thermal Stress*: $\sigma_{\text{th}} = E \cdot \alpha \cdot \Delta T$
  4. *1D Steady-State Conduction*: $q = k \cdot A \cdot \frac{\Delta T}{L}$
  5. *Thermal Diffusion Timescale*: $\tau \approx \frac{L^2}{\alpha_d} = \frac{L^2 \cdot \rho \cdot c_p}{k}$
* **Derived Figures of Merit**:
  * *Specific Tensile Strength*: $\sigma_{\text{uts}} / \rho$
  * *Specific Yield Strength*: $\sigma_y / \rho$
  * *Specific Flexural Strength*: $\sigma_{\text{flex}} / \rho$
  * *Specific Modulus (Stiffness)*: $E / \rho$
  * *Thermal Diffusivity*: $\alpha_d = k / (\rho \cdot c_p)$
  * *Aerospace Suitability Heuristic Index*: Multi-criteria weighted index evaluating strength-to-weight, stiffness-to-weight, thermal conductivity, and melting point, with transparent completeness scoring.
* **Screening Engine**:
  * Evaluates candidates against strict user constraints.
  * Explicitly categorizes results into `Passed`, `Failed`, or `InsufficientData`.
  * Provides 5 engineering presets (Aerospace Structures, TPS, Heat Exchangers, Lunar Infrastructure, Lightweight Structural).

### Layer 5: User Interface and Export (`src/ui.rs`, `src/app.rs`, `src/export.rs`)
* Built with `eframe` and `egui 0.36`.
* Interactive pages:
  * **Dashboard**: Key database statistics and quick-access cards.
  * **Materials Database**: Real-time multi-field search and category filtering.
  * **Compare Materials**: Side-by-side property comparison with normalized progress bars.
  * **Material Details**: Deep property inspector, ASTM test condition display, and verified citations.
  * **Requirements Screening**: Interactive criteria sliders, presets, and pass/fail checklists.
  * **Engineering Rankings**: 12 selectable criteria with relative performance bars.
  * **Physics Calculator**: Interactive physical calculators with synchronized property transfer.
  * **About & Methodology**: Mathematical formulations, citations, and scientific honesty notes.
* **RFC 4180 CSV Export**: One-click generation of fully quoted, timestamped CSV files for spreadsheets and Python research workflows.

---

## 3. Safe Rust State Management & Beginner-Friendly Architecture

To keep the application accessible to developers learning Rust, MaterialSpace adheres to three key programming design rules:

### A. Explicit `Option<T>` Flow Over Silent Fallbacks
Materials data in the real world is inherently sparse. Missing measurements are represented as `Option<Property>` and unwrapped using idiomatic pattern matching:
```rust
match material.tensile_strength_val() {
    Some(uts) => ui.label(format!("{:.1} MPa", uts / 1.0e6)),
    None => ui.colored_label(egui::Color32::from_rgb(140, 140, 140), "Data unavailable"),
};
```
Missing data is never silently substituted with zero, preventing false rankings and flawed calculations.

### B. Single-Owner GUI State Pattern
In immediate-mode GUIs (`egui`), widget closures render on every frame. Complex architectures often introduce reference cycles, `Rc<RefCell<T>>`, or unsafe pointers. MaterialSpace avoids this entirely:
* `MaterialSpaceApp` owns the database, active page, filter strings, and calculator inputs directly.
* Clones are performed on small lightweight data structures (such as `Vec<ScreeningResult>` or `String`) only when necessary across UI frames.
* Zero `unsafe` code blocks exist anywhere in the project.

### C. Calculator Material Synchronization
When the user clicks **Use in Calculator** from any material card, properties are synchronized directly into the calculator state:
```rust
pub fn sync_calculator_material(&mut self, material_index: usize) {
    if let Some(mat) = self.db.get(material_index) {
        self.calculator_material_index = Some(material_index);
        // Safely set or clear property inputs
        if let Some(alpha) = mat.thermal_expansion_val() {
            self.calculator_thermal_expansion = alpha;
        }
        // Missing specific heat must explicitly clear previous value to avoid stale data
        self.calculator_specific_heat = mat.specific_heat_val();
    }
}
```
This guarantees that stale properties from previously inspected materials cannot contaminate subsequent calculations.
