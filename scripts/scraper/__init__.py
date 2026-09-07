"""
MaterialSpace material-data scrapers.
"""

from .common import (
    MaterialRecord,
    PropertyValue,
    SourceReference,
    slugify,
)
from .matweb import MatWebImporter
from .nasa_tpsx import NASATPSXScraper
from .nist import NISTScraper

__all__ = [
    "MatWebImporter",
    "MaterialRecord",
    "NASATPSXScraper",
    "NISTScraper",
    "PropertyValue",
    "SourceReference",
    "slugify",
]
