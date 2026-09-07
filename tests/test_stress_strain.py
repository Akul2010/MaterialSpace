"""
Tests for Stress and Strain Physics Model.
"""

import pytest
from app.models.stress_strain import (
    calculate_stress_strain,
    generate_stress_strain_curve,
    YieldStatus,
)


def test_stress_strain_elastic_safe():
    # Force = 10,000 N, Area = 100 mm^2 (1e-4 m^2), E = 70 GPa (70e9 Pa), L0 = 1.0 m, Yield = 276 MPa
    res = calculate_stress_strain(
        force=10000.0,
        area=1e-4,
        youngs_modulus=70e9,
        initial_length=1.0,
        yield_strength=276e6,
    )
    # Stress = 10000 / 1e-4 = 100 MPa (1e8 Pa)
    assert abs(res.stress - 1e8) < 1e-3
    # Strain = 1e8 / 70e9 = 1.42857e-3
    assert abs(res.strain - (1e8 / 70e9)) < 1e-9
    # Def = Strain * 1.0 m = 1.42857 mm
    assert abs(res.deformation - (1e8 / 70e9)) < 1e-9
    assert abs(res.factor_of_safety - 2.76) < 1e-3
    assert res.yield_status == YieldStatus.ELASTIC_SAFE


def test_stress_strain_yielding_exceeded():
    # Stress = 300 MPa > 276 MPa
    res = calculate_stress_strain(
        force=30000.0,
        area=1e-4,
        youngs_modulus=70e9,
        initial_length=1.0,
        yield_strength=276e6,
    )
    assert res.stress == 300e6
    assert res.yield_status == YieldStatus.YIELDING_EXCEEDED
    assert "exceeds listed yield strength" in res.status_message


def test_stress_strain_near_yield():
    # Stress = 250 MPa (~90.5% of 276 MPa)
    res = calculate_stress_strain(
        force=25000.0,
        area=1e-4,
        youngs_modulus=70e9,
        initial_length=1.0,
        yield_strength=276e6,
    )
    assert res.yield_status == YieldStatus.NEAR_YIELD


def test_stress_strain_curve():
    strains, stresses = generate_stress_strain_curve(
        youngs_modulus=100e9, yield_strength=500e6, max_strain=0.01
    )
    assert len(strains) == 100
    assert len(stresses) == 100
    assert stresses[-1] == 100e9 * 0.01


def test_stress_strain_invalid():
    with pytest.raises(ValueError):
        calculate_stress_strain(force=100, area=-1.0, youngs_modulus=10e9)
    with pytest.raises(ValueError):
        calculate_stress_strain(force=100, area=1e-4, youngs_modulus=-10e9)
