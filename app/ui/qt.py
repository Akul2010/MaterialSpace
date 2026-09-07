"""
Unified Qt compatibility layer for MaterialSpace.
Supports QtPy, PyQt5, and PySide6 transparently.
"""

from __future__ import annotations

# Prefer PyQt5 if PySide6 DLLs are conflicted in the current environment
try:
    from qtpy import QtCore, QtGui, QtWidgets
    from qtpy.QtCore import Qt, Signal, Slot
    from qtpy.QtGui import QFont, QIcon
    from qtpy.QtWidgets import (
        QApplication,
        QButtonGroup,
        QCheckBox,
        QComboBox,
        QDoubleSpinBox,
        QFrame,
        QGridLayout,
        QHBoxLayout,
        QHeaderView,
        QLabel,
        QLineEdit,
        QMainWindow,
        QProgressBar,
        QPushButton,
        QScrollArea,
        QSlider,
        QSpinBox,
        QSplitter,
        QStackedWidget,
        QTableWidget,
        QTableWidgetItem,
        QTabWidget,
        QVBoxLayout,
        QWidget,
    )
except ImportError:
    try:
        from PySide6 import QtCore, QtGui, QtWidgets
        from PySide6.QtCore import Qt, Signal, Slot
        from PySide6.QtGui import QFont, QIcon
        from PySide6.QtWidgets import (
            QApplication,
            QButtonGroup,
            QCheckBox,
            QComboBox,
            QDoubleSpinBox,
            QFrame,
            QGridLayout,
            QHBoxLayout,
            QHeaderView,
            QLabel,
            QLineEdit,
            QMainWindow,
            QProgressBar,
            QPushButton,
            QScrollArea,
            QSlider,
            QSpinBox,
            QSplitter,
            QStackedWidget,
            QTableWidget,
            QTableWidgetItem,
            QTabWidget,
            QVBoxLayout,
            QWidget,
        )
    except ImportError:
        from PyQt5 import QtCore, QtGui, QtWidgets
        from PyQt5.QtCore import Qt
        from PyQt5.QtCore import pyqtSignal as Signal
        from PyQt5.QtCore import pyqtSlot as Slot
        from PyQt5.QtGui import QFont, QIcon
        from PyQt5.QtWidgets import (
            QApplication,
            QButtonGroup,
            QCheckBox,
            QComboBox,
            QDoubleSpinBox,
            QFrame,
            QGridLayout,
            QHBoxLayout,
            QHeaderView,
            QLabel,
            QLineEdit,
            QMainWindow,
            QProgressBar,
            QPushButton,
            QScrollArea,
            QSlider,
            QSpinBox,
            QSplitter,
            QStackedWidget,
            QTableWidget,
            QTableWidgetItem,
            QTabWidget,
            QVBoxLayout,
            QWidget,
        )

__all__ = [
    "QApplication",
    "QButtonGroup",
    "QCheckBox",
    "QComboBox",
    "QDoubleSpinBox",
    "QFont",
    "QFrame",
    "QGridLayout",
    "QHBoxLayout",
    "QHeaderView",
    "QIcon",
    "QLabel",
    "QLineEdit",
    "QMainWindow",
    "QProgressBar",
    "QPushButton",
    "QScrollArea",
    "QSlider",
    "QSpinBox",
    "QSplitter",
    "QStackedWidget",
    "QTabWidget",
    "QTableWidget",
    "QTableWidgetItem",
    "QVBoxLayout",
    "QWidget",
    "Qt",
    "QtCore",
    "QtGui",
    "QtWidgets",
    "Signal",
    "Slot",
]
