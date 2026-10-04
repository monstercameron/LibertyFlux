(function () {
  var data = window.LF_PROGRESS || {};
  var stages = data.stages || {};
  var functions = data.functions || {};
  var total = functions.game;
  var fmt = new Intl.NumberFormat("en-US");

  var STAGE_NAMES = {
    unmeasured: "Not measured",
    identified: "Identified",
    named: "Named",
    rewritten: "Rewritten in Rust",
    verified: "Verified"
  };

  var PHASES = [
    ["Measure", "measuring the executable", "Analyse the executable, count its functions, set library code aside, recover class names.", "The function count and class list exist."],
    ["Harness", "building the harness", "Build the work queue, the emulator comparison, and the loader that swaps functions into the game.", "50 functions pass the comparison and run in the game."],
    ["Pilot", "running the pilot", "Run a few agents for several days. Measure how many functions pass, and how many passed wrongly.", "A measured rate replaces the estimate."],
    ["Rewrite", "rewriting functions", "Scale up the agents and rewrite every game function in Rust.", "The game plays with every replacement switched on."],
    ["Standalone", "building the standalone game", "Build the Rust code as its own 32-bit program.", "It runs without the original executable."],
    ["64-bit", "moving to 64-bit", "Convert the game's 32-bit data files as they load, and replace every Windows-only call.", "Windows x64 and Windows on ARM builds play."],
    ["Renderer", "building the renderer", "Write a Vulkan renderer, run the simulation at a fixed rate, and bring up macOS.", "Frame rate is measured on all three platforms."],
    ["Upscalers and new work", "adding upscalers and new work", "Add DLSS, FSR, XeSS and MetalFX, then new gameplay, graphics and path tracing.", "Each upscaler is verified on hardware that supports it."]
  ];

  function el(tag, className, text) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (text != null) node.textContent = text;
    return node;
  }

  function prettyDate(iso) {
    var d = new Date(iso + "T00:00:00Z");
    if (isNaN(d)) return null;
    return d.toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" });
  }

  // Status line
  var current = data.phase ? data.phase.index : 0;
  var statusLine = document.getElementById("status-line");
  statusLine.replaceChildren();
  statusLine.appendChild(el("strong", null, "Phase " + current + " of " + (PHASES.length - 1) + ": " + PHASES[current][1] + "."));
  var done = stages.rewritten || 0;
  var summary = done
    ? " " + fmt.format(done) + " functions rewritten, " + fmt.format(stages.verified || 0) + " verified."
    : " Nothing is rewritten yet.";
  if (current < 4) summary += " The game is not playable.";
  var when = data.updated ? prettyDate(data.updated) : null;
  if (when) summary += " Updated " + when + ".";
  statusLine.appendChild(document.createTextNode(summary));

  // Code map
  var mapEl = document.getElementById("map");
  var emptyEl = document.getElementById("map-empty");
  var legendEl = document.getElementById("legend");
  var readout = document.getElementById("map-readout");
  var tip = document.getElementById("tip");
  var cells = Array.isArray(data.map) && data.map.length ? data.map : null;

  if (!cells) {
    emptyEl.hidden = false;
  } else {
    mapEl.hidden = false;
    legendEl.hidden = false;
    var perCell = total ? Math.round(total / cells.length) : null;
    var title = document.getElementById("map-title");
    title.appendChild(document.createTextNode(
      " Each square is " + (perCell ? "about " + fmt.format(perCell) + " functions" : "a slice of the code") +
      " in address order, left to right and top to bottom, and shows its least-advanced function."));

    var defaultReadout = data.mapRange || "";
    readout.textContent = defaultReadout;

    var nodes = cells.map(function (entry) {
      var cell = el("span", "cell");
      var stage = STAGE_NAMES[entry.stage] ? entry.stage : "unmeasured";
      cell.dataset.stage = stage;
      cell.dataset.range = entry.from + " to " + entry.to;
      cell.dataset.detail = STAGE_NAMES[stage] + (entry.count ? ", " + fmt.format(entry.count) + " functions" : "");
      mapEl.appendChild(cell);
      return cell;
    });

    mapEl.tabIndex = 0;
    mapEl.setAttribute("role", "group");
    mapEl.setAttribute("aria-label",
      "Map of the game's code. " +
      (total ? fmt.format(stages.identified || 0) + " of " + fmt.format(total) + " functions identified, " +
        fmt.format(stages.rewritten || 0) + " rewritten, " + fmt.format(stages.verified || 0) + " verified. " : "") +
      "Use the arrow keys to inspect a square.");

    var active = -1;
    function select(index) {
      if (active >= 0) nodes[active].classList.remove("active");
      active = index;
      if (index < 0) { readout.textContent = defaultReadout; return; }
      var node = nodes[index];
      node.classList.add("active");
      readout.textContent = node.dataset.range + ": " + node.dataset.detail;
    }
    function columns() {
      return getComputedStyle(mapEl).gridTemplateColumns.split(" ").length;
    }

    mapEl.addEventListener("keydown", function (e) {
      var step = { ArrowRight: 1, ArrowLeft: -1, ArrowDown: columns(), ArrowUp: -columns() }[e.key];
      if (!step) return;
      e.preventDefault();
      var next = active < 0 ? 0 : active + step;
      if (next >= 0 && next < nodes.length) select(next);
    });
    mapEl.addEventListener("blur", function () { select(-1); });
    mapEl.addEventListener("click", function (e) {
      var index = nodes.indexOf(e.target);
      if (index >= 0) select(index);
    });

    mapEl.addEventListener("mousemove", function (e) {
      var index = nodes.indexOf(e.target);
      if (index < 0) return; // over a gap: keep the last square's tooltip, no flicker
      var node = nodes[index];
      tip.replaceChildren(el("span", "addr", node.dataset.range), document.createElement("br"), document.createTextNode(node.dataset.detail));
      tip.style.left = Math.min(e.clientX + 14, window.innerWidth - tip.offsetWidth - 8) + "px";
      tip.style.top = (e.clientY + 16) + "px";
      tip.classList.add("on");
    });
    mapEl.addEventListener("mouseleave", function () { tip.classList.remove("on"); });
  }

  // What has been measured so far, and how much work is in flight. Shown until functions are counted.
  function renderMeasured(after) {
    var facts = Array.isArray(data.measured) ? data.measured : [];
    var activity = data.activity;
    if (!facts.length && !activity) return;
    var wrap = el("div", "measured");
    if (activity) {
      var line = "Right now: " + fmt.format(activity.lanes_running || 0) + " agent lanes running, " +
        fmt.format(activity.lanes_finished || 0) + " finished, " +
        fmt.format(activity.devlog_entries || 0) + " devlog entries written.";
      if (data.updated_at) line += " As of " + data.updated_at + ".";
      wrap.appendChild(el("p", "prose", line));
    }
    if (facts.length) {
      wrap.appendChild(el("h3", null, "Measured so far"));
      var list = el("ul", "facts");
      facts.forEach(function (fact) {
        var li = el("li");
        li.appendChild(document.createTextNode(fact.label + ": "));
        li.appendChild(el("strong", null, fmt.format(fact.value)));
        if (fact.unit) li.appendChild(document.createTextNode(" " + fact.unit));
        list.appendChild(li);
      });
      wrap.appendChild(list);
    }
    after.insertAdjacentElement("afterend", wrap);
  }

  // Meters
  var metersEl = document.getElementById("meters");
  var note = document.getElementById("meter-note");
  if (!total) {
    var empty = document.getElementById("progress-empty");
    empty.hidden = false;
    empty.textContent = "No functions have been counted yet, so there is nothing to chart. For scale: the LibertyRecomp and GTA-IV-RECOMP projects count between about 31,800 and 36,400 functions in the Xbox 360 version of the same game. The PC version should be similar.";
    metersEl.hidden = true;
    note.hidden = true;
    renderMeasured(empty);
  } else {
    ["identified", "named", "rewritten", "verified"].forEach(function (key) {
      var count = stages[key] || 0;
      var pct = Math.min(100, (count / total) * 100);
      var li = el("li", "meter-row");
      li.appendChild(el("span", "meter-label", STAGE_NAMES[key]));
      li.appendChild(el("span", "meter-value", fmt.format(count) + " of " + fmt.format(total) + " (" + pct.toFixed(1) + "%)"));
      var track = el("div", "meter-track");
      track.setAttribute("role", "progressbar");
      track.setAttribute("aria-label", STAGE_NAMES[key]);
      track.setAttribute("aria-valuemin", "0");
      track.setAttribute("aria-valuemax", "100");
      track.setAttribute("aria-valuenow", pct.toFixed(1));
      var fill = el("div", "meter-fill");
      fill.style.width = pct + "%";
      track.appendChild(fill);
      li.appendChild(track);
      metersEl.appendChild(li);
    });
    note.textContent =
      "Each stage includes the ones after it. Identified: the function's start and end are known. Named: it has a meaningful name. " +
      "Rewritten: it has a Rust implementation. Verified: the comparison against the original passed. " +
      fmt.format(data.structures || 0) + " structures and " + fmt.format(data.symbols || 0) + " symbols are documented. " +
      "The compiler's runtime and third-party library code (" + fmt.format(functions.library || 0) + " functions) is not counted.";
  }

  // Roadmap
  var body = document.getElementById("roadmap-body");
  PHASES.forEach(function (p, idx) {
    var tr = document.createElement("tr");
    tr.setAttribute("role", "row");
    tr.className = idx < current ? "done" : idx === current ? "current" : "todo";
    var name = el("td");
    name.appendChild(el("span", "phase-num", String(idx)));
    name.appendChild(document.createTextNode(" " + p[0]));
    var work = el("td", null, p[2]);
    var gate = el("td", null, p[3]);
    var status = el("td", "status", idx < current ? "Done" : idx === current ? "In progress" : "Not started");
    gate.dataset.label = "Finished when";
    status.dataset.label = "Status";
    [name, work, gate, status].forEach(function (td) { td.setAttribute("role", "cell"); tr.appendChild(td); });
    body.appendChild(tr);
  });
})();
