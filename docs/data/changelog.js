window.LF_CHANGELOG = [
  {
    "date": "2026-10-03",
    "title": "Add the project site overview",
    "commit": null,
    "summary": "The front page of the GitHub Pages site in docs.",
    "changes": [
      "A map of the executable's code from start to end, where each square changes shade as its functions are identified, named, rewritten and verified. With no data it shows a plain statement that nothing is measured yet.",
      "Progress meters, the method step by step, the repository rule, the roadmap, the planned features and platforms, and credits.",
      "Six placeholder concept images, generated for the page and captioned as not being screenshots.",
      "Plain HTML, CSS and JavaScript with no build step. Works at phone, tablet and desktop widths, in light and dark themes, and the map can be read with the keyboard."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add progress tracking",
    "commit": "1ec4a61c483daed946bd3d847d10247a175f4d1b",
    "summary": "One data file drives the badges and the site's numbers.",
    "changes": [
      "docs/data/progress.json holds the counts: phase, functions, stages, structures, symbols and the code map. Everything is zero or empty because nothing has been measured.",
      "scripts/update_progress.py regenerates the seven SVG badges and the site's data files from it, and fills commit hashes into the changelog.",
      "The badges are generated into the repository, so they need no external badge service."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the project plan",
    "commit": "5beeaffe71b2d4ede67a603735d4aa0f8db07661",
    "summary": "plan.md: the goal, the decisions and why, and the eight phases.",
    "changes": [
      "Goal and target platforms: Windows x64, Windows on ARM and macOS, with upscaling, frame generation and path tracing as later work.",
      "Decisions: rewrite straight to Rust with no committed C++ stage, check each function by emulator comparison, then swap it into the running game behind a switch.",
      "Eight phases with exit gates, and the phase 0 task list.",
      "Harness design taken from other projects, the dependency table, pitfalls from the recompilation projects' history, risks and open questions.",
      "First findings from the executable's headers: built with Visual Studio 2012, link-time code generation in use, main code not encrypted, class names present."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the agent rules",
    "commit": "234c64e7d96d4d6b1919b3c8292f62c8395b59af",
    "summary": "AGENTS.md: what every agent and contributor must follow.",
    "changes": [
      "Rule 1: no decompiled code in any tracked file. Only structures, symbols and Rust rewrites derived from the game may be committed.",
      "Rule 2: publishing is the owner's decision; agents never push or post.",
      "Rule 3: a machine check decides when a function is done, and fake passes are banned.",
      "Rule 4: the coordinator keeps the progress file true, from machine sources only.",
      "Rule 5: anything private goes in the untracked notes file and nowhere else.",
      "Commit history is one change per commit, each with a changelog entry; lessons go in the devlog."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the repository layout",
    "commit": "ebf5d33ed400908a24b350d760d4928169e2b960",
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
