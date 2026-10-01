# 🔬 MaterialSpace — Materials Engineering Exploration & Analysis

**MaterialSpace** is a desktop engineering application built in **Rust** using `eframe` and `egui`. It provides an intuitive, scientifically honest platform to explore normalized materials data, perform first-principles physics calculations, evaluate derived engineering metrics, and rank materials according to transparent engineering criteria.

---

## 🚀 Key Features

* **📚 Materials Database Explorer**: Search and filter over 50+ normalized engineering materials (aerospace alloys, ceramics, refractory metals, and polymers) with real-time multi-field querying.
* **🔍 Deep Property Inspection**: View mechanical and thermal properties with SI units, original values, test conditions (e.g. 0.2% offset, Solidus), and verified source citations.
* **⚖️ Side-by-Side Comparison**: Compare two materials with property alignment and visual relative performance comparison bars.
* **🧮 Physics Calculator**:
  * *Linear Thermal Expansion*: $\Delta L = \alpha \cdot L_0 \cdot \Delta T$
  * *Thermal Heat Transfer*: $Q = m \cdot c \cdot \Delta T$
  * *Constrained Thermal Stress*: $\sigma = E \cdot \alpha \cdot \Delta T$ with yield strength threshold warnings.
* **📊 Derived Engineering Figures of Merit**:
  * *Specific Strength* ($\sigma / \rho$)
  * *Specific Modulus* ($E / \rho$)
  * *Thermal Diffusivity* ($\alpha = k / (\rho \cdot c_p)$)
  * *Temperature Capability* ($T_{\text{melt}}$)
  * *Aerospace Suitability Heuristic Score* (0–100 weighted index)
* **🏆 Engineering Rankings**: Rank materials according to chosen criteria (Specific Strength, Thermal Conductivity, Temperature Capability, Lowest Density, etc.) with relative bar scaling.
* **📑 Rule-Based Engineering Insights**: Transparent observations highlighting lightweight candidates, high thermal conductors, insulators, and structural stiffness.
* **📜 Verified Data Provenance**: Traceable citations linking properties directly to NASA TPSX, NIST Chemistry WebBook, MatWeb, and ASM Handbook datasets.

---

## 🏛️ Scientific Honesty & Methodology

1. **Explicit Missing Data Handling**: Missing material properties are stored as `Option<Property>` and displayed as `Data unavailable`. A missing measurement is never assumed to be zero.
2. **Heuristic Labeling**: Derived scores (such as the Aerospace Suitability Score) are explicitly labeled as *heuristics* rather than industry qualification standards.
3. **Transparent Physics Assumptions**: Physical approximations state their assumptions explicitly (e.g., 1D uniaxial constraints, isotropic behavior, absence of buckling).

---

## 📖 Governing Physics & Engineering Formulas

| Metric / Calculation | Formula | Units | Significance |
| :--- | :--- | :--- | :--- |
| **Linear Thermal Expansion** | $\Delta L = \alpha \cdot L_0 \cdot \Delta T$ | $\text{m}$ (or $\text{mm}$) | Predicts thermal dimensional changes |
| **Sensible Heat Energy** | $Q = m \cdot c \cdot \Delta T$ | $\text{J}$ (or $\text{kJ}$) | Energy required to raise/lower temperature |
| **Constrained Thermal Stress** | $\sigma = E \cdot \alpha \cdot \Delta T$ | $\text{Pa}$ (or $\text{MPa}$) | Stress generated under rigid 1D boundary fixity |
| **Specific Strength** | $\text{SS} = \frac{\sigma}{\rho}$ | $\text{kN}\cdot\text{m}/\text{kg}$ | Structural efficiency under tensile/yield loading |
| **Specific Stiffness** | $\text{SM} = \frac{E}{\rho}$ | $\text{GPa}/(\text{g}/\text{cm}^3)$ | Deflection & aeroelastic flutter resistance |
| **Thermal Diffusivity** | $\alpha = \frac{k}{\rho \cdot c_p}$ | $\text{mm}^2/\text{s}$ | Rate of temperature change propagation |

---

## 📦 Project Architecture

```text
MaterialSpace/
├── Cargo.toml
├── README.md
├── docs/
│   └── architecture.md
├── data/
│   ├── materials_normalized.json
│   ├── materials.json
│   └── sources.json
└── src/
    ├── main.rs         # Application entry point and window setup
    ├── app.rs          # Central state management and page routing
    ├── database.rs     # JSON loading, category indexing, and search
    ├── material.rs     # Data models and property provenance structures
    ├── physics.rs      # Transparent physics calculations and unit tests
    ├── metrics.rs      # Derived engineering metrics, scores, and rankings
    └── ui.rs           # egui immediate-mode GUI implementation
```

---

## 🛠️ Building and Running

### Prerequisites
* [Rust toolchain](https://rustup.rs/) (edition 2024 / stable 1.85+)
* Cargo

### Build and Run
```bash
# Check code compilation
cargo check

# Run all unit tests
cargo test

# Launch the desktop application
cargo run --release
```

---

## 🧪 Unit Tests

The test suite covers known physical cases and edge cases:
- `physics::tests::test_thermal_expansion_known_case`: Verifies $\Delta L = \alpha L_0 \Delta T$ for aluminum alloy.
- `physics::tests::test_heat_energy_known_case`: Verifies $Q = mc\Delta T$ calculation.
- `physics::tests::test_thermal_stress_known_case`: Verifies $\sigma = E\alpha\Delta T$ for structural steel.
- `metrics::tests::test_specific_strength_known_case`: Verifies strength-to-weight calculation.
- `metrics::tests::test_thermal_diffusivity_known_case`: Verifies $\alpha = k/(\rho c_p)$.
- `metrics::tests::test_aerospace_suitability_score_valid_and_bounds`: Verifies score is bounded within 0–100.
- `metrics::tests::test_aerospace_suitability_score_missing_data`: Ensures missing strength or density safely returns `None`.

---

## 🔮 Future Improvements

- **Temperature-Dependent Property Curves**: Plotting property variation as a function of temperature ($k(T)$, $c_p(T)$, $\sigma_y(T)$).
- **Ashby Charts**: 2D scatter plots mapping Specific Modulus vs. Specific Strength on logarithmic scales.
- **Export Capabilities**: Exporting custom comparison reports and calculation summaries to CSV and PDF.
- **Composite Laminate Calculations**: Classical Lamination Theory (CLT) for multi-ply composites.