from __future__ import annotations

import argparse
from pathlib import Path

from common import configure_logging, save_json

from nasa_tpsx import NASATPSXScraper
from nist import NISTScraper
from matweb import MatWebImporter


ROOT = Path(__file__).resolve().parents[2]

RAW_DIR = ROOT / "data" / "raw"

SOURCES_FILE = ROOT / "data" / "sources.json"


def save_source(
    source,
) -> None:

    SOURCES_FILE.parent.mkdir(
        parents=True,
        exist_ok=True,
    )

    existing = {}

    if SOURCES_FILE.exists():
        from common import load_json

        existing = load_json(SOURCES_FILE)

    existing[source.source_reference().source_id] = source.source_reference().to_dict()

    save_json(
        existing,
        SOURCES_FILE,
    )


def run_nasa(
    ids: list[str],
) -> None:

    scraper = NASATPSXScraper()

    save_source(scraper)

    records = scraper.scrape_many(ids)

    output = RAW_DIR / "nasa_tpsx.json"

    save_json(
        records,
        output,
    )

    print(f"Saved {len(records)} NASA records to {output}")


def run_nist(
    identifiers: list[str],
) -> None:

    scraper = NISTScraper()

    save_source(scraper)

    records = scraper.scrape_many(identifiers)

    output = RAW_DIR / "nist.json"

    save_json(
        records,
        output,
    )

    print(f"Saved {len(records)} NIST records to {output}")


def run_matweb(
    csv_path: str,
) -> None:

    importer = MatWebImporter()

    save_source(importer)

    records = importer.import_csv(csv_path)

    output = RAW_DIR / "matweb.json"

    save_json(
        records,
        output,
    )

    print(f"Saved {len(records)} MatWeb records to {output}")


def main():

    parser = argparse.ArgumentParser(
        description=("MaterialSpace material data acquisition")
    )

    subparsers = parser.add_subparsers(
        dest="source",
        required=True,
    )

    # NASA
    nasa = subparsers.add_parser("nasa")

    nasa.add_argument(
        "ids",
        nargs="+",
        help="NASA TPSX material IDs",
    )

    # NIST
    nist = subparsers.add_parser("nist")

    nist.add_argument(
        "identifiers",
        nargs="+",
        help="NIST substance IDs",
    )

    # MatWeb
    matweb = subparsers.add_parser("matweb")

    matweb.add_argument(
        "csv",
        help="Permitted local CSV",
    )

    args = parser.parse_args()

    configure_logging()

    RAW_DIR.mkdir(
        parents=True,
        exist_ok=True,
    )

    if args.source == "nasa":
        run_nasa(args.ids)

    elif args.source == "nist":
        run_nist(args.identifiers)

    elif args.source == "matweb":
        run_matweb(args.csv)


if __name__ == "__main__":
    main()
