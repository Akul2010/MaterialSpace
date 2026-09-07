"""
Material Comparison View for MaterialSpace.

Allows side-by-side tabular comparison of 2–4 materials with comparative bar charts.
"""

from __future__ import annotations

from typing import List, Optional
import numpy as np
from app.ui.qt import (
    QComboBox,
    QFrame,
    QGridLayout,
    QHBoxLayout,
    QHeaderView,
    QLabel,
    QPushButton,
    QScrollArea,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
    QWidget,
    Qt,
    Signal,
)
from app.core.constants import PROPERTY_METADATA
from app.core.material import Material
from app.core.units import format_property_value
from app.data.database import MaterialDatabase
from app.ui.widgets import MplCanvas


class ComparisonView(QWidget):
    """
    Side-by-side comparison table and comparative scientific charts for 2 to 4 materials.
    """

    material_clicked = Signal(str)  # material_id

    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self.selected_material_ids: List[str] = [
            "aluminum_6061_t6",
            "titanium_ti6al4v_grade5",
            "cfrp_unidirectional_highmod",
            "alumina_99_5",
        ]
        self._init_ui()
        self._refresh_comparison()

    def _init_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(24, 20, 24, 20)
        main_layout.setSpacing(16)

        # Header Title
        title_box = QVBoxLayout()
        title = QLabel("Material Comparison Matrix")
        title.setStyleSheet("font-size: 22px; font-weight: 700; color: #ffffff;")
        title_box.addWidget(title)

        subtitle = QLabel(
            "Cross-compare 2 to 4 candidate materials side-by-side across density, stiffness, strength, and thermal properties."
        )
        subtitle.setStyleSheet("color: #8b949e; font-size: 13px;")
        title_box.addWidget(subtitle)
        main_layout.addLayout(title_box)

        # Selectors Row
        selectors_box = QFrame()
        selectors_box.setStyleSheet(
            "background-color: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        sel_layout = QHBoxLayout(selectors_box)
        sel_layout.setSpacing(12)

        sel_title = QLabel("CANDIDATES:")
        sel_title.setStyleSheet("color: #58a6ff; font-weight: 700; font-size: 11px;")
        sel_layout.addWidget(sel_title)

        self.combos: List[QComboBox] = []
        all_materials = self.db.get_all_materials()

        for slot in range(4):
            combo = QComboBox()
            combo.addItem(f"— Empty Slot {slot + 1} —", "")
            for m in all_materials:
                combo.addItem(m.name, m.id)

            if slot < len(self.selected_material_ids):
                idx = combo.findData(self.selected_material_ids[slot])
                if idx >= 0:
                    combo.setCurrentIndex(idx)

            combo.currentIndexChanged.connect(self._on_material_selected)
            self.combos.append(combo)
            sel_layout.addWidget(combo, stretch=1)

        main_layout.addWidget(selectors_box)

        # Main Scrollable Comparison Area
        scroll = QScrollArea()
        scroll.setWidgetResizable(True)
        scroll.setFrameShape(QFrame.NoFrame)
        scroll_content = QWidget()
        self.scroll_layout = QVBoxLayout(scroll_content)
        self.scroll_layout.setSpacing(16)

        # Comparison Table
        self.table = QTableWidget()
        self.table.setStyleSheet(
            "background-color: #161b22; border: 1px solid #30363d; border-radius: 8px;"
        )
        self.scroll_layout.addWidget(self.table)

        # Chart Section Header
        chart_header_row = QHBoxLayout()
        chart_title = QLabel("Comparative Property Visualizer")
        chart_title.setStyleSheet("font-size: 16px; font-weight: 700; color: #e6edf3;")
        chart_header_row.addWidget(chart_title)
        chart_header_row.addStretch()

        chart_sel_lbl = QLabel("Plot Property:")
        chart_sel_lbl.setStyleSheet("color: #8b949e; font-weight: 600;")
        chart_header_row.addWidget(chart_sel_lbl)

        self.chart_prop_combo = QComboBox()
        self.chart_prop_combo.addItem("Density (kg/m³)", "density")
        self.chart_prop_combo.addItem("Young's Modulus (GPa)", "youngs_modulus")
        self.chart_prop_combo.addItem("Yield Strength (MPa)", "yield_strength")
        self.chart_prop_combo.addItem(
            "Thermal Conductivity (W/m·K)", "thermal_conductivity"
        )
        self.chart_prop_combo.addItem("Specific Heat (J/kg·K)", "specific_heat")
        self.chart_prop_combo.addItem(
            "Thermal Expansion CTE (µm/m·K)", "thermal_expansion"
        )
        self.chart_prop_combo.addItem(
            "Specific Stiffness (E/ρ) [GPa / (g/cm³)]", "spec_stiffness"
        )
        self.chart_prop_combo.addItem(
            "Specific Strength (σ_y/ρ) [MPa / (g/cm³)]", "spec_strength"
        )
        self.chart_prop_combo.currentIndexChanged.connect(self._update_chart)
        chart_header_row.addWidget(self.chart_prop_combo)

        self.scroll_layout.addLayout(chart_header_row)

        # Canvas
        self.canvas = MplCanvas(self, width=8, height=3.5)
        self.scroll_layout.addWidget(self.canvas)

        scroll.setWidget(scroll_content)
        main_layout.addWidget(scroll)

    def add_material(self, material_id: str):
        """Add a material to the comparison slots."""
        # Find first empty slot or replace last slot
        for combo in self.combos:
            if not combo.currentData():
                idx = combo.findData(material_id)
                if idx >= 0:
                    combo.setCurrentIndex(idx)
                    return

        # Replace first slot
        idx = self.combos[0].findData(material_id)
        if idx >= 0:
            self.combos[0].setCurrentIndex(idx)

    def _on_material_selected(self):
        ids = []
        for combo in self.combos:
            mid = combo.currentData()
            if mid and mid not in ids:
                ids.append(mid)
        self.selected_material_ids = ids
        self._refresh_comparison()

    def _refresh_comparison(self):
        mats = [
            self.db.get_material(mid)
            for mid in self.selected_material_ids
            if self.db.get_material(mid)
        ]

        if not mats:
            self.table.setColumnCount(1)
            self.table.setRowCount(1)
            self.table.setHorizontalHeaderLabels(["No materials selected"])
            self.table.setItem(
                0,
                0,
                QTableWidgetItem("Select materials in the dropdowns above to compare."),
            )
            self._update_chart()
            return

        cols = ["Property"] + [m.name for m in mats]
        self.table.setColumnCount(len(cols))
        self.table.setHorizontalHeaderLabels(cols)
        self.table.horizontalHeader().setSectionResizeMode(
            0, QHeaderView.ResizeToContents
        )
        for i in range(1, len(cols)):
            self.table.horizontalHeader().setSectionResizeMode(i, QHeaderView.Stretch)

        rows = [
            ("Category", lambda m: m.category.title().replace("_", " ")),
            ("Composition", lambda m: m.composition or "—"),
            ("Grade / Temper", lambda m: m.grade or "—"),
            (
                "Density",
                lambda m: format_property_value("density", m.get_value("density")),
            ),
            (
                "Young's Modulus (E)",
                lambda m: format_property_value(
                    "youngs_modulus", m.get_value("youngs_modulus")
                ),
            ),
            (
                "Yield Strength (σ_y)",
                lambda m: format_property_value(
                    "yield_strength", m.get_value("yield_strength")
                ),
            ),
            (
                "Tensile Strength (σ_uts)",
                lambda m: format_property_value(
                    "tensile_strength", m.get_value("tensile_strength")
                ),
            ),
            (
                "Thermal Conductivity (k)",
                lambda m: format_property_value(
                    "thermal_conductivity", m.get_value("thermal_conductivity")
                ),
            ),
            (
                "Specific Heat (c)",
                lambda m: format_property_value(
                    "specific_heat", m.get_value("specific_heat")
                ),
            ),
            (
                "Thermal Expansion (α)",
                lambda m: format_property_value(
                    "thermal_expansion", m.get_value("thermal_expansion")
                ),
            ),
            (
                "Surface Emissivity (ε)",
                lambda m: format_property_value(
                    "emissivity", m.get_value("emissivity")
                ),
            ),
            (
                "Melting Point",
                lambda m: format_property_value(
                    "melting_point", m.get_value("melting_point")
                ),
            ),
        ]

        self.table.setRowCount(len(rows))

        for r_idx, (prop_label, getter) in enumerate(rows):
            # Row header
            prop_item = QTableWidgetItem(prop_label)
            prop_item.setForeground(Qt.GlobalColor.lightGray)
            self.table.setItem(r_idx, 0, prop_item)

            for c_idx, mat in enumerate(mats, start=1):
                val_text = getter(mat)
                item = QTableWidgetItem(val_text)
                if val_text == "Data unavailable" or val_text == "—":
                    item.setForeground(Qt.GlobalColor.darkGray)
                self.table.setItem(r_idx, c_idx, item)

        self._update_chart()

    def _update_chart(self):
        mats = [
            self.db.get_material(mid)
            for mid in self.selected_material_ids
            if self.db.get_material(mid)
        ]
        ax = self.canvas.axes
        ax.clear()

        if not mats:
            ax.text(
                0.5,
                0.5,
                "No candidate materials selected",
                color="#8b949e",
                ha="center",
                va="center",
            )
            self.canvas.draw()
            return

        prop_mode = self.chart_prop_combo.currentData()
        names = [m.name.split("(")[0].strip() for m in mats]
        values = []
        labels = []

        colors = ["#58a6ff", "#3fb950", "#bc8cff", "#d29922"]

        for m in mats:
            if prop_mode == "spec_stiffness":
                e = m.get_value("youngs_modulus")
                rho = m.get_value("density")
                v = (e / 1e9) / (rho / 1000.0) if e and rho else None
            elif prop_mode == "spec_strength":
                sy = m.get_value("yield_strength")
                rho = m.get_value("density")
                v = (sy / 1e6) / (rho / 1000.0) if sy and rho else None
            elif prop_mode == "youngs_modulus":
                raw = m.get_value("youngs_modulus")
                v = (raw / 1e9) if raw else None
            elif prop_mode == "yield_strength":
                raw = m.get_value("yield_strength")
                v = (raw / 1e6) if raw else None
            elif prop_mode == "thermal_expansion":
                raw = m.get_value("thermal_expansion")
                v = (raw * 1e6) if raw else None
            else:
                v = m.get_value(prop_mode)

            values.append(v if v is not None else 0.0)
            labels.append(f"{v:.2f}" if v is not None else "N/A")

        x = np.arange(len(mats))
        bars = ax.bar(
            x,
            values,
            color=colors[: len(mats)],
            width=0.5,
            edgecolor="#ffffff",
            linewidth=0.5,
        )

        for bar, lbl, val in zip(bars, labels, values):
            y_pos = bar.get_height()
            if val < 0:
                ax.text(
                    bar.get_x() + bar.get_width() / 2,
                    y_pos - (abs(val) * 0.15 + 0.5),
                    lbl,
                    ha="center",
                    va="top",
                    color="#e6edf3",
                    fontweight="bold",
                    fontsize=10,
                )
            else:
                ax.text(
                    bar.get_x() + bar.get_width() / 2,
                    y_pos + (val * 0.02 + 0.2),
                    lbl,
                    ha="center",
                    va="bottom",
                    color="#e6edf3",
                    fontweight="bold",
                    fontsize=10,
                )

        ax.set_xticks(x)
        ax.set_xticklabels(names, color="#e6edf3", fontsize=10, fontweight="bold")
        ax.set_title(
            self.chart_prop_combo.currentText(),
            color="#e6edf3",
            fontsize=12,
            fontweight="bold",
            pad=12,
        )
        ax.grid(axis="y", linestyle="--", alpha=0.4, color="#21262d")

        self.canvas.draw()
