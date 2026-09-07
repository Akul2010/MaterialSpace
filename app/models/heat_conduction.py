"""
One-Dimensional Steady-State Heat Conduction Physics Model (Fourier's Law).

Mathematical Formulation:
    Q̇ = k · A · (T_hot - T_cold) / L
    ∇T = (T_hot - T_cold) / L
    R_th = L / (k · A)
    Q̇ = ΔT / R_th

Where:
    Q̇     = Heat transfer rate (Watts, W = J/s)
    k      = Thermal conductivity (W/(m·K))
    A      = Cross-sectional heat flow area (m²)
    L      = Material wall thickness / conduction path length (m)
    T_hot  = Hot boundary temperature (K)
    T_cold = Cold boundary temperature (K)
    ∇T     = Thermal gradient (K/m)
    R_th   = Thermal resistance (K/W)
"""

from __future__ import annotations

from dataclasses import dataclass

import numpy as np


@dataclass(frozen=True)
class HeatConductionResult:
    thermal_conductivity: float  # W/(m*K)
    area: float  # m^2
    thickness: float  # m
    t_hot: float  # K
    t_cold: float  # K
    delta_temperature: float  # K
    heat_rate: float  # Watts (W)
    heat_flux: float  # W/m^2
    thermal_gradient: float  # K/m
    thermal_resistance: float  # K/W


def calculate_conduction(
    thermal_conductivity: float,
    area: float,
    thickness: float,
    t_hot: float,
    t_cold: float,
) -> HeatConductionResult:
    """
    Calculate 1D steady-state conductive heat transfer rate.
    All inputs must be in SI units (W/(m*K), m^2, m, K, K).
    """
    if thermal_conductivity <= 0:
        raise ValueError("Thermal conductivity must be positive.")
    if area <= 0:
        raise ValueError("Cross-sectional area must be positive.")
    if thickness <= 0:
        raise ValueError("Material thickness must be positive.")

    delta_t = t_hot - t_cold
    gradient = delta_t / thickness
    q_dot = thermal_conductivity * area * gradient
    flux = q_dot / area
    r_th = thickness / (thermal_conductivity * area)

    return HeatConductionResult(
        thermal_conductivity=thermal_conductivity,
        area=area,
        thickness=thickness,
        t_hot=t_hot,
        t_cold=t_cold,
        delta_temperature=delta_t,
        heat_rate=q_dot,
        heat_flux=flux,
        thermal_gradient=gradient,
        thermal_resistance=r_th,
    )


def generate_temperature_profile(
    t_hot: float,
    t_cold: float,
    thickness: float,
    points: int = 100,
) -> tuple[np.ndarray, np.ndarray]:
    """
    Generate (positions_x_m, temperatures_K) linear steady-state profile.
    """
    x = np.linspace(0, thickness, points)
    t = t_hot - (t_hot - t_cold) * (x / thickness)
    return x, t


MODEL_INFO = {
    "name": "1D Steady-State Heat Conduction",
    "equation": "Q̇ = k · A · (T_hot - T_cold) / L",
    "equation_latex": r"\dot{Q} = k A \frac{T_{hot} - T_{cold}}{L}",
    "variables": {
        "Q̇": "Heat transfer rate (Watts, W = J/s)",
        "k": "Thermal conductivity of the material (W/(m·K))",
        "A": "Cross-sectional surface area perpendicular to heat flow (m²)",
        "L": "Material wall thickness / conductive path length (m, mm)",
        "T_hot": "Temperature on hot boundary surface (K or °C)",
        "T_cold": "Temperature on cold boundary surface (K or °C)",
        "∇T": "Linear temperature gradient ΔT/L (K/m or °C/mm)",
        "R_th": "Conductive thermal resistance L/(k·A) (K/W)",
    },
    "assumptions": [
        "One-dimensional heat flow: lateral side edges are perfectly insulated with zero edge losses.",
        "Steady-state thermal equilibrium: temperatures at every coordinate do not change over time (∂T/∂t = 0).",
        "Uniform, isotropic thermal conductivity (k) independent of local temperature across the slab.",
        "Zero internal heat generation within the bulk material (e.g., no Joule heating, nuclear decay, or chemical reactions).",
        "Zero contact thermal resistance at boundary interfaces.",
    ],
    "limitations": [
        "Does not simulate transient / unsteady heat warm-up or cool-down curves.",
        "Does not include surface convection or radiative heat transfer from external boundaries.",
        "Does not capture multi-dimensional corner geometry effects or anisotropic composite layup conduction.",
    ],
}
