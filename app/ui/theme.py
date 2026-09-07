"""
MaterialSpace Theme & Design System.

Clean, refined scientific dark-mode aesthetic with high contrast,
crisp typography, subtle border elevation, and curated accents.
"""

STYLESHEET = """
/* Global Window & Fonts */
QMainWindow, QWidget {
    background-color: #0d1117;
    color: #e6edf3;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto", "Helvetica Neue", Arial, sans-serif;
    font-size: 13px;
    selection-background-color: #1f6feb;
    selection-color: #ffffff;
}

/* Scrollbars */
QScrollBar:vertical {
    border: none;
    background: #161b22;
    width: 8px;
    margin: 0px;
    border-radius: 4px;
}

QScrollBar::handle:vertical {
    background: #30363d;
    min-height: 24px;
    border-radius: 4px;
}

QScrollBar::handle:vertical:hover {
    background: #58a6ff;
}

QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {
    height: 0px;
}

QScrollBar:horizontal {
    border: none;
    background: #161b22;
    height: 8px;
    margin: 0px;
    border-radius: 4px;
}

QScrollBar::handle:horizontal {
    background: #30363d;
    min-width: 24px;
    border-radius: 4px;
}

QScrollBar::handle:horizontal:hover {
    background: #58a6ff;
}

QScrollBar::add-line:horizontal, QScrollBar::sub-line:horizontal {
    width: 0px;
}

/* Navigation Bar */
#nav_bar {
    background-color: #161b22;
    border-bottom: 1px solid #30363d;
    padding: 6px 16px;
}

#nav_brand_title {
    font-size: 18px;
    font-weight: 700;
    color: #58a6ff;
    letter-spacing: 0.5px;
}

#nav_brand_subtitle {
    font-size: 11px;
    color: #8b949e;
    font-weight: 500;
}

/* Nav Buttons */
QPushButton.nav_button {
    background-color: transparent;
    color: #8b949e;
    border: none;
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 600;
    border-radius: 6px;
}

QPushButton.nav_button:hover {
    background-color: #21262d;
    color: #e6edf3;
}

QPushButton.nav_button:checked {
    background-color: #1f6feb;
    color: #ffffff;
}

/* Buttons */
QPushButton {
    background-color: #21262d;
    color: #e6edf3;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 6px 14px;
    font-weight: 500;
}

QPushButton:hover {
    background-color: #30363d;
    border-color: #8b949e;
}
QPushButton:pressed {
    background-color: #161b22;
}

QPushButton.primary_button {
    background-color: #238636;
    color: #ffffff;
    border: 1px solid #2ea043;
    font-weight: 600;
}

QPushButton.primary_button:hover {
    background-color: #2ea043;
}

QPushButton.primary_button:pressed {
    background-color: #1f772e;
}

QPushButton.accent_button {
    background-color: #1f6feb;
    color: #ffffff;
    border: 1px solid #388bfd;
    font-weight: 600;
}

QPushButton.accent_button:hover {
    background-color: #388bfd;
}

/* Line Edit / Search Input */
QLineEdit {
    background-color: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 8px 12px;
    color: #e6edf3;
    font-size: 13px;
}

QLineEdit:focus {
    border: 1px solid #58a6ff;
    background-color: #161b22;
}

/* ComboBox */
QComboBox {
    background-color: #21262d;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 6px 12px;
    color: #e6edf3;
    min-height: 20px;
}

QComboBox:hover {
    border-color: #58a6ff;
}

QComboBox::drop-down {
    border: none;
    width: 20px;
}

QComboBox QAbstractItemView {
    background-color: #161b22;
    border: 1px solid #30363d;
    selection-background-color: #1f6feb;
    selection-color: #ffffff;
    padding: 4px;
}

/* Tables */
QTableWidget, QTableView {
    background-color: #161b22;
    border: 1px solid #30363d;
    border-radius: 6px;
    gridline-color: #21262d;
    selection-background-color: #1f6feb;
    selection-color: #ffffff;
}

QHeaderView::section {
    background-color: #0d1117;
    color: #8b949e;
    padding: 6px 10px;
    border: none;
    border-bottom: 1px solid #30363d;
    font-weight: 600;
    font-size: 12px;
}

/* Cards & Frames */
QFrame.card_frame {
    background-color: #161b22;
    border: 1px solid #30363d;
    border-radius: 8px;
    padding: 12px;
}

QFrame.highlight_card {
    background-color: #161b22;
    border: 1px solid #1f6feb;
    border-radius: 8px;
    padding: 12px;
}

QFrame.info_box {
    background-color: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 10px;
}

/* Tab Widget */
QTabWidget::pane {
    border: 1px solid #30363d;
    border-radius: 6px;
    background-color: #161b22;
    padding: 8px;
}

QTabBar::tab {
    background-color: #0d1117;
    color: #8b949e;
    border: 1px solid #30363d;
    border-bottom: none;
    padding: 8px 18px;
    margin-right: 4px;
    border-top-left-radius: 6px;
    border-top-right-radius: 6px;
    font-weight: 600;
}

QTabBar::tab:selected {
    background-color: #161b22;
    color: #58a6ff;
    border-bottom: 2px solid #58a6ff;
}

QTabBar::tab:hover:!selected {
    background-color: #21262d;
    color: #e6edf3;
}

/* Sliders */
QSlider::groove:horizontal {
    height: 6px;
    background: #30363d;
    border-radius: 3px;
}

QSlider::sub-page:horizontal {
    background: #1f6feb;
    border-radius: 3px;
}

QSlider::handle:horizontal {
    background: #58a6ff;
    border: 2px solid #ffffff;
    width: 16px;
    margin-top: -5px;
    margin-bottom: -5px;
    border-radius: 8px;
}

QSlider::handle:horizontal:hover {
    background: #79c0ff;
}

/* Spinboxes */
QDoubleSpinBox, QSpinBox {
    background-color: #0d1117;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 6px 8px;
    color: #e6edf3;
    font-weight: 600;
}

QDoubleSpinBox:focus, QSpinBox:focus {
    border-color: #58a6ff;
}

/* Tooltips */
QToolTip {
    background-color: #21262d;
    color: #e6edf3;
    border: 1px solid #30363d;
    padding: 6px;
    border-radius: 4px;
    font-size: 12px;
}
"""

# Matplotlib dark mode style settings
MATPLOTLIB_DARK_STYLE = {
    "figure.facecolor": "#161b22",
    "axes.facecolor": "#0d1117",
    "axes.edgecolor": "#30363d",
    "axes.labelcolor": "#8b949e",
    "text.color": "#e6edf3",
    "xtick.color": "#8b949e",
    "ytick.color": "#8b949e",
    "grid.color": "#21262d",
    "grid.linestyle": "--",
    "grid.alpha": 0.7,
    "legend.facecolor": "#161b22",
    "legend.edgecolor": "#30363d",
    "legend.labelcolor": "#e6edf3",
}
