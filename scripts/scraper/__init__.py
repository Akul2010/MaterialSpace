"""
MaterialSpace material-data scrapers.
"""

from .common import (
    MaterialRecord,
    PropertyValue,
    SourceReference,
    slugify,
)

from .nasa_tpsx import NASATPSXScraper
from .nist import NISTScraper
from .matweb import MatWebImporter

__all__ = [
    "MaterialRecord",
    "PropertyValue",
    "SourceReference",
    "slugify",
    "NASATPSXScraper",
    "NISTScraper",
    "MatWebImporter",
]
