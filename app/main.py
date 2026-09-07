"""
MaterialSpace Application Entry Point.
"""

from __future__ import annotations

import sys
from pathlib import Path

# Ensure application root directory is on Python path
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from app.ui.qt import QApplication, QFont, Qt

from app.data.database import MaterialDatabase
from app.ui.main_window import MainWindow


def main() -> int:
    app = QApplication(sys.argv)
    app.setApplicationName("MaterialSpace")
    app.setOrganizationName("MaterialSpace")

    # Set default high-resolution font
    font = QFont("Segoe UI", 10)
    app.setFont(font)

    # Initialize indexed material database
    db = MaterialDatabase.default()

    # Launch main window
    window = MainWindow(database=db)
    window.show()

    return app.exec()


if __name__ == "__main__":
    sys.exit(main())
