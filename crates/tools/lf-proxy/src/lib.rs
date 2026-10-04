//! `lf-proxy`: the winmm proxy DLL that hosts the replacement loader.
//!
//! Build as i686 cdylib, installed next to the game executable under the
//! name `winmm.dll`. Forwards every export of the real system winmm to the
//! system copy (resolved by absolute path), then waits for the game to be
//! provably ready before enabling any registered replacement.
//!
//! `DllMain` does the minimum: resolve forward pointers, stash the handle,
//! start the init thread. Everything else — logging, readiness gating,
//! hook installation, the control-file watcher — runs outside the loader
//! lock on the init thread.
//!
//! Usage (from the repository root): build the 32-bit DLL with
//! `cargo build --release --target i686-pc-windows-msvc -p lf-proxy`
//! (the build script forwards every export of the system's 32-bit
//! `winmm.dll`; override with `LF_WINMM_PATH`). Only the i686 build ever
//! ships: install it next to the game executable as `winmm.dll`, run the
//! game to the menu, and read `libertyflux.log` beside the DLL.
//!
//! Switches live in `switches.lf` beside the DLL (`all on|off`,
//! `<name> on|off`, `sub:<name> on|off`, `bisect <a> <b>`, later lines win)
//! and are re-applied whenever the file changes. Safe mode (forwarding
//! only, no hooks): `LIBERTYFLUX_SAFE=1` or an `lf-safe-mode` marker file
//! beside the DLL. Test without the game with `lf-test-target`.

// DllMain, the forward table, and the init thread all cross the FFI
// boundary and touch process-global state; unsafe is inherent here.
#![allow(unsafe_code)]
// Integrated lane code, proven by the 32-bit live suite; pedantic style
// lints stay off here while correctness lints (clippy::all) apply.
#![allow(clippy::pedantic)]

mod hooks;
mod ready;

use lf_hook::{log, mem};
use lf_registry::Registry;
#[cfg(target_arch = "x86")]
use std::arch::global_asm;
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once};

include!(concat!(env!("OUT_DIR"), "/forward_table.rs"));

#[cfg(target_arch = "x86")]
global_asm!(include_str!(concat!(env!("OUT_DIR"), "/stubs.inc")));

/// Resolved system addresses, one per forwarded export. Written once in
/// `DllMain` before any game code can call through the stubs.
#[unsafe(no_mangle)]
#[used]
pub static mut LF_TARGETS: [usize; FORWARD_COUNT] = [0; FORWARD_COUNT];

static mut G_HMODULE: mem::Hmodule = std::ptr::null_mut();
static INIT_ONCE: Once = Once::new();
static SHUTDOWN: std::sync::OnceLock<Arc<AtomicBool>> = std::sync::OnceLock::new();
static REG: Mutex<Option<Arc<Mutex<Registry>>>> = Mutex::new(None);

fn shutdown_flag() -> Arc<AtomicBool> {
    SHUTDOWN
        .get_or_init(|| Arc::new(AtomicBool::new(false)))
        .clone()
}

const DLL_PROCESS_ATTACH: u32 = 1;
const DLL_PROCESS_DETACH: u32 = 0;

fn system_winmm_path() -> Vec<u16> {
    let mut buf = [0u16; 260];
    let mut path = String::from(r"C:\Windows\System32\winmm.dll");
    unsafe {
        let n = mem::GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32);
        if n > 0 && (n as usize) < buf.len() {
            let dir = String::from_utf16_lossy(&buf[..n as usize]);
            path = format!("{dir}\\winmm.dll");
        }
    }
    mem::to_wide(&path)
}

/// Resolve every forwarded export against the *system* winmm. No heap use
/// beyond fixed stack buffers; safe under the loader lock.
unsafe fn resolve_forwards() {
    // SAFETY: called once from DllMain while the loader lock is held.
    unsafe {
        let path = system_winmm_path();
        let real = mem::LoadLibraryExW(path.as_ptr(), std::ptr::null_mut(), 0);
        if real.is_null() {
            return;
        }
        // On the real (i686) target the table holds every system export; the
        // host build's empty table only exists so `cargo build` keeps working.
        #[allow(clippy::reversed_empty_ranges)]
        for i in 0..FORWARD_COUNT {
            let addr = match FORWARD_NAMES[i] {
                Some(name) => {
                    let bytes = name.as_bytes();
                    if bytes.len() >= 120 {
                        std::ptr::null_mut()
                    } else {
                        let mut tmp = [0u8; 128];
                        tmp[..bytes.len()].copy_from_slice(bytes);
                        mem::GetProcAddress(real, tmp.as_ptr())
                    }
                }
                None => mem::GetProcAddress(real, FORWARD_ORDINALS[i] as *const u8),
            };
            // No reader exists while this table is being filled: DllMain runs
            // before any game thread can call through the stubs.
            (*std::ptr::addr_of_mut!(LF_TARGETS))[i] = addr as usize;
        }
    }
}

/// Loader entry point, called by the OS. Never fails the load.
#[unsafe(no_mangle)]
// Called by the OS, never from Rust; the signature is fixed by the system.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn DllMain(hmodule: mem::Hmodule, reason: u32, _reserved: *mut c_void) -> i32 {
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            std::ptr::addr_of_mut!(G_HMODULE).write(hmodule);
            resolve_forwards();
            let mut _tid = 0u32;
            // The init thread blocks until the loader lock is released, then
            // waits for game readiness. Creation here is the standard proxy
            // pattern; all real work happens off the loader lock.
            mem::CreateThread(
                std::ptr::null_mut(),
                0,
                init_thread,
                std::ptr::null_mut(),
                0,
                &mut _tid,
            );
            1 // never fail the load; fail closed instead
        },
        DLL_PROCESS_DETACH => {
            shutdown_flag().store(true, Ordering::Relaxed);
            if let Ok(g) = REG.lock()
                && let Some(r) = g.as_ref()
                && let Ok(mut reg) = r.lock()
            {
                let _ = reg.set_all(false);
            }
            log::info("detach: all hooks disabled");
            1
        }
        _ => 1,
    }
}

/// Community-loader entry point: Ultimate ASI Loader calls this after
/// loading us as a plugin. Joins the same once-only init as the proxy path.
#[unsafe(no_mangle)]
pub extern "system" fn InitializeASI() {
    INIT_ONCE.call_once(|| {
        std::thread::spawn(init_body);
    });
}

extern "system" fn init_thread(_param: *mut c_void) -> u32 {
    INIT_ONCE.call_once(|| {
        init_body();
    });
    0
}

fn dll_dir() -> PathBuf {
    unsafe {
        let mut buf = [0u16; 32768];
        let h = std::ptr::addr_of!(G_HMODULE).read();
        let n = if h.is_null() {
            0
        } else {
            mem::GetModuleFileNameW(h, buf.as_mut_ptr(), 32768)
        };
        if n == 0 {
            return PathBuf::from(".");
        }
        let full = String::from_utf16_lossy(&buf[..n as usize]);
        PathBuf::from(full)
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .to_path_buf()
    }
}

fn safe_mode(dir: &std::path::Path) -> bool {
    if let Ok(v) = std::env::var("LIBERTYFLUX_SAFE")
        && matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes")
    {
        return true;
    }
    dir.join("lf-safe-mode").exists()
}

extern "system" fn crash_filter(info: *mut mem::ExceptionPointers) -> i32 {
    // Best effort only: try-locks, no allocation that can deadlock.
    let (code, addr) = if info.is_null() {
        (0, 0)
    } else {
        unsafe {
            let r = (*info).record;
            if r.is_null() {
                (0, 0)
            } else {
                ((*r).code, (*r).address as usize)
            }
        }
    };
    let active = match REG.try_lock() {
        Ok(g) => g
            .as_ref()
            .and_then(|r| r.try_lock().ok())
            .map(|reg| reg.active_names().join(","))
            .unwrap_or_else(|| "<registry locked>".to_string()),
        Err(_) => "<registry locked>".to_string(),
    };
    log::log(
        "crash",
        &format!("exception {code:#X} at {addr:#X}; active hooks: [{active}]"),
    );
    mem::EXCEPTION_CONTINUE_SEARCH
}

fn init_body() {
    let _ = std::panic::catch_unwind(|| {
        let dir = dll_dir();
        log::init(dir.join("libertyflux.log"));
        log::info("libertyflux proxy init: start");
        if safe_mode(&dir) {
            log::info("safe mode: all hooks disabled, proxy forwards only");
            return;
        }
        unsafe {
            mem::SetUnhandledExceptionFilter(Some(crash_filter));
        }
        let exe_base = unsafe { mem::GetModuleHandleW(std::ptr::null()) } as usize;
        log::info(&format!("exe base: {exe_base:#X}"));
        match ready::wait_ready(exe_base, &[]) {
            Ok(()) => {}
            Err(e) => {
                log::error(&format!("init: not ready ({e}); staying disabled"));
                return;
            }
        }
        let reg = Arc::new(Mutex::new(Registry::new(exe_base)));
        {
            let mut guard = match reg.lock() {
                Ok(g) => g,
                Err(_) => return,
            };
            hooks::register_all(&mut guard);
            guard.resolve_all();
            let control = dir.join("switches.lf");
            match std::fs::read_to_string(&control) {
                Ok(text) => {
                    for item in lf_registry::parse_control(&text) {
                        match item {
                            Ok(cmd) => {
                                for line in lf_registry::apply_commands(&mut guard, &[cmd]) {
                                    log::info(&format!("control: {line}"));
                                }
                            }
                            Err(bad) => log::warn(&format!("control: bad line: {bad}")),
                        }
                    }
                }
                Err(_) => log::info("no switches.lf; all hooks off"),
            }
            log::info(&format!(
                "init: ready, {} hooks registered, {} on",
                guard.len(),
                guard.active_names().len()
            ));
        }
        *REG.lock().unwrap() = Some(Arc::clone(&reg));
        // Control-file watcher (detached; exits on the shutdown flag).
        let control = dir.join("switches.lf");
        let flag = shutdown_flag();
        std::thread::spawn(move || {
            lf_registry::watch_control_file(reg, control, 500, flag);
        });
    });
}

#[cfg(test)]
mod tests {
    //! Tests of the generated forwarding table. They run on the host (where
    //! the build script emits an empty table, so the checks are trivially
    //! true) and, on the i686 Windows runner, against the real system
    //! `winmm.dll` export table the proxy was built from. The generator
    //! itself is unit-tested in `lf_hook::forward` with synthetic exports.
    use super::{FORWARD_COUNT, FORWARD_NAMES, FORWARD_ORDINALS};
    use std::collections::HashSet;

    #[test]
    fn table_arrays_agree_in_length() {
        assert_eq!(FORWARD_NAMES.len(), FORWARD_COUNT);
        assert_eq!(FORWARD_ORDINALS.len(), FORWARD_COUNT);
    }

    #[test]
    fn ordinals_are_unique_and_sorted() {
        // The loader fills LF_TARGETS[i] for FORWARD_NAMES[i]; a repeated or
        // unsorted ordinal would mean two exports share a slot or the stub
        // index and the table index disagree.
        let mut seen = HashSet::new();
        let mut previous: Option<u32> = None;
        for &ordinal in &FORWARD_ORDINALS {
            assert!(seen.insert(ordinal), "ordinal {ordinal} appears twice");
            if let Some(p) = previous {
                assert!(ordinal > p, "ordinals are not strictly ascending");
            }
            previous = Some(ordinal);
        }
    }

    #[test]
    fn named_exports_are_non_empty_and_distinct() {
        let mut names = HashSet::new();
        for name in FORWARD_NAMES.iter().flatten() {
            assert!(!name.is_empty(), "a forwarded export has an empty name");
            assert!(names.insert(*name), "export {name} is listed twice");
        }
    }
}
