#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import Counter
from pathlib import Path


DEFAULT_MODELS = (
    "gpt_5_2_chat",
    "gpt_5_4",
    "gpt_5_4_mini",
    "haiku",
    "opus_4_6",
    "sonnet_4_6",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Snapshot and compare current failure counts for generated article fixtures."
    )
    parser.add_argument("command", choices=("snapshot", "compare"))
    parser.add_argument(
        "--binary",
        default="apps/prosesmasher/target/debug/prosesmasher",
        help="Path to the built prosesmasher binary.",
    )
    parser.add_argument(
        "--fixtures-root",
        default="fixtures",
        help="Root directory containing generated model fixture folders.",
    )
    parser.add_argument(
        "--preset",
        default="general-en",
        help="Preset name used for snapshot and compare.",
    )
    parser.add_argument(
        "--model",
        action="append",
        dest="models",
        help="Restrict to one or more model folders. Defaults to all generated-model folders.",
    )
    return parser.parse_args()


def iter_articles(fixtures_root: Path, models: tuple[str, ...]) -> list[Path]:
    articles: list[Path] = []
    for model in models:
        model_root = fixtures_root / model
        if not model_root.exists():
            continue
        articles.extend(sorted(model_root.glob("*/article.md")))
    return articles


def sidecar_path(article: Path, preset: str) -> Path:
    return article.with_suffix(f".baseline.{preset}.json")


def failure_hit_count(failure: dict[str, object]) -> int:
    observed = failure.get("observed")
    if isinstance(observed, int):
        return observed

    evidence = failure.get("evidence")
    if isinstance(evidence, list):
        return len(evidence)

    return 1


def run_cli(binary: Path, article: Path, preset: str) -> Counter[str]:
    proc = subprocess.run(
        [str(binary), "check", str(article), "--preset", preset, "--format", "json"],
        capture_output=True,
        text=True,
    )
    if proc.returncode not in (0, 1):
        print(f"FAILED {article}: exit {proc.returncode}", file=sys.stderr)
        if proc.stderr:
            print(proc.stderr, file=sys.stderr)
        raise SystemExit(1)

    data = json.loads(proc.stdout)
    counter: Counter[str] = Counter()
    for failure in data.get("failures", []):
        rule_id = failure.get("id")
        if not isinstance(rule_id, str):
            continue
        counter[rule_id] += failure_hit_count(failure)
    return counter


def serialize_counter(counter: Counter[str]) -> list[dict[str, int | str]]:
    return [
        {"rule": rule, "count": count}
        for rule, count in sorted(counter.items(), key=lambda item: item[0])
    ]


def write_snapshot(binary: Path, article: Path, preset: str) -> None:
    counter = run_cli(binary, article, preset)
    payload = {
        "preset": preset,
        "failures": serialize_counter(counter),
    }
    sidecar = sidecar_path(article, preset)
    sidecar.write_text(json.dumps(payload, indent=2) + "\n")
    print(sidecar)


def load_snapshot(article: Path, preset: str) -> Counter[str]:
    sidecar = sidecar_path(article, preset)
    data = json.loads(sidecar.read_text())
    return Counter({item["rule"]: item["count"] for item in data.get("failures", [])})


def compare(binary: Path, article: Path, preset: str) -> dict[str, object] | None:
    before = load_snapshot(article, preset)
    after = run_cli(binary, article, preset)

    changed_rules = sorted(set(before) | set(after))
    added = []
    removed = []
    changed = []

    for rule in changed_rules:
        old_count = before.get(rule, 0)
        new_count = after.get(rule, 0)
        if old_count == new_count:
            continue
        record = {"rule": rule, "before": old_count, "after": new_count}
        if old_count == 0:
            added.append(record)
        elif new_count == 0:
            removed.append(record)
        else:
            changed.append(record)

    if not (added or removed or changed):
        return None

    return {
        "article": str(article),
        "added": added,
        "removed": removed,
        "changed": changed,
    }


def main() -> int:
    args = parse_args()
    fixtures_root = Path(args.fixtures_root)
    binary = Path(args.binary)
    models = tuple(args.models) if args.models else DEFAULT_MODELS
    articles = iter_articles(fixtures_root, models)

    if not articles:
        print("No article.md fixtures found for the selected models.", file=sys.stderr)
        return 1

    if args.command == "snapshot":
        for article in articles:
            write_snapshot(binary, article, args.preset)
        return 0

    differences = [
        diff
        for article in articles
        if (diff := compare(binary, article, args.preset)) is not None
    ]
    print(json.dumps(differences, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
