"""
MaterialSpace centralized unit conversion and formatting module.

Internally, all physics calculations use standard SI units:
- Temperature: Kelvin (K)
- Length: meters (m)
- Area: square meters (m^2)
- Mass: kilograms (kg)
- Force: Newtons (N)
- Stress / Pressure / Modulus: Pascals (Pa)
- Energy / Heat: Joules (J)
- Power / Heat Rate: Watts (W)
- Density: kg/m^3
- Thermal Conductivity: W/(m*K)
- Specific Heat: J/(kg*K)
- Thermal Expansion (CTE): 1/K
- Emissivity: dimensionless (0 to 1)
"""

from __future__ import annotations

from typing import Union

# ============================================================
# TEMPERATURE CONVERSIONS (Base: Kelvin K)
# ============================================================


def temp_to_kelvin(value: float, unit: str) -> float:
    """Convert a temperature value from any supported unit to Kelvin."""
    u = unit.strip().upper()
    if u in ("K", "KELVIN"):
        return float(value)
    if u in ("C", "°C", "CELSIUS"):
        return float(value) + 273.15
    if u in ("F", "°F", "FAHRENHEIT"):
        return (float(value) - 32.0) * (5.0 / 9.0) + 273.15
    raise ValueError(f"Unknown temperature unit: {unit}")


def kelvin_to_unit(kelvin: float, unit: str) -> float:
    """Convert a temperature value from Kelvin to target unit."""
    u = unit.strip().upper()
    if u in ("K", "KELVIN"):
        return float(kelvin)
    if u in ("C", "°C", "CELSIUS"):
        return float(kelvin) - 273.15
    if u in ("F", "°F", "FAHRENHEIT"):
        return (float(kelvin) - 273.15) * 1.8 + 32.0
    raise ValueError(f"Unknown temperature unit: {unit}")


def convert_temperature(value: float, from_unit: str, to_unit: str) -> float:
    """Convert temperature between any supported units."""
    k = temp_to_kelvin(value, from_unit)
    return kelvin_to_unit(k, to_unit)


# ============================================================
# LENGTH CONVERSIONS (Base: meter m)
# ============================================================

_LENGTH_FACTORS = {
    "m": 1.0,
    "meter": 1.0,
    "meters": 1.0,
    "mm": 1e-3,
    "millimeter": 1e-3,
    "millimeters": 1e-3,
    "cm": 1e-2,
    "centimeter": 1e-2,
    "centimeters": 1e-2,
    "um": 1e-6,
    "µm": 1e-6,
    "micrometer": 1e-6,
    "nm": 1e-9,
    "in": 0.0254,
    "inch": 0.0254,
    "inches": 0.0254,
    "ft": 0.3048,
    "foot": 0.3048,
    "feet": 0.3048,
}


def convert_length(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _LENGTH_FACTORS or tu not in _LENGTH_FACTORS:
        raise ValueError(
            f"Unsupported length conversion from '{from_unit}' to '{to_unit}'"
        )
    m = value * _LENGTH_FACTORS[fu]
    return m / _LENGTH_FACTORS[tu]


# ============================================================
# AREA CONVERSIONS (Base: m^2)
# ============================================================

_AREA_FACTORS = {
    "m^2": 1.0,
    "m2": 1.0,
    "mm^2": 1e-6,
    "mm2": 1e-6,
    "cm^2": 1e-4,
    "cm2": 1e-4,
    "in^2": 0.00064516,
    "in2": 0.00064516,
    "ft^2": 0.092903,
    "ft2": 0.092903,
}


def convert_area(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _AREA_FACTORS or tu not in _AREA_FACTORS:
        raise ValueError(
            f"Unsupported area conversion from '{from_unit}' to '{to_unit}'"
        )
    m2 = value * _AREA_FACTORS[fu]
    return m2 / _AREA_FACTORS[tu]


# ============================================================
# MASS CONVERSIONS (Base: kg)
# ============================================================

_MASS_FACTORS = {
    "kg": 1.0,
    "kilogram": 1.0,
    "kilograms": 1.0,
    "g": 1e-3,
    "gram": 1e-3,
    "grams": 1e-3,
    "mg": 1e-6,
    "lb": 0.45359237,
    "lbs": 0.45359237,
    "pound": 0.45359237,
    "pounds": 0.45359237,
    "oz": 0.028349523,
    "ounce": 0.028349523,
}


def convert_mass(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _MASS_FACTORS or tu not in _MASS_FACTORS:
        raise ValueError(
            f"Unsupported mass conversion from '{from_unit}' to '{to_unit}'"
        )
    kg = value * _MASS_FACTORS[fu]
    return kg / _MASS_FACTORS[tu]


# ============================================================
# FORCE CONVERSIONS (Base: Newton N)
# ============================================================

_FORCE_FACTORS = {
    "n": 1.0,
    "newton": 1.0,
    "newtons": 1.0,
    "kn": 1e3,
    "kilonewton": 1e3,
    "mn": 1e6,
    "lbf": 4.4482216,
    "pound-force": 4.4482216,
}


def convert_force(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _FORCE_FACTORS or tu not in _FORCE_FACTORS:
        raise ValueError(
            f"Unsupported force conversion from '{from_unit}' to '{to_unit}'"
        )
    n = value * _FORCE_FACTORS[fu]
    return n / _FORCE_FACTORS[tu]


# ============================================================
# PRESSURE / STRESS / MODULUS CONVERSIONS (Base: Pascal Pa)
# ============================================================

_PRESSURE_FACTORS = {
    "pa": 1.0,
    "pascal": 1.0,
    "kpa": 1e3,
    "mpa": 1e6,
    "gpa": 1e9,
    "psi": 6894.757,
    "ksi": 6894757.0,
    "bar": 1e5,
    "atm": 101325.0,
    "n/m^2": 1.0,
    "n/m2": 1.0,
    "n/mm^2": 1e6,
    "n/mm2": 1e6,
}


def convert_pressure(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _PRESSURE_FACTORS or tu not in _PRESSURE_FACTORS:
        raise ValueError(
            f"Unsupported pressure conversion from '{from_unit}' to '{to_unit}'"
        )
    pa = value * _PRESSURE_FACTORS[fu]
    return pa / _PRESSURE_FACTORS[tu]


# ============================================================
# DENSITY CONVERSIONS (Base: kg/m^3)
# ============================================================

_DENSITY_FACTORS = {
    "kg/m^3": 1.0,
    "kg/m3": 1.0,
    "g/cm^3": 1000.0,
    "g/cm3": 1000.0,
    "g/ml": 1000.0,
    "lb/in^3": 27679.904,
    "lb/in3": 27679.904,
    "lb/ft^3": 16.018463,
    "lb/ft3": 16.018463,
}


def convert_density(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _DENSITY_FACTORS or tu not in _DENSITY_FACTORS:
        raise ValueError(
            f"Unsupported density conversion from '{from_unit}' to '{to_unit}'"
        )
    kg_m3 = value * _DENSITY_FACTORS[fu]
    return kg_m3 / _DENSITY_FACTORS[tu]


# ============================================================
# THERMAL CONDUCTIVITY CONVERSIONS (Base: W/(m*K))
# ============================================================

_THERMAL_COND_FACTORS = {
    "w/(m*k)": 1.0,
    "w/m*k": 1.0,
    "w/mk": 1.0,
    "w/(m·k)": 1.0,
    "btu/(hr*ft*degf)": 1.730735,
    "btu/(hr*ft*°f)": 1.730735,
    "cal/(s*cm*degc)": 418.68,
}


def convert_thermal_conductivity(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower().replace("·", "*")
    tu = to_unit.strip().lower().replace("·", "*")
    if fu not in _THERMAL_COND_FACTORS or tu not in _THERMAL_COND_FACTORS:
        raise ValueError(
            f"Unsupported thermal conductivity conversion from '{from_unit}' to '{to_unit}'"
        )
    si = value * _THERMAL_COND_FACTORS[fu]
    return si / _THERMAL_COND_FACTORS[tu]


# ============================================================
# SPECIFIC HEAT CONVERSIONS (Base: J/(kg*K))
# ============================================================

_SPECIFIC_HEAT_FACTORS = {
    "j/(kg*k)": 1.0,
    "j/kg*k": 1.0,
    "j/kgk": 1.0,
    "j/(kg·k)": 1.0,
    "kj/(kg*k)": 1000.0,
    "kj/kg*k": 1000.0,
    "kj/kgk": 1000.0,
    "cal/(g*degc)": 4186.8,
    "btu/(lb*degf)": 4186.8,
}


def convert_specific_heat(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower().replace("·", "*")
    tu = to_unit.strip().lower().replace("·", "*")
    if fu not in _SPECIFIC_HEAT_FACTORS or tu not in _SPECIFIC_HEAT_FACTORS:
        raise ValueError(
            f"Unsupported specific heat conversion from '{from_unit}' to '{to_unit}'"
        )
    si = value * _SPECIFIC_HEAT_FACTORS[fu]
    return si / _SPECIFIC_HEAT_FACTORS[tu]


# ============================================================
# THERMAL EXPANSION (CTE) CONVERSIONS (Base: 1/K)
# ============================================================

_CTE_FACTORS = {
    "1/k": 1.0,
    "1/c": 1.0,
    "1/°c": 1.0,
    "k^-1": 1.0,
    "um/(m*k)": 1e-6,
    "µm/(m*k)": 1e-6,
    "um/m*k": 1e-6,
    "um/mk": 1e-6,
    "1e-6/k": 1e-6,
    "ppm/k": 1e-6,
    "1e-6/c": 1e-6,
    "10^-6/k": 1e-6,
    "1/f": 1.8,
    "1/°f": 1.8,
    "1e-6/f": 1.8e-6,
    "µin/(in*°f)": 1.8e-6,
}


def convert_thermal_expansion(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower().replace("·", "*")
    tu = to_unit.strip().lower().replace("·", "*")
    if fu not in _CTE_FACTORS or tu not in _CTE_FACTORS:
        raise ValueError(
            f"Unsupported CTE conversion from '{from_unit}' to '{to_unit}'"
        )
    si = value * _CTE_FACTORS[fu]
    return si / _CTE_FACTORS[tu]


# ============================================================
# ENERGY CONVERSIONS (Base: Joule J)
# ============================================================

_ENERGY_FACTORS = {
    "j": 1.0,
    "joule": 1.0,
    "joules": 1.0,
    "kj": 1e3,
    "kilojoule": 1e3,
    "mj": 1e6,
    "megajoule": 1e6,
    "cal": 4.184,
    "kcal": 4184.0,
    "btu": 1055.06,
    "kwh": 3.6e6,
    "w*s": 1.0,
}


def convert_energy(value: float, from_unit: str, to_unit: str) -> float:
    fu = from_unit.strip().lower()
    tu = to_unit.strip().lower()
    if fu not in _ENERGY_FACTORS or tu not in _ENERGY_FACTORS:
        raise ValueError(
            f"Unsupported energy conversion from '{from_unit}' to '{to_unit}'"
        )
    j = value * _ENERGY_FACTORS[fu]
    return j / _ENERGY_FACTORS[tu]


# ============================================================
# VALUE FORMATTING HELPERS
# ============================================================


def format_property_value(
    prop_name: str,
    value: Union[float, int, None],
    display_unit: str | None = None,
    sig_figs: int = 4,
) -> str:
    """Format a property value nicely with scientific engineering conventions."""
    if value is None:
        return "Data unavailable"

    v = float(value)

    if prop_name == "density":
        # Usually kg/m^3 or g/cm^3
        if display_unit in ("g/cm^3", "g/cm3"):
            g = v / 1000.0
            return f"{g:.3f} g/cm³"
        return f"{v:,.0f} kg/m³" if v >= 100 else f"{v:.2f} kg/m³"

    if prop_name == "thermal_conductivity":
        return f"{v:.1f} W/(m·K)" if v >= 1 else f"{v:.3f} W/(m·K)"

    if prop_name == "specific_heat":
        return f"{v:,.0f} J/(kg·K)"

    if prop_name in ("youngs_modulus", "modulus"):
        gpa = v / 1e9
        return f"{gpa:.1f} GPa" if gpa >= 1 else f"{v / 1e6:.1f} MPa"

    if prop_name in ("yield_strength", "tensile_strength", "stress"):
        mpa = v / 1e6
        return f"{mpa:.1f} MPa" if mpa >= 1 else f"{v:.0f} Pa"

    if prop_name in ("thermal_expansion", "cte"):
        um = v * 1e6
        return f"{um:.2f} µm/(m·K)"

    if prop_name in ("melting_point", "boiling_point", "temperature"):
        c = v - 273.15
        return f"{c:.1f} °C ({v:.1f} K)"

    if prop_name == "emissivity":
        return f"{v:.3f}"

    if prop_name == "strain":
        return f"{v:.4e}" if abs(v) < 0.001 else f"{v:.4f}"

    if prop_name == "energy":
        if abs(v) >= 1e6:
            return f"{v / 1e6:.3f} MJ"
        if abs(v) >= 1e3:
            return f"{v / 1e3:.2f} kJ"
        return f"{v:.1f} J"

    if prop_name in ("power", "heat_rate"):
        if abs(v) >= 1e6:
            return f"{v / 1e6:.3f} MW"
        if abs(v) >= 1e3:
            return f"{v / 1e3:.2f} kW"
        return f"{v:.1f} W"

    # Default fallback
    if abs(v) >= 1e6 or (abs(v) < 1e-3 and v != 0):
        return f"{v:.{sig_figs}e}"
    return f"{v:.{sig_figs}g}"
