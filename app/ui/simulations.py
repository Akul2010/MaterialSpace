"""
Physics Simulations Workbench for MaterialSpace.

Houses 4 interactive physics laboratory experiments:
1. Linear Thermal Expansion
2. 1D Steady-State Heat Conduction
3. Uniaxial Stress, Strain, & Hooke's Law (with Yield Strength checks)
4. Heat Capacity & Sensible Thermal Energy Storage
"""

from __future__ import annotations

import numpy as np

import app.models.heat_capacity as hcap_model
import app.models.heat_conduction as hc_model
import app.models.stress_strain as ss_model
import app.models.thermal_expansion as te_model

from app.data.database import MaterialDatabase
from app.ui.qt import (
    QComboBox,
    QDoubleSpinBox,
    QFrame,
    QHBoxLayout,
    QLabel,
    QScrollArea,
    QSlider,
    Qt,
    QTabWidget,
    QVBoxLayout,
    QWidget,
)

from app.ui.widgets import EducationalExplanationCard, MplCanvas, StatBadge


class SimulationsView(QWidget):
    """
    Simulation laboratory interface housing tabbed physics experiments.
    """

    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self._init_ui()

    def _init_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(24, 20, 24, 20)
        main_layout.setSpacing(16)

        # Header Title
        title_box = QVBoxLayout()
        title = QLabel("Physics Simulation Workbench")
        title.setStyleSheet("font-size: 22px; font-weight: 700; color: #ffffff;")
        title_box.addWidget(title)

        subtitle = QLabel(
            "Experiment with interactive physical models, manipulate boundary conditions in real time, and analyze scientific assumptions."
        )
        subtitle.setStyleSheet("color: #8b949e; font-size: 13px;")
        title_box.addWidget(subtitle)
        main_layout.addLayout(title_box)

        # Tabs for 4 Physics Models
        self.tabs = QTabWidget()
        self.tabs.addTab(ThermalExpansionTab(self.db), "📏 Thermal Expansion")
        self.tabs.addTab(HeatConductionTab(self.db), "🔥 Heat Conduction")
        self.tabs.addTab(StressStrainTab(self.db), "⚙️ Stress & Strain")
        self.tabs.addTab(HeatCapacityTab(self.db), "🌡️ Heat Capacity")

        main_layout.addWidget(self.tabs)

    def select_material_in_sim(self, material_id: str):
        """Set material across active simulation tabs."""
        current_widget = self.tabs.currentWidget()
        if hasattr(current_widget, "set_material"):
            current_widget.set_material(material_id)


# ============================================================
# TAB 1: THERMAL EXPANSION
# ============================================================


class ThermalExpansionTab(QWidget):
    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self._init_ui()

    def _init_ui(self):
        main_layout = QHBoxLayout(self)
        main_layout.setContentsMargins(12, 12, 12, 12)
        main_layout.setSpacing(16)

        # Left Controls & Educational Column
        left_scroll = QScrollArea()
        left_scroll.setWidgetResizable(True)
        left_scroll.setFrameShape(QFrame.NoFrame)
        left_widget = QWidget()
        left_layout = QVBoxLayout(left_widget)
        left_layout.setSpacing(14)

        # Material Selector
        mat_box = QFrame()
        mat_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        mat_layout = QVBoxLayout(mat_box)
        mat_lbl = QLabel("SELECT MATERIAL:")
        mat_lbl.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        mat_layout.addWidget(mat_lbl)

        self.material_combo = QComboBox()
        self._populate_materials()
        self.material_combo.currentIndexChanged.connect(self._recalculate)
        mat_layout.addWidget(self.material_combo)

        self.cte_display = QLabel("")
        self.cte_display.setStyleSheet("color: #8b949e; font-size: 12px;")
        mat_layout.addWidget(self.cte_display)
        left_layout.addWidget(mat_box)

        # Parameters Box
        param_box = QFrame()
        param_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        p_layout = QVBoxLayout(param_box)
        p_layout.setSpacing(12)

        p_title = QLabel("EXPERIMENT PARAMETERS:")
        p_title.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        p_layout.addWidget(p_title)

        # Initial Length (mm)
        l0_row = QHBoxLayout()
        l0_lbl = QLabel("Initial Length (L₀):")
        l0_lbl.setStyleSheet("font-weight: 600;")
        self.l0_spin = QDoubleSpinBox()
        self.l0_spin.setRange(1.0, 10000.0)
        self.l0_spin.setValue(1000.0)
        self.l0_spin.setSuffix(" mm")
        self.l0_spin.valueChanged.connect(self._recalculate)
        l0_row.addWidget(l0_lbl)
        l0_row.addStretch()
        l0_row.addWidget(self.l0_spin)
        p_layout.addLayout(l0_row)

        # Initial Temp (°C)
        t0_row = QHBoxLayout()
        t0_lbl = QLabel("Initial Temp (T₀):")
        t0_lbl.setStyleSheet("font-weight: 600;")
        self.t0_spin = QDoubleSpinBox()
        self.t0_spin.setRange(-270.0, 2000.0)
        self.t0_spin.setValue(20.0)
        self.t0_spin.setSuffix(" °C")
        self.t0_spin.valueChanged.connect(self._recalculate)
        t0_row.addWidget(t0_lbl)
        t0_row.addStretch()
        t0_row.addWidget(self.t0_spin)
        p_layout.addLayout(t0_row)

        # Final Temp (°C) Slider & Spinbox
        t1_lbl = QLabel("Final Temp (T_final):")
        t1_lbl.setStyleSheet("font-weight: 600;")
        p_layout.addWidget(t1_lbl)

        t1_row = QHBoxLayout()
        self.t1_slider = QSlider(Qt.Horizontal)
        self.t1_slider.setRange(-200, 1000)
        self.t1_slider.setValue(200)
        self.t1_slider.valueChanged.connect(self._on_slider_changed)

        self.t1_spin = QDoubleSpinBox()
        self.t1_spin.setRange(-273.15, 3000.0)
        self.t1_spin.setValue(200.0)
        self.t1_spin.setSuffix(" °C")
        self.t1_spin.valueChanged.connect(self._on_spin_changed)

        t1_row.addWidget(self.t1_slider, stretch=2)
        t1_row.addWidget(self.t1_spin, stretch=1)
        p_layout.addLayout(t1_row)

        left_layout.addWidget(param_box)

        # Educational Card
        edu = EducationalExplanationCard(te_model.MODEL_INFO)
        left_layout.addWidget(edu)
        left_layout.addStretch()

        left_scroll.setWidget(left_widget)
        main_layout.addWidget(left_scroll, stretch=4)

        # Right Results & Visualization Column
        right_widget = QWidget()
        right_layout = QVBoxLayout(right_widget)
        right_layout.setSpacing(12)

        # Metrics Row
        metrics_layout = QHBoxLayout()
        metrics_layout.setSpacing(10)
        self.badge_delta_t = StatBadge(
            "Temperature Delta (ΔT)", "+180.0 °C", "T_final - T_initial", "#58a6ff"
        )
        self.badge_delta_l = StatBadge(
            "Length Change (ΔL)", "+4.248 mm", "α · L₀ · ΔT", "#3fb950"
        )
        self.badge_final_l = StatBadge(
            "Final Length (L)", "1004.248 mm", "L₀ + ΔL", "#e6edf3"
        )
        self.badge_strain = StatBadge(
            "Thermal Strain (ε)", "+0.425 %", "ΔL / L₀", "#d29922"
        )

        metrics_layout.addWidget(self.badge_delta_t)
        metrics_layout.addWidget(self.badge_delta_l)
        metrics_layout.addWidget(self.badge_final_l)
        metrics_layout.addWidget(self.badge_strain)
        right_layout.addLayout(metrics_layout)

        # Canvas
        self.canvas = MplCanvas(self, width=6, height=4)
        right_layout.addWidget(self.canvas, stretch=1)

        main_layout.addWidget(right_widget, stretch=6)

        self._recalculate()

    def _populate_materials(self):
        materials = self.db.get_all_materials()
        for mat in materials:
            cte = mat.get_value("thermal_expansion")
            cte_str = f"{cte * 1e6:.1f} µm/(m·K)" if cte is not None else "No CTE"
            self.material_combo.addItem(f"{mat.name} ({cte_str})", mat.id)

    def set_material(self, material_id: str):
        idx = self.material_combo.findData(material_id)
        if idx >= 0:
            self.material_combo.setCurrentIndex(idx)

    def _on_slider_changed(self, val):
        self.t1_spin.blockSignals(True)
        self.t1_spin.setValue(float(val))
        self.t1_spin.blockSignals(False)
        self._recalculate()

    def _on_spin_changed(self, val):
        self.t1_slider.blockSignals(True)
        self.t1_slider.setValue(int(val))
        self.t1_slider.blockSignals(False)
        self._recalculate()

    def _recalculate(self):
        mat_id = self.material_combo.currentData()
        mat = self.db.get_material(mat_id)
        if not mat:
            return

        cte = mat.get_value("thermal_expansion")
        if cte is None:
            self.cte_display.setText(
                "⚠️ CTE unavailable for this material. Assuming 0.0."
            )
            cte = 0.0
        else:
            self.cte_display.setText(
                f"Coefficient of Thermal Expansion (α): <b>{cte * 1e6:.2f} µm/(m·K)</b>"
            )

        l0_m = self.l0_spin.value() / 1000.0  # mm to m
        t0_c = self.t0_spin.value()
        t1_c = self.t1_spin.value()

        t0_k = t0_c + 273.15
        t1_k = t1_c + 273.15

        try:
            res = te_model.calculate_expansion(
                initial_length=l0_m,
                alpha=cte,
                t_initial=t0_k,
                t_final=t1_k,
            )

            # Update Badges
            self.badge_delta_t.findChildren(QLabel)[1].setText(
                f"{res.delta_temperature:+.1f} °C"
            )
            delta_l_mm = res.length_change * 1000.0
            self.badge_delta_l.findChildren(QLabel)[1].setText(f"{delta_l_mm:+.3f} mm")
            final_l_mm = res.final_length * 1000.0
            self.badge_final_l.findChildren(QLabel)[1].setText(f"{final_l_mm:.3f} mm")
            self.badge_strain.findChildren(QLabel)[1].setText(
                f"{res.strain * 100:+.4f} %"
            )

            # Update Plot
            ax = self.canvas.axes
            ax.clear()

            t_min = min(t0_c, t1_c) - 50.0
            t_max = max(t0_c, t1_c) + 50.0
            temps_c = np.linspace(t_min, t_max, 100)
            lengths_mm = (l0_m * (1.0 + cte * (temps_c - t0_c))) * 1000.0

            ax.plot(
                temps_c,
                lengths_mm,
                color="#58a6ff",
                linewidth=2.5,
                label=f"{mat.name} (α={cte * 1e6:.1f} µm/m·K)",
            )
            ax.scatter(
                [t0_c],
                [l0_m * 1000.0],
                color="#8b949e",
                s=60,
                zorder=5,
                label="Initial State (T₀, L₀)",
            )
            ax.scatter(
                [t1_c],
                [final_l_mm],
                color="#3fb950" if delta_l_mm >= 0 else "#f85149",
                s=90,
                zorder=6,
                label="Current State (T_final, L)",
            )

            ax.set_title(
                "Length vs Temperature Response",
                color="#e6edf3",
                fontsize=12,
                fontweight="bold",
                pad=10,
            )
            ax.set_xlabel("Temperature (°C)", color="#8b949e", fontsize=10)
            ax.set_ylabel("Specimen Length (mm)", color="#8b949e", fontsize=10)
            ax.grid(True, linestyle="--", alpha=0.5, color="#21262d")
            ax.legend(loc="best", fontsize=9, framealpha=0.8)
            self.canvas.draw()

        except Exception as e:
            self.cte_display.setText(f"Calculation error: {e}")


# ============================================================
# TAB 2: HEAT CONDUCTION
# ============================================================


class HeatConductionTab(QWidget):
    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self._init_ui()

    def _init_ui(self):
        main_layout = QHBoxLayout(self)
        main_layout.setContentsMargins(12, 12, 12, 12)
        main_layout.setSpacing(16)

        left_scroll = QScrollArea()
        left_scroll.setWidgetResizable(True)
        left_scroll.setFrameShape(QFrame.NoFrame)
        left_widget = QWidget()
        left_layout = QVBoxLayout(left_widget)
        left_layout.setSpacing(14)

        # Material Selector
        mat_box = QFrame()
        mat_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        mat_layout = QVBoxLayout(mat_box)
        mat_lbl = QLabel("SELECT CONDUCTOR MATERIAL:")
        mat_lbl.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        mat_layout.addWidget(mat_lbl)

        self.material_combo = QComboBox()
        self._populate_materials()
        self.material_combo.currentIndexChanged.connect(self._recalculate)
        mat_layout.addWidget(self.material_combo)

        self.k_display = QLabel("")
        self.k_display.setStyleSheet("color: #8b949e; font-size: 12px;")
        mat_layout.addWidget(self.k_display)
        left_layout.addWidget(mat_box)

        # Geometry & Temperatures Box
        param_box = QFrame()
        param_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        p_layout = QVBoxLayout(param_box)
        p_layout.setSpacing(12)

        p_title = QLabel("BOUNDARY CONDITIONS & GEOMETRY:")
        p_title.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        p_layout.addWidget(p_title)

        # Area (m^2)
        area_row = QHBoxLayout()
        area_lbl = QLabel("Surface Area (A):")
        area_lbl.setStyleSheet("font-weight: 600;")
        self.area_spin = QDoubleSpinBox()
        self.area_spin.setRange(0.001, 100.0)
        self.area_spin.setValue(1.0)
        self.area_spin.setSuffix(" m²")
        self.area_spin.valueChanged.connect(self._recalculate)
        area_row.addWidget(area_lbl)
        area_row.addStretch()
        area_row.addWidget(self.area_spin)
        p_layout.addLayout(area_row)

        # Thickness (mm)
        thick_row = QHBoxLayout()
        thick_lbl = QLabel("Wall Thickness (L):")
        thick_lbl.setStyleSheet("font-weight: 600;")
        self.thick_spin = QDoubleSpinBox()
        self.thick_spin.setRange(0.1, 1000.0)
        self.thick_spin.setValue(10.0)
        self.thick_spin.setSuffix(" mm")
        self.thick_spin.valueChanged.connect(self._recalculate)
        thick_row.addWidget(thick_lbl)
        thick_row.addStretch()
        thick_row.addWidget(self.thick_spin)
        p_layout.addLayout(thick_row)

        # Hot Temp (°C)
        thot_row = QHBoxLayout()
        thot_lbl = QLabel("Hot Side Temp (T_hot):")
        thot_lbl.setStyleSheet("font-weight: 600;")
        self.thot_spin = QDoubleSpinBox()
        self.thot_spin.setRange(-200.0, 3000.0)
        self.thot_spin.setValue(150.0)
        self.thot_spin.setSuffix(" °C")
        self.thot_spin.valueChanged.connect(self._recalculate)
        thot_row.addWidget(thot_lbl)
        thot_row.addStretch()
        thot_row.addWidget(self.thot_spin)
        p_layout.addLayout(thot_row)

        # Cold Temp (°C)
        tcold_row = QHBoxLayout()
        tcold_lbl = QLabel("Cold Side Temp (T_cold):")
        tcold_lbl.setStyleSheet("font-weight: 600;")
        self.tcold_spin = QDoubleSpinBox()
        self.tcold_spin.setRange(-270.0, 3000.0)
        self.tcold_spin.setValue(25.0)
        self.tcold_spin.setSuffix(" °C")
        self.tcold_spin.valueChanged.connect(self._recalculate)
        tcold_row.addWidget(tcold_lbl)
        tcold_row.addStretch()
        tcold_row.addWidget(self.tcold_spin)
        p_layout.addLayout(tcold_row)

        left_layout.addWidget(param_box)

        # Educational Card
        edu = EducationalExplanationCard(hc_model.MODEL_INFO)
        left_layout.addWidget(edu)
        left_layout.addStretch()

        left_scroll.setWidget(left_widget)
        main_layout.addWidget(left_scroll, stretch=4)

        # Right Visualization Column
        right_widget = QWidget()
        right_layout = QVBoxLayout(right_widget)
        right_layout.setSpacing(12)

        # Metrics Badges
        metrics_layout = QHBoxLayout()
        metrics_layout.setSpacing(10)
        self.badge_qdot = StatBadge(
            "Heat Transfer Rate (Q̇)", "0 W", "k · A · ΔT / L", "#f85149"
        )
        self.badge_flux = StatBadge("Heat Flux (q″)", "0 W/m²", "Q̇ / A", "#ff7b72")
        self.badge_grad = StatBadge("Thermal Gradient", "0 °C/mm", "ΔT / L", "#d29922")
        self.badge_rth = StatBadge(
            "Thermal Resistance", "0 K/W", "L / (k · A)", "#58a6ff"
        )

        metrics_layout.addWidget(self.badge_qdot)
        metrics_layout.addWidget(self.badge_flux)
        metrics_layout.addWidget(self.badge_grad)
        metrics_layout.addWidget(self.badge_rth)
        right_layout.addLayout(metrics_layout)

        # Canvas
        self.canvas = MplCanvas(self, width=6, height=4)
        right_layout.addWidget(self.canvas, stretch=1)

        main_layout.addWidget(right_widget, stretch=6)

        self._recalculate()

    def _populate_materials(self):
        materials = self.db.get_all_materials()
        for mat in materials:
            k = mat.get_value("thermal_conductivity")
            k_str = f"{k:.1f} W/(m·K)" if k is not None else "No k"
            self.material_combo.addItem(f"{mat.name} ({k_str})", mat.id)

    def set_material(self, material_id: str):
        idx = self.material_combo.findData(material_id)
        if idx >= 0:
            self.material_combo.setCurrentIndex(idx)

    def _recalculate(self):
        mat_id = self.material_combo.currentData()
        mat = self.db.get_material(mat_id)
        if not mat:
            return

        k = mat.get_value("thermal_conductivity")
        if k is None:
            self.k_display.setText(
                "⚠️ Thermal conductivity unavailable for this material."
            )
            k = 1.0
        else:
            self.k_display.setText(f"Thermal Conductivity (k): <b>{k:.2f} W/(m·K)</b>")

        area = self.area_spin.value()
        thick_m = self.thick_spin.value() / 1000.0  # mm to m
        t_hot_c = self.thot_spin.value()
        t_cold_c = self.tcold_spin.value()

        t_hot_k = t_hot_c + 273.15
        t_cold_k = t_cold_c + 273.15

        try:
            res = hc_model.calculate_conduction(
                thermal_conductivity=k,
                area=area,
                thickness=thick_m,
                t_hot=t_hot_k,
                t_cold=t_cold_k,
            )

            # Format Q̇
            if abs(res.heat_rate) >= 1e6:
                q_str = f"{res.heat_rate / 1e6:.2f} MW"
            elif abs(res.heat_rate) >= 1e3:
                q_str = f"{res.heat_rate / 1e3:.2f} kW"
            else:
                q_str = f"{res.heat_rate:.1f} W"

            # Format Flux
            if abs(res.heat_flux) >= 1e6:
                flux_str = f"{res.heat_flux / 1e6:.2f} MW/m²"
            elif abs(res.heat_flux) >= 1e3:
                flux_str = f"{res.heat_flux / 1e3:.2f} kW/m²"
            else:
                flux_str = f"{res.heat_flux:.1f} W/m²"

            self.badge_qdot.findChildren(QLabel)[1].setText(q_str)
            self.badge_flux.findChildren(QLabel)[1].setText(flux_str)
            self.badge_grad.findChildren(QLabel)[1].setText(
                f"{res.thermal_gradient / 1000.0:.2f} °C/mm"
            )
            self.badge_rth.findChildren(QLabel)[1].setText(
                f"{res.thermal_resistance:.4e} K/W"
            )

            # Plot
            ax = self.canvas.axes
            ax.clear()

            x_mm = np.linspace(0, self.thick_spin.value(), 100)
            t_profile_c = t_hot_c - (t_hot_c - t_cold_c) * (
                x_mm / self.thick_spin.value()
            )

            ax.plot(
                x_mm,
                t_profile_c,
                color="#ff7b72",
                linewidth=3.0,
                label=f"T(x) Gradient ({mat.name})",
            )
            ax.scatter(
                [0],
                [t_hot_c],
                color="#f85149",
                s=80,
                zorder=5,
                label=f"Hot Boundary ({t_hot_c}°C)",
            )
            ax.scatter(
                [self.thick_spin.value()],
                [t_cold_c],
                color="#58a6ff",
                s=80,
                zorder=5,
                label=f"Cold Boundary ({t_cold_c}°C)",
            )

            ax.set_title(
                "1D Steady-State Temperature Profile T(x)",
                color="#e6edf3",
                fontsize=12,
                fontweight="bold",
                pad=10,
            )
            ax.set_xlabel(
                "Depth into Material Slab x (mm)", color="#8b949e", fontsize=10
            )
            ax.set_ylabel("Temperature (°C)", color="#8b949e", fontsize=10)
            ax.grid(True, linestyle="--", alpha=0.5, color="#21262d")
            ax.legend(loc="best", fontsize=9, framealpha=0.8)
            self.canvas.draw()

        except Exception as e:
            self.k_display.setText(f"Calculation error: {e}")


# ============================================================
# TAB 3: STRESS & STRAIN
# ============================================================


class StressStrainTab(QWidget):
    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self._init_ui()

    def _init_ui(self):
        main_layout = QHBoxLayout(self)
        main_layout.setContentsMargins(12, 12, 12, 12)
        main_layout.setSpacing(16)

        left_scroll = QScrollArea()
        left_scroll.setWidgetResizable(True)
        left_scroll.setFrameShape(QFrame.NoFrame)
        left_widget = QWidget()
        left_layout = QVBoxLayout(left_widget)
        left_layout.setSpacing(14)

        # Material Selector
        mat_box = QFrame()
        mat_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        mat_layout = QVBoxLayout(mat_box)
        mat_lbl = QLabel("SELECT STRUCTURAL MATERIAL:")
        mat_lbl.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        mat_layout.addWidget(mat_lbl)

        self.material_combo = QComboBox()
        self._populate_materials()
        self.material_combo.currentIndexChanged.connect(self._recalculate)
        mat_layout.addWidget(self.material_combo)

        self.props_display = QLabel("")
        self.props_display.setStyleSheet("color: #8b949e; font-size: 12px;")
        mat_layout.addWidget(self.props_display)
        left_layout.addWidget(mat_box)

        # Load & Geometry Parameters
        param_box = QFrame()
        param_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        p_layout = QVBoxLayout(param_box)
        p_layout.setSpacing(12)

        p_title = QLabel("APPLIED LOAD & SECTION GEOMETRY:")
        p_title.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        p_layout.addWidget(p_title)

        # Force (kN) Slider & Spinbox
        f_lbl = QLabel("Applied Axial Load (Force F):")
        f_lbl.setStyleSheet("font-weight: 600;")
        p_layout.addWidget(f_lbl)

        f_row = QHBoxLayout()
        self.f_slider = QSlider(Qt.Horizontal)
        self.f_slider.setRange(1, 500)
        self.f_slider.setValue(25)
        self.f_slider.valueChanged.connect(self._on_slider_changed)

        self.f_spin = QDoubleSpinBox()
        self.f_spin.setRange(0.01, 50000.0)
        self.f_spin.setValue(25.0)
        self.f_spin.setSuffix(" kN")
        self.f_spin.valueChanged.connect(self._on_spin_changed)

        f_row.addWidget(self.f_slider, stretch=2)
        f_row.addWidget(self.f_spin, stretch=1)
        p_layout.addLayout(f_row)

        # Area (mm^2)
        area_row = QHBoxLayout()
        area_lbl = QLabel("Cross-Section Area (A):")
        area_lbl.setStyleSheet("font-weight: 600;")
        self.area_spin = QDoubleSpinBox()
        self.area_spin.setRange(1.0, 100000.0)
        self.area_spin.setValue(100.0)
        self.area_spin.setSuffix(" mm²")
        self.area_spin.valueChanged.connect(self._recalculate)
        area_row.addWidget(area_lbl)
        area_row.addStretch()
        area_row.addWidget(self.area_spin)
        p_layout.addLayout(area_row)

        # Length (mm)
        l0_row = QHBoxLayout()
        l0_lbl = QLabel("Initial Gauge Length (L₀):")
        l0_lbl.setStyleSheet("font-weight: 600;")
        self.l0_spin = QDoubleSpinBox()
        self.l0_spin.setRange(10.0, 10000.0)
        self.l0_spin.setValue(1000.0)
        self.l0_spin.setSuffix(" mm")
        self.l0_spin.valueChanged.connect(self._recalculate)
        l0_row.addWidget(l0_lbl)
        l0_row.addStretch()
        l0_row.addWidget(self.l0_spin)
        p_layout.addLayout(l0_row)

        left_layout.addWidget(param_box)

        # Educational Card
        edu = EducationalExplanationCard(ss_model.MODEL_INFO)
        left_layout.addWidget(edu)
        left_layout.addStretch()

        left_scroll.setWidget(left_widget)
        main_layout.addWidget(left_scroll, stretch=4)

        # Right Visualization Column
        right_widget = QWidget()
        right_layout = QVBoxLayout(right_widget)
        right_layout.setSpacing(12)

        # Metrics Badges
        metrics_layout = QHBoxLayout()
        metrics_layout.setSpacing(10)
        self.badge_stress = StatBadge("Applied Stress (σ)", "0 MPa", "F / A", "#e6edf3")
        self.badge_strain = StatBadge("Elastic Strain (ε)", "0 %", "σ / E", "#58a6ff")
        self.badge_def = StatBadge("Deformation (ΔL)", "0 mm", "ε · L₀", "#3fb950")
        self.badge_fos = StatBadge(
            "Factor of Safety (FoS)", "—", "σ_yield / σ", "#d29922"
        )

        metrics_layout.addWidget(self.badge_stress)
        metrics_layout.addWidget(self.badge_strain)
        metrics_layout.addWidget(self.badge_def)
        metrics_layout.addWidget(self.badge_fos)
        right_layout.addLayout(metrics_layout)

        # Yield Status Banner
        self.status_banner = QFrame()
        self.status_banner.setStyleSheet("border-radius: 6px; padding: 10px;")
        sb_layout = QVBoxLayout(self.status_banner)
        self.status_text = QLabel("")
        self.status_text.setWordWrap(True)
        self.status_text.setStyleSheet("font-size: 13px; font-weight: 600;")
        sb_layout.addWidget(self.status_text)
        right_layout.addWidget(self.status_banner)

        # Canvas
        self.canvas = MplCanvas(self, width=6, height=4)
        right_layout.addWidget(self.canvas, stretch=1)

        main_layout.addWidget(right_widget, stretch=6)

        self._recalculate()

    def _populate_materials(self):
        materials = self.db.get_all_materials()
        for mat in materials:
            e = mat.get_value("youngs_modulus")
            e_str = f"{e / 1e9:.1f} GPa" if e is not None else "No E"
            self.material_combo.addItem(f"{mat.name} ({e_str})", mat.id)

    def set_material(self, material_id: str):
        idx = self.material_combo.findData(material_id)
        if idx >= 0:
            self.material_combo.setCurrentIndex(idx)

    def _on_slider_changed(self, val):
        self.f_spin.blockSignals(True)
        self.f_spin.setValue(float(val))
        self.f_spin.blockSignals(False)
        self._recalculate()

    def _on_spin_changed(self, val):
        self.f_slider.blockSignals(True)
        self.f_slider.setValue(int(val))
        self.f_slider.blockSignals(False)
        self._recalculate()

    def _recalculate(self):
        mat_id = self.material_combo.currentData()
        mat = self.db.get_material(mat_id)
        if not mat:
            return

        e = mat.get_value("youngs_modulus")
        sy = mat.get_value("yield_strength")

        if e is None:
            self.props_display.setText(
                "⚠️ Young's modulus unavailable. Defaulting to 100 GPa."
            )
            e = 100e9
        else:
            sy_str = f"{sy / 1e6:.1f} MPa" if sy is not None else "Unavailable"
            self.props_display.setText(
                f"Stiffness (E): <b>{e / 1e9:.1f} GPa</b> | Yield Strength (σ_y): <b>{sy_str}</b>"
            )

        force_n = self.f_spin.value() * 1000.0  # kN to N
        area_m2 = self.area_spin.value() * 1e-6  # mm^2 to m^2
        l0_m = self.l0_spin.value() / 1000.0  # mm to m

        try:
            res = ss_model.calculate_stress_strain(
                force=force_n,
                area=area_m2,
                youngs_modulus=e,
                initial_length=l0_m,
                yield_strength=sy,
            )

            # Update Badges
            self.badge_stress.findChildren(QLabel)[1].setText(
                f"{res.stress / 1e6:.1f} MPa"
            )
            self.badge_strain.findChildren(QLabel)[1].setText(
                f"{res.strain * 100:.4f} %"
            )
            self.badge_def.findChildren(QLabel)[1].setText(
                f"{res.deformation * 1000.0:.3f} mm"
            )

            if res.factor_of_safety is not None and not np.isinf(res.factor_of_safety):
                self.badge_fos.findChildren(QLabel)[1].setText(
                    f"{res.factor_of_safety:.2f}"
                )
            else:
                self.badge_fos.findChildren(QLabel)[1].setText("—")

            # Status Banner
            if res.yield_status == ss_model.YieldStatus.YIELDING_EXCEEDED:
                self.status_banner.setStyleSheet(
                    "background-color: #3d1418; border: 1px solid #f85149; border-radius: 6px;"
                )
                self.status_text.setText(f"⚠️ {res.status_message}")
                self.status_text.setStyleSheet("color: #ff7b72; font-weight: 600;")
            elif res.yield_status == ss_model.YieldStatus.NEAR_YIELD:
                self.status_banner.setStyleSheet(
                    "background-color: #3b2300; border: 1px solid #d29922; border-radius: 6px;"
                )
                self.status_text.setText(f"⚠️ {res.status_message}")
                self.status_text.setStyleSheet("color: #e3b341; font-weight: 600;")
            elif res.yield_status == ss_model.YieldStatus.ELASTIC_SAFE:
                self.status_banner.setStyleSheet(
                    "background-color: #0e2d1d; border: 1px solid #238636; border-radius: 6px;"
                )
                self.status_text.setText(f"✓ {res.status_message}")
                self.status_text.setStyleSheet("color: #7ee787; font-weight: 600;")
            else:
                self.status_banner.setStyleSheet(
                    "background-color: #161b22; border: 1px solid #30363d; border-radius: 6px;"
                )
                self.status_text.setText(f"ℹ️ {res.status_message}")
                self.status_text.setStyleSheet("color: #8b949e; font-weight: 500;")

            # Plot Hookean curve
            ax = self.canvas.axes
            ax.clear()

            max_strain_plot = max(0.01, res.strain * 1.5)
            strains = np.linspace(0, max_strain_plot, 100)
            stresses_mpa = (e * strains) / 1e6

            ax.plot(
                strains * 100.0,
                stresses_mpa,
                color="#58a6ff",
                linewidth=2.5,
                label=f"Hookean Response (E={e / 1e9:.1f} GPa)",
            )

            if sy is not None:
                ax.axhline(
                    y=sy / 1e6,
                    color="#d29922",
                    linestyle="--",
                    linewidth=1.8,
                    label=f"Yield Limit σ_y ({sy / 1e6:.0f} MPa)",
                )

            ax.scatter(
                [res.strain * 100.0],
                [res.stress / 1e6],
                color="#3fb950"
                if res.yield_status != ss_model.YieldStatus.YIELDING_EXCEEDED
                else "#f85149",
                s=90,
                zorder=6,
                label=f"Current Load ({res.stress / 1e6:.1f} MPa)",
            )

            ax.set_title(
                "Engineering Stress vs Strain (Hooke's Law)",
                color="#e6edf3",
                fontsize=12,
                fontweight="bold",
                pad=10,
            )
            ax.set_xlabel("Engineering Strain ε (%)", color="#8b949e", fontsize=10)
            ax.set_ylabel("Applied Stress σ (MPa)", color="#8b949e", fontsize=10)
            ax.grid(True, linestyle="--", alpha=0.5, color="#21262d")
            ax.legend(loc="best", fontsize=9, framealpha=0.8)
            self.canvas.draw()

        except Exception as ex:
            self.props_display.setText(f"Calculation error: {ex}")


# ============================================================
# TAB 4: HEAT CAPACITY
# ============================================================


class HeatCapacityTab(QWidget):
    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self._init_ui()

    def _init_ui(self):
        main_layout = QHBoxLayout(self)
        main_layout.setContentsMargins(12, 12, 12, 12)
        main_layout.setSpacing(16)

        left_scroll = QScrollArea()
        left_scroll.setWidgetResizable(True)
        left_scroll.setFrameShape(QFrame.NoFrame)
        left_widget = QWidget()
        left_layout = QVBoxLayout(left_widget)
        left_layout.setSpacing(14)

        # Material Selector
        mat_box = QFrame()
        mat_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        mat_layout = QVBoxLayout(mat_box)
        mat_lbl = QLabel("SELECT THERMAL STORAGE MATERIAL:")
        mat_lbl.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        mat_layout.addWidget(mat_lbl)

        self.material_combo = QComboBox()
        self._populate_materials()
        self.material_combo.currentIndexChanged.connect(self._recalculate)
        mat_layout.addWidget(self.material_combo)

        self.cp_display = QLabel("")
        self.cp_display.setStyleSheet("color: #8b949e; font-size: 12px;")
        mat_layout.addWidget(self.cp_display)
        left_layout.addWidget(mat_box)

        # Parameters Box
        param_box = QFrame()
        param_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 8px; padding: 12px;"
        )
        p_layout = QVBoxLayout(param_box)
        p_layout.setSpacing(12)

        p_title = QLabel("SYSTEM MASS & TEMPERATURE CHANGE:")
        p_title.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        p_layout.addWidget(p_title)

        # Mass (kg)
        mass_row = QHBoxLayout()
        mass_lbl = QLabel("Component Mass (m):")
        mass_lbl.setStyleSheet("font-weight: 600;")
        self.mass_spin = QDoubleSpinBox()
        self.mass_spin.setRange(0.01, 10000.0)
        self.mass_spin.setValue(5.0)
        self.mass_spin.setSuffix(" kg")
        self.mass_spin.valueChanged.connect(self._recalculate)
        mass_row.addWidget(mass_lbl)
        mass_row.addStretch()
        mass_row.addWidget(self.mass_spin)
        p_layout.addLayout(mass_row)

        # Initial Temp (°C)
        t0_row = QHBoxLayout()
        t0_lbl = QLabel("Starting Temp (T_start):")
        t0_lbl.setStyleSheet("font-weight: 600;")
        self.t0_spin = QDoubleSpinBox()
        self.t0_spin.setRange(-270.0, 3000.0)
        self.t0_spin.setValue(20.0)
        self.t0_spin.setSuffix(" °C")
        self.t0_spin.valueChanged.connect(self._recalculate)
        t0_row.addWidget(t0_lbl)
        t0_row.addStretch()
        t0_row.addWidget(self.t0_spin)
        p_layout.addLayout(t0_row)

        # Target Temp (°C) Slider & Spinbox
        t1_lbl = QLabel("Target Temp (T_target):")
        t1_lbl.setStyleSheet("font-weight: 600;")
        p_layout.addWidget(t1_lbl)

        t1_row = QHBoxLayout()
        self.t1_slider = QSlider(Qt.Horizontal)
        self.t1_slider.setRange(-200, 1000)
        self.t1_slider.setValue(120)
        self.t1_slider.valueChanged.connect(self._on_slider_changed)

        self.t1_spin = QDoubleSpinBox()
        self.t1_spin.setRange(-273.15, 3000.0)
        self.t1_spin.setValue(120.0)
        self.t1_spin.setSuffix(" °C")
        self.t1_spin.valueChanged.connect(self._on_spin_changed)

        t1_row.addWidget(self.t1_slider, stretch=2)
        t1_row.addWidget(self.t1_spin, stretch=1)
        p_layout.addLayout(t1_row)

        left_layout.addWidget(param_box)

        # Educational Card
        edu = EducationalExplanationCard(hcap_model.MODEL_INFO)
        left_layout.addWidget(edu)
        left_layout.addStretch()

        left_scroll.setWidget(left_widget)
        main_layout.addWidget(left_scroll, stretch=4)

        # Right Visualization Column
        right_widget = QWidget()
        right_layout = QVBoxLayout(right_widget)
        right_layout.setSpacing(12)

        # Metrics Badges
        metrics_layout = QHBoxLayout()
        metrics_layout.setSpacing(10)
        self.badge_energy = StatBadge(
            "Thermal Energy (Q)", "0 kJ", "m · c · ΔT", "#f85149"
        )
        self.badge_deltat = StatBadge(
            "Temperature Rise (ΔT)", "0 °C", "T_target - T_start", "#58a6ff"
        )
        self.badge_thmass = StatBadge(
            "Thermal Mass (m·c)", "0 J/K", "Heat Capacity", "#3fb950"
        )
        self.badge_cp = StatBadge(
            "Specific Heat (c)", "0 J/(kg·K)", "Intrinsic Property", "#bc8cff"
        )

        metrics_layout.addWidget(self.badge_energy)
        metrics_layout.addWidget(self.badge_deltat)
        metrics_layout.addWidget(self.badge_thmass)
        metrics_layout.addWidget(self.badge_cp)
        right_layout.addLayout(metrics_layout)

        # Canvas
        self.canvas = MplCanvas(self, width=6, height=4)
        right_layout.addWidget(self.canvas, stretch=1)

        main_layout.addWidget(right_widget, stretch=6)

        self._recalculate()

    def _populate_materials(self):
        materials = self.db.get_all_materials()
        for mat in materials:
            cp = mat.get_value("specific_heat")
            cp_str = f"{cp:,.0f} J/(kg·K)" if cp is not None else "No c_p"
            self.material_combo.addItem(f"{mat.name} ({cp_str})", mat.id)

    def set_material(self, material_id: str):
        idx = self.material_combo.findData(material_id)
        if idx >= 0:
            self.material_combo.setCurrentIndex(idx)

    def _on_slider_changed(self, val):
        self.t1_spin.blockSignals(True)
        self.t1_spin.setValue(float(val))
        self.t1_spin.blockSignals(False)
        self._recalculate()

    def _on_spin_changed(self, val):
        self.t1_slider.blockSignals(True)
        self.t1_slider.setValue(int(val))
        self.t1_slider.blockSignals(False)
        self._recalculate()

    def _recalculate(self):
        mat_id = self.material_combo.currentData()
        mat = self.db.get_material(mat_id)
        if not mat:
            return

        cp = mat.get_value("specific_heat")
        if cp is None:
            self.cp_display.setText(
                "⚠️ Specific heat unavailable. Assuming 900 J/(kg·K)."
            )
            cp = 900.0
        else:
            self.cp_display.setText(
                f"Specific Heat Capacity (c): <b>{cp:,.0f} J/(kg·K)</b>"
            )

        mass = self.mass_spin.value()
        t0_c = self.t0_spin.value()
        t1_c = self.t1_spin.value()

        t0_k = t0_c + 273.15
        t1_k = t1_c + 273.15

        try:
            res = hcap_model.calculate_heat_energy(
                mass=mass,
                specific_heat=cp,
                t_initial=t0_k,
                t_final=t1_k,
            )

            # Format Energy
            if abs(res.energy) >= 1e6:
                e_str = f"{res.energy / 1e6:+.2f} MJ"
            elif abs(res.energy) >= 1e3:
                e_str = f"{res.energy / 1e3:+.2f} kJ"
            else:
                e_str = f"{res.energy:+.1f} J"

            self.badge_energy.findChildren(QLabel)[1].setText(e_str)
            self.badge_deltat.findChildren(QLabel)[1].setText(
                f"{res.delta_temperature:+.1f} °C"
            )
            self.badge_thmass.findChildren(QLabel)[1].setText(
                f"{res.thermal_mass:,.0f} J/K"
            )
            self.badge_cp.findChildren(QLabel)[1].setText(f"{cp:,.0f} J/(kg·K)")

            # Plot
            ax = self.canvas.axes
            ax.clear()

            t_min = min(t0_c, t1_c) - 20.0
            t_max = max(t0_c, t1_c) + 20.0
            temps_c = np.linspace(t_min, t_max, 100)
            energies_kj = (mass * cp * (temps_c - t0_c)) / 1000.0

            ax.plot(
                temps_c,
                energies_kj,
                color="#bc8cff",
                linewidth=2.5,
                label=f"{mat.name} (m={mass:.1f} kg)",
            )
            ax.scatter(
                [t0_c],
                [0.0],
                color="#8b949e",
                s=60,
                zorder=5,
                label=f"Start ({t0_c}°C, 0 kJ)",
            )
            ax.scatter(
                [t1_c],
                [res.energy / 1000.0],
                color="#f85149" if res.energy >= 0 else "#58a6ff",
                s=90,
                zorder=6,
                label=f"Target ({t1_c}°C, {res.energy / 1000.0:+.1f} kJ)",
            )

            ax.set_title(
                "Sensible Heat Energy Storage vs Temperature",
                color="#e6edf3",
                fontsize=12,
                fontweight="bold",
                pad=10,
            )
            ax.set_xlabel("Target Temperature (°C)", color="#8b949e", fontsize=10)
            ax.set_ylabel(
                "Thermal Energy Required Q (kJ)", color="#8b949e", fontsize=10
            )
            ax.grid(True, linestyle="--", alpha=0.5, color="#21262d")
            ax.legend(loc="best", fontsize=9, framealpha=0.8)
            self.canvas.draw()

        except Exception as ex:
            self.cp_display.setText(f"Calculation error: {ex}")
