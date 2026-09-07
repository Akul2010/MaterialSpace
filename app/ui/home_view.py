"""
MaterialSpace Home Dashboard View.
"""

from __future__ import annotations

from app.data.database import MaterialDatabase
from app.ui.qt import (
    QFrame,
    QGridLayout,
    QHBoxLayout,
    QLabel,
    QPushButton,
    QScrollArea,
    QVBoxLayout,
    QWidget,
    Signal,
)
from app.ui.widgets import StatBadge


class HomeView(QWidget):
    """
    Landing dashboard view introducing MaterialSpace and providing quick actions.
    """

    navigate_to = Signal(str)  # "explore", "simulations", "compare", "challenges"
    select_material = Signal(str)  # material_id

    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self._init_ui()

    def _init_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(0, 0, 0, 0)

        scroll = QScrollArea()
        scroll.setWidgetResizable(True)
        scroll.setFrameShape(QFrame.NoFrame)
        scroll_content = QWidget()
        layout = QVBoxLayout(scroll_content)
        layout.setContentsMargins(32, 28, 32, 28)
        layout.setSpacing(24)

        # Hero Banner Frame
        hero = QFrame()
        hero.setStyleSheet("""
            QFrame {
                background: qlineargradient(x1:0, y1:0, x2:1, y2:1, stop:0 #161b22, stop:1 #0d1117);
                border: 1px solid #30363d;
                border-radius: 12px;
                padding: 24px;
            }
        """)
        hero_layout = QVBoxLayout(hero)
        hero_layout.setSpacing(10)

        badge_row = QHBoxLayout()
        tag = QLabel("INTERACTIVE MATERIALS-ENGINEERING SANDBOX")
        tag.setStyleSheet(
            "color: #58a6ff; font-size: 11px; font-weight: 700; letter-spacing: 1px;"
        )
        badge_row.addWidget(tag)
        badge_row.addStretch()
        hero_layout.addLayout(badge_row)

        title = QLabel("MaterialSpace")
        title.setStyleSheet("font-size: 32px; font-weight: 800; color: #ffffff;")
        hero_layout.addWidget(title)

        motto = QLabel(
            "Experiment with materials. Understand the physics. Make engineering decisions."
        )
        motto.setStyleSheet("font-size: 16px; color: #79c0ff; font-weight: 500;")
        hero_layout.addWidget(motto)

        desc = QLabel(
            "MaterialSpace is a scientific desktop laboratory for high-school students. "
            "Instead of simply reading textbook numbers, investigate deterministic physics models, "
            "visualize real material property limits, compare structural candidates, and make requirement-driven engineering trade-offs."
        )
        desc.setStyleSheet("color: #8b949e; font-size: 13px; line-height: 1.4;")
        desc.setWordWrap(True)
        hero_layout.addWidget(desc)

        layout.addWidget(hero)

        # Quick Statistics Grid
        stats_layout = QHBoxLayout()
        stats_layout.setSpacing(12)

        stats_layout.addWidget(
            StatBadge(
                "Database Materials",
                f"{self.db.count()}",
                "Curated from NASA, NIST & ASM",
                "#58a6ff",
            )
        )
        stats_layout.addWidget(
            StatBadge(
                "Physics Simulators",
                "4 Models",
                "Thermal, Conduction, Stress, Heat Cap",
                "#3fb950",
            )
        )
        stats_layout.addWidget(
            StatBadge(
                "Engineering Challenges",
                "3 Scenarios",
                "Transparent Weighted Decision Engine",
                "#d29922",
            )
        )
        stats_layout.addWidget(
            StatBadge(
                "Scientific Validation",
                "100% SI",
                "No Hallucinated / Black-Box Data",
                "#bc8cff",
            )
        )

        layout.addLayout(stats_layout)

        # Core Navigation Feature Cards
        features_title = QLabel("Explore Lab Modules")
        features_title.setStyleSheet(
            "font-size: 18px; font-weight: 700; color: #e6edf3;"
        )
        layout.addWidget(features_title)

        grid = QGridLayout()
        grid.setSpacing(16)

        # Module 1: Material Explorer
        c1 = self._create_feature_card(
            icon="🔍",
            title="Material Explorer",
            desc="Search and filter through 55+ authenticated engineering materials across metals, ceramics, polymers, composites, and glasses with full source citations.",
            btn_text="Browse Materials",
            btn_action=lambda: self.navigate_to.emit("explore"),
            accent="#58a6ff",
        )
        grid.addWidget(c1, 0, 0)

        # Module 2: Physics Simulations
        c2 = self._create_feature_card(
            icon="⚡",
            title="Physics Experiments",
            desc="Run interactive simulations with real-time sliders: Thermal Expansion, 1D Heat Conduction, Hookean Stress/Strain with Yield Checks, and Heat Capacity.",
            btn_text="Launch Experiments",
            btn_action=lambda: self.navigate_to.emit("simulations"),
            accent="#3fb950",
        )
        grid.addWidget(c2, 0, 1)

        # Module 3: Material Comparison
        c3 = self._create_feature_card(
            icon="⚖️",
            title="Material Comparison",
            desc="Select 2 to 4 candidate materials side-by-side to cross-compare densities, stiffness, strengths, and thermal properties with visual comparative charts.",
            btn_text="Compare Candidates",
            btn_action=lambda: self.navigate_to.emit("compare"),
            accent="#bc8cff",
        )
        grid.addWidget(c3, 1, 0)

        # Module 4: Engineering Decision Challenges
        c4 = self._create_feature_card(
            icon="🎯",
            title="Engineering Challenges",
            desc="Solve real aerospace challenges (Lunar Rover chassis, Re-entry Heat Shield, Radiator panel) using an auditable, transparent weighted scoring engine.",
            btn_text="Solve Challenges",
            btn_action=lambda: self.navigate_to.emit("challenges"),
            accent="#d29922",
        )
        grid.addWidget(c4, 1, 1)

        layout.addLayout(grid)

        # Featured Materials Quick-List
        featured_label = QLabel("Quick Access: Space & High-Performance Materials")
        featured_label.setStyleSheet(
            "font-size: 15px; font-weight: 700; color: #e6edf3; margin-top: 10px;"
        )
        layout.addWidget(featured_label)

        featured_row = QHBoxLayout()
        featured_row.setSpacing(10)

        featured_mats = [
            ("aluminum_6061_t6", "Aluminum 6061-T6", "metal"),
            ("titanium_ti6al4v_grade5", "Titanium Ti-6Al-4V", "metal"),
            ("cfrp_unidirectional_highmod", "CFRP High-Modulus", "composite"),
            ("tps_shuttle_tile_li900", "LI-900 Space Shuttle Tile", "ceramic"),
            ("diamond_cvd", "CVD Diamond", "pure_substance"),
        ]

        for mid, name, cat in featured_mats:
            btn = QPushButton(f"{name}")
            btn.setStyleSheet("""
                QPushButton {
                    background-color: #161b22;
                    border: 1px solid #30363d;
                    border-radius: 6px;
                    padding: 8px 12px;
                    color: #c9d1d9;
                    font-size: 12px;
                    font-weight: 600;
                }
                QPushButton:hover {
                    background-color: #21262d;
                    border-color: #58a6ff;
                    color: #58a6ff;
                }
            """)
            btn.clicked.connect(
                lambda checked=False, m=mid: self._on_featured_clicked(m)
            )
            featured_row.addWidget(btn)

        featured_row.addStretch()
        layout.addLayout(featured_row)
        layout.addStretch()

        scroll.setWidget(scroll_content)
        main_layout.addWidget(scroll)

    def _on_featured_clicked(self, material_id: str):
        self.select_material.emit(material_id)

    def _create_feature_card(
        self, icon: str, title: str, desc: str, btn_text: str, btn_action, accent: str
    ) -> QFrame:
        card = QFrame()
        card.setStyleSheet(f"""
            QFrame {{
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 10px;
                padding: 18px;
            }}
            QFrame:hover {{
                border-color: {accent};
            }}
        """)
        layout = QVBoxLayout(card)
        layout.setSpacing(10)

        h_layout = QHBoxLayout()
        icon_lbl = QLabel(icon)
        icon_lbl.setStyleSheet("font-size: 24px;")
        h_layout.addWidget(icon_lbl)

        t_lbl = QLabel(title)
        t_lbl.setStyleSheet("font-size: 16px; font-weight: 700; color: #e6edf3;")
        h_layout.addWidget(t_lbl)
        h_layout.addStretch()
        layout.addLayout(h_layout)

        d_lbl = QLabel(desc)
        d_lbl.setStyleSheet("color: #8b949e; font-size: 12px; line-height: 1.3;")
        d_lbl.setWordWrap(True)
        layout.addWidget(d_lbl)

        layout.addStretch()

        btn = QPushButton(btn_text)
        btn.setStyleSheet(f"""
            QPushButton {{
                background-color: #21262d;
                color: {accent};
                border: 1px solid #30363d;
                border-radius: 6px;
                padding: 8px 14px;
                font-weight: 600;
            }}
            QPushButton:hover {{
                background-color: {accent};
                color: #ffffff;
            }}
        """)
        btn.clicked.connect(btn_action)
        layout.addWidget(btn)

        return card
