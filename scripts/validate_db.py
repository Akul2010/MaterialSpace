from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]

REQUIRED_FIELDS = {
    "id",
    "name",
    "category",
    "properties",
}


PROPERTY_RULES = {
    "density": {
        "unit": {
            "kg/m^3",
            "kg/m3",
            "g/cm^3",
            "g/cm3",
        },
        "minimum": 0,
    },
    "thermal_conductivity": {
        "unit": {
            "W/(m*K)",
            "W/m*K",
            "W/mK",
            "W/(m·K)",
        },
        "minimum": 0,
    },
    "specific_heat": {
        "unit": {
            "J/(kg*K)",
            "J/kg*K",
            "J/kgK",
        },
        "minimum": 0,
    },
    "youngs_modulus": {
        "unit": {
            "Pa",
            "MPa",
            "GPa",
        },
        "minimum": 0,
    },
    "yield_strength": {
        "unit": {
            "Pa",
            "MPa",
            "GPa",
        },
        "minimum": 0,
    },
    "thermal_expansion": {
        "unit": {
            "1/K",
            "1/K",
            "um/(m*K)",
            "µm/(m*K)",
            "um/mK",
        },
    },
    "emissivity": {
        "unit": {
            "",
            "dimensionless",
        },
        "minimum": 0,
        "maximum": 1,
    },
    "melting_point": {
        "unit": {
            "K",
            "C",
            "°C",
        },
        "minimum": 0,
    },
    "boiling_point": {
        "unit": {
            "K",
            "C",
            "°C",
        },
        "minimum": 0,
    },
}


def load(
    path: Path,
):

    with path.open(
        "r",
        encoding="utf-8",
    ) as file:
        return json.load(file)


def validate_material(
    material: dict,
    index: int,
) -> list[str]:

    errors = []

    # --------------------------------------------------------
    # Required fields
    # --------------------------------------------------------

    missing = REQUIRED_FIELDS - material.keys()

    for field in sorted(missing):
        errors.append(f"Material {index}: missing field '{field}'")

    if errors:
        return errors

    # --------------------------------------------------------
    # Basic fields
    # --------------------------------------------------------

    if (
        not isinstance(
            material["id"],
            str,
        )
        or not material["id"]
    ):
        errors.append(f"Material {index}: invalid id")

    if (
        not isinstance(
            material["name"],
            str,
        )
        or not material["name"]
    ):
        errors.append(f"Material {index}: invalid name")

    if not isinstance(
        material["properties"],
        dict,
    ):
        errors.append(f"Material {index}: 'properties' must be an object")

        return errors

    # --------------------------------------------------------
    # Properties
    # --------------------------------------------------------

    for name, prop in material["properties"].items():
        if not isinstance(
            prop,
            dict,
        ):
            errors.append(f"{material['id']}: property '{name}' is not an object")

            continue

        if "value" not in prop:
            errors.append(f"{material['id']}: property '{name}' has no value")

            continue

        if "unit" not in prop:
            errors.append(f"{material['id']}: property '{name}' has no unit")

        if "source_id" not in prop:
            errors.append(f"{material['id']}: property '{name}' has no source_id")

        value = prop["value"]

        if not isinstance(
            value,
            (int, float),
        ):
            errors.append(f"{material['id']}: property '{name}' value is not numeric")

            continue

        rules = PROPERTY_RULES.get(name)

        if rules is None:
            # Unknown properties are allowed.
            continue

        minimum = rules.get("minimum")

        maximum = rules.get("maximum")

        if minimum is not None and value < minimum:
            errors.append(
                f"{material['id']}: {name}={value} is below minimum {minimum}"
            )

        if maximum is not None and value > maximum:
            errors.append(
                f"{material['id']}: {name}={value} is above maximum {maximum}"
            )

        allowed_units = rules.get("unit")

        if allowed_units and prop["unit"] not in allowed_units:
            errors.append(
                f"{material['id']}: {name} uses unrecognized unit '{prop['unit']}'"
            )

    return errors


def validate_database(
    path: Path,
) -> tuple[list[str], int]:

    data = load(path)

    if not isinstance(
        data,
        list,
    ):
        return ["Database root must be a list."], 0

    errors = []

    ids = set()

    for index, material in enumerate(
        data,
        start=1,
    ):
        material_errors = validate_material(
            material,
            index,
        )

        errors.extend(material_errors)

        material_id = material.get("id")

        if material_id:
            if material_id in ids:
                errors.append(f"Duplicate material id: {material_id}")

            ids.add(material_id)

    return errors, len(data)


def main():

    parser = argparse.ArgumentParser()

    parser.add_argument(
        "database",
        help="JSON database to validate",
    )

    args = parser.parse_args()

    path = Path(args.database)

    errors, count = validate_database(path)

    print(f"Checked {count} materials.")

    if not errors:
        print("[OK] Database passed validation.")

        return 0

    print(f"Found {len(errors)} validation errors:")

    for error in errors:
        print(f"  - {error}")

    return 1


if __name__ == "__main__":
    raise SystemExit(main())
