"""
MaterialSpace core constants and definitions.
"""

CATEGORIES = [
    {"id": "metal", "name": "Metals & Alloys", "icon": "🔩"},
    {"id": "ceramic", "name": "Ceramics", "icon": "🏺"},
    {"id": "polymer", "name": "Polymers & Plastics", "icon": "🧪"},
    {"id": "composite", "name": "Composites", "icon": "🛡️"},
    {"id": "glass", "name": "Glasses & Glass-Ceramics", "icon": "🔍"},
    {"id": "pure_substance", "name": "Pure Elements & Carbon", "icon": "💎"},
]

PROPERTY_METADATA = {
    "density": {
        "label": "Density",
        "symbol": "ρ",
        "si_unit": "kg/m^3",
        "display_unit": "kg/m³",
        "description": "Mass per unit volume of the material.",
    },
    "thermal_conductivity": {
        "label": "Thermal Conductivity",
        "symbol": "k",
        "si_unit": "W/(m*K)",
        "display_unit": "W/(m·K)",
        "description": "Rate at which heat passes through a material per unit area per unit temperature gradient.",
    },
    "specific_heat": {
        "label": "Specific Heat Capacity",
        "symbol": "c",
        "si_unit": "J/(kg*K)",
        "display_unit": "J/(kg·K)",
        "description": "Energy required to raise 1 kg of material by 1 Kelvin.",
    },
    "thermal_expansion": {
        "label": "Coefficient of Thermal Expansion (CTE)",
        "symbol": "α",
        "si_unit": "1/K",
        "display_unit": "µm/(m·K)",
        "description": "Fractional change in size per degree change in temperature.",
    },
    "youngs_modulus": {
        "label": "Young's Modulus (Stiffness)",
        "symbol": "E",
        "si_unit": "Pa",
        "display_unit": "GPa",
        "description": "Measure of material elasticity and resistance to linear deformation under load.",
    },
    "yield_strength": {
        "label": "Yield Strength",
        "symbol": "σ_y",
        "si_unit": "Pa",
        "display_unit": "MPa",
        "description": "Stress at which a material begins to deform plastically (permanently).",
    },
    "tensile_strength": {
        "label": "Ultimate Tensile Strength",
        "symbol": "σ_uts",
        "si_unit": "Pa",
        "display_unit": "MPa",
        "description": "Maximum stress a material can withstand before necking and fracture.",
    },
    "emissivity": {
        "label": "Surface Emissivity",
        "symbol": "ε",
        "si_unit": "dimensionless",
        "display_unit": "dimensionless",
        "description": "Ratio of thermal radiation emitted by surface to radiation from a black body (0 to 1).",
    },
    "melting_point": {
        "label": "Melting Point",
        "symbol": "T_m",
        "si_unit": "K",
        "display_unit": "°C (K)",
        "description": "Temperature at which solid and liquid phases coexist at standard pressure.",
    },
    "boiling_point": {
        "label": "Boiling Point",
        "symbol": "T_b",
        "si_unit": "K",
        "display_unit": "°C (K)",
        "description": "Temperature at which liquid transitions to vapor at standard pressure.",
    },
}
