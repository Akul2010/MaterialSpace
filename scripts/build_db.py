from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def load(
    path: Path,
) -> list[dict]:

    if not path.exists():
        return []

    with path.open(
        "r",
        encoding="utf-8",
    ) as file:
        data = json.load(file)

    if not isinstance(
        data,
        list,
    ):
        raise ValueError(f"{path} must contain a list")

    return data


def merge_material(
    existing: dict,
    incoming: dict,
) -> dict:

    # --------------------------------------------------------
    # Properties
    # --------------------------------------------------------

    existing_properties = existing.setdefault(
        "properties",
        {},
    )

    incoming_properties = incoming.get(
        "properties",
        {},
    )

    for name, prop in incoming_properties.items():
        # Existing data wins.
        #
        # This prevents a second source from
        # silently replacing a previously
        # reviewed value.
        if name not in existing_properties:
            existing_properties[name] = prop

    # --------------------------------------------------------
    # Sources
    # --------------------------------------------------------

    existing_sources = existing.setdefault(
        "source_ids",
        [],
    )

    for source in incoming.get(
        "source_ids",
        [],
    ):
        if source not in existing_sources:
            existing_sources.append(source)

    return existing


def build(
    raw_files: list[Path],
    output: Path,
) -> None:

    materials = {}

    for path in raw_files:
        for material in load(path):
            material_id = material.get("id")

            if not material_id:
                continue

            if material_id not in materials:
                materials[material_id] = material

            else:
                materials[material_id] = merge_material(
                    materials[material_id],
                    material,
                )

    result = sorted(
        materials.values(),
        key=lambda x: x.get(
            "name",
            "",
        ).lower(),
    )

    output.parent.mkdir(
        parents=True,
        exist_ok=True,
    )

    with output.open(
        "w",
        encoding="utf-8",
    ) as file:
        json.dump(
            result,
            file,
            indent=2,
            ensure_ascii=False,
        )

        file.write("\n")

    print(f"[OK] Built database with {len(result)} materials.")


def main():

    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--output",
        default=str(ROOT / "data" / "materials.json"),
    )

    args = parser.parse_args()

    raw_dir = ROOT / "data" / "raw"

    raw_files = [
        raw_dir / "nasa_tpsx.json",
        raw_dir / "nist.json",
        raw_dir / "matweb.json",
    ]

    build(
        raw_files,
        Path(args.output),
    )


if __name__ == "__main__":
    main()
