"""
Sources management module for MaterialSpace.
"""

from __future__ import annotations

import json
from pathlib import Path


class SourceCatalog:
    """
    Catalog of citations and provenance sources for material properties.
    """

    def __init__(self, sources_data: dict[str, dict]):
        self._sources = sources_data

    @classmethod
    def load_from_file(cls, path: Path | str) -> SourceCatalog:
        p = Path(path)
        if not p.exists():
            return cls({})
        with p.open("r", encoding="utf-8") as f:
            data = json.load(f)
        return cls(data)

    def get_source(self, source_id: str) -> dict | None:
        return self._sources.get(source_id)

    def get_citation_text(self, source_id: str) -> str:
        s = self.get_source(source_id)
        if not s:
            return source_id
        org = s.get("organization", "")
        title = s.get("title", source_id)
        url = s.get("url", "")
        if org and url:
            return f"{org}: {title} ({url})"
        if org:
            return f"{org}: {title}"
        return f"{title} ({url})" if url else title

    def all_sources(self) -> dict[str, dict]:
        return dict(self._sources)
