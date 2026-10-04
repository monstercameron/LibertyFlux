function render(data) {
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
    ["Harness", "building the harness", "Build the work queue, the side-by-side comparison, and the loader that swaps functions into the game.", "50 functions pass the comparison and run in the game."],
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

  // The map can count a slice by functions (always) or by bytes of code (when the data has them).
  // Bytes field, written by the coordinator's progress script when function sizes are available: each slice
  // may carry  "bytes": {"total": n, "named": n, "rewritten": n, "verified": n}  where total is the size of
  // the slice's game functions and each stage is the size of those that have reached it (cumulative, like the
  // function counts). A top-level "bytes": {"game": n, ...} may carry the totals. The bytes view is offered
  // only when every slice that holds game functions has a numeric bytes.total; otherwise the map shows
  // functions and the switch stays hidden. No size is ever estimated here.
  function hasBytes(list) {
    return list.some(function (entry) { return entry.count > 0; }) && list.every(function (entry) {
      return !entry.count || (entry.bytes && typeof entry.bytes.total === "number" && entry.bytes.total > 0);
    });
  }
  // The stage at least half of a slice has reached, by bytes, mirroring how "stage" is defined by functions.
  function byteStage(b) {
    if (!b || !b.total) return "unmeasured";
    var reached = ["verified", "rewritten", "named"].filter(function (key) { return (b[key] || 0) * 2 >= b.total; })[0];
    return reached || "identified";
  }
  function bytesText(n) {
    if (n >= 1048576) return (n / 1048576).toFixed(n >= 10485760 ? 0 : 1) + " MB";
    if (n >= 1024) return (n / 1024).toFixed(n >= 10240 ? 0 : 1) + " KB";
    return fmt.format(n) + " bytes";
  }

  if (!cells) {
    emptyEl.hidden = false;
  } else {
    mapEl.hidden = false;
    legendEl.hidden = false;
    var perCell = total ? Math.round(total / cells.length) : null;
    var title = document.getElementById("map-title");
    var titleText = document.createTextNode("");
    title.appendChild(titleText);
    var bytesReady = hasBytes(cells);
    var mode = "functions";
    var modeButtons = [];
    if (bytesReady) {
      try { if (localStorage.getItem("map-mode") === "bytes") mode = "bytes"; } catch (e) { /* storage blocked: default view */ }
      var switcher = el("div", "map-mode");
      switcher.setAttribute("role", "group");
      switcher.setAttribute("aria-label", "Count each square by");
      switcher.appendChild(el("span", "map-mode-label", "Count by"));
      [["functions", "Functions"], ["bytes", "Bytes of code"]].forEach(function (option) {
        var button = el("button", null, option[1]);
        button.type = "button";
        button.dataset.mode = option[0];
        button.addEventListener("click", function () {
          mode = option[0];
          try { localStorage.setItem("map-mode", mode); } catch (e) { /* not remembered */ }
          paint();
        });
        switcher.appendChild(button);
        modeButtons.push(button);
      });
      title.parentNode.insertAdjacentElement("afterend", switcher);
    }

    var defaultReadout = data.mapRange || "";
    readout.textContent = defaultReadout;

    var nodes = cells.map(function (entry) {
      var cell = el("span", "cell");
      cell.dataset.range = entry.from + " to " + entry.to;
      mapEl.appendChild(cell);
      return cell;
    });

    // Shade every square, its verified band and its tooltip for the current count.
    function paint() {
      var byBytes = mode === "bytes";
      modeButtons.forEach(function (button) { button.setAttribute("aria-pressed", String(button.dataset.mode === mode)); });
      var bytesTotal = byBytes ? cells.reduce(function (sum, entry) { return sum + (entry.count ? entry.bytes.total : 0); }, 0) : 0;
      titleText.textContent = byBytes
        ? " Each square is a slice of the code in address order, about " + bytesText(Math.round(bytesTotal / cells.length)) +
          " of game code on average, left to right and top to bottom. Its shade is the stage at least half of its bytes have reached;" +
          " the pale band rising from the bottom is the share of bytes already verified."
        : " Each square is " + (perCell ? "about " + fmt.format(perCell) + " functions" : "a slice of the code") +
          " in address order, left to right and top to bottom. Its shade is the stage at least half of them have reached;" +
          " the pale band rising from the bottom is the share already verified.";
      cells.forEach(function (entry, index) {
        var cell = nodes[index];
        var stage, detail, share = 0;
        cell.classList.remove("part");
        if (byBytes) {
          var b = entry.count ? entry.bytes : null;
          stage = byteStage(b);
          detail = b ? bytesText(b.total) + " of game code in " + fmt.format(entry.count) + " functions: " +
            bytesText(b.named || 0) + " named, " + bytesText(b.rewritten || 0) + " rewritten, " + bytesText(b.verified || 0) + " verified"
            : "no game functions";
          if (b && b.verified > 0) share = b.verified / b.total;
        } else {
          stage = STAGE_NAMES[entry.stage] ? entry.stage : "unmeasured";
          detail = entry.count ? fmt.format(entry.count) + " functions" : "no game functions";
          if (entry.count && typeof entry.verified === "number") {
            detail += ": " + fmt.format(entry.named || 0) + " named, " + fmt.format(entry.rewritten || 0) + " rewritten, " +
              fmt.format(entry.verified) + " verified";
            if (entry.verified > 0) share = entry.verified / entry.count;
          } else if (entry.count) {
            detail = STAGE_NAMES[stage] + ", " + detail;
          }
        }
        cell.dataset.stage = stage;
        if (share > 0 && stage !== "verified") {
          cell.style.setProperty("--done", Math.max(12, Math.round(100 * share)) + "%");
          cell.classList.add("part");
        }
        cell.dataset.detail = detail;
      });
      if (active >= 0) select(active);
    }

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
    paint();

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
}

// Progress over time: one small chart per stage, drawn as inline SVG from data/history.json
// (written by scripts/site/history.py from the git history of progress.json). The three counts differ in size
// and two of them sit close together, so they get a panel each, with a shared time axis, rather than one
// chart where rewritten and verified would draw on top of each other.
function renderHistory(file) {
  var points = file && Array.isArray(file.points) ? file.points.filter(function (p) {
    return p && typeof p.t === "string" && !isNaN(Date.parse(p.t)) && typeof p.game === "number";
  }) : [];
  var host = document.getElementById("history");
  if (!points.length || !host) return;
  host.hidden = false;

  var SVG = "http://www.w3.org/2000/svg";
  var HEIGHT = 150, LEFT = 52, RIGHT = 12, TOP = 10, BOTTOM = 24, DOT = 4;
  var SERIES = [["named", "Named"], ["rewritten", "Rewritten in Rust"], ["verified", "Verified"]];
  var fmt = new Intl.NumberFormat("en-US");
  var last = points[points.length - 1];
  var times = points.map(function (p) { return Date.parse(p.t); });
  var t0 = times[0], t1 = times[times.length - 1];

  function when(ms, withDay) {
    var options = { hour: "2-digit", minute: "2-digit" };
    if (withDay) { options.day = "numeric"; options.month = "short"; }
    return new Date(ms).toLocaleString("en-GB", options);
  }
  function node(tag, attrs, text) {
    var n = document.createElementNS(SVG, tag);
    Object.keys(attrs || {}).forEach(function (k) { n.setAttribute(k, attrs[k]); });
    if (text != null) n.textContent = text;
    return n;
  }
  function el(tag, className, text) {
    var n = document.createElement(tag);
    if (className) n.className = className;
    if (text != null) n.textContent = text;
    return n;
  }
  // Three round tick values spanning [lo, hi], with the domain widened to the outer ticks.
  function ticks(lo, hi) {
    if (hi - lo < 2) { lo -= 1; hi += 1; }
    var raw = (hi - lo) / 2, power = Math.pow(10, Math.floor(Math.log10(raw)));
    var step = [1, 2, 2.5, 5, 10].map(function (m) { return m * power; }).filter(function (s) { return s >= raw; })[0];
    var start = Math.floor(lo / step) * step;
    while (start + 2 * step < hi) step = step * 2;
    return [start, start + step, start + 2 * step];
  }

  var spanDays = (t1 - t0) / 86400000;
  document.getElementById("history-note").textContent =
    points.length < 2
      ? "One recorded point so far, at " + when(t1, true) + ". A line appears once the next update is recorded."
      : "From " + when(t0, true) + " to " + when(t1, true) + " (your time zone), one point per recorded update. " +
        "Each chart's scale starts near its own lowest value, so the slopes show change over time, not each count's share of the " +
        fmt.format(last.game) + " game functions." + (spanDays < 2 ? " The record starts recently, so the window is short." : "");

  var charts = document.getElementById("history-charts");
  var panels = SERIES.map(function (series) {
    var key = series[0];
    var figure = el("figure", "history-panel");
    var caption = el("figcaption");
    caption.appendChild(el("span", "history-label", series[1]));
    var value = el("span", "history-value");
    value.setAttribute("aria-live", "polite");
    caption.appendChild(value);
    figure.appendChild(caption);
    var plot = el("div", "history-plot");
    plot.tabIndex = 0;
    plot.setAttribute("role", "group");
    var first = points[0][key], now = last[key];
    plot.setAttribute("aria-label", series[1] + ": " + fmt.format(first) + " at " + when(t0, true) + ", " +
      fmt.format(now) + " at " + when(t1, true) + ". Use the left and right arrow keys to read each point.");
    figure.appendChild(plot);
    charts.appendChild(figure);
    return { key: key, value: value, plot: plot, active: -1 };
  });

  function showValue(panel, index) {
    var p = points[index < 0 ? points.length - 1 : index];
    var v = p[panel.key];
    var text = fmt.format(v) + " (" + (100 * v / p.game).toFixed(1) + "%)";
    if (index < 0 && points.length > 1) {
      var change = v - points[0][panel.key];
      text += ", " + (change >= 0 ? "+" : "−") + fmt.format(Math.abs(change)) + " in this window";
    } else if (index >= 0) {
      text += " at " + when(Date.parse(p.t), true);
    }
    panel.value.textContent = text;
  }

  function draw() {
    panels.forEach(function (panel) {
      var width = Math.max(220, panel.plot.clientWidth || 300);
      var values = points.map(function (p) { return p[panel.key]; });
      var axis = ticks(Math.min.apply(null, values), Math.max.apply(null, values));
      var lo = axis[0], hi = axis[2];
      var x = function (ms) { return t1 === t0 ? (LEFT + width - RIGHT) / 2 : LEFT + (ms - t0) / (t1 - t0) * (width - LEFT - RIGHT); };
      var y = function (v) { return TOP + (1 - (v - lo) / (hi - lo)) * (HEIGHT - TOP - BOTTOM); };
      var svg = node("svg", { width: width, height: HEIGHT, viewBox: "0 0 " + width + " " + HEIGHT, "aria-hidden": "true", focusable: "false" });
      axis.forEach(function (v) {
        svg.appendChild(node("line", { class: "grid", x1: LEFT, x2: width - RIGHT, y1: y(v), y2: y(v) }));
        svg.appendChild(node("text", { class: "tick", x: LEFT - 8, y: y(v) + 4, "text-anchor": "end" }, fmt.format(v)));
      });
      svg.appendChild(node("text", { class: "tick", x: LEFT, y: HEIGHT - 6, "text-anchor": "start" }, when(t0, spanDays >= 1)));
      if (t1 !== t0) svg.appendChild(node("text", { class: "tick", x: width - RIGHT, y: HEIGHT - 6, "text-anchor": "end" }, when(t1, spanDays >= 1)));
      var path = points.map(function (p, i) { return (i ? "L" : "M") + x(times[i]).toFixed(1) + " " + y(p[panel.key]).toFixed(1); }).join(" ");
      if (points.length > 1) svg.appendChild(node("path", { class: "line", d: path }));
      svg.appendChild(node("circle", { class: "dot", cx: x(t1), cy: y(last[panel.key]), r: DOT }));
      var cross = node("line", { class: "cross", y1: TOP, y2: HEIGHT - BOTTOM, visibility: "hidden" });
      var mark = node("circle", { class: "dot hover", r: DOT, visibility: "hidden" });
      svg.appendChild(cross);
      svg.appendChild(mark);
      panel.plot.replaceChildren(svg);
      panel.pick = function (index) {
        panel.active = index;
        var on = index >= 0;
        cross.setAttribute("visibility", on ? "visible" : "hidden");
        mark.setAttribute("visibility", on ? "visible" : "hidden");
        if (on) {
          var cx = x(times[index]);
          cross.setAttribute("x1", cx); cross.setAttribute("x2", cx);
          mark.setAttribute("cx", cx); mark.setAttribute("cy", y(points[index][panel.key]));
        }
        showValue(panel, index);
      };
      panel.indexAt = function (clientX) {
        var box = panel.plot.getBoundingClientRect(), px = clientX - box.left, best = 0;
        times.forEach(function (ms, i) { if (Math.abs(x(ms) - px) < Math.abs(x(times[best]) - px)) best = i; });
        return best;
      };
      panel.pick(panel.active);
    });
  }

  // Hovering or stepping through one panel moves the crosshair in all three, so the counts read together.
  function pickAll(index) { panels.forEach(function (panel) { panel.pick(index); }); }
  panels.forEach(function (panel) {
    panel.plot.addEventListener("pointermove", function (e) { pickAll(panel.indexAt(e.clientX)); });
    panel.plot.addEventListener("pointerleave", function () { pickAll(-1); });
    panel.plot.addEventListener("blur", function () { pickAll(-1); });
    panel.plot.addEventListener("keydown", function (e) {
      var step = { ArrowRight: 1, ArrowLeft: -1, Home: -Infinity, End: Infinity }[e.key];
      if (!step) return;
      e.preventDefault();
      var next = panel.active < 0 ? points.length - 1 : panel.active + step;
      pickAll(Math.max(0, Math.min(points.length - 1, isFinite(next) ? next : (step > 0 ? points.length - 1 : 0))));
    });
  });
  draw();
  var pending = null;
  window.addEventListener("resize", function () { clearTimeout(pending); pending = setTimeout(draw, 120); });

  // The table: newest first, every recorded point.
  var rows = document.getElementById("history-rows");
  points.slice().reverse().forEach(function (p) {
    var tr = document.createElement("tr");
    var cells = [when(Date.parse(p.t), true), fmt.format(p.named), fmt.format(p.rewritten), fmt.format(p.verified)];
    ["Time", "Named", "Rewritten", "Verified"].forEach(function (label, i) {
      var td = el("td", null, cells[i]);
      if (i) td.dataset.label = label;
      tr.appendChild(td);
    });
    rows.appendChild(tr);
  });
}

// The page's numbers come straight from data/progress.json.
fetch("data/progress.json", { cache: "no-store" })
  .then(function (response) { return response.ok ? response.json() : {}; })
  .catch(function () { return {}; })
  .then(render);

fetch("data/history.json", { cache: "no-store" })
  .then(function (response) { return response.ok ? response.json() : null; })
  .catch(function () { return null; })
  .then(renderHistory);
