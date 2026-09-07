"""
MaterialDatabase for local indexed material querying and management.
"""

from __future__ import annotations

import json
from pathlib import Path

from app.core.material import Material
from app.data.sources import SourceCatalog


class MaterialDatabase:
    """
    In-memory indexed database of materials.
    """

    def __init__(
        self, materials: list[Material], source_catalog: SourceCatalog | None = None
    ):
        self._materials = materials
        self._by_id: dict[str, Material] = {m.id: m for m in materials}
        self._source_catalog = source_catalog or SourceCatalog({})

    @classmethod
    def load_from_json(
        cls,
        materials_path: Path | str,
        sources_path: Path | str | None = None,
    ) -> MaterialDatabase:
        m_path = Path(materials_path)
        if not m_path.exists():
            raise FileNotFoundError(f"Materials database file not found: {m_path}")

        with m_path.open("r", encoding="utf-8") as f:
            raw_data = json.load(f)

        if not isinstance(raw_data, list):
            raise ValueError(f"Database root must be a JSON array: {m_path}")

        materials = [
            Material.from_dict(item) for item in raw_data if isinstance(item, dict)
        ]

        catalog = SourceCatalog({})
        if sources_path:
            s_path = Path(sources_path)
            if s_path.exists():
                catalog = SourceCatalog.load_from_file(s_path)

        return cls(materials, catalog)

    @classmethod
    def default(cls) -> MaterialDatabase:
        """Load default materials database from workspace data directory."""
        root = Path(__file__).resolve().parents[2]
        data_dir = root / "data"
        norm_path = data_dir / "materials_normalized.json"
        raw_path = data_dir / "materials.json"
        sources_path = data_dir / "sources.json"

        target_path = norm_path if norm_path.exists() else raw_path
        return cls.load_from_json(target_path, sources_path)

    @property
    def source_catalog(self) -> SourceCatalog:
        return self._source_catalog

    def count(self) -> int:
        return len(self._materials)

    def get_material(self, material_id: str) -> Material | None:
        """Lookup a material by exact ID."""
        return self._by_id.get(material_id)

    def get_all_materials(self) -> list[Material]:
        """Return all materials sorted alphabetically by name."""
        return list(self._materials)

    def get_categories(self) -> list[str]:
        """Return unique sorted categories present in the database."""
        cats = {m.category for m in self._materials if m.category}
        return sorted(cats)

    def filter_by_category(self, category: str) -> list[Material]:
        """Filter materials by category ID ('all' returns all)."""
        cat_lower = category.lower().strip()
        if cat_lower in ("all", ""):
            return self.get_all_materials()
        return [m for m in self._materials if m.category.lower() == cat_lower]

    def search(self, query: str = "", category: str = "all") -> list[Material]:
        """
        Fast multi-term search across material name, composition, grade, category, and ID.
        """
        q = query.lower().strip()
        filtered = self.filter_by_category(category)

        if not q:
            return filtered

        terms = q.split()
        results = []
        for m in filtered:
            searchable = f"{m.name} {m.id} {m.category} {m.composition or ''} {m.grade or ''} {m.description or ''}".lower()
            if all(term in searchable for term in terms):
                results.append(m)

        return results

    def compare(self, material_ids: list[str]) -> list[Material]:
        """Retrieve list of materials for side-by-side comparison (order preserved)."""
        res = []
        for mid in material_ids:
            mat = self.get_material(mid)
            if mat:
                res.append(mat)
        return res
