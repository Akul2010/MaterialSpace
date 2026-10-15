# 🔬 MaterialSpace: Research-Grade Materials Exploration & Engineering Screening

> **A scientifically honest, beginner-friendly desktop application for materials data retrieval, multi-criteria screening, transparent ranking, and reproducible engineering physics calculations.**

Built with **Rust** (2024 edition) using `eframe` and `egui`.

---

## 📋 Table of Contents
1. [Project Overview & Philosophy](#-project-overview--philosophy)
2. [Core 5-Layer Architecture](#-core-5-layer-architecture)
3. [Strict Data Retrieval & No-Scraping Policy](#-strict-data-retrieval--no-scraping-policy)
4. [Scientific Corrections & Methodological Integrity](#-scientific-corrections--methodological-integrity)
5. [Governing Physics & Engineering Formulas](#-governing-physics--engineering-formulas)
6. [Multi-Criteria Screening & Transparent Rankings](#-multi-criteria-screening--transparent-rankings)
7. [Automated Sanity Checks & Warning System](#-automated-sanity-checks--warning-system)
8. [Export & Reproducibility Records](#-export--reproducibility-records)
9. [Building, Testing, and Running](#-building-testing-and-running)
10. [Step-by-Step Reproducible Case Study](#-step-by-step-reproducible-case-study)
11. [Source Attribution & Known Limitations](#-source-attribution--known-limitations)

---

## 🎯 Project Overview & Philosophy

**MaterialSpace** was created to bridge the gap between high-school/undergraduate physics learning and research-grade materials selection. Rather than treating materials databases as black boxes or using arbitrary ranking formulas, MaterialSpace adheres to rigorous scientific and software engineering principles:

* **Scientific Honesty Above All**: Missing property data is preserved as `None` (displayed as `Data unavailable`). It is **never** silently assumed to be zero, and missing values never satisfy engineering constraints.
* **Separation of Brittle & Ductile Strength**: Brittle ceramics and glasses measure *flexural strength* (modulus of rupture), not *tensile strength*. These metrics are rigorously separated; flexural strength never pollutes tensile rankings.
* **Transparent Figures of Merit**: All composite scores (e.g., Aerospace Suitability) are explicitly designated as *heuristic screening indices*, not qualified aerospace flight certifiers.
* **Zero Web Scraping**: All data is sourced strictly through official APIs, authorized downloadable datasets, or verified peer-reviewed reference handbooks (e.g., NASA TPSX, NIST SRD, MatWeb, MIL-HDBK-5J / MMPDS).
* **Beginner-Friendly Rust**: Built using clean, idiomatic, single-ownership patterns, explicit `Option<T>` matching, descriptive variable names, and clear module boundaries without complex lifetimes, unsafe code, or convoluted macro systems.

---

## 🏛️ Core 5-Layer Architecture

MaterialSpace is organized into five decoupled layers to ensure maintainability, testability, and scientific auditability:

```text
MaterialSpace/
├── Cargo.toml
├── README.md
├── docs/
│   └── architecture.md
├── data/
│   ├── materials_normalized.json   # 55 normalized baseline materials
│   ├── materials.json              # Raw input data with original units
│   └── sources.json                # Verified citations and access metadata
├── scripts/
│   ├── normalize_db.py             # Schema conversion & SI unit normalization
│   └── validate_db.py              # Data integrity & warning verification
└── src/
    ├── main.rs         # Entry point, eframe options, and native window bootstrap
    ├── app.rs          # Central application state, page routing, and sync logic
    ├── database.rs     # Embedded baseline (include_str!), search, and cache loading
    ├── material.rs     # Provenance-aware Property and Material data structures
    ├── physics.rs      # Transparent physics calculations and sanity check engine
    ├── metrics.rs      # Derived metrics, heuristic scores, and transparent rankings
    ├── screening.rs    # Multi-criteria screening engine with engineering presets
    ├── retrieval.rs    # Official source adapters, no-scraping policy, and cache manager
    ├── export.rs       # RFC 4180 CSV export for datasheets, screenings, and reports
    └── ui.rs           # Modern immediate-mode GUI with egui 0.36
```

### Module Responsibilities:
1. **Material Data Layer (`material.rs`)**: Represents materials with full provenance: SI normalized values, original values, original units, measurement conditions, test standards (e.g. `ASTM E8`, `ASTM C1161`), uncertainties, and citations.
2. **Data Retrieval Layer (`retrieval.rs`)**: Manages external data sources under an absolute no-scraping restriction. Handles the local filesystem cache (`cache/materials_cache.json`), freshness checks, and conservative identity matching.
3. **Normalization & Validation Layer (`normalize_db.py`, `validate_db.py`, `database.rs`)**: Normalizes units into standard SI (`kg/m³`, `GPa`, `MPa`, `W/(m·K)`, `J/(kg·K)`, `1/K`, `K`). Flags unexpected conversions or missing fields.
4. **Research Engine (`physics.rs`, `metrics.rs`, `screening.rs`)**: Evaluates physical formulas, runs physical boundary sanity checks, screens materials against user constraints, and computes transparent weighted rankings.
5. **User Interface (`ui.rs`, `app.rs`)**: Renders an intuitive desktop application with real-time search, side-by-side comparisons, interactive screening checklists, physics calculators with synchronized material properties, and one-click CSV export.

---

## 🚫 Strict Data Retrieval & No-Scraping Policy

MaterialsSpace strictly enforces an **absolute ban on web scraping**, page parsing, and automated browser sessions:

* **NASA TPSX (Thermal Protection Systems Expert)**:
  * *Access Method*: Official structured dataset downloads and authorized data releases.
  * *Limitation*: NASA TPSX does not provide an open unauthenticated REST API. MaterialsSpace ingests verified TPSX datasets via authorized offline JSON/CSV dumps and documents the exact provenance.
* **NIST Chemistry WebBook (SRD 69)**:
  * *Access Method*: Documented NIST Standard Reference Data endpoints and thermophysical property tables.
  * *Limitation*: NIST WebBook focuses primarily on fluid/gas thermophysical properties and select pure elemental substances. It is **not** a universal structural alloy database. MaterialsSpace clearly labels NIST records and does not extrapolate beyond its published data.
* **MatWeb**:
  * *Access Method*: Authorized export files (CSV/XML) or registered enterprise data feeds.
  * *Limitation*: MatWeb prohibits automated screen-scraping under its Terms of Service. MaterialsSpace provides an offline ingest adapter for authorized personal exports and never scrapes HTML pages.

### Hybrid Offline-First Caching
The application embeds a verified baseline dataset directly into the binary using `include_str!`, ensuring that MaterialsSpace operates reliably in air-gapped or offline laboratory environments. When external authorized JSON updates are placed into the local cache directory (`MaterialSpace/cache/`), the application safely loads and validates them, falling back to the embedded baseline if any parsing error occurs.

---

## 🔬 Scientific Corrections & Methodological Integrity

In this release, several critical scientific and numerical bugs present in standard materials databases were resolved:

1. **Separation of Flexural from Tensile Strength**:
   * *Problem*: In many databases, brittle ceramics and glasses (e.g., Sintered SiC, Silicon Nitride $\text{Si}_3\text{N}_4$, Zirconia YSZ, Alumina 99.5%, Borosilicate Glass Pyrex, Zerodur, Macor, Boron Carbide) erroneously store flexural strength (Modulus of Rupture) in the `tensile_strength` field.
   * *Correction*: A dedicated `flexural_strength` field was introduced. Tensile strength for these brittle materials is now correctly marked as `None` (unavailable). They are barred from entering specific tensile strength rankings, preventing flawed comparisons against ductile structural alloys.
2. **Adaptive Scientific Notation**:
   * *Problem*: Fixed decimal rounding (e.g., 5 decimal places) rounded small expansion coefficients like Zerodur ($\approx 5 \times 10^{-8}\ \text{K}^{-1}$) or Carbon Fiber Composite ($\approx -4 \times 10^{-7}\ \text{K}^{-1}$) to `0.00000`.
   * *Correction*: Implemented `format_scientific()`, which formats values $< 10^{-3}$ or $\ge 10^5$ into standard exponential notation (e.g., `5.00e-8`, `2.36e-5`), while keeping human-readable values clean.
3. **Explicit Measurement Conditions**:
   * *Problem*: Yield strength was unconditionally labeled with `(0.2% offset)` and unspecified conditions were rendered as `Standard / Ambient`.
   * *Correction*: Conditions now explicitly display `Not recorded` unless substantiated by source citations. Test methods (e.g., `ASTM E8 0.2% offset`, `ASTM C1161 4-point bend`) are rendered only when explicitly cited in the record.
4. **Correction of the Aerospace Suitability Index**:
   * *Problem*: The thermal term $k/200$ was misdescribed as a thermal diffusivity/stability index, and density was inadvertently double-counted through both specific strength ($\sigma/\rho$) and an isolated density term.
   * *Correction*: The metric was overhauled:
     $$\text{Score} = w_{\text{strength}} \cdot S_{\text{strength}} + w_{\text{stiffness}} \cdot S_{\text{stiffness}} + w_{\text{thermal}} \cdot S_{\text{thermal}} + w_{\text{melt}} \cdot S_{\text{melt}}$$
     Density appears exclusively in the specific property denominators. Thermal conductivity is documented honestly as a heat-dissipation/spreading factor. Materials lacking critical properties receive documented completeness penalties and are clearly flagged.
5. **Melting Point vs. Service Temperature**:
   * Renamed generic "Temperature Capability" labels to **Melting Point** ($T_{\text{melt}}$ or Solidus). Clarified throughout the UI that melting point does **not** represent maximum allowable service temperature, which is governed by creep, oxidation, sublimation, and microstructural phase changes.

---

## 📖 Governing Physics & Engineering Formulas

| Calculation / Metric | Governing Formula | Default Units | Physical Assumptions & Notes |
| :--- | :--- | :--- | :--- |
| **Linear Thermal Expansion** | $\Delta L = \alpha \cdot L_0 \cdot \Delta T$ | $\text{m}$ (or $\text{mm}$) | Assumes isotropic material and constant $\alpha$ over $\Delta T$. |
| **Sensible Heat Energy** | $Q = m \cdot c_p \cdot \Delta T$ | $\text{J}$ (or $\text{kJ}$) | Assumes constant specific heat $c_p$ and no phase change. |
| **1D Fully Constrained Thermal Stress** | $\sigma_{\text{th}} = E \cdot \alpha \cdot \Delta T$ | $\text{MPa}$ | Assumes rigid boundary fixity ($\epsilon_{\text{total}} = 0$) and elastic behavior ($\sigma < \sigma_y$). |
| **1D Steady-State Conduction** | $q = k \cdot A \cdot \frac{T_1 - T_2}{L}$ | $\text{W}$ | Fourier's Law. Steady-state, constant $k$, 1D planar geometry without internal heat generation. |
| **Thermal Diffusion Timescale** | $\tau \approx \frac{L^2}{\alpha_d} = \frac{L^2 \cdot \rho \cdot c_p}{k}$ | $\text{s}$ | Characteristic time for a thermal disturbance to propagate across thickness $L$. |
| **Specific Tensile Strength** | $\text{SS}_{\text{tensile}} = \frac{\sigma_{\text{uts}}}{\rho}$ | $\text{kN}\cdot\text{m}/\text{kg}$ | Structural efficiency under tensile load. Requires true tensile test data. |
| **Specific Yield Strength** | $\text{SS}_{\text{yield}} = \frac{\sigma_y}{\rho}$ | $\text{kN}\cdot\text{m}/\text{kg}$ | Elastic limit structural efficiency. |
| **Specific Stiffness (Modulus)** | $\text{SM} = \frac{E}{\rho}$ | $\text{GPa}/(\text{g}/\text{cm}^3)$ | Resistance to elastic deflection and aeroelastic flutter. |
| **Thermal Diffusivity** | $\alpha_d = \frac{k}{\rho \cdot c_p}$ | $\text{mm}^2/\text{s}$ | Governs transient heat propagation rates. |

---

## 🎯 Multi-Criteria Screening & Transparent Rankings

### Requirements-Based Screening Engine
The screening engine allows engineers to set strict hard constraints. It categorizes every material into one of three distinct statuses:
1. **Passed**: The candidate demonstrably satisfies all active constraints.
2. **Failed**: The candidate has verified property data that violates at least one constraint.
3. **Uncertain / Incomplete**: The candidate is missing one or more required properties. Crucially, **missing data is never assumed to satisfy a constraint**.

#### Pre-Configured Engineering Profiles:
* **Aerospace Structures**: High specific tensile strength ($\ge 120\ \text{kN}\cdot\text{m}/\text{kg}$), high stiffness ($\ge 70\ \text{GPa}$), lightweight density ($\le 4500\ \text{kg/m³}$).
* **Thermal Protection Systems (TPS)**: Ultra-low thermal conductivity ($\le 2.0\ \text{W/(m·K)}$), low expansion ($\le 5 \times 10^{-6}\ \text{K}^{-1}$), high melting point ($\ge 1800\ \text{K}$).
* **Heat Exchangers & Heat Sinks**: Ultra-high thermal conductivity ($\ge 150\ \text{W/(m·K)}$), moderate operating temperature ($\ge 500\ \text{K}$).
* **Lunar & Space Infrastructure**: Low thermal expansion ($\le 8 \times 10^{-6}\ \text{K}^{-1}$), high stiffness ($\ge 50\ \text{GPa}$), broad thermal survivability ($\ge 1200\ \text{K}$).
* **General Lightweight Structural**: Minimum yield strength ($\ge 250\ \text{MPa}$), low density ($\le 3000\ \text{kg/m³}$).

### Transparent Multi-Metric Ranking
Users can rank eligible materials across 12 distinct physical metrics or composite indices. For composite ranking, properties are min-max normalized:
$$\text{Normalized}_i = \frac{x_i - x_{\min}}{x_{\max} - x_{\min}} \quad (\text{if higher is better})$$
$$\text{Normalized}_i = \frac{x_{\max} - x_i}{x_{\max} - x_{\min}} \quad (\text{if lower is better})$$
Every ranking table displays the unweighted component scores, data completeness percentage, and eligibility rationale.

---

## ⚠️ Automated Sanity Checks & Warning System

To prevent unphysical engineering conclusions, the calculation engine runs automated sanity checks on all inputs:

* **Melting Point Violations**: Triggers an alert if final temperature $T_{\text{final}} > T_{\text{melt}}$.
* **Yield Stress Violations**: In the thermal stress calculator, warns if calculated thermal stress $\sigma_{\text{th}} \ge \sigma_y$, indicating plastic deformation or cracking.
* **Extrapolation Warnings**: Alerts if temperature changes exceed $300\ \text{K}$ from ambient, reminding the user that room-temperature properties ($\alpha$, $k$, $c_p$) vary significantly at elevated temperatures.
* **Missing Specific Heat Safety**: When transferring a material to the calculator, if specific heat is not recorded in the database, the calculator value is cleared (`None`), preventing the accidental reuse of stale properties from previously selected materials.

---

## 📊 Export & Reproducibility Records

All scientific calculations, screening runs, and database records can be exported directly to standard **RFC 4180 compliant CSV files** using the in-app export buttons:

1. **Complete Database Export**: Full dump of all 55 baseline materials, properties, units, conditions, and citations.
2. **Side-by-Side Comparison Export**: Aligned tabular comparison of any two materials.
3. **Screening Audit Trail**: Detailed list of candidates, pass/fail status, violated constraints, and missing property checklists.
4. **Ranking Results**: Ordered candidates with normalized property components, raw values, and completeness scores.
5. **Calculation Record**: Complete snapshot containing calculation inputs, governing equation, calculated results, physical assumptions, triggered warnings, material metadata, and UTC analysis timestamp.

---

## 🛠️ Building, Testing, and Running

### Prerequisites
* [Rust Toolchain](https://rustup.rs/) (Stable 1.85+ / Edition 2024 recommended)
* [Python 3.10+](https://www.python.org/) (for database normalization and validation scripts)

### Commands
```bash
# 1. Validate and normalize the database
python scripts/normalize_db.py
python scripts/validate_db.py

# 2. Check compilation
cargo check

# 3. Run the automated test suite
cargo test

# 4. Launch the application
cargo run --release
```

---

## 🧪 Step-by-Step Reproducible Case Study

### Objective: Select a high-temperature structural alloy for a rocket nozzle skirt.
1. **Launch MaterialSpace** and navigate to the **Screening Engine** tab.
2. **Apply Constraints**:
   * Minimum Melting Point: $1500\ \text{K}$
   * Maximum Density: $9000\ \text{kg/m³}$
   * Minimum Tensile Strength: $800\ \text{MPa}$
3. **Observe Results**:
   * **Inconel 718** (Nickel Superalloy): **PASSED** ($T_{\text{melt}} = 1609\ \text{K}$, $\rho = 8190\ \text{kg/m³}$, $\sigma_{\text{uts}} = 1240\ \text{MPa}$).
   * **Ti-6Al-4V Grade 5**: **PASSED** ($T_{\text{melt}} = 1928\ \text{K}$, $\rho = 4430\ \text{kg/m³}$, $\sigma_{\text{uts}} = 950\ \text{MPa}$).
   * **Sintered SiC**: **EXCLUDED FROM TENSILE LIST** — Flexural strength is $410\ \text{MPa}$, but tensile data is unavailable. The screening engine transparently flags it as *Uncertain / Incomplete* for tensile criteria rather than making an invalid substitution.
4. **Export Results**: Click **Export Screening Results (CSV)** to save the reproducible record for peer review.

---

## 📜 Source Attribution & Known Limitations

### Data Provenance
Data in MaterialSpace has been cross-referenced against primary references:
* **NASA TPSX Database**: High-temperature ceramics, tiles, and carbon-carbon composites.
* **NIST Chemistry WebBook & SRD**: Pure elemental thermophysical properties.
* **MMPDS-01 / MIL-HDBK-5J**: Metallic aerospace alloys (Aluminum 2024/7075, Titanium 6Al-4V, Stainless steels).
* **ASM International Handbooks**: Engineering plastics, refractories, and copper alloys.

### Known Limitations
* **Constant Property Assumption**: Calculations currently assume constant room-temperature values. Temperature-dependent curves $k(T)$, $\alpha(T)$, and $\sigma(T)$ are planned for future versions.
* **1D Geometry**: Conduction and thermal stress calculations utilize 1D planar approximations and do not substitute for 3D finite element analysis (FEA).
* **Heuristic Scores**: Ranking scores provide directional guidance during conceptual design and do not replace formal aerospace flight qualification testing.

---

## 📄 License & Academic Integrity
Developed as an educational and scientific research project. Distributed under the MIT License. Feel free to use, inspect, and extend for research and teaching!