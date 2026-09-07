from __future__ import annotations

import json
import logging
import re
from dataclasses import dataclass, field
from datetime import date
from pathlib import Path
from typing import Any

logger = logging.getLogger(__name__)


# ============================================================
# DATA CLASSES
# ============================================================


@dataclass
class SourceReference:
    """
    Information about a data source.
    """

    source_id: str
    organization: str
    title: str
    url: str
    accessed: str = field(default_factory=lambda: date.today().isoformat())
    notes: str | None = None

    def to_dict(self) -> dict:
        result = {
            "source_id": self.source_id,
            "organization": self.organization,
            "title": self.title,
            "url": self.url,
            "accessed": self.accessed,
        }

        if self.notes:
            result["notes"] = self.notes

        return result


@dataclass
class PropertyValue:
    """
    Represents one material property.

    Example:

        density:
            value = 2700
            unit = kg/m^3
            source_id = nasa_tpsx
    """

    value: float
    unit: str
    source_id: str

    uncertainty: float | None = None

    minimum: float | None = None
    maximum: float | None = None

    temperature: float | None = None
    temperature_unit: str | None = None

    condition: str | None = None

    original_value: str | None = None

    notes: str | None = None

    def to_dict(self) -> dict:

        result = {
            "value": self.value,
            "unit": self.unit,
            "source_id": self.source_id,
        }

        if self.uncertainty is not None:
            result["uncertainty"] = self.uncertainty

        if self.minimum is not None:
            result["minimum"] = self.minimum

        if self.maximum is not None:
            result["maximum"] = self.maximum

        if self.temperature is not None:
            result["temperature"] = self.temperature

        if self.temperature_unit is not None:
            result["temperature_unit"] = self.temperature_unit

        if self.condition:
            result["condition"] = self.condition

        if self.original_value:
            result["original_value"] = self.original_value

        if self.notes:
            result["notes"] = self.notes

        return result


@dataclass
class MaterialRecord:
    """
    Standard MaterialSpace representation of a material.
    """

    material_id: str
    name: str
    category: str

    composition: str | None = None
    grade: str | None = None

    properties: dict[str, PropertyValue] = field(default_factory=dict)

    source_ids: list[str] = field(default_factory=list)

    source_material_id: str | None = None
    source_url: str | None = None

    notes: str | None = None

    def add_property(
        self,
        name: str,
        prop: PropertyValue,
    ) -> None:

        self.properties[name] = prop

        if prop.source_id not in self.source_ids:
            self.source_ids.append(prop.source_id)

    def to_dict(self) -> dict:

        result = {
            "id": self.material_id,
            "name": self.name,
            "category": self.category,
            "properties": {
                name: prop.to_dict() for name, prop in self.properties.items()
            },
            "source_ids": self.source_ids,
        }

        if self.composition:
            result["composition"] = self.composition

        if self.grade:
            result["grade"] = self.grade

        if self.source_material_id:
            result["source_material_id"] = self.source_material_id

        if self.source_url:
            result["source_url"] = self.source_url

        if self.notes:
            result["notes"] = self.notes

        return result


# ============================================================
# TEXT / PARSING UTILITIES
# ============================================================


def slugify(value: str) -> str:
    """
    Converts:

        Aluminum 6061-T6

    into:

        aluminum_6061_t6
    """

    value = value.lower().strip()

    value = re.sub(
        r"[^a-z0-9]+",
        "_",
        value,
    )

    return value.strip("_")


def clean_text(value: Any) -> str:
    """
    Safely normalize scraped text.
    """

    if value is None:
        return ""

    return " ".join(str(value).split()).strip()


def parse_float(
    value: Any,
) -> float | None:
    """
    Parse a simple numeric value.

    Handles:

        2700
        "2700"
        "2,700"
        "2700 kg/m3"
        "2.7e3"
    """

    if value is None:
        return None

    if isinstance(value, (int, float)):
        return float(value)

    text = clean_text(value)

    match = re.search(
        r"[-+]?\d+(?:,\d{3})*(?:\.\d+)?(?:[eE][-+]?\d+)?",
        text,
    )

    if not match:
        return None

    try:
        return float(match.group(0).replace(",", ""))
    except ValueError:
        return None


def normalize_property_name(
    value: str,
) -> str | None:

    value = clean_text(value).lower()

    value = value.replace(
        "’",
        "'",
    )

    aliases = {
        # Density
        "density": "density",
        # Thermal
        "thermal conductivity": "thermal_conductivity",
        "thermal conductivity (w/m-k)": "thermal_conductivity",
        "thermal conductivity w/m-k": "thermal_conductivity",
        "specific heat": "specific_heat",
        "specific heat capacity": "specific_heat",
        "heat capacity": "specific_heat",
        "coefficient of thermal expansion": "thermal_expansion",
        "thermal expansion coefficient": "thermal_expansion",
        "cte": "thermal_expansion",
        # Mechanical
        "young's modulus": "youngs_modulus",
        "youngs modulus": "youngs_modulus",
        "modulus of elasticity": "youngs_modulus",
        "yield strength": "yield_strength",
        "tensile strength": "tensile_strength",
        # Other
        "emissivity": "emissivity",
        "melting point": "melting_point",
        "boiling point": "boiling_point",
    }

    return aliases.get(value)


# ============================================================
# JSON UTILITIES
# ============================================================


def load_json(
    path: str | Path,
) -> Any:

    path = Path(path)

    with path.open(
        "r",
        encoding="utf-8",
    ) as file:
        return json.load(file)


def save_json(
    data: Any,
    path: str | Path,
) -> None:

    path = Path(path)

    path.parent.mkdir(
        parents=True,
        exist_ok=True,
    )

    with path.open(
        "w",
        encoding="utf-8",
    ) as file:
        json.dump(
            data,
            file,
            indent=2,
            ensure_ascii=False,
        )

        file.write("\n")


def configure_logging() -> None:

    logging.basicConfig(
        level=logging.INFO,
        format="%(levelname)s: %(message)s",
    )
