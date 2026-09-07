"""
Core package for MaterialSpace.
"""

from app.core.material import Material, MaterialProperty
from app.core.units import (
    convert_area,
    convert_density,
    convert_energy,
    convert_force,
    convert_length,
    convert_mass,
    convert_pressure,
    convert_specific_heat,
    convert_temperature,
    convert_thermal_conductivity,
    convert_thermal_expansion,
    format_property_value,
)

__all__ = [
    "Material",
    "MaterialProperty",
    "convert_area",
    "convert_density",
    "convert_energy",
    "convert_force",
    "convert_length",
    "convert_mass",
    "convert_pressure",
    "convert_specific_heat",
    "convert_temperature",
    "convert_thermal_conductivity",
    "convert_thermal_expansion",
    "format_property_value",
]
