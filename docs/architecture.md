# How MaterialSpace is organized

This document describes how the main parts of MaterialSpace work together.

## Overview

The application has five main parts:

1. **Getting data:** `retrieval.rs` handles supported ways of importing material data and managing local cached records. The project is not intended to scrape websites.
2. **Storing materials:** `material.rs` defines the structure of a material and its properties. `database.rs` loads the prepared records and provides search functions.
3. **Preparing and checking data:** the Python scripts combine records, convert units, and check for missing or suspicious information. `physics.rs` also checks some calculation inputs and results.
4. **Calculations and comparisons:** `physics.rs` contains the equations; `metrics.rs` calculates useful values derived from material properties; `screening.rs` checks materials against requirements.
5. **Screens and exports:** `app.rs` stores the current application state, `ui.rs` draws the interface, and `export.rs` saves results as CSV files.

## Material records and sources

A material record stores details such as its name, category, composition, and available properties. A property includes its numeric value and unit, and may also include the original value and unit, test conditions, test method, uncertainty, notes, and a source ID.

The source ID connects a property to an entry in `data/sources.json`. This lets the app show where a value is said to come from. It is useful to keep this information alongside the number, since measurements can depend on the grade, temperature, sample preparation, and test method. Source information should still be checked before a value is used for important engineering work.

Some properties are optional. For example, if tensile strength is not available, the record stores no value for it instead of using zero. This prevents the app from treating missing information as a real measurement.

Tensile strength and flexural strength are stored separately. Tensile strength comes from pulling a sample, while flexural strength comes from bending one. The values should not be substituted for each other.

## Preparing the database

The Python scripts support three main tasks:

- `build_db.py` combines material records from the available input files.
- `normalize_db.py` converts supported units to consistent units so that values can be compared and used in formulas.
- `validate_db.py` checks the records for issues such as missing fields, duplicate IDs, and invalid values.

Unit conversion does not make two measurements automatically comparable. Test conditions and material grades still matter, and the app should not invent details that the source did not provide.

## Calculations

The physics module implements simplified models for common calculations, including:

- Linear thermal expansion: `change in length = expansion coefficient × original length × temperature change`
- Heat required for a temperature change: `Q = mass × specific heat × temperature change`
- Thermal stress under full restraint: `stress = Young's modulus × expansion coefficient × temperature change`
- Steady one-dimensional heat conduction and an approximate thermal diffusion time.

These formulas depend on assumptions. For example, the thermal-stress calculation assumes the material is fully prevented from expanding. The results are estimates and may not describe a real component in service.

The metrics module calculates values such as strength divided by density, stiffness divided by density, and thermal diffusivity. A combined aerospace score is also available. Its weights are chosen for this application, so it should be treated as a way to compare candidates—not as an official standard or a certification.

## Screening and rankings

Screening checks each material against the requirements selected by the user. Results distinguish materials that meet the entered requirements, materials that fail one or more requirements, and materials that cannot be assessed because information is missing.

Rankings sort materials by the chosen metric. Materials without the information needed for that metric should not receive an invented value; they are left out of that ranking.

## Application state and interface

`MaterialSpaceApp` in `app.rs` keeps track of the current screen, search text, selected materials, screening requirements, and calculator inputs. When the user changes screens or selects a material, this state is updated. Functions in `ui.rs` draw the appropriate screen using that state and call the calculation or database functions when needed.

This separation means the interface displays results, while the database and calculation modules handle the work behind those results.

## Local data and cache

The app includes a baseline data file so it can open without first fetching information from the internet. The retrieval code also supports a local cache for data imported through permitted methods. The embedded baseline is a fallback if a local file cannot be read or parsed.

## CSV export

The export module writes selected material details, screening results, and ranking results to CSV. CSV files can be opened in spreadsheet software or processed by scripts. Exported results should be interpreted with the same care as the underlying data: check units, source details, and assumptions before relying on them.
