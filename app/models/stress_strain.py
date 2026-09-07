"""
Uniaxial Stress, Strain, and Elastic Deformation Physics Model (Hooke's Law).

Mathematical Formulation:
    σ = F / A
    ε = σ / E
    ΔL = ε · L₀ = (F · L₀) / (A · E)
    FoS (Factor of Safety) = σ_yield / σ

Where:
    σ       = Normal tensile / compressive engineering stress (Pascals, Pa = N/m²)
    F       = Applied axial tensile or compressive force (Newtons, N)
    A       = Cross-sectional load-bearing area (m²)
    E       = Young's Modulus of elasticity (Pa)
    ε       = Engineering strain (dimensionless, m/m)
    L₀      = Original specimen gauge length (m)
    ΔL      = Elastic elongation / compression deflection (m)
    σ_yield = Listed 0.2% offset yield strength (Pa)
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

import numpy as np


class YieldStatus(Enum):
    ELASTIC_SAFE = "Within Elastic Limit (Safe)"
    NEAR_YIELD = "Approaching Yield Threshold (>85% σ_y)"
    YIELDING_EXCEEDED = (
        "Applied Stress Exceeds Yield Strength (Plastic Deformation / Yielding)"
    )
    NO_YIELD_DATA = "Yield Strength Data Unavailable"


@dataclass(frozen=True)
class StressStrainResult:
    force: float  # N
    area: float  # m^2
    youngs_modulus: float  # Pa
    initial_length: float  # m
    yield_strength: float | None  # Pa
    stress: float  # Pa
    strain: float  # dimensionless
    deformation: float  # m
    final_length: float  # m
    yield_ratio: float | None  # stress / yield_strength
    factor_of_safety: float | None  # yield_strength / stress
    yield_status: YieldStatus
    status_message: str


def calculate_stress_strain(
    force: float,
    area: float,
    youngs_modulus: float,
    initial_length: float = 1.0,
    yield_strength: float | None = None,
) -> StressStrainResult:
    """
    Calculate uniaxial engineering stress, strain, deformation, and yield status.
    All inputs in SI units (N, m^2, Pa, m, Pa).
    """
    if area <= 0:
        raise ValueError("Cross-sectional area must be positive.")
    if youngs_modulus <= 0:
        raise ValueError("Young's modulus must be positive.")
    if initial_length <= 0:
        raise ValueError("Initial length must be positive.")

    stress = force / area
    strain = stress / youngs_modulus
    deformation = strain * initial_length
    final_l = initial_length + deformation

    yield_ratio: float | None = None
    fos: float | None = None
    status: YieldStatus
    message: str

    if yield_strength is not None and yield_strength > 0:
        yield_ratio = stress / yield_strength
        fos = (yield_strength / stress) if stress > 0 else float("inf")

        if stress > yield_strength:
            status = YieldStatus.YIELDING_EXCEEDED
            message = (
                f"Applied stress ({stress / 1e6:.1f} MPa) exceeds listed yield strength ({yield_strength / 1e6:.1f} MPa). "
                "The simplified Hookean model predicts plastic yielding will occur."
            )
        elif stress >= 0.85 * yield_strength:
            status = YieldStatus.NEAR_YIELD
            message = (
                f"Applied stress ({stress / 1e6:.1f} MPa) is near yield strength ({yield_strength / 1e6:.1f} MPa). "
                f"Factor of Safety is low (FoS = {fos:.2f})."
            )
        else:
            status = YieldStatus.ELASTIC_SAFE
            message = (
                f"Applied stress ({stress / 1e6:.1f} MPa) is comfortably below yield strength ({yield_strength / 1e6:.1f} MPa). "
                f"Factor of Safety = {fos:.2f}."
            )
    else:
        status = YieldStatus.NO_YIELD_DATA
        message = f"Applied stress is {stress / 1e6:.1f} MPa. Yield strength is not available in database for this material."

    return StressStrainResult(
        force=force,
        area=area,
        youngs_modulus=youngs_modulus,
        initial_length=initial_length,
        yield_strength=yield_strength,
        stress=stress,
        strain=strain,
        deformation=deformation,
        final_length=final_l,
        yield_ratio=yield_ratio,
        factor_of_safety=fos,
        yield_status=status,
        status_message=message,
    )


def generate_stress_strain_curve(
    youngs_modulus: float,
    yield_strength: float | None,
    max_strain: float = 0.02,
    points: int = 100,
) -> tuple[np.ndarray, np.ndarray]:
    """
    Generate (strain_array, stress_Pa_array) for visual graphing.
    """
    strains = np.linspace(0, max_strain, points)
    stresses = youngs_modulus * strains
    return strains, stresses


MODEL_INFO = {
    "name": "Uniaxial Stress, Strain, & Hooke's Law",
    "equation": "σ = F / A,   ε = σ / E,   ΔL = ε · L₀",
    "equation_latex": r"\sigma = \frac{F}{A}, \quad \varepsilon = \frac{\sigma}{E}, \quad \Delta L = \varepsilon L_0",
    "variables": {
        "σ": "Engineering normal stress (Pascals, MPa)",
        "F": "Applied axial tensile or compressive load (Newtons, kN)",
        "A": "Cross-sectional load-bearing area (m², mm²)",
        "E": "Young's modulus of elasticity / stiffness (Pa, GPa)",
        "ε": "Engineering strain ΔL/L₀ (dimensionless)",
        "L₀": "Initial gauge length (m, mm)",
        "ΔL": "Axial elastic deformation (elongation or compression) (m, mm)",
        "σ_yield": "0.2% offset yield strength threshold (Pa, MPa)",
    },
    "assumptions": [
        "Pure uniaxial loading without bending moments, torsion, or shear stresses.",
        "Uniform stress distribution across the entire cross-sectional area (Saint-Venant's principle).",
        "Linear elastic behavior obeying Hooke's Law (σ = E·ε).",
        "Small deformations where engineering stress/strain approximations remain valid.",
        "Zero buckling effects under compressive loading.",
    ],
    "limitations": [
        "Does not simulate post-yield non-linear plastic flow, strain hardening, necking, or ductile void coalescence.",
        "Does not account for geometric stress concentrations (fillets, holes, notches) or fatigue cycling.",
        "Does not model column buckling under compressive slender column geometries (Euler buckling).",
    ],
}
