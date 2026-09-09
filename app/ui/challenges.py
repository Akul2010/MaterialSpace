"""
Engineering Challenges & Material Selection Decision Engine View for MaterialSpace.
"""

from __future__ import annotations

from app.data.database import MaterialDatabase
from app.models.material_selection import (
    BUILTIN_CHALLENGES,
    CandidateEvaluation,
    Criterion,
    EngineeringChallenge,
    evaluate_materials,
)
from app.ui.qt import (
    QCheckBox,
    QComboBox,
    QFrame,
    QGridLayout,
    QHBoxLayout,
    QLabel,
    QProgressBar,
    QPushButton,
    QScrollArea,
    QSlider,
    Qt,
    QVBoxLayout,
    QWidget,
    Signal,
)


class ChallengesView(QWidget):
    """
    Engineering challenge solver using transparent multi-criteria decision models.
    """

    material_clicked = Signal(str)

    def __init__(self, database: MaterialDatabase, parent=None):
        super().__init__(parent)
        self.db = database
        self.current_challenge: EngineeringChallenge = BUILTIN_CHALLENGES[0]
        self.active_criteria: list[Criterion] = [
            Criterion(
                c.property_name,
                c.label,
                c.weight,
                c.higher_is_better,
                c.min_threshold,
                c.max_threshold,
                c.description,
            )
            for c in self.current_challenge.default_criteria
        ]
        self.active_candidate_ids: list[str] = list(
            self.current_challenge.recommended_candidate_ids
        )
        self.sliders: dict[str, QSlider] = {}
        self.slider_labels: dict[str, QLabel] = {}
        self.candidate_checks: dict[str, QCheckBox] = {}
        self._init_ui()
        self._recalculate_rankings()

    def _init_ui(self):
        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(24, 20, 24, 20)
        main_layout.setSpacing(16)

        # Header Title & Challenge Selector
        header_row = QHBoxLayout()
        title_box = QVBoxLayout()
        title = QLabel("Engineering Decision Engine")
        title.setStyleSheet("font-size: 22px; font-weight: 700; color: #ffffff;")
        title_box.addWidget(title)

        subtitle = QLabel(
            "Solve aerospace and structural engineering trade-offs using transparent, requirement-weighted multi-criteria evaluation."
        )
        subtitle.setStyleSheet("color: #8b949e; font-size: 13px;")
        title_box.addWidget(subtitle)
        header_row.addLayout(title_box)
        header_row.addStretch()

        sel_box = QVBoxLayout()
        sel_lbl = QLabel("SELECT SCENARIO:")
        sel_lbl.setStyleSheet("color: #58a6ff; font-weight: 700; font-size: 11px;")
        sel_box.addWidget(sel_lbl)

        self.scenario_combo = QComboBox()
        for ch in BUILTIN_CHALLENGES:
            self.scenario_combo.addItem(f"🎯 {ch.title}", ch.id)
        self.scenario_combo.currentIndexChanged.connect(self._on_scenario_changed)
        sel_box.addWidget(self.scenario_combo)
        header_row.addLayout(sel_box)

        main_layout.addLayout(header_row)

        # Scrollable Main Body
        scroll = QScrollArea()
        scroll.setWidgetResizable(True)
        scroll.setFrameShape(QFrame.NoFrame)
        scroll_content = QWidget()
        self.body_layout = QVBoxLayout(scroll_content)
        self.body_layout.setSpacing(18)

        # Scenario Description Banner
        self.desc_card = QFrame()
        self.desc_card.setStyleSheet("""
            QFrame {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 16px;
            }
        """)
        self.desc_layout = QVBoxLayout(self.desc_card)
        self.body_layout.addWidget(self.desc_card)

        # Two Column Layout: Left (Weights & Candidates), Right (Live Ranked Scorecards & Breakdown)
        columns_row = QHBoxLayout()
        columns_row.setSpacing(18)

        # Left Column (Controls)
        left_col = QVBoxLayout()
        left_col.setSpacing(16)

        # Weight Adjustment Box
        self.weights_card = QFrame()
        self.weights_card.setStyleSheet("""
            QFrame {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 16px;
            }
        """)
        self.weights_layout = QVBoxLayout(self.weights_card)
        left_col.addWidget(self.weights_card)

        # Candidate Pool Selector Box
        self.candidates_card = QFrame()
        self.candidates_card.setStyleSheet("""
            QFrame {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 16px;
            }
        """)
        self.candidates_layout = QVBoxLayout(self.candidates_card)
        left_col.addWidget(self.candidates_card)

        columns_row.addLayout(left_col, stretch=4)

        # Right Column (Live Scorecards & Ranking)
        right_col = QVBoxLayout()
        right_col.setSpacing(16)

        self.results_card = QFrame()
        self.results_card.setStyleSheet("""
            QFrame {
                background-color: #161b22;
                border: 1px solid #30363d;
                border-radius: 8px;
                padding: 18px;
            }
        """)
        self.results_layout = QVBoxLayout(self.results_card)
        right_col.addWidget(self.results_card)

        columns_row.addLayout(right_col, stretch=6)
        self.body_layout.addLayout(columns_row)

        scroll.setWidget(scroll_content)
        main_layout.addWidget(scroll)

        self._build_scenario_header()
        self._build_weight_controls()
        self._build_candidate_controls()

    def _on_scenario_changed(self, idx):
        self.current_challenge = BUILTIN_CHALLENGES[idx]
        self.active_criteria = [
            Criterion(
                c.property_name,
                c.label,
                c.weight,
                c.higher_is_better,
                c.min_threshold,
                c.max_threshold,
                c.description,
            )
            for c in self.current_challenge.default_criteria
        ]
        self.active_candidate_ids = list(
            self.current_challenge.recommended_candidate_ids
        )

        self._build_scenario_header()
        self._build_weight_controls()
        self._build_candidate_controls()
        self._recalculate_rankings()

    def _clear_layout(self, layout):
        while layout.count():
            item = layout.takeAt(0)
            widget = item.widget()
            child_layout = item.layout()
            if widget:
                widget.hide()
                widget.deleteLater()
            elif child_layout:
                self._clear_layout(child_layout)
                child_layout.deleteLater()

    def _build_scenario_header(self):
        self._clear_layout(self.desc_layout)

        ch = self.current_challenge

        t_lbl = QLabel(f"MISSION: {ch.title.upper()}")
        t_lbl.setStyleSheet(
            "color: #d29922; font-size: 14px; font-weight: 800; letter-spacing: 0.5px;"
        )
        self.desc_layout.addWidget(t_lbl)

        d_lbl = QLabel(ch.description)
        d_lbl.setStyleSheet(
            "color: #e6edf3; font-size: 14px; font-weight: 600; margin-top: 2px;"
        )
        d_lbl.setWordWrap(True)
        self.desc_layout.addWidget(d_lbl)

        ctx_lbl = QLabel(
            f"<b>Engineering Context & Trade-offs:</b> {ch.engineering_context}"
        )
        ctx_lbl.setStyleSheet(
            "color: #8b949e; font-size: 12px; line-height: 1.4; margin-top: 4px;"
        )
        ctx_lbl.setWordWrap(True)
        self.desc_layout.addWidget(ctx_lbl)

    def _build_weight_controls(self):
        self._clear_layout(self.weights_layout)

        self.sliders.clear()
        self.slider_labels.clear()

        w_title = QLabel("ADJUST REQUIREMENT WEIGHTS:")
        w_title.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        self.weights_layout.addWidget(w_title)

        expl = QLabel(
            "Slide to adjust how heavily each physical constraint influences the decision score:"
        )
        expl.setStyleSheet("color: #8b949e; font-size: 11px;")
        self.weights_layout.addWidget(expl)

        for crit in self.active_criteria:
            crit_row = QVBoxLayout()
            crit_row.setSpacing(4)

            header = QHBoxLayout()
            lbl = QLabel(f"<b>{crit.label}</b>")
            lbl.setStyleSheet("color: #e6edf3; font-size: 12px;")
            header.addWidget(lbl)
            header.addStretch()

            pct_lbl = QLabel(f"{crit.weight * 100:.0f}%")
            pct_lbl.setStyleSheet("color: #58a6ff; font-weight: 700; font-size: 12px;")
            header.addWidget(pct_lbl)
            self.slider_labels[crit.property_name] = pct_lbl
            crit_row.addLayout(header)

            slider = QSlider(Qt.Horizontal)
            slider.setRange(0, 100)
            slider.setValue(int(crit.weight * 100))
            slider.valueChanged.connect(
                lambda val, p=crit.property_name: self._on_weight_slider_moved(p, val)
            )
            self.sliders[crit.property_name] = slider
            crit_row.addWidget(slider)

            self.weights_layout.addLayout(crit_row)

        reset_btn = QPushButton("Reset to Default Mission Weights")
        reset_btn.setStyleSheet("padding: 4px 10px; font-size: 11px; margin-top: 6px;")
        reset_btn.clicked.connect(self._reset_weights)
        self.weights_layout.addWidget(reset_btn)

    def _build_candidate_controls(self):
        self._clear_layout(self.candidates_layout)

        self.candidate_checks.clear()

        c_title = QLabel("CANDIDATE MATERIAL POOL:")
        c_title.setStyleSheet("color: #58a6ff; font-size: 11px; font-weight: 700;")
        self.candidates_layout.addWidget(c_title)

        grid = QGridLayout()
        grid.setSpacing(6)

        all_mats = self.db.get_all_materials()
        col = 0
        row = 0

        for mat in all_mats:
            cb = QCheckBox(mat.name)
            cb.setStyleSheet("color: #c9d1d9; font-size: 11px;")
            is_active = mat.id in self.active_candidate_ids
            cb.setChecked(is_active)
            cb.toggled.connect(
                lambda checked, mid=mat.id: self._on_candidate_toggled(mid, checked)
            )
            self.candidate_checks[mat.id] = cb
            grid.addWidget(cb, row, col)

            col += 1
            if col > 1:
                col = 0
                row += 1

        self.candidates_layout.addLayout(grid)

    def _on_weight_slider_moved(self, prop_name: str, val: int):
        for crit in self.active_criteria:
            if crit.property_name == prop_name:
                crit.weight = val / 100.0

        total = sum(c.weight for c in self.active_criteria)
        for crit in self.active_criteria:
            norm_pct = (crit.weight / total * 100.0) if total > 0 else 0.0
            if crit.property_name in self.slider_labels:
                self.slider_labels[crit.property_name].setText(f"{norm_pct:.0f}%")

        self._recalculate_rankings()

    def _reset_weights(self):
        for crit, def_crit in zip(
            self.active_criteria, self.current_challenge.default_criteria
        ):
            crit.weight = def_crit.weight
            if crit.property_name in self.sliders:
                self.sliders[crit.property_name].setValue(int(def_crit.weight * 100))
        self._recalculate_rankings()

    def _on_candidate_toggled(self, material_id: str, checked: bool):
        if checked and material_id not in self.active_candidate_ids:
            self.active_candidate_ids.append(material_id)
        elif not checked and material_id in self.active_candidate_ids:
            self.active_candidate_ids.remove(material_id)
        self._recalculate_rankings()

    def _recalculate_rankings(self):
        self._clear_layout(self.results_layout)

        candidates = [
            self.db.get_material(mid)
            for mid in self.active_candidate_ids
            if self.db.get_material(mid)
        ]

        if not candidates:
            lbl = QLabel(
                "No candidate materials selected. Check candidate boxes on the left."
            )
            lbl.setStyleSheet("color: #8b949e; font-style: italic; margin: 20px 0;")
            self.results_layout.addWidget(lbl)
            return

        evaluations = evaluate_materials(candidates, self.active_criteria)

        r_title = QLabel(
            f"DECISION ENGINE RESULTS ({len(evaluations)} Ranked Candidates):"
        )
        r_title.setStyleSheet(
            "color: #3fb950; font-size: 13px; font-weight: 700; letter-spacing: 0.5px;"
        )
        self.results_layout.addWidget(r_title)

        for ev in evaluations:
            card = self._create_scorecard(ev)
            self.results_layout.addWidget(card)

    def _create_scorecard(self, ev: CandidateEvaluation) -> QFrame:
        card = QFrame()
        is_winner = ev.rank == 1
        border = "#3fb950" if is_winner else "#30363d"
        bg = "#161b22" if is_winner else "#0d1117"

        card.setStyleSheet(f"""
            QFrame {{
                background-color: {bg};
                border: 1px solid {border};
                border-radius: 8px;
                padding: 12px;
            }}
        """)
        c_layout = QVBoxLayout(card)
        c_layout.setSpacing(8)

        # Header Row
        header_row = QHBoxLayout()
        rank_badge = QLabel(f"#{ev.rank}")
        rank_color = "#3fb950" if is_winner else "#8b949e"
        rank_badge.setStyleSheet(
            f"color: {rank_color}; font-size: 18px; font-weight: 800; min-width: 32px;"
        )
        header_row.addWidget(rank_badge)

        name_lbl = QLabel(ev.material.name)
        name_lbl.setStyleSheet("color: #ffffff; font-size: 15px; font-weight: 700;")
        header_row.addWidget(name_lbl)
        header_row.addStretch()

        score_lbl = QLabel(f"{ev.overall_score:.1f} / 100")
        score_lbl.setStyleSheet(
            f"color: {rank_color}; font-size: 18px; font-weight: 800;"
        )
        header_row.addWidget(score_lbl)
        c_layout.addLayout(header_row)

        # Score progress bar
        bar = QProgressBar()
        bar.setRange(0, 100)
        bar.setValue(int(ev.overall_score))
        bar.setTextVisible(False)
        bar.setFixedHeight(6)
        bar_color = "#3fb950" if is_winner else "#58a6ff"
        bar.setStyleSheet(f"""
            QProgressBar {{
                background-color: #21262d;
                border: none;
                border-radius: 3px;
            }}
            QProgressBar::chunk {{
                background-color: {bar_color};
                border-radius: 3px;
            }}
        """)
        c_layout.addWidget(bar)

        # Sub-score criteria breakdown pills
        breakdown_row = QHBoxLayout()
        breakdown_row.setSpacing(6)

        for prop_name, bd in ev.breakdowns.items():
            pill = QFrame()
            pill.setStyleSheet(
                "background-color: #21262d; border-radius: 4px; padding: 4px 8px;"
            )
            p_l = QVBoxLayout(pill)
            p_l.setContentsMargins(4, 2, 4, 2)
            p_l.setSpacing(2)

            p_title = QLabel(bd.label.split("(")[0].strip())
            p_title.setStyleSheet(
                "color: #8b949e; font-size: 9px; font-weight: 700; text-transform: uppercase;"
            )
            p_l.addWidget(p_title)

            p_val = QLabel(
                f"{bd.normalized_score:.0f}/100 (+{bd.weighted_contribution:.1f} pts)"
            )
            p_val.setStyleSheet("color: #e6edf3; font-size: 10px; font-weight: 600;")
            p_l.addWidget(p_val)

            breakdown_row.addWidget(pill)

        breakdown_row.addStretch()
        c_layout.addLayout(breakdown_row)

        # Explanation Rationale
        exp_lbl = QLabel(f"ℹ️ {ev.summary_explanation}")
        exp_lbl.setStyleSheet("color: #8b949e; font-size: 11px; font-style: italic;")
        exp_lbl.setWordWrap(True)
        c_layout.addWidget(exp_lbl)

        return card
