"""
Material Detail View for MaterialSpace.
"""

from __future__ import annotations

from typing import Optional
from app.ui.qt import (
    QFrame,
    QGridLayout,
    QHBoxLayout,
    QLabel,
    QPushButton,
    QScrollArea,
    QVBoxLayout,
    QWidget,
    Qt,
    Signal,
)
from app.core.constants import PROPERTY_METADATA
from app.core.material import Material
from app.core.units import format_property_value
from app.data.database import MaterialDatabase
from app.ui.widgets import PropertyCard


class MaterialDetailView(QWidget):
    """
    Shows full specifications, provenance citations, and physical properties of a single material.
    """

    back_requested = Signal()
    simulate_requested = Signal(str)  # material_id
    compare_requested = Signal(str)  # material_id

    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self.current_material: Optional[Material] = None
        self._init_ui()

    def _init_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(0, 0, 0, 0)

        self.scroll = QScrollArea()
        self.scroll.setWidgetResizable(True)
        self.scroll.setFrameShape(QFrame.NoFrame)
        self.content_widget = QWidget()
        self.layout = QVBoxLayout(self.content_widget)
        self.layout.setContentsMargins(32, 24, 32, 24)
        self.layout.setSpacing(20)

        self.scroll.setWidget(self.content_widget)
        main_layout.addWidget(self.scroll)

    def display_material(self, material_id: str):
        material = self.db.get_material(material_id)
        if not material:
            return

        self.current_material = material

        # Clear existing layout
        while self.layout.count():
            item = self.layout.takeAt(0)
            if item.widget():
                item.widget().deleteLater()
            elif item.layout():
                # clear sub-layout
                pass

        # Top Action Bar
        top_bar = QHBoxLayout()
        back_btn = QPushButton("← Back to Explorer")
        back_btn.setStyleSheet("padding: 6px 14px; font-weight: 600;")
        back_btn.clicked.connect(lambda: self.back_requested.emit())
        top_bar.addWidget(back_btn)
        top_bar.addStretch()

        sim_btn = QPushButton("⚡ Simulate Material")
        sim_btn.setStyleSheet("""
            QPushButton {
                background-color: #238636;
                color: #ffffff;
                border: 1px solid #2ea043;
                font-weight: 600;
                padding: 6px 14px;
            }
            QPushButton:hover {
                background-color: #2ea043;
            }
        """)
        sim_btn.clicked.connect(lambda: self.simulate_requested.emit(material.id))
        top_bar.addWidget(sim_btn)

        comp_btn = QPushButton("⚖️ Add to Comparison")
        comp_btn.setStyleSheet("""
            QPushButton {
                background-color: #1f6feb;
                color: #ffffff;
                border: 1px solid #388bfd;
                font-weight: 600;
                padding: 6px 14px;
            }
            QPushButton:hover {
                background-color: #388bfd;
            }
        """)
        comp_btn.clicked.connect(lambda: self.compare_requested.emit(material.id))
        top_bar.addWidget(comp_btn)

        self.layout.addLayout(top_bar)

        # Header Info Card
        header_card = QFrame()
        header_card.setStyleSheet("""
            QFrame {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 10px;
                padding: 20px;
            }
        """)
        h_layout = QVBoxLayout(header_card)
        h_layout.setSpacing(8)

        cat_tag = QLabel(f"CATEGORY: {material.category.upper().replace('_', ' ')}")
        cat_tag.setStyleSheet(
            "color: #58a6ff; font-size: 11px; font-weight: 700; letter-spacing: 0.5px;"
        )
        h_layout.addWidget(cat_tag)

        title = QLabel(material.name)
        title.setStyleSheet("font-size: 26px; font-weight: 800; color: #ffffff;")
        h_layout.addWidget(title)

        if material.composition or material.grade:
            meta_parts = []
            if material.composition:
                meta_parts.append(f"<b>Composition:</b> {material.composition}")
            if material.grade:
                meta_parts.append(f"<b>Grade/Temper:</b> {material.grade}")
            meta_lbl = QLabel(" | ".join(meta_parts))
            meta_lbl.setStyleSheet("color: #8b949e; font-size: 13px;")
            h_layout.addWidget(meta_lbl)

        if material.description:
            desc_lbl = QLabel(material.description)
            desc_lbl.setStyleSheet(
                "color: #c9d1d9; font-size: 13px; line-height: 1.4; margin-top: 4px;"
            )
            desc_lbl.setWordWrap(True)
            h_layout.addWidget(desc_lbl)

        self.layout.addWidget(header_card)

        # Properties Section
        props_header = QLabel("Physical, Mechanical & Thermal Properties")
        props_header.setStyleSheet(
            "font-size: 18px; font-weight: 700; color: #e6edf3; margin-top: 8px;"
        )
        self.layout.addWidget(props_header)

        # Properties Grid
        grid = QGridLayout()
        grid.setSpacing(12)

        row = 0
        col = 0
        catalog = self.db.source_catalog

        for prop_key, meta in PROPERTY_METADATA.items():
            prop = material.get_property(prop_key)
            is_avail = prop is not None and prop.value is not None

            val_str = ""
            condition = None
            citation = None

            if is_avail and prop is not None:
                val_str = format_property_value(prop_key, prop.value)
                condition = prop.condition
                citation = catalog.get_citation_text(prop.source_id)

            card = PropertyCard(
                prop_label=f"{meta['label']} ({meta['symbol']})",
                value_str=val_str,
                unit_str=meta["display_unit"],
                condition=condition,
                source_citation=citation,
                is_available=is_avail,
            )
            grid.addWidget(card, row, col)

            col += 1
            if col > 1:
                col = 0
                row += 1

        self.layout.addLayout(grid)

        # Data Provenance / Sources Footnote
        source_box = QFrame()
        source_box.setStyleSheet(
            "background-color: #0d1117; border: 1px solid #30363d; border-radius: 6px; padding: 12px;"
        )
        s_layout = QVBoxLayout(source_box)
        s_layout.setSpacing(6)

        s_title = QLabel("Scientific Provenance & Authenticity")
        s_title.setStyleSheet(
            "color: #8b949e; font-size: 12px; font-weight: 700; text-transform: uppercase;"
        )
        s_layout.addWidget(s_title)

        provenance_text = (
            f"Properties for <b>{material.name}</b> are compiled from peer-reviewed databases and handbooks. "
            "Actual material properties vary with heat treatment, temperature, alloy batch, and surface finish. "
            "Where properties are marked 'Data unavailable', they are intentionally left unestimated to maintain strict scientific integrity."
        )
        p_lbl = QLabel(provenance_text)
        p_lbl.setStyleSheet("color: #8b949e; font-size: 12px;")
        p_lbl.setWordWrap(True)
        s_layout.addWidget(p_lbl)

        self.layout.addWidget(source_box)
        self.layout.addStretch()
