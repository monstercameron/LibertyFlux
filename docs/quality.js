// Quality page: fills the numbers into the page from data/quality.json (scripts/site/quality.py) and the
// published counts from data/progress.json. The explanations are static HTML; this only adds counts.
(function () {
  "use strict";
  var fmt = new Intl.NumberFormat("en-US");
  var $ = function (id) { return document.getElementById(id); };

  // Plain names for the labels the data uses. A label without one is shown as it is.
  var NAMES = {
    checker: { "version 1": "Version 1 (no longer counts)", "version 2": "Version 2", "version 3": "Version 3", "version 4": "Version 4", "version 5": "Version 5" },
    kind: { "function": "Engine functions", "native": "Script natives" },
    outcome: {
      "not_reached": "Not reached: the lane stopped first",
      "not yet run": "Not yet run through the checker",
      "deferred": "Deferred, with a reason",
      "failed_v3": "Failed when re-run under version 3",
      "failed_v4": "Failed when re-run under version 4"
    },
    reason: {
      "other": "Other, explained in the lane's notes",
      "needs_encrypted_code": "Needs code from the encrypted megabyte",
      "checker_gap": "The checker cannot observe it yet",
      "frame_pointer_args": "Arguments passed through the caller's frame",
      "indirect_call": "Calls through a pointer the contract cannot plant",
      "thread_local": "Reads thread-local storage",
      "runtime_table": "Depends on a table built at run time",
      "float_register_args": "Takes floats in registers the checker could not set",
      "jump_thunk": "A jump stub, not a function body",
      "not_a_function": "Not a function after all",
      "other (unlisted)": "Other"
    },
    severity: { "high": "High", "medium": "Medium", "low": "Low" },
    when: { "now": "Now", "post-bring-up": "Before the rewrites run in the game", "lift": "Before the 64-bit lift" },
    category: {
      "integration": "Breaks assembly or running in the game",
      "float": "Floating-point order or precision",
      "narrow-proof": "Proof covers less than it seems",
      "panic": "Can panic in a debug build",
      "suspicious-logic": "Logic that looks wrong",
      "ub": "Undefined behaviour in Rust",
      "rule": "Breaks a project rule",
      "doc-mismatch": "Comment disagrees with the code",
      "quality": "Tidiness"
    },
    source: { "review": "A reading review", "lint": "Pattern checks", "build": "The build check" },
    check: {
      "export-name": "Export not named after its function",
      "float-order": "Float arithmetic without a pinned order",
      "float-to-int": "Float converted to an integer with a plain cast",
      "header": "Header comment missing or malformed",
      "transliteration": "Decompiler-style or register-named locals",
      "image-literal": "Raw image-range number instead of a relocated one",
      "constant-for-callee-result": "A callee's result replaced by a constant",
      "no-export": "No exported function",
      "several-exports": "Several exports in one file",
      "wrong-version-tracked": "A wrong version left in the tracked file",
      "misaligned-deref": "Aligned access at an odd offset",
      "partial": "File marks part of the function as not covered",
      "placeholder": "File says part of it is a placeholder",
      "unbalanced": "Truncated or unbalanced file",
      "unused-parameter": "A parameter is never used",
      "forwarder-drops-args": "Forwarder passes fewer arguments than its doc says",
      "args-gap": "A script argument between used ones is never read",
      "checker-only": "Uses a checker-only input"
    }
  };

  function el(tag, className, text) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (text != null) node.textContent = text;
    return node;
  }
  function n(value) { return typeof value === "number" ? fmt.format(value) : "not measured"; }
  function pct(part, whole) {
    if (typeof part !== "number" || typeof whole !== "number" || whole <= 0) return "";
    var share = 100 * part / whole;
    return (share > 0 && share < 1 ? share.toFixed(1) : Math.round(share)) + "%";
  }
  function set(key, text) {
    document.querySelectorAll('[data-q="' + key + '"]').forEach(function (node) { node.textContent = text; });
  }

  // One horizontal bar per entry, one hue: the bars show amounts of the same thing. Values sit at the right
  // in ink, with the share of `total` when it is given.
  function bars(id, counts, names, total, skip) {
    var host = $(id);
    if (!host) return;
    host.replaceChildren();
    var entries = Object.keys(counts || {}).filter(function (k) { return !(skip && skip.indexOf(k) >= 0); })
      .map(function (k) { return [k, counts[k]]; });
    if (!entries.length) { host.appendChild(el("li", "qbars-empty", "Nothing recorded.")); return; }
    var max = Math.max.apply(null, entries.map(function (e) { return e[1]; }));
    entries.forEach(function (entry) {
      var li = el("li");
      li.appendChild(el("span", "qbar-label", (names && names[entry[0]]) || entry[0]));
      var track = el("span", "qbar-track");
      var fill = el("span", "qbar-fill");
      fill.style.width = (max ? Math.max(0.6, 100 * entry[1] / max) : 0) + "%";
      track.appendChild(fill);
      li.appendChild(track);
      var share = total ? pct(entry[1], total) : "";
      li.appendChild(el("span", "qbar-value", fmt.format(entry[1]) + (share ? " (" + share + ")" : "")));
      host.appendChild(li);
    });
  }

  function sum(counts) {
    return Object.keys(counts || {}).reduce(function (s, k) { return s + counts[k]; }, 0);
  }

  function render(q, progress) {
    var tree = q.tree || {}, issues = q.issues || {}, build = q.build, sources = q.sources || {};
    var stages = (progress && progress.stages) || {};

    // Masthead line and tiles.
    var line = [];
    line.push(fmt.format(tree.verified) + " rewrites have passed the checker.");
    if (build) line.push(fmt.format(build.compile) + " of the " + fmt.format(build.checked) + " the last build check covered compile from the repository alone.");
    line.push(fmt.format(issues.files_with_high) + " files carry a known high-severity issue.");
    $("q-status").textContent = line.join(" ");

    set("verified", n(tree.verified));
    set("verified-note", fmt.format((tree.by_kind || {})["function"] || 0) + " engine functions and " +
      fmt.format((tree.by_kind || {})["native"] || 0) + " script natives.");
    set("modern", n(tree.modern_checker));
    set("modern-note", typeof stages.verified === "number"
      ? "The overview's verified count, " + fmt.format(stages.verified) + ", counts only the game code among them."
      : "");
    if (build) {
      set("compile", n(build.compile));
      set("compile-note", "of " + fmt.format(build.checked) + " checked (" + pct(build.compile, build.checked) + "). The check covered the rewrites that existed when it ran.");
    } else {
      set("compile", "not measured");
      set("compile-note", "The build check's result was not available where this data was made. The issue log lists " +
        fmt.format((issues.by_source || {}).build || 0) + " files that failed the last build check it was given.");
    }
    set("high", n(issues.files_with_high));
    set("high-note", "of " + fmt.format(issues.checked) + " files read (" + pct(issues.files_with_high, issues.checked) + "); " +
      fmt.format(issues.files) + " files have at least one issue of any severity.");

    var when = [];
    if (sources.verified) when.push("rewrite indexes as of " + sources.verified.as_of);
    if (sources.issues) when.push("issue log as of " + sources.issues.as_of);
    when.push(sources.build && sources.build.as_of ? "build check as of " + sources.build.as_of : "no build check result");
    $("q-sources").textContent = "Sources: " + when.join("; ") + ". Counts only: no file, address or function is named in the data.";

    // What has passed.
    $("passed-lede").textContent = fmt.format(tree.verified) + " rewrites from " + fmt.format(tree.lanes) +
      " agent lanes are in the verified tree; " + fmt.format(tree.unverified) + " more exist that have not passed.";
    bars("bars-checker", tree.by_checker, NAMES.checker, tree.verified);
    bars("bars-kind", tree.by_kind, NAMES.kind, tree.verified);
    bars("bars-outcome", tree.unverified_by_outcome, NAMES.outcome, tree.unverified);
    var reasons = tree.unverified_by_reason || {};
    bars("bars-reason", reasons, NAMES.reason, sum(reasons) - (reasons["none recorded"] || 0), ["none recorded"]);

    var facts = $("passed-facts");
    facts.replaceChildren();
    function fact(strong, text) {
      var li = el("li");
      li.appendChild(el("strong", null, strong));
      li.appendChild(document.createTextNode(" " + text));
      facts.appendChild(li);
    }
    var trials = tree.trials || {};
    fact(fmt.format(trials.at_least_1000 || 0) + " of " + fmt.format(trials.with_count || 0),
      "verified rewrites with a recorded trial count ran at least 1,000 trials" +
      ((tree.verified || 0) > (trials.with_count || 0) ? "; " + fmt.format(tree.verified - trials.with_count) + " have no count recorded." : "."));
    var proofs = tree.proofs || {};
    fact(fmt.format(proofs.with_proof || 0) + " of " + fmt.format(tree.verified || 0),
      "verified rewrites carry a proof record so far: the exact checker, inputs and wrong-version result, so anyone with the game can rerun the proof. The format was added on 4 October 2026.");
    if (typeof stages.verified === "number") {
      fact(fmt.format(stages.verified), "is the verified count the overview publishes: rewrites accepted under checker version 2 or later that are game code. The " +
        fmt.format(tree.modern_checker) + " in the tree can also include rewrites of library code, which the overview leaves out, and the two are refreshed at different times.");
    }
    if (tree.demoted) fact(fmt.format(tree.demoted), "rewrites were moved back to unverified after failing under a later checker version.");

    // Known problems.
    $("problems-lede").textContent = fmt.format(issues.issues) + " issues are open in " + fmt.format(issues.files) +
      " of the " + fmt.format(issues.checked) + " files read; " + fmt.format(issues.files_with_high) + " files have at least one of high severity.";
    bars("bars-severity", issues.by_severity, NAMES.severity, issues.issues);
    bars("bars-when", issues.by_when, NAMES.when, issues.issues);
    bars("bars-category", issues.by_category, NAMES.category, issues.issues);
    bars("bars-source", issues.by_source, NAMES.source, issues.issues);
    bars("bars-check", issues.by_check, NAMES.check);

    // The seven classes.
    (issues.classes || []).forEach(function (c) {
      var card = document.querySelector('.qcard[data-class="' + c.key + '"]');
      if (!card) return;
      var high = (c.by_severity || {}).high || 0;
      var text = "In the issue log: " + fmt.format(c.files) + " file" + (c.files === 1 ? "" : "s") + ", " +
        fmt.format(c.issues) + " issue" + (c.issues === 1 ? "" : "s") + (high ? ", " + fmt.format(high) + " of them high severity" : "") + ".";
      if (c.key === "lane-runtime" && build && build.families) {
        text += " The build check could not compile " + fmt.format(build.fail) + " of " + fmt.format(build.checked) + ", " +
          fmt.format(build.families["missing-helper"] || 0) + " of them because a name they use is not in the shared runtime.";
      }
      if (typeof c.systemic_files === "number") {
        text += c.key === "debug-overflow"
          ? " The build-profile dependence itself applies to " + fmt.format(c.systemic_files) + " of " + fmt.format(issues.checked) + " files."
          : " Plain reads and writes of game memory appear in " + fmt.format(c.systemic_files) + " of " + fmt.format(issues.checked) + " files.";
      }
      var slot = card.querySelector('[data-q="count"]');
      if (slot) slot.textContent = text;
    });

    // Systemic patterns.
    var host = $("systemic");
    host.replaceChildren();
    (issues.systemic || []).forEach(function (s) {
      var li = el("li", "systemic-item");
      li.appendChild(el("h4", null, s.title));
      var share = el("p", "systemic-share");
      share.appendChild(el("strong", null, fmt.format(s.files)));
      share.appendChild(document.createTextNode(" of " + fmt.format(issues.checked) + " files (" + pct(s.files, issues.checked) + "), " +
        (NAMES.severity[s.severity] || s.severity).toLowerCase() + " severity, matters " + (NAMES.when[s.when] || s.when).toLowerCase() + "."));
      li.appendChild(share);
      var track = el("span", "qbar-track systemic-track");
      track.setAttribute("role", "img");
      track.setAttribute("aria-label", pct(s.files, issues.checked) + " of files");
      var fill = el("span", "qbar-fill");
      fill.style.width = Math.min(100, 100 * s.files / Math.max(1, issues.checked)) + "%";
      track.appendChild(fill);
      li.appendChild(track);
      li.appendChild(el("p", "systemic-detail", s.detail));
      host.appendChild(li);
    });
  }

  function fail() {
    $("q-status").textContent = "The quality data has not been generated yet, so the numbers below are missing. The explanations still apply.";
    ["passed-lede", "problems-lede"].forEach(function (id) { $(id).textContent = "Not measured yet."; });
    ["verified", "modern", "compile", "high"].forEach(function (key) { set(key, "not measured"); });
  }

  function load(path) {
    return fetch(path, { cache: "no-store" }).then(function (r) { return r.ok ? r.json() : null; }).catch(function () { return null; });
  }
  Promise.all([load("data/quality.json"), load("data/progress.json")]).then(function (loaded) {
    if (!loaded[0] || !loaded[0].tree || !loaded[0].issues) { fail(); return; }
    render(loaded[0], loaded[1]);
  });
})();
