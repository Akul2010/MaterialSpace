"""
Tests for Thermal Expansion Physics Model.
"""

import pytest

from app.models.thermal_expansion import calculate_expansion, generate_temperature_curve


def test_thermal_expansion_standard_case():
    # Aluminum 6061: alpha = 23.6e-6 1/K, L0 = 1.0 m, Delta T = 100 K
    res = calculate_expansion(
        initial_length=1.0,
        alpha=23.6e-6,
        t_initial=293.15,
        t_final=393.15,
    )
    assert abs(res.delta_temperature - 100.0) < 1e-9
    assert abs(res.length_change - 0.00236) < 1e-9
    assert abs(res.final_length - 1.00236) < 1e-9
    assert abs(res.strain - 0.00236) < 1e-9


def test_thermal_expansion_cooling_case():
    # Negative Delta T (cooling)
    res = calculate_expansion(
        initial_length=2.0,
        alpha=10e-6,
        t_initial=300.0,
        t_final=200.0,
    )
    assert res.delta_temperature == -100.0
    assert abs(res.length_change - (-0.002)) < 1e-9
    assert abs(res.final_length - 1.998) < 1e-9


def test_thermal_expansion_curve_generation():
    temps, lengths = generate_temperature_curve(
        initial_length=1.0,
        alpha=20e-6,
        t_start=300.0,
        t_end=400.0,
        points=50,
    )
    assert len(temps) == 50
    assert len(lengths) == 50
    assert abs(lengths[0] - 1.0) < 1e-9
    assert abs(lengths[-1] - 1.002) < 1e-9


def test_thermal_expansion_invalid_inputs():
    with pytest.raises(ValueError):
        calculate_expansion(
            initial_length=-1.0, alpha=20e-6, t_initial=300, t_final=400
        )
    with pytest.raises(ValueError):
        calculate_expansion(initial_length=1.0, alpha=1.0, t_initial=300, t_final=400)
