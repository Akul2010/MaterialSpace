"""
Tests for MaterialDatabase.
"""

from app.data.database import MaterialDatabase


def test_database_loading_and_counts():
    db = MaterialDatabase.default()
    assert db.count() >= 50
    al = db.get_material("aluminum_6061_t6")
    assert al is not None
    assert al.name == "Aluminum 6061-T6"
    assert al.get_value("density") == 2700.0


def test_database_categories_and_filtering():
    db = MaterialDatabase.default()
    categories = db.get_categories()
    assert "metal" in categories
    assert "ceramic" in categories
    assert "polymer" in categories
    assert "composite" in categories

    metals = db.filter_by_category("metal")
    assert len(metals) >= 10
    for m in metals:
        assert m.category == "metal"


def test_database_search():
    db = MaterialDatabase.default()
    # Search by name
    results = db.search("aluminum 6061")
    assert len(results) >= 1
    assert results[0].id == "aluminum_6061_t6"

    # Search by category filter
    ti_ceramics = db.search("titanium", category="ceramic")
    assert len(ti_ceramics) == 0

    ti_metals = db.search("titanium", category="metal")
    assert len(ti_metals) >= 1


def test_database_comparison():
    db = MaterialDatabase.default()
    mats = db.compare(
        ["aluminum_6061_t6", "titanium_ti6al4v_grade5", "non_existent_id"]
    )
    assert len(mats) == 2
    assert mats[0].id == "aluminum_6061_t6"
    assert mats[1].id == "titanium_ti6al4v_grade5"
