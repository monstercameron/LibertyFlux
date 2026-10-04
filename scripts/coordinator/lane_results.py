"""Check lanes' results.json rows against lane_results.schema.json. Standard library only; warns, never blocks.

The schema beside this file describes one row. This module implements the part of JSON Schema (2020-12) the
schema uses: type (one name or a list), enum, const, required, properties, items, minimum, minLength, pattern,
allOf and if/then. Annotation keywords (description, title, $comment and the like) are ignored. Any other
keyword makes `unsupported_keywords` name it, and a test fails, so the schema cannot quietly use a rule this
validator does not enforce.

Usage: python lane_results.py <results.json> [...]    print the problems in each file
"""

import json
import os
import re
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

SCHEMA_PATH = Path(__file__).resolve().parent / "lane_results.schema.json"
ANNOTATIONS = {"$schema", "$id", "$comment", "title", "description", "examples", "default"}
SUPPORTED = {"type", "enum", "const", "required", "properties", "items", "minimum", "minLength", "pattern", "allOf",
             "if", "then"}
_schema = None


def load_schema():
    """The row schema, read once."""
    global _schema
    if _schema is None:
        _schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    return _schema


def unsupported_keywords(schema, path="#"):
    """Keywords in `schema` (recursively) that this validator does not enforce, as JSON-pointer-like paths."""
    found = []
    if not isinstance(schema, dict):
        return found
    for key, value in schema.items():
        if key in ANNOTATIONS:
            continue
        if key not in SUPPORTED:
            found.append(f"{path}/{key}")
        elif key == "properties":
            for name, sub in value.items():
                found += unsupported_keywords(sub, f"{path}/properties/{name}")
        elif key == "allOf":
            for i, sub in enumerate(value):
                found += unsupported_keywords(sub, f"{path}/allOf/{i}")
        elif key in ("items", "if", "then"):
            found += unsupported_keywords(value, f"{path}/{key}")
    return found


def is_type(value, name):
    """JSON type test: booleans are not numbers, and integers are numbers."""
    if name == "null":
        return value is None
    if name == "boolean":
        return isinstance(value, bool)
    if name == "integer":
        return isinstance(value, int) and not isinstance(value, bool)
    if name == "number":
        return isinstance(value, (int, float)) and not isinstance(value, bool)
    if name == "string":
        return isinstance(value, str)
    if name == "array":
        return isinstance(value, list)
    if name == "object":
        return isinstance(value, dict)
    raise ValueError(f"unknown type {name!r}")


def validate(value, schema, path=""):
    """Problems with `value` under `schema`, as a list of (path, message); empty when it conforms."""
    problems = []
    kinds = schema.get("type")
    if kinds is not None:
        kinds = [kinds] if isinstance(kinds, str) else kinds
        if not any(is_type(value, k) for k in kinds):
            return [(path, f"expected {' or '.join(kinds)}, found {type(value).__name__} {value!r:.60}")]
    if "enum" in schema and value not in schema["enum"]:
        problems.append((path, f"{value!r:.60} is not one of {schema['enum']}"))
    if "const" in schema and value != schema["const"]:
        problems.append((path, f"must be {schema['const']!r}, found {value!r:.60}"))
    if isinstance(value, (int, float)) and not isinstance(value, bool) and "minimum" in schema and value < schema["minimum"]:
        problems.append((path, f"{value} is below the minimum {schema['minimum']}"))
    if isinstance(value, str):
        if "minLength" in schema and len(value) < schema["minLength"]:
            problems.append((path, f"shorter than {schema['minLength']} characters"))
        if "pattern" in schema and not re.search(schema["pattern"], value):
            problems.append((path, f"{value!r:.60} does not match {schema['pattern']}"))
    if isinstance(value, dict):
        for name in schema.get("required", []):
            if name not in value:
                problems.append((f"{path}/{name}", "missing"))
        for name, sub in schema.get("properties", {}).items():
            if name in value:
                problems += validate(value[name], sub, f"{path}/{name}")
    if isinstance(value, list) and "items" in schema:
        for i, item in enumerate(value):
            problems += validate(item, schema["items"], f"{path}/{i}")
    for sub in schema.get("allOf", []):
        problems += validate(value, sub, path)
    if "if" in schema and not validate(value, schema["if"], path):
        problems += validate(value, schema.get("then", {}), path)
    return problems


def validate_rows(rows, schema=None):
    """Problems in a lane's rows: (row number, field path, message). Rows that are not objects are problems too."""
    schema = schema or load_schema()
    problems = []
    for i, row in enumerate(rows):
        problems += [(i, field or "/", message) for field, message in validate(row, schema)]
    return problems


def validate_file(path):
    """Problems in one results.json (list or object-with-list shape); an unreadable file is one problem."""
    try:
        data = json.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, ValueError) as error:
        return [(None, "/", f"unreadable: {error}")]
    if not isinstance(data, list) and not common.load_rows(data, "functions", "results"):
        return [(None, "/", "not a list of rows, nor an object holding one under functions or results")]
    return validate_rows(common.load_rows(data, "functions", "results"))


def summarise(lane, problems, limit=3):
    """One warning line for a lane: how many problems, how many rows, and the commonest few."""
    if not problems:
        return None
    rows = {p[0] for p in problems}
    counts = {}
    for _, field, message in problems:
        key = f"{field}: {message}"
        counts[key] = counts.get(key, 0) + 1
    common_ones = sorted(counts.items(), key=lambda kv: -kv[1])[:limit]
    return (f"schema warning: {lane}: {len(problems)} problems in {len(rows)} rows; "
            + "; ".join(f"{k} ({n})" for k, n in common_ones))


def main(argv=None):
    paths = argv if argv is not None else sys.argv[1:]
    for path in paths:
        problems = validate_file(path)
        print(summarise(Path(path).parent.name or path, problems, limit=10) or f"{path}: conforms")
    return 0  # warnings only, by design


if __name__ == "__main__":
    sys.exit(main())
