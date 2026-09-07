from __future__ import annotations

import csv
import logging
from pathlib import Path

from common import MaterialRecord, PropertyValue, SourceReference, parse_float, slugify

logger = logging.getLogger(__name__)


class MatWebImporter:
    """
    Imports a locally available CSV containing permitted MatWeb data.

    This is not intended to circumvent MatWeb's access controls,
    licensing, or terms.
    """

    SOURCE_ID = "matweb"

    def source_reference(
        self,
    ) -> SourceReference:

        return SourceReference(
            source_id=self.SOURCE_ID,
            organization="MatWeb",
            title="MatWeb Material Property Data",
            url="https://www.matweb.com/",
            notes=(
                "Use only data obtained and used "
                "in accordance with applicable "
                "MatWeb terms and permissions."
            ),
        )

    def import_csv(
        self,
        path: str | Path,
    ) -> list[dict]:

        path = Path(path)

        if not path.exists():
            raise FileNotFoundError(path)

        records = []

        with path.open(
            "r",
            encoding="utf-8-sig",
            newline="",
        ) as file:
            reader = csv.DictReader(file)

            for row_number, row in enumerate(
                reader,
                start=2,
            ):
                try:
                    record = self.parse_row(row)

                    if record:
                        records.append(record.to_dict())

                except Exception as exc:
                    logger.warning(
                        "Could not parse MatWeb row %d: %s",
                        row_number,
                        exc,
                    )

        return records

    def parse_row(
        self,
        row: dict,
    ) -> MaterialRecord | None:

        name = row.get("name") or row.get("Name")

        if not name:
            return None

        category = row.get("category") or row.get("Category") or "unknown"

        grade = row.get("grade") or row.get("Grade")

        record = MaterialRecord(
            material_id=slugify(name),
            name=name.strip(),
            category=category.strip(),
            grade=(grade.strip() if grade else None),
            source_material_id=(row.get("source_id")),
            source_url=(row.get("source_url")),
        )

        self.add_property(
            record,
            row,
            "density",
        )

        self.add_property(
            record,
            row,
            "thermal_conductivity",
        )

        self.add_property(
            record,
            row,
            "specific_heat",
        )

        self.add_property(
            record,
            row,
            "youngs_modulus",
        )

        self.add_property(
            record,
            row,
            "yield_strength",
        )

        self.add_property(
            record,
            row,
            "thermal_expansion",
        )

        return record

    def add_property(
        self,
        record: MaterialRecord,
        row: dict,
        property_name: str,
    ) -> None:

        value = parse_float(row.get(property_name))

        if value is None:
            return

        unit = row.get(f"{property_name}_unit") or ""

        source_id = row.get(f"{property_name}_source_id") or self.SOURCE_ID

        record.add_property(
            property_name,
            PropertyValue(
                value=value,
                unit=unit,
                source_id=source_id,
            ),
        )
