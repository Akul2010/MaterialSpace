"""
Tests for Material Selection Decision Engine.
"""

from app.data.database import MaterialDatabase
from app.models.material_selection import (
    BUILTIN_CHALLENGES,
    Criterion,
    evaluate_materials,
)


def test_material_evaluation_ranking():
    db = MaterialDatabase.default()
    al = db.get_material("aluminum_6061_t6")
    ti = db.get_material("titanium_ti6al4v_grade5")
    cfrp = db.get_material("cfrp_unidirectional_highmod")
    assert al and ti and cfrp

    # Challenge criteria: minimize density (weight 0.5), maximize stiffness (weight 0.5)
    criteria = [
        Criterion("density", "Low Density", weight=0.5, higher_is_better=False),
        Criterion(
            "youngs_modulus", "High Stiffness", weight=0.5, higher_is_better=True
        ),
    ]

    evals = evaluate_materials([al, ti, cfrp], criteria)
    assert len(evals) == 3

    # Check that rank 1 has highest score
    assert evals[0].rank == 1
    assert evals[0].overall_score >= evals[1].overall_score >= evals[2].overall_score

    # Unidirectional high-mod CFRP has lowest density (1580 kg/m3) and highest stiffness (230 GPa)
    assert evals[0].material.id == "cfrp_unidirectional_highmod"
    assert evals[0].overall_score == 100.0


def test_builtin_challenges_evaluations():
    db = MaterialDatabase.default()
    for challenge in BUILTIN_CHALLENGES:
        candidates = [
            db.get_material(cid) for cid in challenge.recommended_candidate_ids
        ]
        valid_candidates = [c for c in candidates if c is not None]
        assert len(valid_candidates) >= 5

        evals = evaluate_materials(valid_candidates, challenge.default_criteria)
        assert len(evals) == len(valid_candidates)
        assert evals[0].rank == 1
        assert evals[0].summary_explanation != ""
