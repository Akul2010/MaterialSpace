import pytest

from app.data.database import MaterialDatabase
from app.ui.main_window import MainWindow
from app.ui.qt import QApplication


@pytest.fixture(scope="session")
def qapp():
    app = QApplication.instance()
    if app is None:
        app = QApplication([])
    return app


def test_main_window_init(qapp):
    db = MaterialDatabase.default()
    window = MainWindow(database=db)
    assert window.db.count() >= 50

    # Test navigation across all views
    window.switch_view("explore")
    assert window.stack.currentIndex() == 1

    window.switch_view("simulations")
    assert window.stack.currentIndex() == 3

    window.switch_view("compare")
    assert window.stack.currentIndex() == 4

    window.switch_view("challenges")
    assert window.stack.currentIndex() == 5

    window.switch_view("home")
    assert window.stack.currentIndex() == 0

    # Test opening material detail
    window.open_material_detail("aluminum_6061_t6")
    assert window.stack.currentIndex() == 2
    assert window.detail_view.current_material.id == "aluminum_6061_t6"

    # Test opening simulation
    window.open_material_simulation("titanium_ti6al4v_grade5")
    assert window.stack.currentIndex() == 3

    # Test adding comparison
    window.open_material_comparison("alumina_99_5")
    assert window.stack.currentIndex() == 4
