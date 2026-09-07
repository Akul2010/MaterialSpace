"""
Reusable custom widgets for MaterialSpace desktop interface.
"""

from __future__ import annotations

import matplotlib
from matplotlib.backends.backend_qtagg import FigureCanvasQTAgg
from matplotlib.figure import Figure

from app.ui.qt import (
    QFrame,
    QHBoxLayout,
    QLabel,
    QVBoxLayout,
)
from app.ui.theme import MATPLOTLIB_DARK_STYLE


class MplCanvas(FigureCanvasQTAgg):
    """Matplotlib Figure Canvas configured for dark theme."""

    def __init__(self, parent=None, width=5, height=4, dpi=100):
        matplotlib.rcParams.update(MATPLOTLIB_DARK_STYLE)
        self.fig = Figure(figsize=(width, height), dpi=dpi, facecolor="#161b22")
        self.axes = self.fig.add_subplot(111, facecolor="#0d1117")
        self.fig.tight_layout(pad=2.5)
        super().__init__(self.fig)
        self.setParent(parent)


class StatBadge(QFrame):
    """Small card showing a metric label and large value."""

    def __init__(
        self,
        label: str,
        value: str,
        subtext: str = "",
        accent_color: str = "#58a6ff",
        parent=None,
    ):
        super().__init__(parent)
        self.setObjectName("stat_badge")
        self.setStyleSheet("""
            QFrame#stat_badge {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 12px;
            }
        """)
        layout = QVBoxLayout(self)
        layout.setContentsMargins(12, 10, 12, 10)
        layout.setSpacing(4)

        lbl = QLabel(label.upper())
        lbl.setStyleSheet(
            "color: #8b949e; font-size: 11px; font-weight: 700; letter-spacing: 0.5px;"
        )
        layout.addWidget(lbl)

        val_lbl = QLabel(value)
        val_lbl.setStyleSheet(
            f"color: {accent_color}; font-size: 20px; font-weight: 700;"
        )
        layout.addWidget(val_lbl)

        if subtext:
            sub_lbl = QLabel(subtext)
            sub_lbl.setStyleSheet("color: #8b949e; font-size: 11px;")
            layout.addWidget(sub_lbl)


class PropertyCard(QFrame):
    """Card displaying a physical property name, formatted value, condition, and source attribution."""

    def __init__(
        self,
        prop_label: str,
        value_str: str,
        unit_str: str = "",
        condition: str | None = None,
        source_citation: str | None = None,
        is_available: bool = True,
        parent=None,
    ):
        super().__init__(parent)
        self.setObjectName("prop_card")
        border_color = "#30363d" if is_available else "#21262d"
        bg_color = "#161b22" if is_available else "#0d1117"

        self.setStyleSheet(f"""
            QFrame#prop_card {{
                background-color: {bg_color};
                border: 1px solid {border_color};
                border-radius: 6px;
                padding: 10px;
            }}
        """)
        layout = QVBoxLayout(self)
        layout.setContentsMargins(10, 8, 10, 8)
        layout.setSpacing(4)

        header_layout = QHBoxLayout()
        name_lbl = QLabel(prop_label)
        name_lbl.setStyleSheet("color: #8b949e; font-size: 12px; font-weight: 600;")
        header_layout.addWidget(name_lbl)
        header_layout.addStretch()

        if condition:
            cond_lbl = QLabel(f"[{condition}]")
            cond_lbl.setStyleSheet(
                "color: #6e7681; font-size: 10px; font-style: italic;"
            )
            header_layout.addWidget(cond_lbl)

        layout.addLayout(header_layout)

        if is_available:
            val_lbl = QLabel(value_str)
            val_lbl.setStyleSheet("color: #e6edf3; font-size: 16px; font-weight: 700;")
            layout.addWidget(val_lbl)
        else:
            val_lbl = QLabel("Data unavailable")
            val_lbl.setStyleSheet(
                "color: #6e7681; font-size: 13px; font-style: italic;"
            )
            layout.addWidget(val_lbl)

        if source_citation and is_available:
            src_lbl = QLabel(f"Source: {source_citation}")
            src_lbl.setStyleSheet("color: #58a6ff; font-size: 10px;")
            src_lbl.setWordWrap(True)
            layout.addWidget(src_lbl)


class EducationalExplanationCard(QFrame):
    """Panel explaining equation, variables, assumptions, and limitations for a physics simulation."""

    def __init__(self, model_info: dict, parent=None):
        super().__init__(parent)
        self.setObjectName("edu_card")
        self.setStyleSheet("""
            QFrame#edu_card {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 16px;
            }
        """)
        layout = QVBoxLayout(self)
        layout.setContentsMargins(16, 14, 16, 14)
        layout.setSpacing(12)

        # Title & Equation
        title = QLabel(model_info.get("name", "Physics Model"))
        title.setStyleSheet("color: #58a6ff; font-size: 16px; font-weight: 700;")
        layout.addWidget(title)

        eq_frame = QFrame()
        eq_frame.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 6px; padding: 10px;"
        )
        eq_layout = QVBoxLayout(eq_frame)
        eq_layout.setContentsMargins(10, 8, 10, 8)
        eq_layout.setSpacing(4)

        eq_title = QLabel("Governing Equation:")
        eq_title.setStyleSheet(
            "color: #8b949e; font-size: 11px; font-weight: 700; text-transform: uppercase;"
        )
        eq_layout.addWidget(eq_title)

        eq_text = QLabel(model_info.get("equation", ""))
        eq_text.setStyleSheet(
            "color: #7ee787; font-family: monospace; font-size: 15px; font-weight: 700;"
        )
        eq_layout.addWidget(eq_text)
        layout.addWidget(eq_frame)

        # Variables breakdown
        var_box = QFrame()
        var_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #21262d; border-radius: 6px; padding: 8px;"
        )
        var_layout = QVBoxLayout(var_box)
        var_layout.setSpacing(4)

        var_title = QLabel("Variables & Symbols:")
        var_title.setStyleSheet(
            "color: #8b949e; font-size: 11px; font-weight: 700; text-transform: uppercase;"
        )
        var_layout.addWidget(var_title)

        for sym, desc in model_info.get("variables", {}).items():
            row = QLabel(f"<b>{sym}</b>: {desc}")
            row.setStyleSheet("color: #c9d1d9; font-size: 12px;")
            row.setWordWrap(True)
            var_layout.addWidget(row)
        layout.addWidget(var_box)

        # Assumptions
        assumptions = model_info.get("assumptions", [])
        if assumptions:
            ass_title = QLabel("Scientific Assumptions:")
            ass_title.setStyleSheet(
                "color: #d29922; font-size: 12px; font-weight: 700;"
            )
            layout.addWidget(ass_title)
            for a in assumptions:
                lbl = QLabel(f"• {a}")
                lbl.setStyleSheet("color: #8b949e; font-size: 12px;")
                lbl.setWordWrap(True)
                layout.addWidget(lbl)

        # Limitations
        limitations = model_info.get("limitations", [])
        if limitations:
            lim_title = QLabel("Model Limitations:")
            lim_title.setStyleSheet(
                "color: #f85149; font-size: 12px; font-weight: 700;"
            )
            layout.addWidget(lim_title)
            for lim in limitations:
                lbl = QLabel(f"• {lim}")
                lbl.setStyleSheet("color: #8b949e; font-size: 12px;")
                lbl.setWordWrap(True)
                layout.addWidget(lbl)
