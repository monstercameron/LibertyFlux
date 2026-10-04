window.LF_CHANGELOG = [
  {
    "date": "2026-10-03",
    "title": "Fix the squeezed column in devlog tables",
    "commit": null,
    "summary": "A long hash in one cell was starving the column beside it.",
    "changes": [
      "Long hashes and file names in table cells can now break across lines, so the columns share the width evenly.",
      "Seen on the executable entry, where the meaning column had collapsed into a tall strip at desktop width."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the README",
    "commit": "f03f21044bbc8988d2a02eefa7e084ff6d374863",
    "summary": "The repository's front page.",
    "changes": [
      "Status, goals, the method with a diagram, the repository rule, the roadmap, how progress is tracked, the layout, requirements, credits and a legal notice.",
      "Shows the progress badges generated from docs/data/progress.json."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the upscaler reference",
    "commit": "2a5a2809a0a7320b6e02e54f8d6e7741de174d35",
    "summary": "documentation/upscaling-and-frame-generation.md.",
    "changes": [
      "Reference for DLSS, FSR, XeSS and MetalFX: components, platforms, per-frame inputs, integration outline, licences, pitfalls and links to the official guides.",
      "Also covers ray-tracing denoisers, Rust bindings and how other projects integrated these.",
      "Every claim carries a source link and a verified or inferred label. The frameworks' exact motion vector, depth and jitter conventions are not established yet."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the devlog",
    "commit": "880d3efda7267f9e167b4bdc28ba09b4696e8f63",
    "summary": "Research notes and lessons, so nobody has to rediscover them.",
    "changes": [
      "Sixteen entries covering a first look at the executable's headers, what the first estimates got wrong, why the route is direct to Rust, harness design from other projects, replacing functions in the running game, what is known about the executable, the Xbox 360 recompilation projects and what went wrong for them, tools, dependencies, frame-rate coupling, working with agent lanes, building the site, precedents, upscalers and open questions.",
      "Every finding is labelled Verified, Inferred or Unknown and linked to its source."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the changelog page",
    "commit": "2b89370ee052e73270d864055ae6aa2ae4c1ee47",
    "summary": "This page.",
    "changes": [
      "Lists every commit, newest first, from docs/data/changelog.json.",
      "An entry's commit hash is filled in by the update script on the following commit, so the newest entry shows no hash yet."
    ]
  },
  {
    "date": "2026-10-03",
    "title": "Add the project site overview",
    "commit": "e39ab37f9238427c75d5fc8535ed7982ed15f8da",
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
