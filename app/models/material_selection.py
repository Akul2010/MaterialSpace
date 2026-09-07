"""
Transparent Multi-Criteria Material Selection Decision Engine.

Uses transparent linear normalization (0–100) and user-configurable weighting
to evaluate candidate materials for specific engineering scenarios.

No black-box machine learning. 100% auditable and explainable.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Dict, List, Optional
from app.core.material import Material


@dataclass
class Criterion:
    """
    Defines a physical property evaluation criterion for material selection.
    """

    property_name: str
    label: str
    weight: float  # Relative weight (0.0 to 1.0)
    higher_is_better: bool = True  # True for strength/modulus/conductivity, False for density/thermal_expansion
    min_threshold: Optional[float] = None  # Hard minimum constraint
    max_threshold: Optional[float] = None  # Hard maximum constraint
    description: str = ""


@dataclass
class PropertyScoreBreakdown:
    property_name: str
    label: str
    raw_value: Optional[float]
    unit: str
    normalized_score: float  # 0.0 to 100.0
    weight: float  # Fraction (e.g. 0.30)
    weighted_contribution: float  # normalized_score * weight
    higher_is_better: bool
    is_available: bool
    status_note: str


@dataclass
class CandidateEvaluation:
    material: Material
    overall_score: float  # 0.0 to 100.0
    rank: int = 0
    passed_hard_constraints: bool = True
    failure_reasons: List[str] = field(default_factory=list)
    breakdowns: Dict[str, PropertyScoreBreakdown] = field(default_factory=dict)
    summary_explanation: str = ""


@dataclass
class EngineeringChallenge:
    id: str
    title: str
    description: str
    engineering_context: str
    default_criteria: List[Criterion]
    recommended_candidate_ids: List[str]
    notes_and_tradeoffs: str


def evaluate_materials(
    materials: List[Material],
    criteria: List[Criterion],
) -> List[CandidateEvaluation]:
    """
    Evaluate and rank a list of candidate materials against weighted criteria.
    """
    if not materials:
        return []
    if not criteria:
        raise ValueError("At least one evaluation criterion is required.")

    # Normalize weights so sum = 1.0
    total_weight = sum(c.weight for c in criteria)
    norm_weights = [
        c.weight / total_weight if total_weight > 0 else 1.0 / len(criteria)
        for c in criteria
    ]

    # Find min and max for each criterion across available values in the pool
    prop_ranges: Dict[str, tuple[float, float]] = {}
    for crit in criteria:
        vals = [
            m.get_value(crit.property_name)
            for m in materials
            if m.get_value(crit.property_name) is not None
        ]
        if vals:
            min_v = min(vals)
            max_v = max(vals)
            prop_ranges[crit.property_name] = (min_v, max_v)
        else:
            prop_ranges[crit.property_name] = (0.0, 1.0)

    evaluations: List[CandidateEvaluation] = []

    for mat in materials:
        breakdowns: Dict[str, PropertyScoreBreakdown] = {}
        passed_constraints = True
        failures: List[str] = []
        overall = 0.0

        for crit, w in zip(criteria, norm_weights):
            prop = mat.get_property(crit.property_name)
            val = prop.value if prop is not None else None
            unit = prop.unit if prop is not None else ""

            if val is None:
                # Missing property penalty: 0 score for that criterion
                score = 0.0
                note = "Data unavailable (0/100 for this criterion)"
            else:
                # Check hard constraints
                if crit.min_threshold is not None and val < crit.min_threshold:
                    passed_constraints = False
                    failures.append(
                        f"{crit.label} ({val:g} {unit}) below required min ({crit.min_threshold:g})"
                    )

                if crit.max_threshold is not None and val > crit.max_threshold:
                    passed_constraints = False
                    failures.append(
                        f"{crit.label} ({val:g} {unit}) exceeds max allowed ({crit.max_threshold:g})"
                    )

                min_v, max_v = prop_ranges[crit.property_name]
                if max_v == min_v:
                    score = 100.0
                else:
                    if crit.higher_is_better:
                        score = ((val - min_v) / (max_v - min_v)) * 100.0
                    else:
                        score = ((max_v - val) / (max_v - min_v)) * 100.0

                score = max(0.0, min(100.0, score))
                note = f"Score: {score:.1f}/100"

            weighted_sub = score * w
            overall += weighted_sub

            breakdowns[crit.property_name] = PropertyScoreBreakdown(
                property_name=crit.property_name,
                label=crit.label,
                raw_value=val,
                unit=unit,
                normalized_score=score,
                weight=w,
                weighted_contribution=weighted_sub,
                higher_is_better=crit.higher_is_better,
                is_available=(val is not None),
                status_note=note,
            )

        if not passed_constraints:
            overall *= 0.5  # Penalize failing hard constraints

        evaluations.append(
            CandidateEvaluation(
                material=mat,
                overall_score=overall,
                passed_hard_constraints=passed_constraints,
                failure_reasons=failures,
                breakdowns=breakdowns,
            )
        )

    # Sort candidates by overall score descending
    evaluations.sort(key=lambda x: x.overall_score, reverse=True)

    for rank, ev in enumerate(evaluations, start=1):
        ev.rank = rank
        # Build explanation
        top_prop = max(
            ev.breakdowns.values(), key=lambda b: b.weighted_contribution, default=None
        )
        weak_prop = min(
            ev.breakdowns.values(), key=lambda b: b.weighted_contribution, default=None
        )

        explanation = f"Overall score: {ev.overall_score:.1f}/100."
        if top_prop:
            explanation += f" Strongest factor: {top_prop.label} (+{top_prop.weighted_contribution:.1f} pts)."
        if weak_prop and weak_prop != top_prop:
            explanation += f" Limiting factor: {weak_prop.label} (+{weak_prop.weighted_contribution:.1f} pts)."
        if not ev.passed_hard_constraints:
            explanation += f" Note: Failed {len(ev.failure_reasons)} constraint(s)."

        ev.summary_explanation = explanation

    return evaluations


# ============================================================
# PREDEFINED CHALLENGES
# ============================================================

BUILTIN_CHALLENGES: List[EngineeringChallenge] = [
    EngineeringChallenge(
        id="lunar_rover_chassis",
        title="Lunar Rover Structural Chassis",
        description="Select an optimal structural material for a lightweight lunar exploration rover frame operating under extreme temperature swings (-180 °C to +120 °C) and severe launch mass restrictions.",
        engineering_context=(
            "Every kilogram launched to the lunar surface costs tens of thousands of dollars. "
            "The chassis must minimize total mass (low density) while maintaining high structural stiffness (high Young's modulus) "
            "to prevent frame flex and high yield strength to withstand lunar terrain impacts. "
            "Low thermal expansion is critical to prevent thermal warping across sunlit and shadowed lunar craters."
        ),
        default_criteria=[
            Criterion(
                "density",
                "Low Density (Mass Minimization)",
                weight=0.30,
                higher_is_better=False,
                description="Minimize launch payload mass",
            ),
            Criterion(
                "youngs_modulus",
                "High Stiffness (Young's Modulus)",
                weight=0.25,
                higher_is_better=True,
                description="Resist chassis deflection",
            ),
            Criterion(
                "yield_strength",
                "High Yield Strength",
                weight=0.25,
                higher_is_better=True,
                description="Prevent permanent plastic damage",
            ),
            Criterion(
                "thermal_expansion",
                "Low Thermal Expansion (CTE)",
                weight=0.20,
                higher_is_better=False,
                description="Maintain dimensional precision across thermal swings",
            ),
        ],
        recommended_candidate_ids=[
            "aluminum_6061_t6",
            "aluminum_7075_t6",
            "titanium_ti6al4v_grade5",
            "magnesium_az31b",
            "cfrp_quasi_isotropic",
            "cfrp_unidirectional_highmod",
            "steel_4140_chromoly",
        ],
        notes_and_tradeoffs="CFRP and Titanium Ti-6Al-4V offer exceptional specific stiffness and strength, while Aluminum 6061 provides balanced machinability and cost.",
    ),
    EngineeringChallenge(
        id="reentry_heat_shield",
        title="Atmospheric Re-entry Heat Shield",
        description="Select a thermal protection material for a planetary re-entry capsule experiencing intense aerothermal shock heating exceeding 1400 °C during atmospheric entry.",
        engineering_context=(
            "During planetary atmospheric entry, compression of atmospheric gas creates extreme convective and radiative heat fluxes. "
            "The outer heat shield must survive ultra-high temperatures (high melting point/sublimation), insulate the spacecraft interior by blocking heat flow (ultra-low thermal conductivity), "
            "radiate heat back to space (high surface emissivity), and maintain light weight."
        ),
        default_criteria=[
            Criterion(
                "melting_point",
                "High Melting / Decomposition Temp",
                weight=0.35,
                higher_is_better=True,
                description="Must survive aerothermal peak heating",
            ),
            Criterion(
                "thermal_conductivity",
                "Low Thermal Conductivity",
                weight=0.30,
                higher_is_better=False,
                description="Insulate internal crew capsule",
            ),
            Criterion(
                "emissivity",
                "High Surface Emissivity",
                weight=0.20,
                higher_is_better=True,
                description="Radiate heat back to environment",
            ),
            Criterion(
                "density",
                "Low Density",
                weight=0.15,
                higher_is_better=False,
                description="Minimize TPS heat shield parasitic mass",
            ),
        ],
        recommended_candidate_ids=[
            "tps_shuttle_tile_li900",
            "tps_pica_phenolic_carbon",
            "carbon_carbon_composite",
            "cmc_c_sic",
            "zirconia_ysz_3y_tzp",
            "alumina_99_5",
            "fused_silica_quartz",
            "inconel_718",
        ],
        notes_and_tradeoffs="Sintered silica tiles (LI-900) and PICA excel in thermal insulation and low density, while Carbon-Carbon and C/SiC provide higher structural toughness.",
    ),
    EngineeringChallenge(
        id="satellite_thermal_radiator",
        title="Spacecraft Heat-Rejection Radiator Panel",
        description="Select a material for deep-space satellite thermal radiator panels to reject waste heat from onboard avionics and power systems into space via thermal radiation.",
        engineering_context=(
            "In the vacuum of space, heat cannot be removed by convection. "
            "Radiators must rapidly conduct internal heat from coolant loops across the panel area (high thermal conductivity), "
            "efficiently emit infrared photons into deep space (high emissivity), and remain lightweight to preserve spacecraft mass budget."
        ),
        default_criteria=[
            Criterion(
                "thermal_conductivity",
                "High Thermal Conductivity",
                weight=0.35,
                higher_is_better=True,
                description="Rapid heat spreading across radiator surface",
            ),
            Criterion(
                "emissivity",
                "High Surface Emissivity",
                weight=0.30,
                higher_is_better=True,
                description="Maximize radiated radiative power (Stefan-Boltzmann law)",
            ),
            Criterion(
                "density",
                "Low Density",
                weight=0.25,
                higher_is_better=False,
                description="Minimize structural panel mass",
            ),
            Criterion(
                "thermal_expansion",
                "Low Thermal Expansion",
                weight=0.10,
                higher_is_better=False,
                description="Prevent thermal distortion between sunlit and shadow passes",
            ),
        ],
        recommended_candidate_ids=[
            "aluminum_6061_t6",
            "copper_c10100_ofhc",
            "graphite_synthetic_isotropic",
            "diamond_cvd",
            "aluminum_nitride_aln",
            "magnesium_az31b",
            "cfrp_unidirectional_highmod",
            "silver_pure",
        ],
        notes_and_tradeoffs="Aluminum 6061-T6 is standard in space engineering for high thermal conductivity and low mass; synthetic graphite and diamond offer extreme conductivity at different cost/fabrication tradeoffs.",
    ),
]
