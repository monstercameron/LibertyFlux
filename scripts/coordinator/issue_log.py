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
"""

import argparse
import json
import os
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import lint_rewrites

SEVERITY = {"high": 0, "medium": 1, "low": 2}
FIELDS = ("severity", "category", "when", "title", "detail")


def from_lint(findings):
    return [dict({k: f[k] for k in FIELDS}, file=f["file"], source="lint:" + f["code"], confidence=f["confidence"]) for f in findings]


def from_build(rows):
    return [{"file": r["file"], "severity": "high", "category": "integration", "when": "now", "source": "build",
             "confidence": "certain", "title": "Does not compile from the repository",
             "detail": "Against the shared runtime alone: " + r["error"]} for r in rows]


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


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--build")
    parser.add_argument("--review")
    parser.add_argument("--out")
    args = parser.parse_args(argv)
    root = common.find_root()
    out = args.out or str(root / "rewrites" / "review" / "issues.json")
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
