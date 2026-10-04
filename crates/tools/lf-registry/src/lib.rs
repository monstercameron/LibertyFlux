//! `lf-registry`: the replacement table with run-time on/off switches.
//!
//! Each entry names one original function, where to find it, and the Rust
//! replacement. Entries resolve once (address + expected-bytes check), then
//! toggle freely. Bulk operations (`all`, per-subsystem, `bisect`) and a
//! watched control file drive live testing without rebuilding.
//!
//! Addresses are stored relative to the executable's preferred base and
//! rebased at run time: `absolute = actual_exe_base + rva`.
//!
//! Usage: register each replacement with [`Registry::add`] as a [`HookDef`]
//! (name, subsystem, [`Target`], detour address), then drive it with
//! `set`/`all`/`bisect` or the `switches.lf` control file beside the DLL
//! (`parse_control` + `watch_control_file`). Tested on the host with
//! `cargo test -p lf-registry` and live by `lf-test-target`.
//!
//! [`table`] reads the replacement table an assembled rewrite library
//! exports and turns it into `HookDef`s; [`bisect`] plans the `bisect a b`
//! ranges of a regression hunt. Both are pure and host-tested; tests that
//! link the live registry run on Windows only.

// Integrated lane code, proven by host unit tests and the 32-bit live
// suite; pedantic style lints stay off here while correctness lints
// (clippy::all) apply. Narrow this if the code is reworked.
#![allow(clippy::pedantic)]

pub mod bisect;
pub mod table;

use lf_hook::detour::{Detour, FollowJumps, HookError};
use lf_hook::pe::live as pelive;
use lf_hook::{log, mem, slot::SlotHook};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Where the original lives. `rva` values are relative to the executable's
/// preferred base and are rebased onto the actual load address.
#[derive(Clone, Debug)]
pub enum Target {
    /// Inline 5-byte hook at `exe_base + rva`. `expected` must match the
    /// bytes found there before anything is patched.
    Inline {
        /// Target RVA relative to the executable's preferred base.
        rva: u32,
        /// Bytes that must be present before anything is patched.
        expected: Vec<u8>,
        /// Follow leading thunk jumps to the real body first.
        follow_jumps: bool,
    },
    /// Inline hook at a fixed absolute address (tests, injected helpers).
    Absolute {
        /// Target address (no rebasing is applied).
        addr: usize,
        /// Bytes that must be present before anything is patched.
        expected: Vec<u8>,
        /// Follow leading thunk jumps to the real body first.
        follow_jumps: bool,
    },
    /// Vtable slot: pointer at `exe_base + vtable_rva + slot * 4`.
    VTable {
        /// RVA of the vtable start, relative to the preferred base.
        vtable_rva: u32,
        /// Slot index (each slot is one pointer).
        slot: u32,
    },
    /// Import slot of the executable: `dll!name` (or `dll!ordinal:<n>`).
    Import {
        /// DLL name, with or without `.dll` (case-insensitive).
        dll: String,
        /// Import name (or `ordinal:<n>`).
        name: String,
    },
}

/// One registered replacement: what to hook and what to run instead.
#[derive(Clone, Debug)]
pub struct HookDef {
    /// Dotted hook name (`subsystem.short-name`); the control-file handle.
    pub name: String,
    /// Subsystem group for bulk toggles.
    pub subsystem: String,
    /// Where the original lives (see [`Target`]).
    pub target: Target,
    /// Replacement entry point. For thiscall hooks this is the generated
    /// detour stub, not the Rust function (see `lf_hook::adapters`).
    pub detour: usize,
}

/// Per-hook state in the [`Registry`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// Resolved (or never resolved) and currently unhooked.
    Off,
    /// Hook installed and active.
    On,
    /// Resolution or toggling failed; carries the reason. Never touched again.
    Failed(String),
}

enum Live {
    Inline(Detour),
    Slot(SlotHook),
}

struct Entry {
    def: HookDef,
    state: State,
    live: Option<Live>,
}

/// The replacement table: owns every registered hook and its state.
pub struct Registry {
    exe_base: usize,
    entries: Vec<Entry>,
}

impl Registry {
    /// Empty table rebased onto the executable's actual load address.
    #[must_use]
    pub fn new(exe_base: usize) -> Self {
        Registry {
            exe_base,
            entries: Vec::new(),
        }
    }

    /// Actual load address RVAs are rebased onto.
    #[must_use]
    pub fn exe_base(&self) -> usize {
        self.exe_base
    }

    /// Register one replacement (starts `Off`, unresolved).
    pub fn add(&mut self, def: HookDef) {
        self.entries.push(Entry {
            def,
            state: State::Off,
            live: None,
        });
    }

    /// Number of registered hooks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True when no hook is registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Hook names in registration order.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.def.name.clone()).collect()
    }

    /// `(name, subsystem, state)` for every registered hook.
    #[must_use]
    pub fn status(&self) -> Vec<(String, String, State)> {
        self.entries
            .iter()
            .map(|e| (e.def.name.clone(), e.def.subsystem.clone(), e.state.clone()))
            .collect()
    }

    /// Names of the hooks currently `On` (for crash reports).
    #[must_use]
    pub fn active_names(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| e.state == State::On)
            .map(|e| e.def.name.clone())
            .collect()
    }

    fn find(&mut self, name: &str) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|e| e.def.name == name)
    }

    /// Resolve every entry: compute addresses, verify expected bytes, and
    /// build the (still disabled) hook objects. Failures mark the entry
    /// `Failed` and never touch code bytes.
    pub fn resolve_all(&mut self) {
        for e in &mut self.entries {
            if e.live.is_some() {
                continue;
            }
            match resolve_one(self.exe_base, &e.def) {
                Ok(live) => {
                    e.live = Some(live);
                    e.state = State::Off;
                    log::info(&format!("resolve {}: ok", e.def.name));
                }
                Err(err) => {
                    e.state = State::Failed(err.clone());
                    log::warn(&format!("resolve {}: FAILED ({err})", e.def.name));
                }
            }
        }
    }

    /// Install the named hook. Unknown, unresolved or `Failed` hooks error.
    pub fn enable(&mut self, name: &str) -> Result<(), String> {
        let e = self
            .find(name)
            .ok_or_else(|| format!("unknown hook {name}"))?;
        if matches!(e.state, State::Failed(_)) {
            return Err(format!("{} is in Failed state", e.def.name));
        }
        let live = e
            .live
            .as_mut()
            .ok_or_else(|| format!("{} is unresolved", e.def.name))?;
        let r = match live {
            Live::Inline(d) => d.enable().map(|_| ()).map_err(|e| e.to_string()),
            Live::Slot(s) => s.enable().map_err(|e| e.to_string()),
        };
        match r {
            Ok(()) => {
                e.state = State::On;
                log::info(&format!("{}: off -> on", e.def.name));
                Ok(())
            }
            Err(err) => {
                e.state = State::Failed(err.clone());
                log::error(&format!("{}: enable FAILED ({err})", e.def.name));
                Err(err)
            }
        }
    }

    /// Remove the named hook (no-op unless it is `On`).
    pub fn disable(&mut self, name: &str) -> Result<(), String> {
        let e = self
            .find(name)
            .ok_or_else(|| format!("unknown hook {name}"))?;
        if !matches!(e.state, State::On) {
            return Ok(());
        }
        let live = e
            .live
            .as_mut()
            .ok_or_else(|| format!("{} is unresolved", e.def.name))?;
        let r = match live {
            Live::Inline(d) => d.disable().map(|_| ()).map_err(|e| e.to_string()),
            Live::Slot(s) => s.disable().map_err(|e| e.to_string()),
        };
        match r {
            Ok(()) => {
                e.state = State::Off;
                log::info(&format!("{}: on -> off", e.def.name));
                Ok(())
            }
            Err(err) => {
                e.state = State::Failed(err.clone());
                log::error(&format!("{}: disable FAILED ({err})", e.def.name));
                Err(err)
            }
        }
    }

    /// Set every resolved entry to `on`. Returns per-hook results.
    pub fn set_all(&mut self, on: bool) -> Vec<(String, Result<(), String>)> {
        let names = self.names();
        names
            .into_iter()
            .map(|n| {
                let r = if on {
                    self.enable(&n)
                } else {
                    self.disable(&n)
                };
                // Disabling an already-off hook is success; enabling a Failed
                // hook reports the stored failure.
                (n, r)
            })
            .collect()
    }

    /// Set every entry in `subsystem` to `on`.
    pub fn set_subsystem(
        &mut self,
        subsystem: &str,
        on: bool,
    ) -> Vec<(String, Result<(), String>)> {
        let names: Vec<String> = self
            .entries
            .iter()
            .filter(|e| e.def.subsystem == subsystem)
            .map(|e| e.def.name.clone())
            .collect();
        names
            .into_iter()
            .map(|n| {
                let r = if on {
                    self.enable(&n)
                } else {
                    self.disable(&n)
                };
                (n, r)
            })
            .collect()
    }

    /// Bisect support: enable the name-sorted entries with indices in
    /// `[a, b)`, disable everything else. A regression hunt halves the
    /// enabled range until one hook remains.
    pub fn bisect(&mut self, a: usize, b: usize) -> Vec<(String, bool)> {
        let names: Vec<&str> = self.entries.iter().map(|e| e.def.name.as_str()).collect();
        let want = bisect::wanted_by_rank(&names, a, b);
        let mut out = Vec::new();
        for (i, w) in want.iter().enumerate() {
            let name = self.entries[i].def.name.clone();
            if *w {
                let _ = self.enable(&name);
            } else {
                let _ = self.disable(&name);
            }
            out.push((name, *w));
        }
        log::info(&format!(
            "bisect [{a},{b}): {} enabled",
            out.iter().filter(|(_, w)| *w).count()
        ));
        out
    }

    /// Integrity self-check: verify every entry's bytes, restore mismatches.
    /// Returns the names that were repaired.
    #[must_use]
    pub fn verify_and_restore(&self) -> Vec<String> {
        let mut repaired = Vec::new();
        for e in &self.entries {
            let ok = match &e.live {
                Some(Live::Inline(d)) => d.verify(),
                Some(Live::Slot(s)) => s.verify(),
                None => true,
            };
            if !ok {
                let r = match &e.live {
                    Some(Live::Inline(d)) => d.restore(),
                    Some(Live::Slot(s)) => s.restore(),
                    None => Ok(()),
                };
                match r {
                    Ok(()) => {
                        log::warn(&format!("integrity: {} bytes repaired", e.def.name));
                        repaired.push(e.def.name.clone());
                    }
                    Err(err) => {
                        log::error(&format!(
                            "integrity: {} corrupted, restore failed ({err})",
                            e.def.name
                        ));
                    }
                }
            }
        }
        repaired
    }
}

fn check_expected(addr: usize, expected: &[u8]) -> Result<(), String> {
    if expected.is_empty() {
        return Ok(());
    }
    match mem::read_bytes(addr, expected.len()) {
        Some(cur) if cur == expected => Ok(()),
        Some(_) => Err("expected-bytes mismatch".to_string()),
        None => Err("target not readable".to_string()),
    }
}

fn resolve_one(exe_base: usize, def: &HookDef) -> Result<Live, String> {
    match &def.target {
        Target::Inline {
            rva,
            expected,
            follow_jumps,
        } => {
            let addr = exe_base.wrapping_add(*rva as usize);
            check_expected(addr, expected)?;
            let follow = if *follow_jumps {
                FollowJumps::On
            } else {
                FollowJumps::Off
            };
            Detour::create(addr, def.detour, follow)
                .map(Live::Inline)
                .map_err(|e: HookError| e.to_string())
        }
        Target::Absolute {
            addr,
            expected,
            follow_jumps,
        } => {
            check_expected(*addr, expected)?;
            let follow = if *follow_jumps {
                FollowJumps::On
            } else {
                FollowJumps::Off
            };
            Detour::create(*addr, def.detour, follow)
                .map(Live::Inline)
                .map_err(|e: HookError| e.to_string())
        }
        Target::VTable { vtable_rva, slot } => {
            let addr = exe_base
                .wrapping_add(*vtable_rva as usize)
                .wrapping_add(*slot as usize * 4);
            SlotHook::create(addr as *mut usize, def.detour)
                .map(Live::Slot)
                .map_err(|e| e.to_string())
        }
        Target::Import { dll, name } => {
            let slot = pelive::find_import_slot(exe_base, dll, name)
                .ok_or_else(|| format!("import slot {dll}!{name} not found"))?;
            SlotHook::create(slot as *mut usize, def.detour)
                .map(Live::Slot)
                .map_err(|e| e.to_string())
        }
    }
}

// ---- Control file ----

/// One parsed control-file line: `all on|off`, `<name> on|off`,
/// `sub:<name> on|off`, or `bisect <a> <b>` (`#` comments, later lines win).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    /// Set every hook (`all on` / `all off`).
    All(bool),
    /// Set one hook by name.
    One {
        /// Hook name.
        name: String,
        /// True for `on`, false for `off`.
        on: bool,
    },
    /// Set every hook in a subsystem.
    Subsystem {
        /// Subsystem name.
        name: String,
        /// True for `on`, false for `off`.
        on: bool,
    },
    /// Enable name-sorted entries `[a, b)`, disable the rest.
    Bisect {
        /// Start of the enabled range (inclusive).
        a: usize,
        /// End of the enabled range (exclusive).
        b: usize,
    },
}

fn parse_on_off(word: &str) -> Option<bool> {
    match word.to_ascii_lowercase().as_str() {
        "on" | "1" | "true" | "enable" | "enabled" => Some(true),
        "off" | "0" | "false" | "disable" | "disabled" => Some(false),
        _ => None,
    }
}

/// Parse control-file text. Unknown lines are returned as `Err(line)` so the
/// caller can log them; parsing never fails as a whole.
pub fn parse_control(text: &str) -> Vec<Result<Command, String>> {
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = match parts.as_slice() {
            ["all", w] => parse_on_off(w).map(Command::All),
            ["bisect", a, b] => match (a.parse::<usize>(), b.parse::<usize>()) {
                (Ok(a), Ok(b)) => Some(Command::Bisect { a, b }),
                _ => None,
            },
            [name, w] if !name.to_ascii_lowercase().starts_with("sub:") => {
                parse_on_off(w).map(|on| Command::One {
                    name: name.to_string(),
                    on,
                })
            }
            ["sub", name, w] => parse_on_off(w).map(|on| Command::Subsystem {
                name: name.to_string(),
                on,
            }),
            _ => None,
        };
        // `sub:<name> on|off` two-word form.
        let cmd = cmd.or_else(|| {
            if parts.len() == 2 && parts[0].to_ascii_lowercase().starts_with("sub:") {
                let name = parts[0][4..].to_string();
                parse_on_off(parts[1]).map(|on| Command::Subsystem { name, on })
            } else {
                None
            }
        });
        match cmd {
            Some(c) => out.push(Ok(c)),
            None => out.push(Err(raw.trim().to_string())),
        }
    }
    out
}

/// Apply parsed commands top-down; later lines override earlier ones.
/// Returns a human-readable transition log.
pub fn apply_commands(reg: &mut Registry, cmds: &[Command]) -> Vec<String> {
    let mut report = Vec::new();
    for cmd in cmds {
        match cmd {
            Command::All(on) => {
                let rs = reg.set_all(*on);
                let ok = rs.iter().filter(|(_, r)| r.is_ok()).count();
                report.push(format!("all {}: {ok}/{} ok", on_str(*on), rs.len()));
            }
            Command::One { name, on } => {
                let r = if *on {
                    reg.enable(name)
                } else {
                    reg.disable(name)
                };
                match r {
                    Ok(()) => report.push(format!("{name} -> {}", on_str(*on))),
                    Err(e) => report.push(format!("{name}: {e}")),
                }
            }
            Command::Subsystem { name, on } => {
                let rs = reg.set_subsystem(name, *on);
                let ok = rs.iter().filter(|(_, r)| r.is_ok()).count();
                report.push(format!("sub {name} {}: {ok}/{} ok", on_str(*on), rs.len()));
            }
            Command::Bisect { a, b } => {
                let rs = reg.bisect(*a, *b);
                report.push(format!("bisect [{a},{b}): {} entries touched", rs.len()));
            }
        }
    }
    report
}

fn on_str(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

/// Pure bisect selection over a name-sorted list. `true` = enabled.
/// Host-unit-testable; [`Registry::bisect`] is the live version.
#[must_use]
pub fn bisect_select(total: usize, a: usize, b: usize) -> Vec<bool> {
    (0..total).map(|i| i >= a && i < b).collect()
}

/// Poll `path` for changes and re-apply the whole file each time.
/// Runs until `shutdown` is set. Intended for a background thread.
pub fn watch_control_file(
    reg: Arc<Mutex<Registry>>,
    path: PathBuf,
    poll_ms: u64,
    shutdown: Arc<std::sync::atomic::AtomicBool>,
) {
    use std::sync::atomic::Ordering;
    use std::time::Duration;
    // Apply once at startup when the file exists.
    apply_file(&reg, &path);
    let mut last = mtime(&path);
    while !shutdown.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(poll_ms));
        let cur = mtime(&path);
        if cur != last {
            last = cur;
            apply_file(&reg, &path);
        }
    }
}

fn mtime(path: &PathBuf) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

fn apply_file(reg: &Arc<Mutex<Registry>>, path: &PathBuf) {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return, // missing file: leave switches as they are
    };
    let mut guard = match reg.lock() {
        Ok(g) => g,
        Err(_) => return,
    };
    for item in parse_control(&text) {
        match item {
            Ok(cmd) => {
                for line in apply_commands(&mut guard, &[cmd]) {
                    log::info(&format!("control: {line}"));
                }
            }
            Err(bad) => log::warn(&format!("control: ignoring bad line: {bad}")),
        }
    }
}

/// Name -> detour lookup helper for generated hook tables.
pub type DetourMap = HashMap<String, usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_control_file() {
        let text = "# comment line\nall off\naudio.update on\nsub:physics off\nsub audio on\nbisect 0 4\nbogus line here\n";
        let cmds = parse_control(text);
        assert_eq!(cmds.len(), 6);
        assert_eq!(cmds[0], Ok(Command::All(false)));
        assert_eq!(
            cmds[1],
            Ok(Command::One {
                name: "audio.update".to_string(),
                on: true
            })
        );
        assert_eq!(
            cmds[2],
            Ok(Command::Subsystem {
                name: "physics".to_string(),
                on: false
            })
        );
        assert_eq!(
            cmds[3],
            Ok(Command::Subsystem {
                name: "audio".to_string(),
                on: true
            })
        );
        assert_eq!(cmds[4], Ok(Command::Bisect { a: 0, b: 4 }));
        assert!(cmds[5].is_err());
    }

    #[test]
    fn parse_on_off_words() {
        assert_eq!(parse_control("all ON\n")[0], Ok(Command::All(true)));
        assert_eq!(parse_control("all 0\n")[0], Ok(Command::All(false)));
        assert_eq!(
            parse_control("x enabled\n")[0],
            Ok(Command::One {
                name: "x".to_string(),
                on: true
            })
        );
    }

    #[test]
    fn bisect_selection() {
        assert_eq!(
            bisect_select(8, 0, 4),
            vec![true, true, true, true, false, false, false, false]
        );
        assert_eq!(bisect_select(4, 2, 2), vec![false, false, false, false]);
        assert_eq!(bisect_select(4, 0, 99), vec![true, true, true, true]);
        assert_eq!(bisect_select(0, 0, 1), Vec::<bool>::new());
    }

    // Links the live registry (Win32 calls), so it runs on Windows only.
    #[cfg(windows)]
    #[test]
    fn watcher_applies_file_changes() {
        // Registry with no hooks: applying must succeed trivially and the
        // watcher must pick up a rewritten file without touching memory.
        let dir = std::env::temp_dir().join("lf-reg-test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("switches-{}.lf", std::process::id()));
        std::fs::write(&path, "all off\n").unwrap();
        let reg = Arc::new(Mutex::new(Registry::new(0x400000)));
        let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let r2 = Arc::clone(&reg);
        let s2 = Arc::clone(&shutdown);
        let p2 = path.clone();
        let h = std::thread::spawn(move || watch_control_file(r2, p2, 50, s2));
        std::thread::sleep(std::time::Duration::from_millis(120));
        std::fs::write(&path, "all on\n").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
        shutdown.store(true, std::sync::atomic::Ordering::Relaxed);
        h.join().unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(reg.lock().unwrap().len(), 0);
    }
    fn unresolved(name: &str, subsystem: &str) -> HookDef {
        HookDef {
            name: name.to_string(),
            subsystem: subsystem.to_string(),
            target: Target::Absolute {
                addr: 0,
                expected: Vec::new(),
                follow_jumps: false,
            },
            detour: 0,
        }
    }

    // The live registry links Win32 calls, so these run on Windows only.
    // Nothing is resolved, so no code bytes are read or patched.
    #[cfg(windows)]
    #[test]
    fn registry_bisect_selects_by_name_rank() {
        let mut reg = Registry::new(0x40_0000);
        for n in ["c.x", "a.x", "d.x", "b.x"] {
            reg.add(unresolved(n, "t"));
        }
        let out = reg.bisect(0, 2);
        let want: Vec<(String, bool)> =
            [("c.x", false), ("a.x", true), ("d.x", false), ("b.x", true)]
                .iter()
                .map(|(n, w)| (n.to_string(), *w))
                .collect();
        assert_eq!(out, want);
        // Unresolved hooks cannot turn on: nothing is active afterwards.
        assert!(reg.active_names().is_empty());
    }

    #[cfg(windows)]
    #[test]
    fn unresolved_hooks_refuse_to_enable() {
        let mut reg = Registry::new(0x40_0000);
        reg.add(unresolved("a.x", "t"));
        assert!(reg.enable("a.x").unwrap_err().contains("unresolved"));
        assert!(reg.enable("nope").unwrap_err().contains("unknown hook"));
        assert_eq!(reg.disable("a.x"), Ok(()));
        let report = apply_commands(&mut reg, &[Command::All(true)]);
        assert_eq!(report, vec!["all on: 0/1 ok".to_string()]);
        assert_eq!(reg.status()[0].2, State::Off);
    }

    #[cfg(windows)]
    #[test]
    fn table_plan_registers_switchable_rows_off() {
        use crate::table::{FLAG_SWITCHABLE, RewriteEntry, Row, plan};
        let rows: Vec<Row> = (0..3u32)
            .map(|i| Row {
                entry: RewriteEntry {
                    address: 0x0050_0000 + i * 0x10,
                    flags: if i == 1 { 0 } else { FLAG_SWITCHABLE },
                    shard: 1,
                    detour: 0x1000,
                    ..RewriteEntry::default()
                },
                name: format!("function.{:08x}", 0x0050_0000 + i * 0x10),
                expected: Vec::new(),
            })
            .collect();
        let p = plan(&rows);
        let mut reg = Registry::new(0x40_0000);
        for d in p.defs {
            reg.add(d);
        }
        assert_eq!(reg.names(), vec!["function.00500000", "function.00500020"]);
        assert!(
            reg.status()
                .iter()
                .all(|(_, sub, st)| sub == "s001" && *st == State::Off)
        );
        assert_eq!(p.skipped.len(), 1);
    }

    #[test]
    fn unresolved_def_is_plain_data() {
        // Host-side guard that the helper above builds what it says.
        let d = unresolved("a.x", "t");
        assert_eq!(
            (d.name.as_str(), d.subsystem.as_str(), d.detour),
            ("a.x", "t", 0)
        );
    }
}
