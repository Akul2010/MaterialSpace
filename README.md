# MaterialSpace

MaterialSpace is a desktop app for exploring engineering materials and comparing their properties. It includes a searchable materials database, side-by-side comparisons, filters for finding materials that meet requirements, rankings, and calculators for common heat and temperature-related problems.

The project is written mainly in Rust, with Python scripts for preparing and checking the data. It uses `eframe` and `egui` for the desktop interface.

## What it can do

- Search materials by name and other details, and filter by category.
- View a material's recorded properties, units, test conditions, and data source when that information is available.
- Compare two materials using the same set of properties.
- Filter materials against requirements, such as minimum strength or maximum density.
- Rank materials using selected properties or a combined score.
- Calculate thermal expansion, heat needed to change a material's temperature, thermal stress, heat conduction, and approximate thermal diffusion time.
- Export tables and results to CSV for use in a spreadsheet or another program.

## Important notes about the data

Material properties depend on the exact grade, how a sample was made, its temperature, and how it was tested. Two values with similar names are not always directly comparable. MaterialSpace shows units and source details where available, but the database may not contain every detail for every entry.

A missing value is shown as **Data unavailable**. It is not treated as zero. This matters because zero would be a real measurement and could lead to incorrect calculations or rankings.

The app keeps tensile strength and flexural strength separate. Tensile strength describes a material being pulled; flexural strength describes a test where a sample is bent. These values are not interchangeable, particularly for ceramics and glass.

The combined aerospace score is an exploratory comparison tool built from selected material properties and weights. It is not an official aerospace rating, a certification, or proof that a material is safe for a particular design. Real material selection also depends on factors the app may not cover, including fatigue, manufacturing, damage tolerance, environment, and applicable engineering standards.

Melting point is not the same as the highest temperature a material can safely withstand during use. The calculators also use simplified models and assumptions; their results should be treated as estimates, not as a replacement for engineering review.

## Data and scripts

The `data/` directory contains the material records, normalized records, and information about the listed sources. The Python scripts help combine records, convert units, and check the database for common problems. The Rust application reads the prepared data and uses it for searching, comparisons, calculations, and rankings.

The project is designed not to scrape websites. External data should only be added through permitted downloads, official data access methods, or authorized exports. A source being listed in `sources.json` does not automatically mean every value in the database has been independently verified; check the record and its source details before relying on a value.

## Project structure

```text
MaterialSpace/
├── data/        Material records and source information
├── docs/        Notes on how the application is organized
├── scripts/     Python tools for building, normalizing, and checking data
└── src/         Rust application, calculations, database, and interface
```

Main Rust files:

- `main.rs` starts the desktop application.
- `app.rs` stores the current page, selections, and calculator inputs.
- `material.rs` defines the material and property data structures.
- `database.rs` loads and searches the material records.
- `physics.rs` contains the physics calculations and checks for questionable inputs.
- `metrics.rs` calculates derived properties and rankings.
- `screening.rs` checks materials against selected requirements.
- `retrieval.rs` handles permitted data imports and the local cache.
- `export.rs` writes results to CSV.
- `ui.rs` builds the screens and controls.

## Running the application

Install Rust and Cargo, then run the project from its directory:

```bash
cargo run
```

Run the automated tests with:

```bash
cargo test
```

Python scripts may require Python and the packages listed by the project. Check each script's arguments before running it; preparing data is separate from launching the desktop app.

## Sources and limitations

The project includes source records for NASA TPSX, the NIST Chemistry WebBook, MatWeb, and ASM reference material. These sources cover different kinds of materials and properties, so they should not be treated as interchangeable or equally suitable for every material. Review the source and test conditions for a value before using it in a serious engineering decision.
