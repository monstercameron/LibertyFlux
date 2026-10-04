"""Quality data: write docs/data/quality.json, the numbers behind the site's quality page.

Usage: python scripts/site/quality.py [--build PATH] [--check]

The page answers "how much does verified mean?". Every number on it comes from a tracked source, read here
and reduced to counts and shares:

  rewrites/verified/index.json     rewrites that passed the checker, by kind and checker version, trial counts
  rewrites/unverified/index.json   rewrites that exist but have not passed, by outcome and deferral reason
  rewrites/review/issues.json      the issue log (scripts/coordinator/issue_log.py): issues by severity,
                                   category, source and check; files with a high-severity issue; the
                                   patterns found in nearly every file; seven named problem classes

and, when it exists, the build check's result (scripts/ci/check_rewrites_build.py, default
.artifacts/build/rewrite-check/result.json; not tracked, so the section is left out on a machine that has not
run the check): how many verified rewrites compile from the repository alone, and why the rest do not.

The index counts reuse scripts/coordinator/ledger.py (load_index and summarise), so the page and the ledger
cannot disagree. Nothing per file is written: no file name, no address, no lane name, no issue text written by
a lane. Labels that come from the data (outcomes, deferral reasons, check names) are kept only when they are
short identifiers; anything else is counted under "other". The finished document is scanned before it is
written and refused if it holds anything that looks like an address, a rewrite's file name or a machine path.

The seven problem classes are matched by words in each issue's title (and, for the build check, by the
compiler's error code), so their counts are approximate and a file can be in several classes. The patterns
are in CLASSES below; the page says so beside the counts.

The published stage counts are not copied here: the page reads docs/data/progress.json itself, which the tick
rewrites every few minutes. The output depends only on the sources above, so it changes only when they do. --check exits 1 if quality.json is
out of date and writes nothing.

The coordinator refreshes it after the issue log, the rewrite indexes or the build check change, and commits
docs/data/quality.json with them:  python scripts/site/quality.py
"""

import argparse
import json
import re
import subprocess
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
OUT = ROOT / "docs" / "data" / "quality.json"
BUILD_RESULT = ROOT / ".artifacts" / "build" / "rewrite-check" / "result.json"
SOURCES = {
    "verified": "rewrites/verified/index.json",
    "unverified": "rewrites/unverified/index.json",
    "issues": "rewrites/review/issues.json",
}
SEVERITIES = ("high", "medium", "low")
# Deferral reasons a lane may give (scripts/coordinator/lane_results.schema.json), plus the index's own fallback.
REASONS = {"indirect_call", "thread_local", "frame_pointer_args", "runtime_table", "float_register_args",
           "jump_thunk", "not_a_function", "needs_encrypted_code", "checker_gap", "other", "none recorded"}
LABEL = re.compile(r"^[a-z][a-z0-9_ -]{0,39}$")

sys.path.insert(0, str(ROOT / "scripts" / "coordinator"))
import ledger  # noqa: E402  (load_index and summarise: the same counts the ledger reports)

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devlog_feed  # noqa: E402  (publication_patterns: the machine-path rule of check_publication.py)


def title_has(pattern, unless=None):
    """A rule matching an issue whose title contains `pattern` (and not `unless`), ignoring case."""
    wanted = re.compile(pattern, re.I)
    unwanted = re.compile(unless, re.I) if unless else None
    return lambda issue: bool(wanted.search(issue.get("title", ""))) and not (
        unwanted and unwanted.search(issue.get("title", "")))


def missing_helper(issue):
    """A build failure because a name the file uses is not in the shared runtime, or a review saying so."""
    if issue.get("source") == "build":
        return bool(re.search(r"E04(?:33|32|25|12)\b|cannot find|unresolved (?:import|module)", issue.get("detail", "")))
    return issue.get("source") == "review" and title_has(
        r"lane[- ]runtime|lane[- ]only|lf_k2_rt|untracked runtime|not defined|defined nowhere|not in the repository"
        r"|absent from the tracked|not imported|undefined helper")(issue)


# The named problem classes: (key, rule over one issue, systemic pattern code or None). The page explains each.
CLASSES = (
    ("lane-runtime", missing_helper, None),
    ("stack-cookie", title_has(r"\bcookie\b|cookie[- ]check|stack-cookie|security[- ]cookie|/GS\b",
                               unless=r"undefined helper"), None),
    ("out-buffer", title_has(r"out-?(?:buffer|frame)|writes at least|fills at least|output reaches"), None),
    ("float-register", title_has(r"(?:float|ST0|x87|f32|f64|sin|cos)[^.;]{0,60}\bEAX\b|\bEAX\b[^.;]{0,60}(?:float|ST0|x87)"
                                 r"|returns in ST0|float-in-ST0|XMM0 float return|declared as returning u32"
                                 r"|float result"), None),
    ("partial-proof", lambda issue: issue.get("category") == "narrow-proof", None),
    ("misaligned-deref", lambda issue: issue.get("source") == "lint:misaligned-deref" or (
        issue.get("source") == "review" and title_has(r"misalign|odd offset|not a multiple of 4")(issue)), "plain-deref"),
    ("debug-overflow", lambda issue: issue.get("category") == "panic" and title_has(
        r"overflow|checked \+|\+ on address|plain [`(]?\w*[^.;]{0,30}\+|arithmetic")(issue), "debug-overflow"),
)

# Why a verified rewrite does not compile from the repository alone, by the compiler's first error.
BUILD_FAMILIES = (
    ("missing-helper", re.compile(r"E04(?:33|32|25|12)\b|cannot find|unresolved")),
    ("duplicate-name", re.compile(r"E0428\b|defined multiple times")),
    ("syntax-or-edition", re.compile(r"reserved keyword|delimiter|expected (?:item|identifier|expression)|unsafe attribute")),
)


def label(value, allowed=None):
    """A data label kept as it is when it is a short identifier (or in `allowed`); otherwise "other"."""
    text = "none recorded" if value is None else str(value)
    if allowed is not None:
        return text if text in allowed else "other (unlisted)"
    return text if LABEL.match(text) else "other (unlisted)"


def relabel(counts, allowed=None):
    out = Counter()
    for key, n in counts.items():
        out[label(None if key in (None, "None") else key, allowed)] += n
    return dict(sorted(out.items(), key=lambda kv: (-kv[1], kv[0])))


def ordered(counter, order):
    """A Counter as a dict in a fixed key order, keeping keys outside `order` after it."""
    keys = [k for k in order if k in counter] + sorted(k for k in counter if k not in order)
    return {k: counter[k] for k in keys}


def tree_section(verified, unverified):
    """Counts from the two rewrite indexes, through the ledger's own summary."""
    summary = ledger.summarise(verified, unverified)
    trials = [r["trials"] for r in verified if isinstance(r.get("trials"), int)]
    schema = ledger.load_proof_schema(ROOT / "rewrites" / "proof_record.schema.json")
    _, _, records = ledger.check_proofs(verified, schema)
    proofs = ledger.summarise_proofs(verified, records)
    return {
        "verified": summary["verified_entries"],
        "by_kind": relabel(summary["by_kind"]),
        "by_checker": dict(sorted((label(k), n) for k, n in summary["by_checker"].items())),
        "modern_checker": summary["modern_checker"],
        "lanes": summary["lanes"],
        "trials": {"with_count": len(trials), "at_least_1000": sum(1 for t in trials if t >= 1000),
                   "least": min(trials) if trials else None},
        "proofs": {k: proofs[k] for k in ("with_proof", "partial", "narrowed", "mutant_caught")},
        "unverified": summary["unverified_entries"],
        "unverified_by_outcome": relabel(summary["unverified_by_outcome"]),
        "unverified_by_reason": relabel(summary["unverified_by_reason"], REASONS),
        "demoted": sum(1 for r in unverified if r.get("demoted_from")),
    }


def issues_section(log):
    """Aggregates of the issue log: totals, breakdowns, the named classes and the systemic patterns."""
    entries = [e for e in log.get("files", []) if isinstance(e, dict)]
    pairs = [(e.get("file"), i) for e in entries for i in e.get("issues", []) if isinstance(i, dict)]
    flat = [i for _, i in pairs]
    checked = log.get("checked")
    systemic = {s.get("code"): s for s in log.get("systemic", []) if isinstance(s, dict)}
    classes = []
    for key, rule, pattern in CLASSES:
        hits = [(path, i) for path, i in pairs if rule(i)]
        entry = {"key": key, "issues": len(hits), "files": len({path for path, _ in hits}),
                 "by_severity": ordered(Counter(i.get("severity") for _, i in hits), SEVERITIES),
                 "by_source": dict(sorted(Counter(str(i.get("source", "")).split(":")[0] for _, i in hits).items()))}
        if pattern and pattern in systemic:
            entry["systemic_files"] = systemic[pattern].get("files")
        classes.append(entry)
    lint_checks = Counter(str(i.get("source", ""))[5:] for i in flat if str(i.get("source", "")).startswith("lint:"))
    return {
        "checked": checked,
        "files": len(entries),
        "issues": len(flat),
        "files_with_high": sum(1 for e in entries if e.get("worst") == "high"),
        "by_severity": ordered(Counter(i.get("severity") for i in flat), SEVERITIES),
        "by_category": relabel(Counter(i.get("category") for i in flat)),
        "by_source": relabel(Counter(str(i.get("source", "")).split(":")[0] for i in flat)),
        "by_when": relabel(Counter(i.get("when") for i in flat)),
        "by_status": relabel(Counter(i.get("status") for i in flat)),
        "by_check": relabel(lint_checks),
        "classes": classes,
        "systemic": [{"code": label(s.get("code")), "title": str(s.get("title", "")), "detail": str(s.get("detail", "")),
                      "severity": label(s.get("severity")), "category": label(s.get("category")),
                      "when": label(s.get("when")), "files": s.get("files")}
                     for s in log.get("systemic", []) if isinstance(s, dict)],
    }


def build_section(result):
    """How many rewrites the build check compiled from the repository alone, and why the rest failed."""
    failed = [r for r in result.get("failed", []) if isinstance(r, dict)]
    families = Counter()
    for row in failed:
        error = str(row.get("error", ""))
        families[next((name for name, pattern in BUILD_FAMILIES if pattern.search(error)), "other")] += 1
    checked = result.get("checked")
    return {"checked": checked, "fail": len(failed),
            "compile": checked - len(failed) if isinstance(checked, int) else None,
            "edition": label(result.get("edition"), {"2021", "2024"}), "families": dict(families.most_common())}


def git(*args):
    return subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True, text=True).stdout.strip()


def as_of(path):
    """When a source last changed: its last commit time, or its modification time when it differs from that
    commit (or is not tracked). UTC, to the minute."""
    rel = path.relative_to(ROOT).as_posix()
    tracked = bool(git("ls-files", "--", rel))
    committed = git("log", "-1", "--format=%cI", "--", rel) if tracked else ""
    dirty = tracked and bool(git("status", "--porcelain", "--", rel))
    if committed and not dirty:
        moment = datetime.fromisoformat(committed)
    else:
        moment = datetime.fromtimestamp(path.stat().st_mtime, timezone.utc)
    return moment.astimezone(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")


FORBIDDEN = [
    ("address", re.compile(r"\b0x[0-9A-Fa-f]{5,}\b")),
    ("rewrite file name", re.compile(r"\bfn_[0-9A-Fa-f]{6,}|\b\w+\.rs\b")),
    ("generated function label", re.compile(r"\b(?:sub|FUN|loc)_[0-9A-Fa-f]{5,}\b")),
]


def leaks(document):
    """Strings in the document that look like an address, a rewrite's file name or a machine path."""
    checks = FORBIDDEN + [("machine path", devlog_feed.publication_patterns()["MACHINE_PATH"])]
    found = []

    def walk(value):
        if isinstance(value, dict):
            for key, item in value.items():
                walk(key)
                walk(item)
        elif isinstance(value, list):
            for item in value:
                walk(item)
        elif isinstance(value, str):
            for name, pattern in checks:
                hit = pattern.search(value)
                if hit:
                    found.append(f"{name}: {hit.group(0)[:40]!r}")

    walk(document)
    return found


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def build(build_result=BUILD_RESULT):
    """The quality document from the repository's sources (and the build result when present)."""
    verified = ledger.load_index(ROOT / "rewrites" / "verified")
    unverified = ledger.load_index(ROOT / "rewrites" / "unverified")
    log = read_json(ROOT / SOURCES["issues"])
    document = {
        "format": "lf-site-quality/1",
        "sources": {name: {"path": rel, "as_of": as_of(ROOT / rel)} for name, rel in SOURCES.items()},
        "tree": tree_section(verified, unverified),
        "issues": issues_section(log),
        "build": None,
    }
    path = Path(build_result) if build_result else None
    if path and path.exists():
        document["build"] = build_section(read_json(path))
        document["sources"]["build"] = {"path": "scripts/ci/check_rewrites_build.py result (not tracked)",
                                        "as_of": as_of(path) if ROOT in path.resolve().parents else None}
    problems = leaks(document)
    if problems:
        raise RuntimeError("quality.json would publish per-file detail: " + "; ".join(problems[:5]))
    return document


def render(document):
    return json.dumps(document, indent=1, ensure_ascii=False) + "\n"


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--build", default=str(BUILD_RESULT), help="the build check's result file (optional)")
    parser.add_argument("--check", action="store_true", help="exit 1 if quality.json is out of date")
    args = parser.parse_args(argv)
    try:
        text = render(build(args.build))
    except RuntimeError as error:
        print(f"quality.json not written: {error}", file=sys.stderr)
        return 1
    current = OUT.read_text(encoding="utf-8") if OUT.exists() else ""
    if args.check:
        print("quality.json is up to date" if text == current else "quality.json is out of date")
        return 0 if text == current else 1
    if text != current:
        OUT.write_text(text, encoding="utf-8", newline="\n")
        print("quality.json written")
    else:
        print("quality.json unchanged")
    return 0


if __name__ == "__main__":
    sys.exit(main())
