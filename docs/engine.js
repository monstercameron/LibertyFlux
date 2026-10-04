// Engine page: the line map's stop details, the class grid, the stack machine and the millisecond ruler.
(function () {
  var MEASURED = "Measured in the executable or the game's files.";
  var STOPS = {
    input: {
      title: "Input",
      text: "Gamepads come in two ways: the newer Xbox controller interface, and an older one with a built-in list of about 20 known pads. Keyboard and mouse arrive as ordinary Windows messages.",
      facts: [["about 20", "gamepads on the built-in list"], ["2", "controller interfaces"]],
      evidence: MEASURED
    },
    scripts: {
      title: "Scripts",
      text: "Missions and world events are scripts run by a small interpreter. A script cannot reach into the engine: it calls a named command and waits for the answer.",
      facts: [["941", "script files"], ["78", "instructions in the interpreter"], ["3,034", "commands scripts can call"]],
      evidence: MEASURED
    },
    people: {
      title: "People",
      text: "Every pedestrian carries a stack of tasks, from a goal down to a single action. Events such as a gunshot or taking damage can replace any level of it.",
      facts: [["334", "task classes"], ["82", "kinds of event"], ["13,286", "functions, the largest share of the code"]],
      evidence: MEASURED
    },
    vehicles: {
      title: "Vehicles",
      text: "Six kinds of vehicle share one base. Helicopters and planes are built on top of the car class. How each one drives comes from a text table, not from code.",
      facts: [["178", "vehicles in the handling table"], ["37", "tuning values for each"], ["4,864 bytes", "of memory per vehicle"]],
      evidence: MEASURED
    },
    physics: {
      title: "Physics",
      text: "Rockstar's own rigid-body code sits on top of the open-source Bullet library for collision detection. The step it takes follows the length of the frame: the code has no fixed step, which is why behaviour changes with frame rate.",
      facts: [["92", "physics classes"], ["9,102", "collision shapes on disk"], ["no", "fixed time step"]],
      evidence: MEASURED
    },
    animation: {
      title: "Animation",
      text: "Clips are stored with 15 different compression schemes and blended by a tree of nodes that runs on worker threads. When a body goes limp, NaturalMotion's Euphoria takes over and drives the skeleton from physics.",
      facts: [["15", "ways a clip can be stored"], ["18", "Euphoria behaviours"], ["148", "animation classes"]],
      evidence: MEASURED
    },
    drawlists: {
      title: "Draw lists",
      text: "The game thread never draws. It writes down what should be drawn as a list of small commands, sorts them into groups and hands the list to the render thread. Every command can report its own size, so the list can be walked without knowing what is in it.",
      facts: [["8 bytes", "header at the start of every command"], ["31", "kinds of draw list"]],
      evidence: MEASURED
    },
    shadows: {
      title: "Shadows and reflections",
      text: "Before the main picture, the scene is drawn from each light's point of view to make shadow maps, and again for reflections in water and mirrors.",
      facts: [["24", "render phases in total"]],
      evidence: "The phases are measured. Their order is inferred from the shaders and from public studies of this engine family."
    },
    gbuffer: {
      title: "Surfaces",
      text: "The renderer is deferred. It first draws every solid surface into four screen-sized images that record what the surface at each pixel is like, without lighting anything yet.",
      facts: [["4", "images written at once"], ["Direct3D 9", "the only graphics interface used"]],
      evidence: MEASURED
    },
    lighting: {
      title: "Lighting",
      text: "Each light is then drawn as a volume that touches only the pixels it can reach, reading the four surface images to work out the lit colour. The cost of a light depends on how much of the screen it covers, not on how much scenery there is.",
      facts: [["about 30", "settings fed to the light shader"]],
      evidence: MEASURED
    },
    transparent: {
      title: "Glass and effects",
      text: "Anything see-through cannot use the surface images, so glass, smoke, sparks and other particles are drawn afterwards, one on top of another, in the older forward style.",
      facts: [["174", "rendering classes"]],
      evidence: "Inferred from the shaders and the render phase classes."
    },
    post: {
      title: "Post effects",
      text: "The finished image is still in high dynamic range. A chain of full-screen passes adjusts it and brings it into the range a monitor can show.",
      facts: [["612", "shader files"], ["10,134", "compiled shader programs"]],
      evidence: MEASURED
    },
    hud: {
      title: "HUD and menus",
      text: "The radar, phone, subtitles and menus are drawn last, by a widget framework that is separate from the 3D renderer. Menus are partly described in data files.",
      facts: [["52", "kinds of widget"], ["78", "interface classes"]],
      evidence: MEASURED
    },
    present: {
      title: "Present",
      text: "The picture is handed to Windows. The executable asks Direct3D 9 for a device and nothing else: every shader was compiled before the game shipped, in six variants for different graphics cards.",
      facts: [["6", "graphics card variants of each shader"], ["no", "shader compiler in the executable"]],
      evidence: MEASURED
    },
    soundengine: {
      title: "Sound engine",
      text: "Decides what should be heard. Most of its behaviour is data: curves that shape volume over distance, schedules for radio stations with their DJs and news, and triggers for speech.",
      facts: [["57", "curve tables"], ["92", "audio classes"]],
      evidence: MEASURED
    },
    usermusic: {
      title: "Your own music",
      text: "Two threads exist only for the player's own music station: one scans the music folder, the other decodes the tracks as they play.",
      facts: [["2", "threads"]],
      evidence: MEASURED
    },
    mixer: {
      title: "Mixer",
      text: "The engine mixes every sound into the final signal itself, on its own thread. No sound middleware is present in the executable.",
      facts: [["4", "audio threads in total"], ["no", "sound middleware"]],
      evidence: MEASURED
    },
    output: {
      title: "Sound card",
      text: "The mixed signal goes to Windows through DirectSound. The audio threads do not wait for a frame, so sound does not depend on the frame rate.",
      facts: [["DirectSound", "output interface"]],
      evidence: MEASURED
    },
    archives: {
      title: "Archives",
      text: "The city is stored in 303 archive files. The streaming system keeps track of what is near the player and asks for those files ahead of time.",
      facts: [["303", "archives"], ["20.3 GB", "inside them"]],
      evidence: MEASURED
    },
    unpack: {
      title: "Unpack",
      text: "Each file inside an archive is a compressed resource in two parts: one for main memory, one for the graphics card. After decompression its internal pointers are corrected and it is usable as it stands.",
      facts: [["2", "parts in every resource"], ["zlib", "compression"]],
      evidence: MEASURED
    },
    world: {
      title: "Into the world",
      text: "Loaded objects take a slot in a fixed-size pool for their type. Far-away pedestrians are held as cheap stand-ins and swapped for the full version when the player gets close.",
      facts: [["26,713", "model files to draw from"]],
      evidence: "The pools and stand-ins are measured. The stop order on this line is inferred."
    }
  };

  var panel = document.getElementById("stop-detail");
  var narrow = window.matchMedia("(max-width: 860px)");
  var current = null;

  function place(button) {
    var line = button.closest(".line");
    panel.style.setProperty("--line", getComputedStyle(line).getPropertyValue("--line"));
    (narrow.matches ? button.parentNode : line).appendChild(panel);
  }

  function select(button) {
    var stop = STOPS[button.dataset.stop];
    if (!stop) return;
    document.querySelectorAll(".stop[aria-pressed='true']").forEach(function (b) { b.setAttribute("aria-pressed", "false"); });
    button.setAttribute("aria-pressed", "true");
    current = button;
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
    document.getElementById("stop-evidence").textContent = stop.evidence;
    place(button);
    panel.hidden = false;
  }

  document.querySelectorAll(".stop").forEach(function (button) {
    button.setAttribute("aria-pressed", "false");
    button.addEventListener("click", function () { select(button); });
  });
  narrow.addEventListener("change", function () { if (current) place(current); });
  var first = document.querySelector('.stop[data-stop="scripts"]');
  if (first) select(first);

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
  }
  stepButton.addEventListener("click", function () {
    var step = PROGRAM[pc];
    step.run(stack);
    pc += 1;
    drawVm(step.say + (pc >= PROGRAM.length ? " The program is finished." : ""));
  });
  document.getElementById("vm-reset").addEventListener("click", function () {
    stack = [];
    pc = 0;
    drawVm("Press Step to run the first instruction.");
  });
  drawVm("Press Step to run the first instruction.");

  // Millisecond ruler: the frame spans the full width; one tick per millisecond.
  var fps = document.getElementById("fps");
  var ruler = document.getElementById("ruler");
  var readout = document.getElementById("clock-readout");
  function drawClock() {
    var rate = Number(fps.value);
    var frame = 1000 / rate;
    var low = Math.floor(frame);
    var high = Math.ceil(frame);
    document.getElementById("fps-out").textContent = rate;
    ruler.style.setProperty("--tick", (100 / frame).toFixed(3) + "%");
    readout.textContent = "";
    var lead = document.createElement("strong");
    lead.textContent = "One frame lasts " + frame.toFixed(2) + " ms. ";
    readout.appendChild(lead);
    var rest;
    if (low === high) {
      rest = "A whole-millisecond clock reads exactly " + low + " ms.";
    } else {
      var worst = Math.max(frame - low, high - frame) / frame * 100;
      rest = "A whole-millisecond clock reads " + low + " or " + high + " ms, so a single frame can be measured up to " + worst.toFixed(worst < 10 ? 1 : 0) + "% wrong.";
    }
    readout.appendChild(document.createTextNode(rest));
  }
  fps.addEventListener("input", drawClock);
  drawClock();
})();
