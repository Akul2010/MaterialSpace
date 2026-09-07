"""
Tests for centralized unit conversion and formatting.
"""

import pytest
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


def test_temperature_conversions():
    # Celsius to Kelvin
    assert abs(convert_temperature(0.0, "C", "K") - 273.15) < 1e-6
    assert abs(convert_temperature(100.0, "C", "K") - 373.15) < 1e-6
    assert abs(convert_temperature(25.0, "°C", "K") - 298.15) < 1e-6

    # Fahrenheit to Kelvin / Celsius
    assert abs(convert_temperature(32.0, "F", "C") - 0.0) < 1e-6
    assert abs(convert_temperature(212.0, "F", "C") - 100.0) < 1e-6

    # Kelvin to Celsius
    assert abs(convert_temperature(300.0, "K", "C") - 26.85) < 1e-6


def test_length_conversions():
    assert abs(convert_length(1000.0, "mm", "m") - 1.0) < 1e-9
    assert abs(convert_length(1.0, "m", "cm") - 100.0) < 1e-9
    assert abs(convert_length(1.0, "in", "mm") - 25.4) < 1e-9
    assert abs(convert_length(1.0, "ft", "m") - 0.3048) < 1e-9


def test_pressure_modulus_conversions():
    assert abs(convert_pressure(1.0, "GPa", "Pa") - 1e9) < 1e-3
    assert abs(convert_pressure(276.0, "MPa", "Pa") - 276e6) < 1e-3
    assert abs(convert_pressure(1.0, "MPa", "N/mm^2") - 1.0) < 1e-3


def test_density_conversions():
    assert abs(convert_density(2.7, "g/cm^3", "kg/m^3") - 2700.0) < 1e-6
    assert abs(convert_density(2700.0, "kg/m^3", "g/cm^3") - 2.7) < 1e-6


def test_thermal_conductivity_conversions():
    assert abs(convert_thermal_conductivity(167.0, "W/(m*K)", "W/(m·K)") - 167.0) < 1e-6


def test_specific_heat_conversions():
    assert abs(convert_specific_heat(0.896, "kJ/(kg*K)", "J/(kg*K)") - 896.0) < 1e-6


def test_thermal_expansion_conversions():
    assert abs(convert_thermal_expansion(23.6, "µm/(m*K)", "1/K") - 23.6e-6) < 1e-12
    assert abs(convert_thermal_expansion(23.6e-6, "1/K", "µm/(m*K)") - 23.6) < 1e-6


def test_energy_conversions():
    assert abs(convert_energy(1.0, "kJ", "J") - 1000.0) < 1e-6
    assert abs(convert_energy(1.0, "MJ", "kJ") - 1000.0) < 1e-6


def test_format_property_value():
    assert format_property_value("density", None) == "Data unavailable"
    assert "2,700 kg/m³" in format_property_value("density", 2700.0)
    assert "68.9 GPa" in format_property_value("youngs_modulus", 68.9e9)
    assert "276.0 MPa" in format_property_value("yield_strength", 276e6)
    assert "23.60 µm/(m·K)" in format_property_value("thermal_expansion", 23.6e-6)
