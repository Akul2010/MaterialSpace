from __future__ import annotations

import logging
import re
from urllib.parse import quote, urlencode

import requests

from bs4 import BeautifulSoup

from common import (
    MaterialRecord,
    PropertyValue,
    SourceReference,
    clean_text,
    parse_float,
    slugify,
)


logger = logging.getLogger(__name__)


class NISTScraper:
    """
    Rudimentary NIST Chemistry WebBook scraper.

    Primarily useful for pure substances and thermophysical data.
    """

    SOURCE_ID = "nist_webbook"

    BASE_URL = "https://webbook.nist.gov"

    def __init__(
        self,
        timeout: int = 20,
    ):

        self.timeout = timeout

        self.session = requests.Session()

        self.session.headers.update(
            {"User-Agent": "MaterialSpace/0.1 (educational project)"}
        )

    # --------------------------------------------------------
    # SOURCE
    # --------------------------------------------------------

    def source_reference(
        self,
    ) -> SourceReference:

        return SourceReference(
            source_id=self.SOURCE_ID,
            organization="NIST",
            title="NIST Chemistry WebBook",
            url=("https://webbook.nist.gov/chemistry/"),
        )

    # --------------------------------------------------------
    # HTTP
    # --------------------------------------------------------

    def fetch(
        self,
        url: str,
    ) -> str:

        logger.info(
            "Downloading %s",
            url,
        )

        response = self.session.get(
            url,
            timeout=self.timeout,
        )

        response.raise_for_status()

        return response.text

    # --------------------------------------------------------
    # MATERIAL
    # --------------------------------------------------------

    def scrape_material(
        self,
        identifier: str,
    ) -> MaterialRecord:

        url = f"{self.BASE_URL}/cgi/cbook.cgi?ID={quote(identifier)}&Units=SI"

        html = self.fetch(url)

        soup = BeautifulSoup(
            html,
            "html.parser",
        )

        name = self.find_name(soup)

        if not name:
            name = f"NIST Substance {identifier}"

        record = MaterialRecord(
            material_id=slugify(name),
            name=name,
            category="pure_substance",
            source_material_id=identifier,
            source_url=url,
        )

        self.parse_text(
            soup,
            record,
        )

        return record

    # --------------------------------------------------------
    # NAME
    # --------------------------------------------------------

    def find_name(
        self,
        soup: BeautifulSoup,
    ) -> str:

        heading = soup.find("h1")

        if heading:
            return clean_text(
                heading.get_text(
                    " ",
                    strip=True,
                )
            )

        if soup.title:
            return clean_text(
                soup.title.get_text(
                    " ",
                    strip=True,
                )
            )

        return ""

    # --------------------------------------------------------
    # TEXT PARSING
    # --------------------------------------------------------

    def parse_text(
        self,
        soup: BeautifulSoup,
        record: MaterialRecord,
    ) -> None:

        text = clean_text(
            soup.get_text(
                " ",
                strip=True,
            )
        )

        self.parse_melting_point(
            text,
            record,
        )

        self.parse_boiling_point(
            text,
            record,
        )

    # --------------------------------------------------------
    # MELTING
    # --------------------------------------------------------

    def parse_melting_point(
        self,
        text: str,
        record: MaterialRecord,
    ) -> None:

        pattern = (
            r"Melting Point"
            r".{0,250}?"
            r"([-+]?\d+(?:\.\d+)?)"
            r"\s*K"
        )

        match = re.search(
            pattern,
            text,
            re.IGNORECASE,
        )

        if not match:
            return

        value = parse_float(match.group(1))

        if value is None:
            return

        record.add_property(
            "melting_point",
            PropertyValue(
                value=value,
                unit="K",
                source_id=self.SOURCE_ID,
            ),
        )

    # --------------------------------------------------------
    # BOILING
    # --------------------------------------------------------

    def parse_boiling_point(
        self,
        text: str,
        record: MaterialRecord,
    ) -> None:

        pattern = (
            r"Boiling Point"
            r".{0,250}?"
            r"([-+]?\d+(?:\.\d+)?)"
            r"\s*K"
        )

        match = re.search(
            pattern,
            text,
            re.IGNORECASE,
        )

        if not match:
            return

        value = parse_float(match.group(1))

        if value is None:
            return

        record.add_property(
            "boiling_point",
            PropertyValue(
                value=value,
                unit="K",
                source_id=self.SOURCE_ID,
            ),
        )

    # --------------------------------------------------------
    # BATCH
    # --------------------------------------------------------

    def scrape_many(
        self,
        identifiers: list[str],
    ) -> list[dict]:

        records = []

        for identifier in identifiers:
            try:
                record = self.scrape_material(identifier)

                records.append(record.to_dict())

                logger.info(
                    "Scraped %s",
                    record.name,
                )

            except Exception as exc:
                logger.warning(
                    "Failed to scrape NIST %s: %s",
                    identifier,
                    exc,
                )

        return records
