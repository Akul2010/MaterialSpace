from __future__ import annotations

import logging
import re

import requests

from bs4 import BeautifulSoup

from common import (
    MaterialRecord,
    PropertyValue,
    SourceReference,
    clean_text,
    normalize_property_name,
    parse_float,
    slugify,
)


logger = logging.getLogger(__name__)


class NASATPSXScraper:
    """
    Rudimentary scraper for NASA TPSX.

    This is intentionally simple.

    Its job is to gather initial candidate materials.
    The validator and human review process are responsible for
    determining whether individual values are suitable for the
    MaterialSpace database.
    """

    NAME = "NASA TPSX"

    BASE_URL = "https://tpsx.arc.nasa.gov"

    MATERIAL_URL = "https://tpsx.arc.nasa.gov/Material"

    DATABASE_URL = "https://tpsx.arc.nasa.gov/MaterialsDatabase"

    SOURCE_ID = "nasa_tpsx"

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
            organization="NASA",
            title="TPSX Materials Database",
            url=self.BASE_URL,
            notes=("Initial material-property source for MaterialSpace."),
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
        material_id: str | int,
    ) -> MaterialRecord:

        material_id = str(material_id)

        url = f"{self.MATERIAL_URL}?id={material_id}"

        html = self.fetch(url)

        soup = BeautifulSoup(
            html,
            "html.parser",
        )

        name = self.find_name(soup)

        if not name:
            name = f"NASA TPSX Material {material_id}"

        record = MaterialRecord(
            material_id=slugify(name),
            name=name,
            category=self.find_category(soup),
            source_material_id=material_id,
            source_url=url,
        )

        self.parse_tables(
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

        for heading in soup.find_all(["h1", "h2", "h3"]):
            text = clean_text(
                heading.get_text(
                    " ",
                    strip=True,
                )
            )

            if text:
                return text

        if soup.title:
            return clean_text(
                soup.title.get_text(
                    " ",
                    strip=True,
                )
            )

        return ""

    # --------------------------------------------------------
    # CATEGORY
    # --------------------------------------------------------

    def find_category(
        self,
        soup: BeautifulSoup,
    ) -> str:

        text = clean_text(
            soup.get_text(
                " ",
                strip=True,
            )
        ).lower()

        categories = [
            (
                "ceramic",
                "ceramic",
            ),
            (
                "polymer",
                "polymer",
            ),
            (
                "composite",
                "composite",
            ),
            (
                "metal",
                "metal",
            ),
            (
                "glass",
                "glass",
            ),
        ]

        for keyword, category in categories:
            if keyword in text:
                return category

        return "unknown"

    # --------------------------------------------------------
    # TABLES
    # --------------------------------------------------------

    def parse_tables(
        self,
        soup: BeautifulSoup,
        record: MaterialRecord,
    ) -> None:

        for table in soup.find_all("table"):
            rows = table.find_all("tr")

            for row in rows:
                cells = row.find_all(["td", "th"])

                if len(cells) < 2:
                    continue

                values = [
                    clean_text(
                        cell.get_text(
                            " ",
                            strip=True,
                        )
                    )
                    for cell in cells
                ]

                self.parse_row(
                    values,
                    record,
                )

    # --------------------------------------------------------
    # ROW
    # --------------------------------------------------------

    def parse_row(
        self,
        values: list[str],
        record: MaterialRecord,
    ) -> None:

        if not values:
            return

        property_name = normalize_property_name(values[0])

        if property_name is None:
            return

        value = None
        value_index = None

        for index in range(
            1,
            len(values),
        ):
            candidate = parse_float(values[index])

            if candidate is not None:
                value = candidate
                value_index = index
                break

        if value is None:
            return

        unit = ""

        if value_index is not None and value_index + 1 < len(values):
            unit = values[value_index + 1]

        record.add_property(
            property_name,
            PropertyValue(
                value=value,
                unit=unit,
                source_id=self.SOURCE_ID,
                original_value=" | ".join(values),
            ),
        )

    # --------------------------------------------------------
    # BATCH
    # --------------------------------------------------------

    def scrape_many(
        self,
        material_ids: list[str | int],
    ) -> list[dict]:

        records = []

        for material_id in material_ids:
            try:
                record = self.scrape_material(material_id)

                records.append(record.to_dict())

                logger.info(
                    "Scraped %s",
                    record.name,
                )

            except Exception as exc:
                logger.warning(
                    "Failed to scrape NASA material %s: %s",
                    material_id,
                    exc,
                )

        return records
