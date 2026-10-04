"""Brief rendering from tracked templates and the numbered-notes data file.

The production brief used to be assembled by slicing the source of older brief
makers and patching the text with one-off scripts. It is now data: template
files under `briefs/` plus `brief_notes.json`, the numbered list of notes with
the brief version. A new brief version is an edit to those files, with its
reason recorded in the note itself, not a script that rewrites a script.

Single-lane briefs use `render_preamble()` plus their own task file; nothing
borrows its opening section by slicing another lane's brief any more.
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

_templates = {}


def template(name):
    """One template file's text, cached."""
    if name not in _templates:
        _templates[name] = (common.templates_dir() / name).read_text(encoding="utf-8")
    return _templates[name]


def load_notes():
    """The brief notes: {"version": "b6", "notes": [{"id", "title", "since", "text"}]}."""
    return json.loads(template("brief_notes.json"))


def brief_version():
    return load_notes()["version"]


def render_learned(root):
    """The 'what earlier production lanes learned' section from the notes."""
    notes = load_notes()["notes"]
    return (template("learned_intro.txt") + "\n".join("- " + n["text"] for n in notes)).format(root=root)


def render_preamble(lane, root, date=None):
    """The standard brief opening for `lane`, ending with "Your task:". The date comes from the clock."""
    return template("preamble.txt").format(lane=lane, root=root, date=date or common.today())


def render_head(lane, root, date=None):
    """The standard opening for single-lane briefs, up to but not including
    "Your task:": the task file supplies that line, as it always did."""
    head = render_preamble(lane, root, date)
    assert head.endswith("Your task:\n\n"), "preamble template lost its trailing task line"
    return head[:head.index("Your task:")]


def render_production(lane, crate, what, slug, root):
    """The production task for one lane: the template plus the current notes."""
    notes = load_notes()
    task = template("production.txt").format(
        root=root, lane=lane, crate=crate, what=what, slug=slug, brief_version=notes["version"])
    return task + render_learned(root)


def render_naming(count, list_path, root, slug):
    """The naming-lane task for one batch."""
    return template("naming_task.txt").format(count=count, list_path=list_path, root=root, slug=slug)
