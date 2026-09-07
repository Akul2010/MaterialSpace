# MaterialSpace Architecture

MaterialSpace is an interactive materials-engineering sandbox desktop application built for high-school students, educational laboratories, and the Congressional App Challenge.

## Design Philosophy & Separation of Concerns

MaterialSpace maintains strict structural separation between **Data**, **Physics Models**, **User Interface**, and **Utilities**:

```text
MaterialSpace/
├── app/
│   ├── main.py                  # Desktop application entry point
│   ├── core/                    # Core dataclasses, units, and constants
│   │   ├── material.py          # Typed Material and MaterialProperty models
│   │   ├── units.py             # Centralized SI unit conversions and formatters
│   │   └── constants.py         # Category metadata & property descriptors
│   ├── models/                  # Pure deterministic physics calculation engines
│   │   ├── thermal_expansion.py # Linear thermal expansion & strain
│   │   ├── heat_conduction.py   # 1D steady-state conductive heat transfer
│   │   ├── stress_strain.py     # Hooke's law, elastic deformation & yield checks
│   │   ├── heat_capacity.py     # Sensible thermal heat storage
│   │   └── material_selection.py# Transparent multi-criteria decision engine
│   ├── data/                    # In-memory indexed database & source management
│   │   ├── database.py          # Fast search, category filtering & comparison
│   │   └── sources.py           # Provenance and citation catalog
│   └── ui/                      # Modern scientific desktop interface (Qt)
│       ├── qt.py                # Unified cross-platform Qt compatibility layer
│       ├── theme.py             # Clean dark-mode stylesheet & design system
│       ├── widgets.py           # Reusable property cards & Matplotlib canvas
│       ├── main_window.py       # Main window navigation and stacked views
│       ├── home_view.py         # Welcome dashboard and metrics summary
│       ├── material_explorer.py # Search, category chips, and live preview
│       ├── material_detail.py   # Full material specification & source display
│       ├── simulations.py       # Interactive 4-experiment physics workbench
│       ├── comparison.py        # Side-by-side 2–4 candidate matrix & charts
│       └── challenges.py        # Interactive engineering decision scenarios
├── data/
│   ├── materials.json           # Curated raw material dataset (55+ materials)
│   ├── materials_normalized.json# Normalized SI property database
│   └── sources.json             # Authoritative source citations (NASA, NIST, ASM)
├── scripts/
│   ├── build_db.py              # Compiles raw scraped records into database
│   ├── normalize_db.py          # Converts units into strict SI base units
│   └── validate_db.py           # Validates property bounds, types, and schema
├── tests/                       # Comprehensive pytest suite (34+ automated tests)
└── docs/                        # Complete technical and scientific documentation
```

## Physics API Isolation

Physics equations are completely isolated from UI handlers. No button handler or widget performs direct arithmetic or mathematical modeling. All calculations pass through deterministic, typed functions in `app/models/` returning immutable dataclasses.
