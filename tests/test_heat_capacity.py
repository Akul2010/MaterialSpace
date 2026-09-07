"""
Tests for Heat Capacity Physics Model.
"""

import pytest
from app.models.heat_capacity import calculate_heat_energy, generate_energy_curve


def test_heat_capacity_standard():
    # m = 2.5 kg, c = 900 J/(kg*K), T0 = 20 °C (293.15 K), T_final = 120 °C (393.15 K) (Delta T = 100 K)
    res = calculate_heat_energy(
        mass=2.5,
        specific_heat=900.0,
        t_initial=293.15,
        t_final=393.15,
    )
    # Q = 2.5 * 900 * 100 = 225,000 J
    assert abs(res.delta_temperature - 100.0) < 1e-9
    assert abs(res.energy - 225000.0) < 1e-6
    assert abs(res.thermal_mass - 2250.0) < 1e-6


def test_heat_capacity_cooling():
    res = calculate_heat_energy(
        mass=1.0,
        specific_heat=500.0,
        t_initial=350.0,
        t_final=300.0,
    )
    assert res.delta_temperature == -50.0
    assert res.energy == -25000.0


def test_heat_capacity_curve():
    temps, energies = generate_energy_curve(
        mass=1.0, specific_heat=1000.0, t_start=300.0, t_end=400.0, points=50
    )
    assert len(temps) == 50
    assert len(energies) == 50
    assert abs(energies[0] - 0.0) < 1e-9
    assert abs(energies[-1] - 100000.0) < 1e-9


def test_heat_capacity_invalid():
    with pytest.raises(ValueError):
        calculate_heat_energy(mass=-1.0, specific_heat=500, t_initial=300, t_final=350)
    with pytest.raises(ValueError):
        calculate_heat_energy(mass=1.0, specific_heat=-500, t_initial=300, t_final=350)
