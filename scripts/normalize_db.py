from __future__ import annotations

import argparse
import json
from pathlib import Path

# ============================================================
# CONVERSION FUNCTIONS
# ============================================================


def convert_value(
    property_name: str,
    value: float,
    unit: str,
) -> tuple[float, str]:

    unit = unit.strip()

    # --------------------------------------------------------
    # Density
    # --------------------------------------------------------

    if property_name == "density":
        if unit in {
            "kg/m^3",
            "kg/m3",
        }:
            return value, "kg/m^3"

        if unit in {
            "g/cm^3",
            "g/cm3",
        }:
            return (
                value * 1000,
                "kg/m^3",
            )

    # --------------------------------------------------------
    # Thermal conductivity
    # --------------------------------------------------------

    if property_name == ("thermal_conductivity"):
        if unit in {
            "W/(m*K)",
            "W/m*K",
            "W/mK",
            "W/(m·K)",
        }:
            return (
                value,
                "W/(m*K)",
            )

    # --------------------------------------------------------
    # Specific heat
    # --------------------------------------------------------

    if property_name == "specific_heat":
        if unit in {
            "J/(kg*K)",
            "J/kg*K",
            "J/kgK",
        }:
            return (
                value,
                "J/(kg*K)",
            )

        if unit in {
            "kJ/(kg*K)",
            "kJ/kg*K",
            "kJ/kgK",
        }:
            return (
                value * 1000,
                "J/(kg*K)",
            )

    # --------------------------------------------------------
    # Young's modulus
    # --------------------------------------------------------

    if property_name == ("youngs_modulus"):
        if unit == "Pa":
            return value, "Pa"

        if unit == "MPa":
            return (
                value * 1_000_000,
                "Pa",
            )

        if unit == "GPa":
            return (
                value * 1_000_000_000,
                "Pa",
            )

    # --------------------------------------------------------
    # Yield strength
    # --------------------------------------------------------

    if property_name == ("yield_strength"):
        if unit == "Pa":
            return value, "Pa"

        if unit == "MPa":
            return (
                value * 1_000_000,
                "Pa",
            )

        if unit == "GPa":
            return (
                value * 1_000_000_000,
                "Pa",
            )

    # --------------------------------------------------------
    # Thermal expansion
    # --------------------------------------------------------

    if property_name == ("thermal_expansion"):
        if unit == "1/K":
            return (
                value,
                "1/K",
            )

        if unit in {
            "um/(m*K)",
            "µm/(m*K)",
            "um/mK",
        }:
            # µm/m = 10^-6
            return (
                value * 1e-6,
                "1/K",
            )

    # --------------------------------------------------------
    # Temperature
    # --------------------------------------------------------

    if property_name in {
        "melting_point",
        "boiling_point",
    }:
        if unit == "K":
            return value, "K"

        if unit in {
            "C",
            "°C",
        }:
            return (
                value + 273.15,
                "K",
            )

    # --------------------------------------------------------
    # Emissivity
    # --------------------------------------------------------

    if property_name == "emissivity":
        return value, "dimensionless"

    # --------------------------------------------------------
    # Unknown
    # --------------------------------------------------------

    raise ValueError(f"Unknown conversion: {property_name} / {unit}")


# ============================================================
# DATABASE
# ============================================================


def normalize_material(
    material: dict,
) -> dict:

    material = json.loads(json.dumps(material))

    properties = material.get(
        "properties",
        {},
    )

    for name, prop in properties.items():
        if "value" not in prop:
            continue

        unit = prop.get(
            "unit",
            "",
        )

        try:
            new_value, new_unit = convert_value(
                name,
                prop["value"],
                unit,
            )

            prop["original_value"] = prop["value"]

            prop["original_unit"] = unit

            prop["value"] = new_value

            prop["unit"] = new_unit

        except ValueError:
            # Unknown properties are
            # intentionally preserved.
            prop["normalization_warning"] = f"Could not normalize {unit}"

    return material


def normalize_database(
    input_path: Path,
    output_path: Path,
) -> None:

    with input_path.open(
        "r",
        encoding="utf-8",
    ) as file:
        database = json.load(file)

    normalized = [normalize_material(material) for material in database]

    output_path.parent.mkdir(
        parents=True,
        exist_ok=True,
    )

    with output_path.open(
        "w",
        encoding="utf-8",
    ) as file:
        json.dump(
            normalized,
            file,
            indent=2,
            ensure_ascii=False,
        )

        file.write("\n")


def main():

    parser = argparse.ArgumentParser()

    parser.add_argument(
        "input",
        help="Input JSON",
    )

    parser.add_argument(
        "output",
        help="Output JSON",
    )

    args = parser.parse_args()

    normalize_database(
        Path(args.input),
        Path(args.output),
    )

    print("[OK] Database normalized.")


if __name__ == "__main__":
    main()
