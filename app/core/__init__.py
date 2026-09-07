"""
Core package for MaterialSpace.
"""

from app.core.material import Material, MaterialProperty
from app.core.units import (
    convert_temperature,
    convert_length,
    convert_area,
    convert_mass,
    convert_force,
    convert_pressure,
    convert_density,
    convert_thermal_conductivity,
    convert_specific_heat,
    convert_thermal_expansion,
    convert_energy,
    format_property_value,
)

__all__ = [
    "Material",
    "MaterialProperty",
    "convert_temperature",
    "convert_length",
    "convert_area",
    "convert_mass",
    "convert_force",
    "convert_pressure",
    "convert_density",
    "convert_thermal_conductivity",
    "convert_specific_heat",
    "convert_thermal_expansion",
    "convert_energy",
    "format_property_value",
]
