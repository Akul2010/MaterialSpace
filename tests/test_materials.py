"""
Tests for Material and MaterialProperty dataclasses.
"""

from app.core.material import Material, MaterialProperty


def test_material_property_instantiation():
    prop = MaterialProperty(
        value=2700.0,
        unit="kg/m^3",
        source_id="matweb",
        condition="Room Temp",
    )
    assert prop.value == 2700.0
    assert prop.unit == "kg/m^3"
    assert prop.source_id == "matweb"
    d = prop.to_dict()
    assert d["value"] == 2700.0
    restored = MaterialProperty.from_dict(d)
    assert restored.value == 2700.0


def test_material_instantiation_and_helpers():
    m = Material(
        id="test_mat",
        name="Test Material",
        category="metal",
        composition="Al-100",
        grade="Test-Grade",
        properties={
            "density": MaterialProperty(value=2700.0, unit="kg/m^3", source_id="src1"),
            "thermal_conductivity": MaterialProperty(
                value=150.0, unit="W/(m*K)", source_id="src2"
            ),
        },
    )

    assert m.name == "Test Material"
    assert m.has_property("density")
    assert not m.has_property("youngs_modulus")
    assert m.get_value("density") == 2700.0
    assert m.get_value("youngs_modulus") is None

    d = m.to_dict()
    restored = Material.from_dict(d)
    assert restored.id == "test_mat"
    assert restored.get_value("thermal_conductivity") == 150.0
