"""
Heat Capacity and Sensible Thermal Energy Storage Physics Model.

Mathematical Formulation:
    Q = m · c · ΔT
    ΔT = Q / (m · c)

Where:
    Q  = Sensible thermal energy added or removed (Joules, J)
    m  = Mass of the material component (kg)
    c  = Specific heat capacity (J/(kg·K))
    ΔT = Temperature rise / drop (T_final - T_initial) (K or °C)
"""

from __future__ import annotations

from dataclasses import dataclass

import numpy as np


@dataclass(frozen=True)
class HeatCapacityResult:
    mass: float  # kg
    specific_heat: float  # J/(kg*K)
    initial_temperature: float  # K
    final_temperature: float  # K
    delta_temperature: float  # K
    energy: float  # Joules (J)
    thermal_mass: float  # J/K (m * c)


def calculate_heat_energy(
    mass: float,
    specific_heat: float,
    t_initial: float,
    t_final: float,
) -> HeatCapacityResult:
    """
    Calculate sensible thermal energy required to change temperature.
    All inputs in SI units (kg, J/(kg*K), K, K).
    """
    if mass <= 0:
        raise ValueError("Mass must be positive.")
    if specific_heat <= 0:
        raise ValueError("Specific heat capacity must be positive.")

    delta_t = t_final - t_initial
    q = mass * specific_heat * delta_t
    thermal_mass = mass * specific_heat

    return HeatCapacityResult(
        mass=mass,
        specific_heat=specific_heat,
        initial_temperature=t_initial,
        final_temperature=t_final,
        delta_temperature=delta_t,
        energy=q,
        thermal_mass=thermal_mass,
    )


def generate_energy_curve(
    mass: float,
    specific_heat: float,
    t_start: float,
    t_end: float,
    points: int = 100,
) -> tuple[np.ndarray, np.ndarray]:
    """
    Generate (temperatures_K, energy_Joules) curve.
    """
    temps = np.linspace(t_start, t_end, points)
    energies = mass * specific_heat * (temps - t_start)
    return temps, energies


MODEL_INFO = {
    "name": "Sensible Heat Capacity Storage",
    "equation": "Q = m · c · ΔT",
    "equation_latex": r"Q = m c \Delta T",
    "variables": {
        "Q": "Thermal energy absorbed or released (Joules, kJ, MJ)",
        "m": "Mass of the material body (kg, grams)",
        "c": "Specific heat capacity at constant pressure c_p (J/(kg·K))",
        "ΔT": "Temperature change T_final - T_initial (Kelvin or °C)",
        "C_th": "Total thermal mass / heat capacity m · c (J/K)",
    },
    "assumptions": [
        "Sensible heating only: zero latent heat of phase changes (melting, vaporization, subliming, solid phase recrystallization).",
        "Constant specific heat capacity (c) over the temperature delta.",
        "Uniform bulk body temperature: zero internal thermal gradients (low Biot number approximation, Bi < 0.1).",
        "Constant mass system with zero chemical reaction enthalpies.",
    ],
    "limitations": [
        "Becomes inaccurate near phase transformation boundaries where latent heat dominates.",
        "Does not account for cryogenic Debye T³ temperature dependence of specific heat.",
        "Does not calculate heat dissipation or heat loss to external ambient environment over time.",
    ],
}
