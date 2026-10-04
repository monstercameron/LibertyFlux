// Engine page: the line map's stop details, the class grid, the stack machine and the millisecond ruler.
(function () {
  // Each stop: what it is, a few numbers, what was measured, and what is only inferred.
  var STOPS = {
    input: {
      title: "Input",
      text: "Gamepads come in two ways: the newer Xbox controller interface, and an older one with a built-in list of about 20 known pads. Keyboard and mouse arrive as ordinary Windows messages.",
      facts: [["about 20", "gamepads on the built-in list"], ["2", "controller interfaces"]],
      measured: "both interfaces and the pad list are in the program."
    },
    scripts: {
      title: "Scripts",
      text: "Missions and world events are scripts run by a small interpreter. A script cannot reach into the engine: it calls a named command and waits for the answer.",
      facts: [["941", "script files"], ["78", "instructions in the interpreter"], ["3,034", "commands scripts can call"]],
      measured: "counted from the script files and the interpreter's code.",
      more: ["#scripts", "Step through the stack machine"]
    },
    people: {
      title: "People",
      text: "Every pedestrian carries layers of tasks, from a goal down to a single action. Events such as a gunshot or taking damage can replace any layer.",
      facts: [["334", "task classes"], ["82", "kinds of event"], ["13,286", "functions, the largest group"]],
      measured: "class and function counts.",
      more: ["#people", "See how the layers fit together"]
    },
    vehicles: {
      title: "Vehicles",
      text: "Six kinds of vehicle share one base. Helicopters and planes are built on top of the car class. How each one drives comes from a text table, not from code.",
      facts: [["178", "vehicles in the handling table"], ["37", "tuning values for each"], ["4,864 bytes", "of memory per vehicle"]],
      measured: "the class tree, the handling table and the object size."
    },
    physics: {
      title: "Physics",
      text: "Rockstar's own rigid-body code sits on top of the open-source Bullet library for collision detection. The step it takes follows the length of the frame: we found no fixed step.",
      facts: [["92", "physics classes"], ["9,102", "collision shapes on disk"]],
      measured: "the classes, the Bullet code, and the absence of a fixed-step constant.",
      inferred: "how much the variable step changes behaviour at high frame rates. That is not measured yet.",
      more: ["#clock", "See what a millisecond clock does at high frame rates"]
    },
    animation: {
      title: "Animation",
      text: "Clips are stored with 15 different compression schemes and blended by a tree of nodes. The game thread starts the work and a pool of worker threads shares it. When a body goes limp, NaturalMotion's Euphoria drives the skeleton from physics.",
      facts: [["15", "ways a clip can be stored"], ["18", "Euphoria behaviours"], ["148", "animation classes"]],
      measured: "codec, behaviour and class counts, and the tree's use of a thread pool."
    },
    drawlists: {
      title: "Draw lists",
      text: "What should be drawn is written down as a list of small commands, sorted into groups, and carried out by the render thread. Every command can report its own size, so the list can be walked without knowing what is in it.",
      facts: [["8 bytes", "header at the start of every command"], ["31", "kinds of draw list"]],
      measured: "the command interface, the header, the list kinds and the separate render thread.",
      inferred: "that the game thread does no drawing itself and moves straight on to the next frame. The main loop is not readable yet."
    },
    shadows: {
      title: "Shadows and reflections",
      text: "Before the main picture, the scene is drawn from each light's point of view to make shadow maps, and again for reflections in water and mirrors.",
      facts: [["24", "render phases in total"]],
      measured: "the render phase classes and the shadow shaders.",
      inferred: "that this comes first, from the shaders and from public studies of this engine family."
    },
    gbuffer: {
      title: "Surfaces",
      text: "The renderer is deferred. It first draws every solid surface into four screen-sized images that record what the surface at each pixel is like, without lighting anything yet.",
      facts: [["4", "images written at once"], ["Direct3D 9", "the only graphics interface used"]],
      measured: "four render targets, and Direct3D 9 as the program's only graphics import."
    },
    lighting: {
      title: "Lighting",
      text: "Each light is then drawn as a volume that covers only the pixels it can reach, and reads the four surface images to work out the lit colour.",
      facts: [["about 30", "settings fed to the light shader"]],
      measured: "the light shader's settings and the volume technique, read from the shaders."
    },
    transparent: {
      title: "Glass and effects",
      text: "Anything see-through cannot use the surface images, so glass, smoke, sparks and other particles are drawn afterwards, one on top of another.",
      facts: [["174", "rendering classes"]],
      measured: "the class count.",
      inferred: "the pass and its position, from the shaders and the render phase classes."
    },
    post: {
      title: "Post effects",
      text: "The lit image holds a wider range of brightness than a monitor can show. A chain of full-screen passes adjusts it and brings it into range.",
      facts: [["612", "shader files"], ["10,134", "compiled shader programs"]],
      measured: "shader counts and the post-processing shaders."
    },
    hud: {
      title: "HUD and menus",
      text: "The radar, phone, subtitles and menus are drawn last, by a widget framework that is separate from the 3D renderer. Menus are partly described in data files.",
      facts: [["52", "kinds of widget"], ["78", "interface classes"]],
      measured: "widget and class counts, and the menu description files."
    },
    present: {
      title: "Present",
      text: "The picture is handed to Windows. The program asks Direct3D 9 for a device and nothing else: every shader was compiled before the game shipped, in six sets for different graphics cards.",
      facts: [["6", "graphics card variants of each shader"], ["no", "shader compiler in the program"]],
      measured: "the program's imports and the six shader sets on disk."
    },
    usermusic: {
      title: "Your own music",
      text: "Two of the four audio threads exist only for the player's own music station: one scans the music folder, the other decodes tracks as they play.",
      facts: [["2", "of the 4 audio threads"]],
      measured: "thread names in the program."
    },
    soundengine: {
      title: "Sound engine",
      text: "One thread decides what should be heard. Most of its behaviour is data: curves that shape volume over distance, schedules for radio stations with their DJs and news, and triggers for speech.",
      facts: [["57", "curve tables"], ["92", "audio classes"]],
      measured: "curve tables, class counts and thread names."
    },
    mixer: {
      title: "Mixer",
      text: "One thread mixes every sound into the final signal. We found no sound middleware in the program: the mixing code is the engine's own.",
      facts: [["1", "mixer thread"]],
      measured: "thread names and the list of libraries the program uses."
    },
    output: {
      title: "Sound card",
      text: "The mixed signal goes to Windows through DirectSound. The audio threads keep their own schedule, and the game thread tells them what changed once a frame.",
      facts: [["DirectSound", "output interface"]],
      measured: "the DirectSound imports.",
      inferred: "the once-a-frame update, from how the audio code is called."
    },
    archives: {
      title: "Archives",
      text: "The city is stored in 303 archive files. The streaming system keeps track of what is near the player and asks for those files.",
      facts: [["303", "archives"], ["20.3 GB", "inside them"]],
      measured: "archive counts and sizes on disk.",
      inferred: "how requests are scheduled. Most of that code is not readable yet.",
      more: ["#disk", "See what is on disk"]
    },
    unpack: {
      title: "Unpack",
      text: "Each file inside an archive is a compressed resource in two parts: one for main memory, one for the graphics card. After decompression the addresses inside it are corrected and it is usable as it stands.",
      facts: [["2", "parts in every resource"], ["zlib", "compression"]],
      measured: "checked on every loose resource file in the game folder."
    },
    world: {
      title: "Into the world",
      text: "Loaded objects take a slot in a fixed-size pool for their type. Far-away pedestrians are held as cheap stand-ins and swapped for the full version when the player gets close.",
      facts: [["26,713", "model files to draw from"]],
      measured: "pool-full error messages and the stand-in classes.",
      inferred: "the order of the stops on this line."
    }
  };
  var LINE_NAMES = { game: "Game thread", render: "Render thread", audio: "Audio", loader: "Streaming" };

  var panel = document.getElementById("stop-detail");
  var aside = document.getElementById("map-aside");
  var narrow = window.matchMedia("(max-width: 1200px)");
  var buttons = Array.prototype.slice.call(document.querySelectorAll(".stop"));
  var lines = Array.prototype.slice.call(document.querySelectorAll(".line"));
  var seen = {};
  var current = null;

  function evidence(id, label, value) {
    var node = document.getElementById(id);
    node.textContent = "";
    if (!value) return;
    var b = document.createElement("b");
    b.textContent = label + ": ";
    node.appendChild(b);
    node.appendChild(document.createTextNode(value));
  }

  function place(button) {
    var line = button.closest(".line");
    panel.style.setProperty("--line", getComputedStyle(line).getPropertyValue("--line"));
    (narrow.matches ? button.parentNode : aside).appendChild(panel);
  }

  function select(button, fromUser) {
    var stop = STOPS[button.dataset.stop];
    if (!stop) return;
    var before = button.getBoundingClientRect().top;
    var line = button.closest(".line");
    line.open = true;
    buttons.forEach(function (b) { b.setAttribute("aria-pressed", "false"); });
    button.setAttribute("aria-pressed", "true");
    if (fromUser) {
      button.classList.add("seen");
      seen[button.dataset.stop] = true;
    }
    current = button;
    document.getElementById("stop-line").textContent = LINE_NAMES[line.dataset.line];
    document.getElementById("stop-title").textContent = stop.title;
    document.getElementById("stop-text").textContent = stop.text;
    var facts = document.getElementById("stop-facts");
    facts.textContent = "";
    stop.facts.forEach(function (fact) {
      var li = document.createElement("li");
      var strong = document.createElement("strong");
      strong.textContent = fact[0];
      li.appendChild(strong);
      li.appendChild(document.createTextNode(" " + fact[1]));
      facts.appendChild(li);
    });
    evidence("stop-measured", "Measured", stop.measured);
    evidence("stop-inferred", "Inferred", stop.inferred);
    var more = document.getElementById("stop-more");
    document.getElementById("stop-more-row").hidden = !stop.more;
    if (stop.more) {
      more.href = stop.more[0];
      more.textContent = stop.more[1];
    }
    document.getElementById("stop-count").textContent = Object.keys(seen).length + " of " + buttons.length + " stops seen";
    place(button);
    panel.hidden = false;
    if (fromUser && narrow.matches) {
      // The panel moved, so content above the pressed stop may have changed height: keep the stop where it was.
      window.scrollBy(0, button.getBoundingClientRect().top - before);
      panel.scrollIntoView({ block: "nearest" });
    }
  }

  buttons.forEach(function (button) {
    button.setAttribute("aria-pressed", "false");
    button.addEventListener("click", function () { select(button, true); });
  });
  document.getElementById("stop-next").addEventListener("click", function () {
    var next = buttons[(buttons.indexOf(current) + 1) % buttons.length];
    select(next, true);
    if (narrow.matches) next.focus();
  });

  // On wide screens every line stays open; on narrow ones they are collapsible and only the first starts open.
  function layout() {
    lines.forEach(function (line, index) {
      line.open = narrow.matches ? (index === 0 || line.contains(current)) : true;
      line.querySelector("summary").tabIndex = narrow.matches ? 0 : -1;
    });
    if (current) place(current);
  }
  lines.forEach(function (line) {
    line.querySelector("summary").addEventListener("click", function (event) {
      if (!narrow.matches) event.preventDefault();
    });
  });
  narrow.addEventListener("change", layout);
  select(document.querySelector('.stop[data-stop="scripts"]'), false);
  layout();

  // Class grid: 100 squares, each about 37 classes.
  var waffle = document.getElementById("waffle");
  for (var i = 0; i < 100; i++) {
    var cell = document.createElement("span");
    cell.className = i < 52 ? "lb" : i < 61 ? "task" : "rest";
    waffle.appendChild(cell);
  }

  // Stack machine
  var PROGRAM = [
    { say: "PUSH 5 puts the number 5 on top of the stack.", run: function (s) { s.push(5); } },
    { say: "PUSH 7 puts 7 on top of it.", run: function (s) { s.push(7); } },
    { say: "ADD takes the top two numbers off and puts their sum back: 12.", run: function (s) { s.push(s.pop() + s.pop()); } }
  ];
  var stack = [];
  var pc = 0;
  var codeItems = document.querySelectorAll("#vm-code li");
  var stackHost = document.getElementById("vm-stack");
  var status = document.getElementById("vm-status");
  var stepButton = document.getElementById("vm-step");
  var resetButton = document.getElementById("vm-reset");

  function drawVm(message) {
    codeItems.forEach(function (li, index) {
      li.className = index < pc ? "done" : index === pc ? "next" : "";
    });
    stackHost.textContent = "";
    if (!stack.length) {
      var empty = document.createElement("em");
      empty.textContent = "empty";
      stackHost.appendChild(empty);
    }
    stack.forEach(function (value) {
      var slot = document.createElement("span");
      slot.textContent = value;
      stackHost.appendChild(slot);
    });
    status.textContent = message;
    stepButton.disabled = pc >= PROGRAM.length;
    resetButton.disabled = pc === 0;
  }
  stepButton.addEventListener("click", function () {
    var step = PROGRAM[pc];
    step.run(stack);
    pc += 1;
    drawVm(step.say + (pc >= PROGRAM.length ? " The program is finished." : ""));
    // A disabled button drops keyboard focus to the page: hand it to the button that can act next.
    if (stepButton.disabled) resetButton.focus();
  });
  resetButton.addEventListener("click", function () {
    stack = [];
    pc = 0;
    drawVm("Press Step to run the first instruction.");
    stepButton.focus();
  });
  drawVm("Press Step to run the first instruction.");

  // Millisecond ruler: the frame spans the full width, with one tick per whole millisecond.
  var fps = document.getElementById("fps");
  var ruler = document.getElementById("ruler");
  var readout = document.getElementById("clock-readout");
  var presets = Array.prototype.slice.call(document.querySelectorAll(".presets button"));
  function drawClock() {
    var rate = Number(fps.value);
    var frame = 1000 / rate;
    var whole = Math.floor(frame + 1e-9);
    document.getElementById("fps-out").textContent = rate;
    presets.forEach(function (b) { b.setAttribute("aria-pressed", String(Number(b.dataset.fps) === rate)); });
    ruler.textContent = "";
    // Label only as many ticks as fit: about 56 pixels per label.
    var width = ruler.clientWidth || 600;
    var need = 56 / (width / frame);
    var every = need <= 1 ? 1 : need <= 2 ? 2 : need <= 5 ? 5 : 10;
    for (var ms = 1; ms <= whole; ms++) {
      if (ms / frame > 0.999) break;
      var tick = document.createElement("span");
      tick.className = "tick";
      tick.style.left = (ms / frame * 100) + "%";
      if (ms % every === 0 && (1 - ms / frame) * width > 50) {
        var label = document.createElement("b");
        label.textContent = ms + " ms";
        tick.appendChild(label);
      }
      ruler.appendChild(tick);
    }
    var rest = frame - whole;
    var hasRest = rest > 0.005;
    if (hasRest) {
      var leftover = document.createElement("span");
      leftover.className = "leftover";
      leftover.style.width = (rest / frame * 100) + "%";
      ruler.insertBefore(leftover, ruler.firstChild);
    }
    document.getElementById("ruler-key").hidden = !hasRest;
    var share = 100 / frame;
    readout.textContent = "";
    var lead = document.createElement("strong");
    lead.textContent = "One frame lasts " + frame.toFixed(2) + " ms. ";
    readout.appendChild(lead);
    var percent = (share < 10 ? share.toFixed(1) : share.toFixed(0)) + "% of this frame.";
    readout.appendChild(document.createTextNode(hasRest
      ? "A clock that counts whole milliseconds sees " + whole + " or " + (whole + 1) + ", never " + frame.toFixed(2) + ". Any reading can be off by up to 1 ms, which is " + percent
      : "A clock that counts whole milliseconds sees " + whole + " only when frames are perfectly steady. Real frames vary, and any reading can be off by up to 1 ms, which is " + percent));
  }
  fps.addEventListener("input", drawClock);
  window.addEventListener("resize", drawClock);
  presets.forEach(function (button) {
    button.addEventListener("click", function () { fps.value = button.dataset.fps; drawClock(); });
  });
  drawClock();
})();
