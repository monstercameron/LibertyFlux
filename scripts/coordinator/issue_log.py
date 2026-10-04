"""The rewrite issue log: every known problem with a verified rewrite, in one file, for review at assembly time.

Three sources, each optional except the first:
  lint_rewrites.py       run live over rewrites/verified (pattern checks, plus corpus-wide systemic counts)
  --build PATH           the JSON written by scripts/ci/check_rewrites_build.py (files that do not compile)
  --review PATH          JSON lines from a reading review: file, family_size, severity, category, title, detail,
                         when, label (one object per line, as the review brief asks)
Writes rewrites/review/issues.json: one entry per file, its issues sorted by severity, the systemic patterns,
and counts by source, category and severity. Every issue keeps its source and status (`open`), so a later
pass can mark them `fixed` or `not-an-issue` and the log is regenerated without losing that.

Usage: python issue_log.py [--build PATH] [--review PATH] [--out PATH]

Working through the log. These read or change the log named by --out (default rewrites/review/issues.json) and
never regenerate it:
  --summary                     totals, then the largest issue classes per severity, high first, one example
                                file each (a class is a lint code, a compiler error kind or a review title)
  --csv PATH                    one row per issue (PATH '-' writes to stdout)
  --filter KEY=VALUE            narrow --summary and --csv (alone, it implies --summary). KEY is severity,
                                category, source, when or status; a source without a colon also matches its
                                sub-sources (source=lint matches lint:partial). A repeated key allows any of
                                its values; different keys must all match.
  --mark FILE SOURCE_OR_TITLE STATUS
                                set STATUS (open, fixed, not-an-issue, wont-fix) on the issues of FILE whose
                                source or title is SOURCE_OR_TITLE. Regenerating keeps it: merge() carries a
                                status over by file, source and title. Review issues stay only while the log
                                is regenerated with --review; without it they leave the log, and their status
                                with them.
"""

import argparse
import csv
import json
import os
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import lint_rewrites

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "ci"))
from check_rewrites_build import error_class  # noqa: E402  (the build check's own grouping of its errors)

SEVERITY = {"high": 0, "medium": 1, "low": 2}
FIELDS = ("severity", "category", "when", "title", "detail")
STATUSES = ("open", "fixed", "not-an-issue", "wont-fix")
FILTER_KEYS = ("severity", "category", "source", "when", "status")
CSV_COLUMNS = ("file", "severity", "category", "when", "source", "confidence", "status", "title", "detail", "family_size")
BUILD_PREFIX = "Against the shared runtime alone: "
TOP_CLASSES = 10  # classes listed per severity by --summary; the rest are counted on one line


def from_lint(findings):
    return [dict({k: f[k] for k in FIELDS}, file=f["file"], source="lint:" + f["code"], confidence=f["confidence"]) for f in findings]


def from_build(rows):
    return [{"file": r["file"], "severity": "high", "category": "integration", "when": "now", "source": "build",
             "confidence": "certain", "title": "Does not compile from the repository",
             "detail": BUILD_PREFIX + r["error"]} for r in rows]


def from_review(rows):
    issues = []
    for r in rows:
        if not r.get("file") or r.get("severity") not in SEVERITY:
            continue
        issue = {k: str(r.get(k) or "") for k in FIELDS}
        issue.update(file=r["file"], source="review", confidence=str(r.get("label") or "Inferred"))
        if isinstance(r.get("family_size"), int) and r["family_size"] > 1:
            issue["family_size"] = r["family_size"]
        issues.append(issue)
    return issues


def merge(issues, previous=None):
    """Group by file, drop exact repeats, keep statuses a reviewer set in the previous log."""
    status = {}
    for entry in (previous or {}).get("files", []):
        for issue in entry["issues"]:
            status[(entry["file"], issue["source"], issue["title"])] = issue.get("status", "open")
    by_file = defaultdict(dict)
    for issue in issues:
        key = (issue["source"], issue["title"], issue["detail"])
        issue["status"] = status.get((issue["file"], issue["source"], issue["title"]), "open")
        by_file[issue.pop("file")].setdefault(key, issue)
    files = []
    for path in sorted(by_file):
        listed = sorted(by_file[path].values(), key=lambda i: (SEVERITY[i["severity"]], i["category"], i["title"]))
        files.append({"file": path, "worst": listed[0]["severity"], "issues": listed})
    return files


def summarise(files):
    flat = [i for f in files for i in f["issues"]]
    return {"files": len(files), "issues": len(flat),
            "by_severity": dict(Counter(i["severity"] for i in flat)),
            "by_category": dict(sorted(Counter(i["category"] for i in flat).items())),
            "by_source": dict(sorted(Counter(i["source"].split(":")[0] for i in flat).items())),
            "files_with_high": sum(1 for f in files if f["worst"] == "high")}


def parse_filters(items):
    """{key: set of allowed values} from KEY=VALUE strings. Raises ValueError for an unknown key, a missing `=`,
    or a severity or status that does not exist (a typo would otherwise match nothing, silently)."""
    filters = defaultdict(set)
    for item in items:
        key, sep, value = item.partition("=")
        if not sep or key not in FILTER_KEYS or not value:
            raise ValueError(f"bad filter {item!r}: use KEY=VALUE with KEY one of {', '.join(FILTER_KEYS)}")
        known = {"severity": tuple(SEVERITY), "status": STATUSES}.get(key)
        if known and value not in known:
            raise ValueError(f"bad filter {item!r}: {key} is one of {', '.join(known)}")
        filters[key].add(value)
    return dict(filters)


def issue_matches(issue, filters):
    """True when the issue passes every filter: one of each key's values, a source also by its prefix."""
    for key, values in filters.items():
        value = str(issue.get(key, "open" if key == "status" else ""))
        if key == "source":
            if not any(value == v or (":" not in v and value.split(":")[0] == v) for v in values):
                return False
        elif value not in values:
            return False
    return True


def flat_issues(log, filters=None):
    """(file, issue) pairs of a log, in its order, keeping the ones that pass the filters."""
    return [(entry["file"], issue) for entry in log.get("files", []) for issue in entry["issues"]
            if issue_matches(issue, filters or {})]


def issue_class(issue):
    """What an issue is counted under in the summary: the lint code for a lint finding, the compiler error's kind
    for a build failure, the title for a review finding (reviews have no codes)."""
    if issue["source"] == "build":
        detail = issue.get("detail", "")
        return "build: " + error_class(detail[len(BUILD_PREFIX):] if detail.startswith(BUILD_PREFIX) else detail)
    if issue["source"] == "review":
        return "review: " + issue.get("title", "")
    return issue["source"]


def render_summary(log, filters=None, top=TOP_CLASSES):
    """The human report: totals, then per severity (high first) the largest classes by count, each with how many
    are still open when some are not and one example file (an open one where there is one)."""
    pairs = flat_issues(log, filters)
    everything = sum(len(entry["issues"]) for entry in log.get("files", []))
    files = {path for path, _ in pairs}
    shown = ", ".join(f"{k}={v}" for k in FILTER_KEYS for v in sorted((filters or {}).get(k, ())))
    lines = [f"issue log{' (filtered: ' + shown + ')' if shown else ''}: {len(pairs)}"
             f"{f' of {everything}' if shown else ''} issues in {len(files)} files, "
             f"{len({p for p, i in pairs if i['severity'] == 'high'})} of them with a high-severity issue"]

    def counts(values, order=None):
        found = Counter(values)
        keys = [k for k in (order or ()) if k in found] + sorted(k for k in found if k not in (order or ()))
        return ", ".join(f"{k} {found[k]}" for k in keys) or "none"

    lines.append("  by severity: " + counts((i["severity"] for _, i in pairs), SEVERITY))
    lines.append("  by status: " + counts((i.get("status", "open") for _, i in pairs), STATUSES))
    lines.append("  by source: " + counts(i["source"].split(":")[0] for _, i in pairs))
    if log.get("systemic"):
        lines.append("  systemic (not per file): " + ", ".join(f"{s['code']} {s['files']} files" for s in log["systemic"]))
    for severity in SEVERITY:
        classes = defaultdict(list)
        for path, issue in pairs:
            if issue["severity"] == severity:
                classes[issue_class(issue)].append((path, issue))
        if not classes:
            continue
        total = sum(len(v) for v in classes.values())
        lines.append(f"{severity}: {total} issues in {len(classes)} classes")
        ranked = sorted(classes.items(), key=lambda kv: (-len(kv[1]), kv[0]))
        for name, members in ranked[:top]:
            open_files = [p for p, i in members if i.get("status", "open") == "open"]
            still = f" ({len(open_files)} open)" if len(open_files) < len(members) else ""
            label = name if len(name) <= 72 else name[:69] + "..."
            lines.append(f"  {len(members):5}  {label}{still}  e.g. {(open_files or [members[0][0]])[0]}")
        if len(ranked) > top:
            rest = ranked[top:]
            lines.append(f"         ... and {len(rest)} more classes ({sum(len(v) for _, v in rest)} issues)")
    return "\n".join(lines)


def write_csv(pairs, path):
    """One row per (file, issue) pair, CSV_COLUMNS in order, LF line ends; `-` writes to stdout."""
    def rows(handle):
        writer = csv.writer(handle, lineterminator="\n")
        writer.writerow(CSV_COLUMNS)
        for file, issue in pairs:
            record = dict(issue, file=file)
            record.setdefault("status", "open")
            writer.writerow(["" if record.get(c) is None else record.get(c, "") for c in CSV_COLUMNS])
    if path == "-":
        rows(sys.stdout)
        return
    with open(path, "w", encoding="utf-8", newline="") as fh:
        rows(fh)


def log_file_name(name):
    """A file as the log names it (relative to rewrites/verified), from what a reviewer may type."""
    name = name.replace("\\", "/")
    name = name[2:] if name.startswith("./") else name
    return name[len("rewrites/verified/"):] if name.startswith("rewrites/verified/") else name


def mark(log, file, key, status):
    """Set `status` on every issue of `file` whose source or title is `key`. Changes `log` in place and returns
    the matching issues (empty when none matches; the log is then unchanged)."""
    if status not in STATUSES:
        raise ValueError(f"status is one of {', '.join(STATUSES)}")
    matched = [issue for entry in log.get("files", []) if entry["file"] == log_file_name(file)
               for issue in entry["issues"] if key in (issue["source"], issue["title"])]
    for issue in matched:
        issue["status"] = status
    return matched


def write_log(log, out):
    """The log as main writes it: indented JSON, LF line ends, a final newline."""
    os.makedirs(os.path.dirname(out) or ".", exist_ok=True)
    with open(out, "w", encoding="utf-8", newline="\n") as fh:
        json.dump(log, fh, indent=1)
        fh.write("\n")


def read_json_lines(path):
    rows = []
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            line = line.strip()
            if line:
                try:
                    rows.append(json.loads(line))
                except ValueError:
                    continue
    return rows


def work_through(parser, args, out):
    """--summary, --csv, --filter and --mark: read (or, for --mark, change) the existing log; never regenerate."""
    if args.build or args.review:
        parser.error("--build and --review regenerate the log; --summary, --csv, --filter and --mark use the existing one")
    if args.mark and (args.summary or args.csv or args.filter):
        parser.error("--mark changes the log; run --summary or --csv on their own")
    try:
        filters = parse_filters(args.filter or [])
    except ValueError as problem:
        parser.error(str(problem))
    if args.mark and args.mark[2] not in STATUSES:
        parser.error(f"STATUS is one of {', '.join(STATUSES)}")
    if not os.path.exists(out):
        print("no issue log there yet; run issue_log.py without these options to write one", file=sys.stderr)
        return 1
    log = json.loads(open(out, encoding="utf-8").read())
    if args.mark:
        file, key, status = args.mark
        matched = mark(log, file, key, status)
        if not matched:
            listed = [i for path, i in flat_issues(log) if path == log_file_name(file)]
            print(f"no issue of {log_file_name(file)} has source or title {key!r}; "
                  + (f"its issues: {'; '.join(i['source'] + ': ' + i['title'] for i in listed)}" if listed else "it has none logged"),
                  file=sys.stderr)
            return 1
        write_log(log, out)
        print(f"marked {len(matched)} issue(s) of {log_file_name(file)} as {status}:")
        for issue in matched:
            print(f"  {issue['source']}: {issue['title']}")
        return 0
    if args.csv:
        write_csv(flat_issues(log, filters), args.csv)
    if args.summary or not args.csv:
        print(render_summary(log, filters))
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--build")
    parser.add_argument("--review")
    parser.add_argument("--out")
    parser.add_argument("--summary", action="store_true", help="print a human report of the existing log")
    parser.add_argument("--csv", metavar="PATH", help="export the existing log, one row per issue ('-' for stdout)")
    parser.add_argument("--filter", action="append", metavar="KEY=VALUE",
                        help=f"narrow --summary and --csv; KEY is one of {', '.join(FILTER_KEYS)}; repeatable")
    parser.add_argument("--mark", nargs=3, metavar=("FILE", "SOURCE_OR_TITLE", "STATUS"),
                        help=f"set an issue's status ({', '.join(STATUSES)}) in the existing log")
    args = parser.parse_args(argv)
    root = common.find_root()
    out = args.out or str(root / "rewrites" / "review" / "issues.json")
    if args.summary or args.csv or args.filter or args.mark:
        return work_through(parser, args, out)
    findings, systemic, checked = lint_rewrites.lint_tree(root)
    issues = from_lint(findings)
    if args.build:
        issues += from_build(json.loads(open(args.build, encoding="utf-8").read())["failed"])
    if args.review:
        issues += from_review(read_json_lines(args.review))
    previous = json.loads(open(out, encoding="utf-8").read()) if os.path.exists(out) else None
    files = merge(issues, previous)
    log = {"checked": checked, "summary": summarise(files), "systemic": systemic, "files": files}
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8", newline="\n") as fh:
        json.dump(log, fh, indent=1)
        fh.write("\n")
    print(json.dumps(log["summary"], indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
