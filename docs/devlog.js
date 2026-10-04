/* The devlog page. The entries live in data/devlog.sqlite; this script downloads the database once, opens it in
   the browser with sql.js, and does all listing, filtering and searching locally.

   Exact search is one SQL query: every word (or quoted phrase) must occur in the title or the text.
   Close search ranks posts by how well each query word matches the words a post uses, allowing near
   spellings (shared three-letter pieces) and word beginnings. */
(function () {
  "use strict";
  var SQL_FILES = "https://cdnjs.cloudflare.com/ajax/libs/sql.js/1.10.3/";
  var PAGE = 30;
  var $ = function (id) { return document.getElementById(id); };
  var db = null, posts = [], byId = Object.create(null), vocabulary = null, shown = PAGE;
  var state = { q: "", mode: "exact", topic: "" };

  function esc(text) { return String(text == null ? "" : text).replace(/[&<>"]/g, function (c) { return { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c]; }); }
  function rows(sql, params) {
    var statement = db.prepare(sql), out = [];
    statement.bind(params || []);
    while (statement.step()) out.push(statement.getAsObject());
    statement.free();
    return out;
  }
  function words(text) { return (text.toLowerCase().match(/[a-z0-9][a-z0-9'-]*[a-z0-9]|[a-z0-9]/g) || []); }
  function terms(query) {
    var out = [], pattern = /"([^"]+)"|(\S+)/g, m;
    while ((m = pattern.exec(query))) { var t = (m[1] || m[2]).toLowerCase().trim(); if (t) out.push(t); }
    return out;
  }

  /* ---- close matching ---- */
  function pieces(word) {
    var padded = "  " + word + " ", set = Object.create(null);
    for (var i = 0; i < padded.length - 2; i++) set[padded.substr(i, 3)] = true;
    return Object.keys(set);
  }
  function buildVocabulary() {
    // word -> { posts: {index: weight}, pieces: [...] }; the title counts three times as much as the text.
    vocabulary = Object.create(null);   // not {}: "constructor" is a word in the devlog and a property of every {}
    var all = rows("SELECT id, title, text FROM posts");
    all.forEach(function (row) {
      var index = byId[row.id].index;
      function add(list, weight) {
        list.forEach(function (w) {
          if (w.length < 2) return;
          var entry = vocabulary[w] || (vocabulary[w] = { posts: {}, pieces: null });
          entry.posts[index] = Math.max(entry.posts[index] || 0, weight);
        });
      }
      add(words(row.text), 1);
      add(words(row.title), 3);
    });
  }
  function similar(queryWord, queryPieces, word, entry) {
    if (word === queryWord) return 1;
    if (queryWord.length >= 3 && word.indexOf(queryWord) === 0) return 0.9;        // a beginning: "check" finds "checker"
    if (queryWord.length < 4 || Math.abs(word.length - queryWord.length) > 3) return 0;
    if (!entry.pieces) entry.pieces = pieces(word);
    var shared = 0, lookup = Object.create(null);
    queryPieces.forEach(function (p) { lookup[p] = true; });
    entry.pieces.forEach(function (p) { if (lookup[p]) shared++; });
    var score = 2 * shared / (queryPieces.length + entry.pieces.length);          // 1 when identical
    return score >= 0.55 ? score * 0.85 : 0;
  }
  function closeSearch(query) {
    if (!vocabulary) buildVocabulary();
    var queryWords = words(query).filter(function (w) { return w.length >= 2; });
    if (!queryWords.length) return null;
    var totals = Object.create(null), matched = Object.create(null), hits = Object.create(null);
    queryWords.forEach(function (qw) {
      var qp = pieces(qw), best = Object.create(null);
      Object.keys(vocabulary).forEach(function (word) {
        var entry = vocabulary[word], s = similar(qw, qp, word, entry);
        if (!s) return;
        Object.keys(entry.posts).forEach(function (index) {
          var value = s * entry.posts[index];
          if (!best[index] || value > best[index].value) best[index] = { value: value, word: word };
        });
      });
      Object.keys(best).forEach(function (index) {
        totals[index] = (totals[index] || 0) + best[index].value;
        hits[index] = (hits[index] || 0) + 1;
        (matched[index] || (matched[index] = [])).push(best[index].word);
      });
    });
    var need = Math.max(1, Math.ceil(queryWords.length * 0.6));
    return Object.keys(totals).filter(function (index) { return hits[index] >= need; })
      .sort(function (a, b) { return totals[b] - totals[a] || posts[b].seq - posts[a].seq; })
      .map(function (index) { return { post: posts[index], words: matched[index] }; });
  }

  /* ---- exact matching, in SQL ---- */
  function exactSearch(query) {
    var list = terms(query);
    if (!list.length) return null;
    var where = list.map(function () { return "instr(lower(title || ' ' || text), ?) > 0"; }).join(" AND ");
    return rows("SELECT id FROM posts WHERE " + where + " ORDER BY seq DESC", list)
      .map(function (row) { return { post: byId[row.id], words: list }; });
  }

  function snippet(post, needles) {
    var text = post.text || (post.text = rows("SELECT text FROM posts WHERE id = ?", [post.id])[0].text);
    var lower = text.toLowerCase(), at = -1, found = "";
    needles.some(function (n) { var i = lower.indexOf(n, post.title.length); if (i >= 0) { at = i; found = n; return true; } return false; });
    if (at < 0) return esc(post.summary);
    var start = Math.max(post.title.length, at - 90), end = Math.min(text.length, at + found.length + 150);
    var cut = (start > post.title.length ? "… " : "") + text.slice(start, end).trim() + (end < text.length ? " …" : "");
    return mark(cut, needles);
  }
  function mark(text, needles) {
    var escaped = esc(text);
    needles.slice().sort(function (a, b) { return b.length - a.length; }).forEach(function (n) {
      if (n.length < 2) return;
      var pattern = new RegExp("(" + esc(n).replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + ")(?![^<]*>)", "gi");
      escaped = escaped.replace(pattern, "<mark>$1</mark>");
    });
    return escaped;
  }

  /* ---- drawing ---- */
  function current() {
    var found = state.q ? (state.mode === "fuzzy" ? closeSearch(state.q) : exactSearch(state.q)) : null;
    var list = found || posts.map(function (p) { return { post: p, words: [] }; });
    if (state.topic) list = list.filter(function (item) { return item.post.topics.indexOf(state.topic) >= 0; });
    return { list: list, searched: !!found };
  }
  function drawList() {
    var result = current(), list = result.list, html = "";
    var what = state.q ? (state.mode === "fuzzy" ? " closely matching " : " containing ") + "“" + state.q + "”" : "";
    $("status").textContent = list.length
      ? list.length + " of " + posts.length + " posts" + what + (state.topic ? " about " + state.topic : "") +
        (result.searched && state.mode === "fuzzy" ? ", best matches first." : ", newest first.")
      : "No posts" + what + (state.topic ? " about " + state.topic : "") + "." +
        (state.q && state.mode === "exact" ? " Try the close match, which allows near spellings." : "");
    list.slice(0, shown).forEach(function (item) {
      var p = item.post;
     var differs = item.words.filter(function (w) { return state.q.toLowerCase().indexOf(w) < 0; });
      html += '<li><span class="when">' + esc(longDate(p.date)) + "</span>" +
        '<h2><a href="#' + esc(p.id) + '">' + (item.words.length ? mark(p.title, item.words) : esc(p.title)) + "</a></h2>" +
        "<p>" + (item.words.length ? snippet(p, item.words) : esc(p.summary)) + "</p>" +
        (state.mode === "fuzzy" && differs.length ? '<p class="why">Matched as: ' + esc(differs.join(", ")) + "</p>" : "") +
        '<p class="topics">' + esc(p.topics.join(", ")) + "</p></li>";
    });
    $("posts").innerHTML = html;
    $("more").hidden = list.length <= shown;
    $("more").textContent = "Show " + Math.min(PAGE, list.length - shown) + " more";
  }
  function longDate(iso) {
    var d = new Date(iso + "T12:00:00");
    return isNaN(d) ? iso : d.toLocaleDateString("en-GB", { weekday: "long", day: "numeric", month: "long", year: "numeric" });
  }
  function drawTopics() {
    var counts = rows("SELECT tag, COUNT(*) AS n FROM tags GROUP BY tag ORDER BY n DESC, tag");
    $("topics").innerHTML = '<button type="button" data-topic="" aria-pressed="' + (!state.topic) + '">All topics<span class="n">' + posts.length + "</span></button>" +
      counts.map(function (c) {
        return '<button type="button" data-topic="' + esc(c.tag) + '" aria-pressed="' + (state.topic === c.tag) + '">' + esc(c.tag) + '<span class="n">' + c.n + "</span></button>";
      }).join("");
  }
  function drawPost(id) {
    var p = byId[id];
    if (!p) return false;
    var row = rows("SELECT html FROM posts WHERE id = ?", [id])[0];
    var newer = posts[p.index - 1], older = posts[p.index + 1];
    $("reader").innerHTML = '<nav class="reader-nav" aria-label="Devlog posts"><a href="#">All posts</a><span class="spacer"></span>' +
      (newer ? '<a href="#' + esc(newer.id) + '" title="' + esc(newer.title) + '">Newer post</a>' : "") +
      (older ? '<a href="#' + esc(older.id) + '" title="' + esc(older.title) + '">Older post</a>' : "") + "</nav>" +
      '<article class="entry" id="' + esc(p.id) + '"><div class="entry-meta"><time datetime="' + esc(p.date) + '">' + esc(p.date) + "</time>" +
      '<span class="topics">' + esc(p.topics.join(", ")) + "</span></div><h2>" + esc(p.title) + "</h2>" + row.html + "</article>";
    document.title = p.title + " (LibertyFlux devlog)";
    return true;
  }
  function route() {
    var id = decodeURIComponent(location.hash.replace(/^#/, ""));
    var reading = !!id && drawPost(id);
    $("reader").hidden = !reading;
    ["finder", "posts"].forEach(function (name) { $(name).hidden = reading; });
    $("status").hidden = reading;
    $("more").hidden = reading || $("more").hidden;
    if (reading) window.scrollTo(0, 0);
    else { document.title = "Devlog: LibertyFlux"; drawList(); }
  }
  function remember() {
    var query = [];
    if (state.q) query.push("q=" + encodeURIComponent(state.q));
    if (state.mode !== "exact") query.push("mode=" + state.mode);
    if (state.topic) query.push("topic=" + encodeURIComponent(state.topic));
    history.replaceState(null, "", location.pathname + (query.length ? "?" + query.join("&") : "") + location.hash);
  }

  function start(SQL, buffer) {
    db = new SQL.Database(new Uint8Array(buffer));
    var topicsOf = {};
    rows("SELECT post_id, tag FROM tags ORDER BY tag").forEach(function (t) { (topicsOf[t.post_id] || (topicsOf[t.post_id] = [])).push(t.tag); });
    posts = rows("SELECT id, seq, date, title, summary FROM posts ORDER BY seq DESC");
    posts.forEach(function (p, index) { p.index = index; p.topics = topicsOf[p.id] || []; byId[p.id] = p; });
    var params = new URLSearchParams(location.search);
    state.q = params.get("q") || "";
    state.mode = params.get("mode") === "fuzzy" ? "fuzzy" : "exact";
    state.topic = params.get("topic") || "";
    $("q").value = state.q;
    document.querySelector('input[name="mode"][value="' + state.mode + '"]').checked = true;
    $("clear").hidden = !state.q;
    drawTopics();

    var timer = null;
    $("q").addEventListener("input", function (e) {
      clearTimeout(timer);
      timer = setTimeout(function () { state.q = e.target.value.trim(); shown = PAGE; $("clear").hidden = !state.q; remember(); drawList(); }, 140);
    });
    $("finder").addEventListener("submit", function (e) { e.preventDefault(); });
    $("clear").addEventListener("click", function () { $("q").value = ""; state.q = ""; shown = PAGE; $("clear").hidden = true; remember(); drawList(); $("q").focus(); });
    document.querySelectorAll('input[name="mode"]').forEach(function (radio) {
      radio.addEventListener("change", function () { state.mode = radio.value; shown = PAGE; remember(); drawList(); });
    });
    $("topics").addEventListener("click", function (e) {
      var button = e.target.closest("button");
      if (!button) return;
      state.topic = button.dataset.topic; shown = PAGE; remember(); drawTopics(); drawList();
    });
    $("more").addEventListener("click", function () { shown += PAGE; drawList(); });
    window.addEventListener("hashchange", route);
    route();
  }

  if (typeof initSqlJs !== "function") { $("status").textContent = "The database reader could not be loaded, so the devlog cannot be shown. Check the connection and reload."; return; }
  Promise.all([
    initSqlJs({ locateFile: function (file) { return SQL_FILES + file; } }),
    fetch("data/devlog.sqlite", { cache: "no-cache" }).then(function (r) { if (!r.ok) throw new Error("database " + r.status); return r.arrayBuffer(); })
  ]).then(function (loaded) { start(loaded[0], loaded[1]); })
    .catch(function (error) { $("status").textContent = "The devlog could not be loaded (" + error.message + "). Reload to try again."; });
})();
