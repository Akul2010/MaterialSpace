"""
MaterialSpace core material data model.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any


@dataclass
class MaterialProperty:
    """
    Represents an individual physical property with SI value, unit, and source citation.
    """

    value: float
    unit: str
    source_id: str
    uncertainty: float | None = None
    temperature: float | None = None  # Reference temperature in Kelvin
    temperature_unit: str | None = None
    condition: str | None = None  # e.g., "T6 temper", "annealed", "room temp"
    original_value: float | str | None = None
    original_unit: str | None = None
    notes: str | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> MaterialProperty:
        return cls(
            value=float(data["value"]),
            unit=str(data.get("unit", "")),
            source_id=str(data.get("source_id", "unknown")),
            uncertainty=data.get("uncertainty"),
            temperature=data.get("temperature"),
            temperature_unit=data.get("temperature_unit"),
            condition=data.get("condition"),
            original_value=data.get("original_value"),
            original_unit=data.get("original_unit"),
            notes=data.get("notes"),
        )

    def to_dict(self) -> dict[str, Any]:
        res: dict[str, Any] = {
            "value": self.value,
            "unit": self.unit,
            "source_id": self.source_id,
        }
        if self.uncertainty is not None:
            res["uncertainty"] = self.uncertainty
        if self.temperature is not None:
            res["temperature"] = self.temperature
        if self.temperature_unit is not None:
            res["temperature_unit"] = self.temperature_unit
        if self.condition:
            res["condition"] = self.condition
        if self.original_value is not None:
            res["original_value"] = self.original_value
        if self.original_unit is not None:
            res["original_unit"] = self.original_unit
        if self.notes:
            res["notes"] = self.notes
        return res


@dataclass
class Material:
    """
    Represents an engineering material with properties and metadata.
    """

    id: str
    name: str
    category: str
    composition: str | None = None
    grade: str | None = None
    properties: dict[str, MaterialProperty] = field(default_factory=dict)
    source_ids: list[str] = field(default_factory=list)
    notes: str | None = None
    description: str | None = None

    def get_property(self, name: str) -> MaterialProperty | None:
        """Retrieve a MaterialProperty by key, or None if unavailable."""
        return self.properties.get(name)

    def get_value(self, name: str) -> float | None:
        """Retrieve the float value of a property, or None if unavailable."""
        prop = self.get_property(name)
        return prop.value if prop is not None else None

    def has_property(self, name: str) -> bool:
        """Check if property exists and has a valid numeric value."""
        return name in self.properties and self.properties[name].value is not None

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> Material:
        props_data = data.get("properties", {})
        props: dict[str, MaterialProperty] = {}
        source_ids = list(data.get("source_ids", []))

        for prop_name, prop_dict in props_data.items():
            if isinstance(prop_dict, dict) and "value" in prop_dict:
                try:
                    mp = MaterialProperty.from_dict(prop_dict)
                    props[prop_name] = mp
                    if mp.source_id and mp.source_id not in source_ids:
                        source_ids.append(mp.source_id)
                except (ValueError, TypeError):
                    continue

        return cls(
            id=str(data["id"]),
            name=str(data["name"]),
            category=str(data.get("category", "other")),
            composition=data.get("composition"),
            grade=data.get("grade"),
            properties=props,
            source_ids=source_ids,
            notes=data.get("notes"),
            description=data.get("description"),
        )

    def to_dict(self) -> dict[str, Any]:
        res: dict[str, Any] = {
            "id": self.id,
            "name": self.name,
            "category": self.category,
            "properties": {k: v.to_dict() for k, v in self.properties.items()},
            "source_ids": self.source_ids,
        }
        if self.composition:
            res["composition"] = self.composition
        if self.grade:
            res["grade"] = self.grade
        if self.notes:
            res["notes"] = self.notes
        if self.description:
            res["description"] = self.description
        return res
