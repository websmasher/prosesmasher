#!/usr/bin/env python3
import argparse
import json
import os
import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = (
    ROOT
    / ".plans"
    / "2026-05-12-203442-textlint-implementation-plan.md.manifest.toml"
)


def load_manifest(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def rel(path: str) -> Path:
    return ROOT / path


def package_json(package_path: str) -> dict | None:
    path = rel(package_path) / "package.json"
    if not path.exists():
        return None
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def fail(message: str, failures: list[str]) -> None:
    failures.append(f"FAIL {message}")


def verify_tree(manifest: dict) -> list[str]:
    failures: list[str] = []
    for row in manifest.get("tree", []):
        path = rel(row["path"])
        if not path.exists():
            fail(f"missing path: {row['path']}", failures)
    return failures


def verify_package(manifest: dict) -> list[str]:
    failures: list[str] = []
    package = manifest["package"]
    pkg = package_json(package["path"])
    if pkg is None:
        fail(f"missing package.json: {package['path']}/package.json", failures)
        return failures

    if pkg.get("name") != package["name"]:
        fail(
            f"package name mismatch: expected {package['name']}, got {pkg.get('name')}",
            failures,
        )

    if pkg.get("type") != package["type"]:
        fail(
            f"package type mismatch: expected {package['type']}, got {pkg.get('type')}",
            failures,
        )

    if pkg.get("private") != package["private"]:
        fail(
            f"package private mismatch: expected {package['private']}, got {pkg.get('private')}",
            failures,
        )

    deps = pkg.get("dependencies", {})
    dev_deps = pkg.get("devDependencies", {})
    all_deps = {**deps, **dev_deps}

    for dep in package.get("runtime_dependencies", []):
        if dep not in deps:
            fail(f"missing runtime dependency: {dep}", failures)

    for dep in package.get("dev_dependencies", []):
        if dep not in dev_deps:
            fail(f"missing dev dependency: {dep}", failures)

    for dep in package.get("forbidden_dependencies", []):
        if dep in all_deps:
            fail(f"forbidden dependency present: {dep}", failures)

    g3ts = manifest.get("g3ts", {})
    if g3ts.get("required") and not rel(g3ts["config_path"]).exists():
        fail(f"missing G3TS config: {g3ts['config_path']}", failures)

    return failures


def verify_rules(manifest: dict) -> list[str]:
    failures: list[str] = []
    rule_ids = set()
    for row in manifest.get("rule", []):
        rule_ids.add(row["id"])
        if not rel(row["path"]).exists():
            fail(f"missing rule file: {row['path']}", failures)
        data_path = row.get("data_path")
        if data_path and not rel(data_path).exists():
            fail(f"missing rule data file: {data_path}", failures)

    for row in manifest.get("shared_file", []):
        if not rel(row["path"]).exists():
            fail(f"missing shared file: {row['path']}", failures)

    for row in manifest.get("preset", []):
        if not rel(row["path"]).exists():
            fail(f"missing preset file: {row['path']}", failures)
        for rule_id in row.get("rules", []):
            if rule_id not in rule_ids:
                fail(f"preset {row['id']} references unknown rule: {rule_id}", failures)

    return failures


def verify_regex_guardrails(manifest: dict) -> list[str]:
    failures: list[str] = []
    package = manifest["package"]
    eslint_files = [
        rel(package["path"]) / "eslint.config.js",
        rel(package["path"]) / "eslint.config.mjs",
        rel(package["path"]) / "eslint.config.cjs",
    ]
    existing = [path for path in eslint_files if path.exists()]
    if not existing:
        fail(f"missing ESLint config in {package['path']}", failures)
        return failures

    text = "\n".join(path.read_text(encoding="utf-8") for path in existing)
    for row in manifest.get("regex_guardrail", []):
        if row["kind"] == "syntax" and row["selector"] not in text:
            fail(f"missing regex syntax guardrail selector: {row['selector']}", failures)
        if row["kind"] == "import" and row["package"] not in text:
            fail(f"missing regex import guardrail package: {row['package']}", failures)

    scan_roots = {
        row["scope"]
        for row in manifest.get("regex_guardrail", [])
        if row.get("scope")
    }
    regex_markers = ["/", "RegExp("]
    for root in scan_roots:
        root_path = rel(root)
        if not root_path.exists():
            continue
        for source in root_path.rglob("*"):
            if source.suffix not in {".ts", ".tsx", ".js", ".jsx", ".mjs"}:
                continue
            content = source.read_text(encoding="utf-8")
            if "RegExp(" in content or "new RegExp" in content:
                fail(f"regex constructor found in guarded source: {source.relative_to(ROOT)}", failures)
            # Regex literal detection is intentionally delegated to ESLint AST.
            _ = regex_markers

    return failures


def verify_behavior(manifest: dict) -> list[str]:
    failures: list[str] = []
    for row in manifest.get("fixture_family", []):
        if not rel(row["fixture"]).exists():
            fail(f"missing fixture markdown: {row['fixture']}", failures)
        if not rel(row["metadata"]).exists():
            fail(f"missing fixture metadata: {row['metadata']}", failures)

    for row in manifest.get("behavior_script", []):
        path = rel(row["path"])
        if not path.exists():
            fail(f"missing behavior script: {row['path']}", failures)
            continue
        if not os.access(path, os.X_OK):
            fail(f"behavior script is not executable: {row['path']}", failures)

    return failures


def run_layer(layer: str, manifest: dict) -> list[str]:
    if layer == "1":
        return verify_tree(manifest)
    if layer == "2":
        return verify_package(manifest)
    if layer == "3":
        return verify_rules(manifest)
    if layer == "4":
        return verify_regex_guardrails(manifest)
    if layer == "5":
        return verify_behavior(manifest)
    if layer == "all":
        failures: list[str] = []
        for item in ["1", "2", "3", "4", "5"]:
            failures.extend(run_layer(item, manifest))
        return failures
    raise ValueError(f"unknown layer: {layer}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("layer", choices=["1", "2", "3", "4", "5", "all"])
    parser.add_argument("--manifest", default=str(DEFAULT_MANIFEST))
    args = parser.parse_args()

    manifest_path = Path(args.manifest)
    manifest = load_manifest(manifest_path)
    failures = run_layer(args.layer, manifest)

    if failures:
        for item in failures:
            print(item)
        return 1

    print(f"PASS layer {args.layer}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
