"""
Physics models package for MaterialSpace.
"""

from app.models.thermal_expansion import (
    calculate_expansion,
    generate_temperature_curve,
    ThermalExpansionResult,
)
from app.models.heat_conduction import (
    calculate_conduction,
    generate_temperature_profile,
    HeatConductionResult,
)
from app.models.stress_strain import (
    calculate_stress_strain,
    generate_stress_strain_curve,
    StressStrainResult,
    YieldStatus,
)
from app.models.heat_capacity import (
    calculate_heat_energy,
    generate_energy_curve,
    HeatCapacityResult,
)
from app.models.material_selection import (
    Criterion,
    CandidateEvaluation,
    EngineeringChallenge,
    evaluate_materials,
    BUILTIN_CHALLENGES,
)

__all__ = [
    "calculate_expansion",
    "generate_temperature_curve",
    "ThermalExpansionResult",
    "calculate_conduction",
    "generate_temperature_profile",
    "HeatConductionResult",
    "calculate_stress_strain",
    "generate_stress_strain_curve",
    "StressStrainResult",
    "YieldStatus",
    "calculate_heat_energy",
    "generate_energy_curve",
    "HeatCapacityResult",
    "Criterion",
    "CandidateEvaluation",
    "EngineeringChallenge",
    "evaluate_materials",
    "BUILTIN_CHALLENGES",
]
