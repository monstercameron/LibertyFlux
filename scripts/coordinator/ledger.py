"""The function ledger: what the tracked rewrite tree says, checked against itself and the published counts.

Reads only tracked files, so it runs anywhere the repository is checked out (CI, a cloud session, a lane):
  rewrites/verified/index.json and the files it names    rewrites that passed the checker
  rewrites/unverified/index.json and the files it names  rewrites that exist but have not passed
  docs/data/progress.json                                the published stage counts
Optionally an inventory (`--inventory`, the queue export's functions.json or any JSON list of rows with
`address` or `start` and `kind`) restricts the verified count to game code the way apply_counts.py does.

Errors are facts that make the tree untrustworthy: an index entry without its file, a file no index names,
an address listed twice, an address both verified and unverified, a file name that disagrees with its address.
Warnings are defects worth fixing that do not change a count: a header comment naming another address,
an entry checked only by checker version 1 (which does not count as verified), a missing trial count.

An entry may carry a `proof` object (rewrites/proof_record.schema.json: checker version, worker, driver,
rewrite and contract hashes, seed, trials, the wrong version's result, what the proof does not cover).
Proof objects are optional; when present they are validated, and a malformed one, a wrong version that was
not caught, or a caught one with no failing trial is an error. A partial proof and a trial count that
disagrees with the entry's are warnings; a narrowed proof that says so is acceptable (rule 3) and is only
counted. The report says how many entries carry one.

Usage: python ledger.py            report
       python ledger.py --json     the ledger as JSON on stdout (includes every proof record under "proofs")
       python ledger.py --check    report, exit 1 if there is any error (CI)
       python ledger.py --since REF
                                   also report what changed in rewrites/verified/index.json between a git ref and
                                   the working tree: entries added, removed and changed, by kind and checker
                                   version (an import wave: --since HEAD~1, or the commit before the wave). With
                                   --json the change is under "since".
       python ledger.py --markdown a short Markdown table instead of the report, for a pull request or the devlog
                                   (with --since, a table of the change); counts only, no addresses or file names
--check keeps its meaning with every option: the exit status is 1 when the ledger has any error.
"""

import argparse
import json
import os
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
from scan import MODERN

PROOF_SCHEMA = Path(__file__).resolve().parent.parent.parent / "rewrites" / "proof_record.schema.json"
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "checker"))
from validate_contracts import SchemaValidator  # noqa: E402  (the stdlib JSON Schema subset)

HEADER = re.compile(r"//\s*original:\s*(0x[0-9A-Fa-f]+)\b")
FILE_NAME = re.compile(r"^fn_([0-9a-f]{8})\.rs$")
VERIFIED_INDEX = "rewrites/verified/index.json"
EXAMPLES = 10  # removed and changed entries named by --since's text report; --json lists them all


def header_address(text):
    """The address named by a rewrite's `// original:` comment, or None when it is missing or malformed."""
    for line in text.splitlines():
        if not line.strip():
            continue
        match = HEADER.match(line.strip())
        return int(match.group(1), 16) if match else None
    return None


def check_tree(verified, unverified, verified_files, unverified_files, read_text):
    """Integrity of both halves of the tree. Pure: the caller supplies the index rows, the file names found on
    disk (relative to each half's folder) and a reader for verified files. Returns (errors, warnings), each a
    list of (kind, detail) pairs."""
    errors, warnings = [], []
    for label, rows, on_disk in (("verified", verified, verified_files), ("unverified", unverified, unverified_files)):
        named = Counter(r.get("file") for r in rows)
        for path in sorted(p for p in named if p not in on_disk):
            errors.append(("missing file", f"{label}/{path}"))
        for path in sorted(on_disk - set(named)):
            errors.append(("file not in index", f"{label}/{path}"))
        for path, n in sorted(named.items()):
            if n > 1:
                errors.append(("file listed twice", f"{label}/{path}"))
        seen = Counter()
        for row in rows:
            try:
                address = common.va(row["address"])
            except (KeyError, ValueError, TypeError):
                errors.append(("bad address", f"{label}: {row.get('address')!r}"))
                continue
            seen[address] += 1
            match = FILE_NAME.match(os.path.basename(str(row.get("file") or "")))
            if not match or int(match.group(1), 16) != address:
                errors.append(("file name disagrees with address", f"{label}/{row.get('file')} for 0x{address:08X}"))
        for address, n in sorted(seen.items()):
            if n > 1:
                errors.append(("address listed twice", f"{label}: 0x{address:08X}"))
    both = {common.va(r["address"]) for r in verified if "address" in r} & {common.va(r["address"]) for r in unverified if "address" in r}
    for address in sorted(both):
        errors.append(("both verified and unverified", f"0x{address:08X}"))
    for row in verified:
        path = row.get("file")
        if path not in verified_files:
            continue
        found = header_address(read_text(path))
        if found is None:
            warnings.append(("header missing or malformed", f"verified/{path}"))
        elif found != common.va(row["address"]):
            warnings.append(("header names another address", f"verified/{path}"))
        if row.get("checker") not in MODERN:
            warnings.append(("checked only by checker version 1", f"verified/{path}"))
        if not isinstance(row.get("trials"), int) or row["trials"] <= 0:
            warnings.append(("no trial count", f"verified/{path}"))
    return errors, warnings


def load_proof_schema(path=PROOF_SCHEMA):
    """The proof-record schema, or None when the file is absent (then proof objects are only counted)."""
    try:
        return json.loads(Path(path).read_text(encoding="utf-8"))
    except OSError:
        return None


def check_proofs(verified, schema):
    """Validate the optional `proof` object of each verified entry. Pure. Returns (errors, warnings, records):
    errors and warnings as (kind, detail) pairs like check_tree's, records as one dict per entry that carries
    a proof (address, file, proof)."""
    errors, warnings, records = [], [], []
    validator = SchemaValidator(schema) if schema else None
    for row in verified:
        if "proof" not in row:
            continue
        proof, where = row["proof"], f"verified/{row.get('file')}"
        records.append({"address": row.get("address"), "file": row.get("file"), "proof": proof})
        problems = validator.validate(proof) if validator else []
        if problems:
            code, path, message = problems[0]
            more = f" (+{len(problems) - 1} more)" if len(problems) > 1 else ""
            errors.append(("bad proof record", f"{where}: {path}: {message}{more}"))
            continue
        if not isinstance(proof, dict):
            continue
        mutant = proof.get("mutant") if isinstance(proof.get("mutant"), dict) else {}
        if mutant.get("caught") is False:
            errors.append(("proof: wrong version not caught", where))
        elif mutant.get("caught") is True and mutant.get("fails") == 0:
            errors.append(("proof: wrong version caught with no failing trial", where))
        if proof.get("partial") is True:
            warnings.append(("proof: partial", where))
        if isinstance(row.get("trials"), int) and isinstance(proof.get("trials"), int) and proof["trials"] != row["trials"]:
            warnings.append(("proof: trial count differs from the entry's", f"{where}: {proof.get('trials')} vs {row['trials']}"))
    return errors, warnings, records


def summarise_proofs(verified, records):
    """How many verified entries carry a proof record, and what those records say."""
    proofs = [r["proof"] for r in records if isinstance(r["proof"], dict)]
    mutants = [p.get("mutant") for p in proofs if isinstance(p.get("mutant"), dict)]
    return {
        "entries": len(verified),
        "with_proof": len(records),
        "without_proof": len(verified) - len(records),
        "partial": sum(1 for p in proofs if p.get("partial") is True),
        "narrowed": sum(1 for p in proofs if p.get("narrowing")),
        "mutant_caught": sum(1 for m in mutants if m.get("caught") is True),
        "by_checker": dict(sorted(Counter(str(p.get("checker_version")) for p in proofs).items())),
    }


def summarise(verified, unverified, game=None):
    """Counts derived from the index rows. `game`, when given, is the set of game-code addresses: verified then
    means checker version 2 or later and game code, the definition apply_counts.py publishes."""
    modern = {common.va(r["address"]) for r in verified if r.get("checker") in MODERN}
    summary = {
        "verified_entries": len(verified),
        "by_kind": dict(sorted(Counter(r.get("kind") for r in verified).items(), key=lambda kv: str(kv[0]))),
        "by_checker": dict(sorted(Counter(r.get("checker") for r in verified).items(), key=lambda kv: str(kv[0]))),
        "modern_checker": len(modern),
        "unverified_entries": len(unverified),
        "unverified_by_outcome": dict(sorted(Counter(r.get("outcome") for r in unverified).items(), key=lambda kv: str(kv[0]))),
        "unverified_by_reason": dict(sorted(Counter(r.get("reason") or "none recorded" for r in unverified).items())),
        "lanes": len({r.get("lane") for r in verified}),
    }
    if game is not None:
        summary["verified_game"] = len(modern & game)
        summary["verified_outside_game"] = len(modern - game)
    return summary


def reconcile(summary, stages):
    """How the published counts relate to the tree. Returns a list of (fact, published, derived, note)."""
    rows = []
    published = stages.get("verified")
    derived = summary.get("verified_game", summary["modern_checker"])
    note = ("same definition" if "verified_game" in summary
            else "no inventory given: derived count includes any entry outside the game set")
    if published != derived:
        note += f"; differs by {derived - published:+d}"
    rows.append(("verified", published, derived, note))
    rows.append(("rewritten at least", stages.get("rewritten"), summary["verified_entries"],
                 "published must be at least the tree's entries" if (stages.get("rewritten") or 0) < summary["verified_entries"] else "ok"))
    return rows


def load_index(folder):
    path = folder / "index.json"
    return json.loads(path.read_text(encoding="utf-8")) if path.exists() else []


def files_under(folder):
    if not folder.exists():
        return set()
    return {str(p.relative_to(folder)).replace(os.sep, "/") for p in folder.rglob("*.rs")}


def load_game_set(path):
    rows = common.load_rows(json.loads(open(path, encoding="utf-8").read()), "functions")
    return {common.va(r.get("address") or r["start"]) for r in rows if r.get("kind") not in ("library", "runtime")}


def build(root, inventory=None):
    verified_dir, unverified_dir = root / "rewrites" / "verified", root / "rewrites" / "unverified"
    verified, unverified = load_index(verified_dir), load_index(unverified_dir)
    errors, warnings = check_tree(verified, unverified, files_under(verified_dir), files_under(unverified_dir),
                                  lambda p: (verified_dir / p).read_text(encoding="utf-8", errors="replace"))
    summary = summarise(verified, unverified, load_game_set(inventory) if inventory else None)
    proof_errors, proof_warnings, proofs = check_proofs(verified, load_proof_schema(root / "rewrites" / "proof_record.schema.json")
                                                         or load_proof_schema())
    errors, warnings = errors + proof_errors, warnings + proof_warnings
    summary["proofs"] = summarise_proofs(verified, proofs)
    progress = json.loads((root / "docs" / "data" / "progress.json").read_text(encoding="utf-8"))
    return {"summary": summary, "published": progress.get("stages", {}),
            "reconcile": [dict(zip(("fact", "published", "derived", "note"), r)) for r in reconcile(summary, progress.get("stages", {}))],
            "errors": [dict(kind=k, detail=d) for k, d in errors], "warnings": [dict(kind=k, detail=d) for k, d in warnings],
            "proofs": proofs}


def report(ledger):
    s = ledger["summary"]
    print(f"verified tree: {s['verified_entries']} entries from {s['lanes']} lanes; by kind {s['by_kind']}")
    print(f"  by checker {s['by_checker']}; version 2 or later {s['modern_checker']}")
    if "verified_game" in s:
        print(f"  game code {s['verified_game']}, outside the game set {s['verified_outside_game']}")
    print(f"unverified tree: {s['unverified_entries']} entries; by outcome {s['unverified_by_outcome']}")
    print(f"  by reason {s['unverified_by_reason']}")
    if "proofs" in s:
        p = s["proofs"]
        print(f"proof records: {p['with_proof']} of {p['entries']} verified entries carry one; partial {p['partial']}, "
              f"narrowed {p['narrowed']}, wrong version caught {p['mutant_caught']}")
    for row in ledger["reconcile"]:
        print(f"published {row['fact']}: {row['published']} | from the tree: {row['derived']} | {row['note']}")
    for label in ("errors", "warnings"):
        items = ledger[label]
        print(f"{label}: {len(items)}")
        for kind, n in Counter(i["kind"] for i in items).most_common():
            examples = [i["detail"] for i in items if i["kind"] == kind][:3]
            print(f"  {kind}: {n} (e.g. {', '.join(examples)})")


def index_at(root, ref, relative=VERIFIED_INDEX):
    """(rows, present) of an index as committed at a git ref: present is False, with no rows, when the ref has no
    such file. Raises ValueError when the ref does not name a commit."""
    def git(*args):
        return subprocess.run(["git", "-C", str(root), *args], capture_output=True)
    if git("rev-parse", "--verify", "--quiet", f"{ref}^{{commit}}").returncode != 0:
        raise ValueError(f"{ref!r} does not name a commit in this repository")
    shown = git("show", f"{ref}:{relative}")
    if shown.returncode != 0:
        return [], False
    return json.loads(shown.stdout.decode("utf-8")), True


def diff_index(old, new):
    """What changed from one version of an index to another, matched by address. Pure. Returns a dict: added and
    removed (lists of rows) and changed (a list of (old row, new row, sorted names of the fields that differ)).
    Rows without a readable address are left out; the ledger's own checks report them."""
    def by_address(rows):
        keyed = {}
        for row in rows:
            try:
                keyed[common.va(row["address"])] = row
            except (KeyError, ValueError, TypeError):
                continue
        return keyed
    before, after = by_address(old), by_address(new)
    changed = []
    for address in sorted(before.keys() & after.keys()):
        fields = sorted(k for k in before[address].keys() | after[address].keys()
                        if k != "address" and before[address].get(k) != after[address].get(k))
        if fields:
            changed.append((before[address], after[address], fields))
    return {"added": [after[a] for a in sorted(after.keys() - before.keys())],
            "removed": [before[a] for a in sorted(before.keys() - after.keys())],
            "changed": changed}


def summarise_diff(diff):
    """Counts for a diff_index result: added and removed by kind, by checker version and by both; changed by
    field and by checker transition; the lanes the added entries came from."""
    def by(rows, key):
        return dict(sorted(Counter(str(r.get(key)) for r in rows).items()))

    def by_both(rows):
        table = {}
        for row in rows:
            cell = table.setdefault(str(row.get("kind")), {})
            cell[str(row.get("checker"))] = cell.get(str(row.get("checker")), 0) + 1
        return {kind: dict(sorted(cells.items())) for kind, cells in sorted(table.items())}

    changed = diff["changed"]
    return {
        "added": len(diff["added"]), "removed": len(diff["removed"]), "changed": len(changed),
        "added_by_kind": by(diff["added"], "kind"), "added_by_checker": by(diff["added"], "checker"),
        "added_by_kind_and_checker": by_both(diff["added"]),
        "removed_by_kind": by(diff["removed"], "kind"), "removed_by_checker": by(diff["removed"], "checker"),
        "changed_fields": dict(sorted(Counter(f for _, _, fields in changed for f in fields).items())),
        "checker_transitions": {f"{a} -> {b}": n for (a, b), n in sorted(Counter(
            (str(o.get("checker")), str(n.get("checker"))) for o, n, fields in changed if "checker" in fields).items())},
        "added_by_lane": dict(Counter(str(r.get("lane")) for r in diff["added"]).most_common()),
    }


def since_record(ref, old, present, new):
    """The --since result as one JSON-ready dict: the ref, entry counts at both ends, the summary and every
    added, removed and changed entry (for a changed one, its current kind and checker and only the fields that
    differ, before and after)."""
    diff = diff_index(old, new)
    return {"ref": ref, "present_at_ref": present, "before": len(old), "after": len(new),
            "summary": summarise_diff(diff), "added": diff["added"], "removed": diff["removed"],
            "changed": [{"address": b.get("address"), "file": b.get("file"), "kind": b.get("kind"), "checker": b.get("checker"),
                         "fields": fields, "before": {f: a.get(f) for f in fields}, "after": {f: b.get(f) for f in fields}}
                        for a, b, fields in diff["changed"]]}


def report_since(record):
    """The --since section of the text report."""
    s = record["summary"]
    absent = "" if record["present_at_ref"] else ", where the index did not exist yet"
    print(f"since {record['ref']}: {record['before']} entries at the ref{absent}, {record['after']} now")
    lanes = ", ".join(f"{lane} {n}" for lane, n in list(s["added_by_lane"].items())[:EXAMPLES])
    print(f"  added {s['added']}" + (f": by kind {s['added_by_kind']}; by checker {s['added_by_checker']}; "
                                     f"from {len(s['added_by_lane'])} lanes ({lanes})" if s["added"] else ""))
    print(f"  removed {s['removed']}" + (f": by kind {s['removed_by_kind']}; by checker {s['removed_by_checker']} (e.g. "
                                         f"{', '.join(str(r.get('file')) for r in record['removed'][:EXAMPLES])})" if s["removed"] else ""))
    transitions = "; checker " + ", ".join(f"{k}: {n}" for k, n in s["checker_transitions"].items()) if s["checker_transitions"] else ""
    print(f"  changed {s['changed']}" + (f": fields {s['changed_fields']}{transitions} (e.g. "
                                         f"{', '.join(str(c['file']) for c in record['changed'][:EXAMPLES])})" if s["changed"] else ""))


def markdown_since(record):
    """A Markdown table of the change since a ref: one row each for added, removed and changed (a changed entry
    counted under its new kind and checker), one column per kind and per checker version. Counts only."""
    s = record["summary"]
    rows = {"Added": record["added"], "Removed": record["removed"], "Changed": record["changed"]}
    kinds = sorted({str(r.get("kind")) for v in rows.values() for r in v})
    checkers = sorted({str(r.get("checker")) for v in rows.values() for r in v})
    head = ["Since `" + record["ref"] + "`", "Entries"] + kinds + checkers
    lines = ["| " + " | ".join(head) + " |", "|---|" + "---:|" * (len(head) - 1)]
    for label, items in rows.items():
        kind_counts = Counter(str(r.get("kind")) for r in items)
        checker_counts = Counter(str(r.get("checker")) for r in items)
        cells = [label, str(len(items))] + [str(kind_counts[k]) for k in kinds] + [str(checker_counts[c]) for c in checkers]
        lines.append("| " + " | ".join(cells) + " |")
    lines.append("")
    lines.append(f"Verified index: {record['before']} entries at `{record['ref']}`, {record['after']} now; "
                 f"the added entries come from {len(s['added_by_lane'])} lanes.")
    if s["checker_transitions"]:
        lines.append("Checker changes: " + ", ".join(f"{k} ({n})" for k, n in s["checker_transitions"].items()) + ".")
    return "\n".join(lines)


def markdown_ledger(ledger):
    """The ledger's summary as a two-column Markdown table. Counts only."""
    s = ledger["summary"]
    rows = [("Verified entries", s["verified_entries"])]
    rows += [(f"by kind: {k}", n) for k, n in s["by_kind"].items()]
    rows += [(f"by checker: {k}", n) for k, n in s["by_checker"].items()]
    rows.append(("Checked by checker version 2 or later", s["modern_checker"]))
    if "verified_game" in s:
        rows.append(("Game code (version 2 or later)", s["verified_game"]))
    rows.append(("Unverified entries", s["unverified_entries"]))
    if "proofs" in s:
        rows.append(("Entries with a proof record", s["proofs"]["with_proof"]))
    rows += [(f"Published {r['fact']} / from the tree", f"{r['published']} / {r['derived']}") for r in ledger["reconcile"]]
    rows += [("Errors", len(ledger["errors"])), ("Warnings", len(ledger["warnings"]))]
    return "\n".join(["| Ledger | Count |", "|---|---:|"] + [f"| {label} | {value} |" for label, value in rows])


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    output = parser.add_mutually_exclusive_group()
    output.add_argument("--json", action="store_true", help="print the ledger as JSON")
    parser.add_argument("--check", action="store_true", help="exit 1 when there is any error")
    parser.add_argument("--inventory", help="JSON rows with address/start and kind, to restrict to game code")
    parser.add_argument("--since", metavar="REF", help="also report what changed in the verified index since a git ref")
    output.add_argument("--markdown", action="store_true", help="a short Markdown table instead of the report")
    args = parser.parse_args(argv)
    root = common.find_root()
    ledger = build(root, args.inventory)
    since = None
    if args.since:
        try:
            old, present = index_at(root, args.since)
        except ValueError as problem:
            print(f"ledger.py: {problem}", file=sys.stderr)
            return 2
        since = since_record(args.since, old, present, load_index(root / "rewrites" / "verified"))
    if args.json:
        print(json.dumps(dict(ledger, since=since) if since else ledger, indent=1))
    elif args.markdown:
        print(markdown_since(since) if since else markdown_ledger(ledger))
    else:
        report(ledger)
        if since:
            report_since(since)
    return 1 if args.check and ledger["errors"] else 0


if __name__ == "__main__":
    sys.exit(main())
