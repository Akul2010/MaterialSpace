"""
Material Explorer View for MaterialSpace.
"""

from __future__ import annotations

from app.core.material import Material
from app.core.units import format_property_value
from app.data.database import MaterialDatabase
from app.ui.qt import (
    QButtonGroup,
    QFrame,
    QHBoxLayout,
    QHeaderView,
    QLabel,
    QLineEdit,
    QPushButton,
    QSplitter,
    Qt,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
    QWidget,
    Signal,
)


class MaterialExplorerView(QWidget):
    """
    Search and filter materials across categories with instant search and property preview.
    """

    material_selected = Signal(str)  # material_id (open detail)
    simulate_material = Signal(str)  # material_id
    compare_material = Signal(str)  # material_id

    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self.current_category = "all"
        self.selected_material: Material | None = None
        self._init_ui()
        self._populate_table()

    def _init_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(24, 20, 24, 20)
        main_layout.setSpacing(16)

        # Header Title
        header_row = QHBoxLayout()
        title_box = QVBoxLayout()
        title = QLabel("Material Explorer & Database")
        title.setStyleSheet("font-size: 22px; font-weight: 700; color: #ffffff;")
        title_box.addWidget(title)

        subtitle = QLabel(
            "Search, filter, and inspect physical properties of 55+ authenticated engineering materials."
        )
        subtitle.setStyleSheet("color: #8b949e; font-size: 13px;")
        title_box.addWidget(subtitle)
        header_row.addLayout(title_box)
        header_row.addStretch()

        self.count_badge = QLabel(f"{self.db.count()} Materials")
        self.count_badge.setStyleSheet("""
            background-color: #21262d;
            border: 1px solid #30363d;
            color: #58a6ff;
            border-radius: 12px;
            padding: 4px 12px;
            font-weight: 700;
            font-size: 12px;
        """)
        header_row.addWidget(self.count_badge)
        main_layout.addLayout(header_row)

        # Search Bar & Filter Controls
        search_filter_row = QHBoxLayout()
        search_filter_row.setSpacing(12)

        self.search_input = QLineEdit()
        self.search_input.setPlaceholderText(
            "🔍 Search materials by name, alloy grade, composition, or application..."
        )
        self.search_input.setClearButtonEnabled(True)
        self.search_input.textChanged.connect(self._on_search_changed)
        search_filter_row.addWidget(self.search_input, stretch=2)

        main_layout.addLayout(search_filter_row)

        # Category Filter Chips
        chips_layout = QHBoxLayout()
        chips_layout.setSpacing(8)

        categories = [
            ("all", "All Materials"),
            ("metal", "🔩 Metals & Alloys"),
            ("ceramic", "🏺 Ceramics"),
            ("polymer", "🧪 Polymers"),
            ("composite", "🛡️ Composites"),
            ("glass", "🔍 Glasses"),
            ("pure_substance", "💎 Pure Elements"),
        ]

        self.category_group = QButtonGroup(self)
        self.category_group.setExclusive(True)

        for cat_id, cat_name in categories:
            btn = QPushButton(cat_name)
            btn.setCheckable(True)
            if cat_id == "all":
                btn.setChecked(True)
            btn.setStyleSheet("""
                QPushButton {
                    background-color: #161b22;
                    border: 1px solid #30363d;
                    border-radius: 16px;
                    padding: 5px 14px;
                    color: #8b949e;
                    font-size: 12px;
                    font-weight: 600;
                }
                QPushButton:hover {
                    background-color: #21262d;
                    color: #e6edf3;
                }
                QPushButton:checked {
                    background-color: #1f6feb;
                    border-color: #388bfd;
                    color: #ffffff;
                }
            """)
            btn.clicked.connect(
                lambda checked=False, cid=cat_id: self._on_category_changed(cid)
            )
            self.category_group.addButton(btn)
            chips_layout.addWidget(btn)

        chips_layout.addStretch()
        main_layout.addLayout(chips_layout)

        # Main Splitter (Table on left, Quick Preview Drawer on right)
        splitter = QSplitter(Qt.Horizontal)
        splitter.setStyleSheet("""
            QSplitter::handle {
                background-color: #21262d;
                width: 2px;
            }
        """)

        # Table Widget
        self.table = QTableWidget()
        self.table.setColumnCount(6)
        self.table.setHorizontalHeaderLabels(
            [
                "Material Name",
                "Category",
                "Density",
                "Young's Modulus",
                "Yield Strength",
                "Thermal Cond.",
            ]
        )
        self.table.horizontalHeader().setSectionResizeMode(0, QHeaderView.Stretch)
        for i in range(1, 6):
            self.table.horizontalHeader().setSectionResizeMode(
                i, QHeaderView.ResizeToContents
            )

        self.table.setSelectionBehavior(QTableWidget.SelectRows)
        self.table.setSelectionMode(QTableWidget.SingleSelection)
        self.table.setEditTriggers(QTableWidget.NoEditTriggers)
        self.table.itemSelectionChanged.connect(self._on_table_selection_changed)
        self.table.cellDoubleClicked.connect(self._on_cell_double_clicked)
        splitter.addWidget(self.table)

        # Quick Preview Card on Right
        self.preview_panel = QFrame()
        self.preview_panel.setStyleSheet("""
            QFrame {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 16px;
            }
        """)
        self.preview_layout = QVBoxLayout(self.preview_panel)
        self.preview_layout.setSpacing(10)
        self._build_preview_placeholder()
        splitter.addWidget(self.preview_panel)

        splitter.setSizes([650, 350])
        main_layout.addWidget(splitter)

    def _build_preview_placeholder(self):
        lbl = QLabel("Select a material to inspect properties")
        lbl.setAlignment(Qt.AlignCenter)
        lbl.setStyleSheet("color: #6e7681; font-style: italic; margin: 40px 0;")
        self.preview_layout.addWidget(lbl)

    def _populate_table(self):
        query = self.search_input.text()
        materials = self.db.search(query=query, category=self.current_category)

        self.count_badge.setText(f"{len(materials)} Materials")
        self.table.setRowCount(len(materials))

        for row, mat in enumerate(materials):
            # Name
            name_item = QTableWidgetItem(mat.name)
            name_item.setData(Qt.UserRole, mat.id)
            self.table.setItem(row, 0, name_item)

            # Category
            cat_item = QTableWidgetItem(mat.category.title().replace("_", " "))
            cat_item.setForeground(Qt.GlobalColor.darkGray)
            self.table.setItem(row, 1, cat_item)

            # Density
            dens_val = mat.get_value("density")
            dens_str = f"{dens_val:,.0f} kg/m³" if dens_val is not None else "—"
            self.table.setItem(row, 2, QTableWidgetItem(dens_str))

            # Young's Modulus
            e_val = mat.get_value("youngs_modulus")
            e_str = f"{e_val / 1e9:.1f} GPa" if e_val is not None else "—"
            self.table.setItem(row, 3, QTableWidgetItem(e_str))

            # Yield Strength
            y_val = mat.get_value("yield_strength")
            y_str = f"{y_val / 1e6:.1f} MPa" if y_val is not None else "—"
            self.table.setItem(row, 4, QTableWidgetItem(y_str))

            # Thermal Conductivity
            k_val = mat.get_value("thermal_conductivity")
            k_str = f"{k_val:.1f} W/(m·K)" if k_val is not None else "—"
            self.table.setItem(row, 5, QTableWidgetItem(k_str))

        if len(materials) > 0:
            self.table.selectRow(0)

    def _on_search_changed(self):
        self._populate_table()

    def _on_category_changed(self, category: str):
        self.current_category = category
        self._populate_table()

    def _on_table_selection_changed(self):
        selected_rows = self.table.selectionModel().selectedRows()
        if not selected_rows:
            return
        row = selected_rows[0].row()
        item = self.table.item(row, 0)
        if not item:
            return
        mat_id = item.data(Qt.UserRole)
        mat = self.db.get_material(mat_id)
        if mat:
            self.selected_material = mat
            self._update_preview(mat)

    def _on_cell_double_clicked(self, row, column):
        item = self.table.item(row, 0)
        if item:
            mat_id = item.data(Qt.UserRole)
            self.material_selected.emit(mat_id)

    def _update_preview(self, mat: Material):
        # Clear preview layout
        while self.preview_layout.count():
            w = self.preview_layout.takeAt(0).widget()
            if w:
                w.deleteLater()

        cat_tag = QLabel(mat.category.upper().replace("_", " "))
        cat_tag.setStyleSheet(
            "color: #58a6ff; font-size: 10px; font-weight: 700; letter-spacing: 0.5px;"
        )
        self.preview_layout.addWidget(cat_tag)

        title = QLabel(mat.name)
        title.setStyleSheet("font-size: 18px; font-weight: 700; color: #ffffff;")
        title.setWordWrap(True)
        self.preview_layout.addWidget(title)

        if mat.grade or mat.composition:
            sub = QLabel(mat.grade or mat.composition or "")
            sub.setStyleSheet("color: #8b949e; font-size: 12px;")
            sub.setWordWrap(True)
            self.preview_layout.addWidget(sub)

        if mat.description:
            d_lbl = QLabel(mat.description)
            d_lbl.setStyleSheet("color: #c9d1d9; font-size: 12px; margin-top: 4px;")
            d_lbl.setWordWrap(True)
            self.preview_layout.addWidget(d_lbl)

        # Mini properties table
        props_frame = QFrame()
        props_frame.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 6px; padding: 8px;"
        )
        p_layout = QVBoxLayout(props_frame)
        p_layout.setSpacing(4)

        props_to_show = [
            ("density", "Density"),
            ("youngs_modulus", "Young's Modulus"),
            ("yield_strength", "Yield Strength"),
            ("thermal_conductivity", "Thermal Cond."),
            ("thermal_expansion", "Thermal Exp. (CTE)"),
            ("specific_heat", "Specific Heat"),
            ("melting_point", "Melting Point"),
        ]

        for p_key, p_lbl in props_to_show:
            val = mat.get_value(p_key)
            val_formatted = format_property_value(p_key, val)
            row_l = QHBoxLayout()
            lbl = QLabel(p_lbl)
            lbl.setStyleSheet("color: #8b949e; font-size: 11px;")
            val_l = QLabel(val_formatted)
            val_l.setStyleSheet("color: #e6edf3; font-weight: 600; font-size: 11px;")
            row_l.addWidget(lbl)
            row_l.addStretch()
            row_l.addWidget(val_l)
            p_layout.addLayout(row_l)

        self.preview_layout.addWidget(props_frame)
        self.preview_layout.addStretch()

        # Action Buttons
        full_btn = QPushButton("Open Full Material Page →")
        full_btn.setStyleSheet("""
            QPushButton {
                background-color: #1f6feb;
                color: #ffffff;
                border: 1px solid #388bfd;
                font-weight: 600;
                padding: 8px;
            }
            QPushButton:hover {
                background-color: #388bfd;
            }
        """)
        full_btn.clicked.connect(lambda: self.material_selected.emit(mat.id))
        self.preview_layout.addWidget(full_btn)

        act_row = QHBoxLayout()
        sim_btn = QPushButton("⚡ Simulate")
        sim_btn.clicked.connect(lambda: self.simulate_material.emit(mat.id))
        act_row.addWidget(sim_btn)

        comp_btn = QPushButton("⚖️ Compare")
        comp_btn.clicked.connect(lambda: self.compare_material.emit(mat.id))
        act_row.addWidget(comp_btn)

        self.preview_layout.addLayout(act_row)
