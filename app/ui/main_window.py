"""
MaterialSpace Desktop Main Window.
"""

from __future__ import annotations

from typing import Optional
from app.ui.qt import (
    QButtonGroup,
    QFrame,
    QHBoxLayout,
    QLabel,
    QMainWindow,
    QPushButton,
    QStackedWidget,
    QVBoxLayout,
    QWidget,
    Qt,
    QIcon,
)
from app.data.database import MaterialDatabase
from app.ui.challenges import ChallengesView
from app.ui.comparison import ComparisonView
from app.ui.home_view import HomeView
from app.ui.material_detail import MaterialDetailView
from app.ui.material_explorer import MaterialExplorerView
from app.ui.simulations import SimulationsView
from app.ui.theme import STYLESHEET


class MainWindow(QMainWindow):
    """
    Main application window managing top navigation and view switching.
    """

    def __init__(self, database: Optional[MaterialDatabase] = None):
        super().__init__()
        self.db = database or MaterialDatabase.default()
        self.setWindowTitle("MaterialSpace — Interactive Materials-Engineering Sandbox")
        self.resize(1280, 840)
        self.setMinimumSize(1024, 700)
        self.setStyleSheet(STYLESHEET)

        self._init_ui()

    def _init_ui(self):
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        root_layout = QVBoxLayout(central_widget)
        root_layout.setContentsMargins(0, 0, 0, 0)
        root_layout.setSpacing(0)

        # Top Navigation Bar
        nav_bar = QFrame()
        nav_bar.setObjectName("nav_bar")
        nav_layout = QHBoxLayout(nav_bar)
        nav_layout.setContentsMargins(16, 8, 16, 8)
        nav_layout.setSpacing(12)

        # Brand / Logo
        brand_box = QHBoxLayout()
        brand_box.setSpacing(8)
        logo_icon = QLabel("🪐")
        logo_icon.setStyleSheet("font-size: 20px;")
        brand_box.addWidget(logo_icon)

        title_box = QVBoxLayout()
        title_box.setSpacing(0)
        brand_title = QLabel("MaterialSpace")
        brand_title.setObjectName("nav_brand_title")
        brand_sub = QLabel("Engineering Sandbox")
        brand_sub.setObjectName("nav_brand_subtitle")
        title_box.addWidget(brand_title)
        title_box.addWidget(brand_sub)
        brand_box.addLayout(title_box)

        nav_layout.addLayout(brand_box)
        nav_layout.addSpacing(24)

        # Nav Buttons
        self.nav_group = QButtonGroup(self)
        self.nav_group.setExclusive(True)

        self.btn_home = QPushButton("🏠  Home")
        self.btn_home.setProperty("class", "nav_button")
        self.btn_home.setCheckable(True)
        self.btn_home.setChecked(True)
        self.btn_home.clicked.connect(lambda: self.switch_view("home"))
        self.nav_group.addButton(self.btn_home)
        nav_layout.addWidget(self.btn_home)

        self.btn_explore = QPushButton("🔍  Explore")
        self.btn_explore.setProperty("class", "nav_button")
        self.btn_explore.setCheckable(True)
        self.btn_explore.clicked.connect(lambda: self.switch_view("explore"))
        self.nav_group.addButton(self.btn_explore)
        nav_layout.addWidget(self.btn_explore)

        self.btn_simulate = QPushButton("⚡  Simulate")
        self.btn_simulate.setProperty("class", "nav_button")
        self.btn_simulate.setCheckable(True)
        self.btn_simulate.clicked.connect(lambda: self.switch_view("simulations"))
        self.nav_group.addButton(self.btn_simulate)
        nav_layout.addWidget(self.btn_simulate)

        self.btn_compare = QPushButton("⚖️  Compare")
        self.btn_compare.setProperty("class", "nav_button")
        self.btn_compare.setCheckable(True)
        self.btn_compare.clicked.connect(lambda: self.switch_view("compare"))
        self.nav_group.addButton(self.btn_compare)
        nav_layout.addWidget(self.btn_compare)

        self.btn_challenges = QPushButton("🎯  Challenges")
        self.btn_challenges.setProperty("class", "nav_button")
        self.btn_challenges.setCheckable(True)
        self.btn_challenges.clicked.connect(lambda: self.switch_view("challenges"))
        self.nav_group.addButton(self.btn_challenges)
        nav_layout.addWidget(self.btn_challenges)

        nav_layout.addStretch()

        # Version tag
        ver_tag = QLabel("Alpha Version • v0.1.0")
        ver_tag.setStyleSheet("color: #6e7681; font-size: 11px; font-weight: 600;")
        nav_layout.addWidget(ver_tag)

        root_layout.addWidget(nav_bar)

        # Stacked Views Container
        self.stack = QStackedWidget()

        self.home_view = HomeView(self.db)
        self.home_view.navigate_to.connect(self.switch_view)
        self.home_view.select_material.connect(self.open_material_detail)
        self.stack.addWidget(self.home_view)  # 0: home

        self.explorer_view = MaterialExplorerView(self.db)
        self.explorer_view.material_selected.connect(self.open_material_detail)
        self.explorer_view.simulate_material.connect(self.open_material_simulation)
        self.explorer_view.compare_material.connect(self.open_material_comparison)
        self.stack.addWidget(self.explorer_view)  # 1: explore

        self.detail_view = MaterialDetailView(self.db)
        self.detail_view.back_requested.connect(lambda: self.switch_view("explore"))
        self.detail_view.simulate_requested.connect(self.open_material_simulation)
        self.detail_view.compare_requested.connect(self.open_material_comparison)
        self.stack.addWidget(self.detail_view)  # 2: detail

        self.simulations_view = SimulationsView(self.db)
        self.stack.addWidget(self.simulations_view)  # 3: simulations

        self.comparison_view = ComparisonView(self.db)
        self.stack.addWidget(self.comparison_view)  # 4: compare

        self.challenges_view = ChallengesView(self.db)
        self.stack.addWidget(self.challenges_view)  # 5: challenges

        root_layout.addWidget(self.stack)

    def switch_view(self, view_name: str):
        if view_name == "home":
            self.btn_home.setChecked(True)
            self.stack.setCurrentIndex(0)
        elif view_name == "explore":
            self.btn_explore.setChecked(True)
            self.stack.setCurrentIndex(1)
        elif view_name == "detail":
            self.stack.setCurrentIndex(2)
        elif view_name == "simulations":
            self.btn_simulate.setChecked(True)
            self.stack.setCurrentIndex(3)
        elif view_name == "compare":
            self.btn_compare.setChecked(True)
            self.stack.setCurrentIndex(4)
        elif view_name == "challenges":
            self.btn_challenges.setChecked(True)
            self.stack.setCurrentIndex(5)

    def open_material_detail(self, material_id: str):
        self.detail_view.display_material(material_id)
        self.switch_view("detail")

    def open_material_simulation(self, material_id: str):
        self.simulations_view.select_material_in_sim(material_id)
        self.switch_view("simulations")

    def open_material_comparison(self, material_id: str):
        self.comparison_view.add_material(material_id)
        self.switch_view("compare")
