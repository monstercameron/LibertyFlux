window.LF_CHANGELOG = [
  {
    "date": "2026-10-03",
    "title": "Add the repository layout",
    "commit": null,
    "summary": "Where things live, and what git never sees.",
    "changes": [
      "Everything generated (build output, caches, logs, agent scratch work, decompiler output) goes under .artifacts, which git ignores.",
      "The local toolchain (tools, .venv), the original executable (orig) and the private notes file are ignored too.",
      "Game files, Ghidra and IDA databases and stray build output are blocked by extension wherever they land.",
      "Cargo builds are redirected into .artifacts so no target folder appears in the tree.",
      "Line endings are LF, with CRLF only for .bat, .cmd and .ps1."
    ]
  }
];
