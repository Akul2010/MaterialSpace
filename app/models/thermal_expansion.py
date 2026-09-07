"""
Thermal Expansion Physics Model.

Mathematical Formulation:
    ΔL = α · L₀ · ΔT
    L = L₀ · (1 + α · ΔT)

Where:
    ΔL = Change in length (m)
    α  = Coefficient of thermal expansion (1/K)
    L₀ = Initial length (m)
    ΔT = Temperature change (T_final - T_initial) (K)
    L  = Final length (m)
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Tuple
import numpy as np


@dataclass(frozen=True)
class ThermalExpansionResult:
    initial_length: float  # meters
    initial_temperature: float  # Kelvin
    final_temperature: float  # Kelvin
    delta_temperature: float  # Kelvin
    alpha: float  # 1/K
    length_change: float  # meters
    final_length: float  # meters
    strain: float  # dimensionless (ΔL / L₀)


def calculate_expansion(
    initial_length: float,
    alpha: float,
    t_initial: float,
    t_final: float,
) -> ThermalExpansionResult:
    """
    Calculate linear thermal expansion.
    All inputs must be in SI units (m, 1/K, K).
    """
    if initial_length <= 0:
        raise ValueError("Initial length must be positive.")
    if alpha < -1e-4 or alpha > 1e-3:
        raise ValueError(
            f"Coefficient of thermal expansion out of realistic physical bounds: {alpha}"
        )

    delta_t = t_final - t_initial
    delta_l = alpha * initial_length * delta_t
    final_l = initial_length + delta_l
    strain = delta_l / initial_length

    return ThermalExpansionResult(
        initial_length=initial_length,
        initial_temperature=t_initial,
        final_temperature=t_final,
        delta_temperature=delta_t,
        alpha=alpha,
        length_change=delta_l,
        final_length=final_l,
        strain=strain,
    )


def generate_temperature_curve(
    initial_length: float,
    alpha: float,
    t_start: float,
    t_end: float,
    points: int = 100,
) -> Tuple[np.ndarray, np.ndarray]:
    """
    Generate (temperatures_K, lengths_m) array for graphing.
    """
    temps = np.linspace(t_start, t_end, points)
    lengths = initial_length * (1.0 + alpha * (temps - t_start))
    return temps, lengths


MODEL_INFO = {
    "name": "Linear Thermal Expansion",
    "equation": "ΔL = α · L₀ · ΔT",
    "equation_latex": r"\Delta L = \alpha L_0 \Delta T",
    "variables": {
        "ΔL": "Change in length (meters, mm, µm)",
        "α": "Linear coefficient of thermal expansion (1/K or µm/(m·K))",
        "L₀": "Initial length at reference temperature (meters)",
        "ΔT": "Temperature variation T_final - T_initial (Kelvin or °C)",
        "L": "Final length L₀ + ΔL (meters)",
    },
    "assumptions": [
        "Isotropic material behavior: expansion is equal in all spatial directions (unless directional laminate).",
        "Constant coefficient of thermal expansion (α) over the specified temperature delta.",
        "Unconstrained thermal motion: zero external mechanical clamping or thermal stress induced by boundaries.",
        "No phase transitions (e.g., solid-liquid melting or solid-state allotropic phase changes) across the temperature interval.",
    ],
    "limitations": [
        "Becomes inaccurate across wide temperature swings where α varies strongly with temperature.",
        "Does not account for anisotropic thermal expansion in non-cubic crystals or unidirectional composites unless specified along principal axis.",
        "Does not predict thermal shock fracture or thermal buckling under compressive restraint.",
    ],
}
