"""
Physics models package for MaterialSpace.
"""

from app.models.heat_capacity import (
    HeatCapacityResult,
    calculate_heat_energy,
    generate_energy_curve,
)
from app.models.heat_conduction import (
    HeatConductionResult,
    calculate_conduction,
    generate_temperature_profile,
)
from app.models.material_selection import (
    BUILTIN_CHALLENGES,
    CandidateEvaluation,
    Criterion,
    EngineeringChallenge,
    evaluate_materials,
)
from app.models.stress_strain import (
    StressStrainResult,
    YieldStatus,
    calculate_stress_strain,
    generate_stress_strain_curve,
)
from app.models.thermal_expansion import (
    ThermalExpansionResult,
    calculate_expansion,
    generate_temperature_curve,
)

__all__ = [
    "BUILTIN_CHALLENGES",
    "CandidateEvaluation",
    "Criterion",
    "EngineeringChallenge",
    "HeatCapacityResult",
    "HeatConductionResult",
    "StressStrainResult",
    "ThermalExpansionResult",
    "YieldStatus",
    "calculate_conduction",
    "calculate_expansion",
    "calculate_heat_energy",
    "calculate_stress_strain",
    "evaluate_materials",
    "generate_energy_curve",
    "generate_stress_strain_curve",
    "generate_temperature_curve",
    "generate_temperature_profile",
]
