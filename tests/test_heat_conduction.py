"""
Tests for Heat Conduction Physics Model.
"""

import pytest
from app.models.heat_conduction import (
    calculate_conduction,
    generate_temperature_profile,
)


def test_heat_conduction_standard_case():
    # k = 167 W/(m*K), A = 0.5 m^2, L = 0.01 m (10 mm), T_hot = 373.15 K, T_cold = 293.15 K (80 K diff)
    res = calculate_conduction(
        thermal_conductivity=167.0,
        area=0.5,
        thickness=0.01,
        t_hot=373.15,
        t_cold=293.15,
    )
    # Q_dot = 167 * 0.5 * 80 / 0.01 = 668,000 W
    assert abs(res.delta_temperature - 80.0) < 1e-9
    assert abs(res.thermal_gradient - 8000.0) < 1e-9
    assert abs(res.heat_rate - 668000.0) < 1e-6
    assert abs(res.heat_flux - 1336000.0) < 1e-6
    assert abs(res.thermal_resistance - (0.01 / (167.0 * 0.5))) < 1e-9


def test_heat_conduction_profile():
    x, t = generate_temperature_profile(
        t_hot=400.0, t_cold=300.0, thickness=0.02, points=50
    )
    assert len(x) == 50
    assert len(t) == 50
    assert abs(t[0] - 400.0) < 1e-9
    assert abs(t[-1] - 300.0) < 1e-9
    assert abs(t[25] - 348.979) < 0.1


def test_heat_conduction_invalid_inputs():
    with pytest.raises(ValueError):
        calculate_conduction(
            thermal_conductivity=-5.0, area=1.0, thickness=0.01, t_hot=350, t_cold=300
        )
    with pytest.raises(ValueError):
        calculate_conduction(
            thermal_conductivity=100.0, area=-1.0, thickness=0.01, t_hot=350, t_cold=300
        )
    with pytest.raises(ValueError):
        calculate_conduction(
            thermal_conductivity=100.0, area=1.0, thickness=0.0, t_hot=350, t_cold=300
        )
