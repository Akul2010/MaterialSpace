"""
Unified Qt compatibility layer for MaterialSpace.
Supports QtPy, PyQt5, and PySide6 transparently.
"""

from __future__ import annotations

import os

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
        from PyQt5.QtCore import Qt, pyqtSignal as Signal, pyqtSlot as Slot
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
    "QtCore",
    "QtGui",
    "QtWidgets",
    "Qt",
    "Signal",
    "Slot",
    "QFont",
    "QIcon",
    "QApplication",
    "QButtonGroup",
    "QCheckBox",
    "QComboBox",
    "QDoubleSpinBox",
    "QFrame",
    "QGridLayout",
    "QHBoxLayout",
    "QHeaderView",
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
    "QTableWidget",
    "QTableWidgetItem",
    "QTabWidget",
    "QVBoxLayout",
    "QWidget",
]
