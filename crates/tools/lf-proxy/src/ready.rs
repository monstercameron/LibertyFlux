//! Deferred-init gating: prove the game is ready before patching.
//!
//! The proxy loads before any game code runs (and before the start-up stub
//! finishes decrypting the first megabyte of code). Installing hooks then
//! would patch ciphertext. The init thread therefore waits for, in order:
//!
//! 1. The start-up wrapper module to be gone (it is loaded, called and
//!    freed by the entry stub, so absent-again means the stub finished).
//! 2. The first code page to differ from the protector's pristine copy and
//!    stay stable across polls (decryption done).
//! 3. A settle delay, then optional anchor checks (expected bytes at known
//!    addresses; empty until the hook table defines them).
//!
//! Any timeout fails closed: hooks stay off and the game runs unmodified.

// The module probes are single Win32 queries through the raw bindings.
#![allow(unsafe_code)]

use lf_hook::{log, mem, pe::live as pelive};
use std::time::{Duration, Instant};

pub const WRAPPER_MODULE: &str = "MTLX";
pub const SETTLE_MS: u64 = 2000;
pub const DEFAULT_TIMEOUT_MS: u64 = 120_000;

#[derive(Clone, Debug)]
pub struct Anchor {
    pub rva: u32,
    pub expected: Vec<u8>,
}

fn timeout_ms() -> u64 {
    std::env::var("LF_INIT_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TIMEOUT_MS)
}

fn module_absent(name: &str) -> bool {
    unsafe { mem::GetModuleHandleW(mem::to_wide(name).as_ptr()).is_null() }
}

/// True once the first code page provably differs from the pristine copy.
///
/// Both ranges come from the loaded module's own section headers; nothing
/// is assumed about load address or file layout.
fn text_decrypted(exe_base: usize) -> bool {
    let sections = match pelive::sections(exe_base) {
        Some(s) => s,
        None => return false,
    };
    let find = |want: &str| sections.iter().find(|(n, _, _)| n == want).cloned();
    let (text_rva, text_size) = match find(".text") {
        Some((_, rva, size)) => (rva, size),
        None => return false,
    };
    // The pristine copy: same length as the encrypted prefix.
    let pristine = match find(".tbm") {
        Some((_, rva, _)) => rva,
        None => return false,
    };
    if text_size < 0x200 {
        return false;
    }
    let probe = 0x100usize; // past any headers, inside the encrypted prefix
    let a = mem::read_bytes(exe_base + text_rva as usize + probe, 16);
    let b = mem::read_bytes(exe_base + pristine as usize + probe, 16);
    match (a, b) {
        (Some(a), Some(b)) => a != b,
        _ => false,
    }
}

fn anchors_match(exe_base: usize, anchors: &[Anchor]) -> bool {
    anchors.iter().all(|a| {
        let addr = exe_base.wrapping_add(a.rva as usize);
        match mem::read_bytes(addr, a.expected.len()) {
            Some(cur) => cur == a.expected,
            None => false,
        }
    })
}

/// Block until the game is ready or the timeout expires.
pub fn wait_ready(exe_base: usize, anchors: &[Anchor]) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms());

    // 1. Wrapper module gone.
    log::info("ready: waiting for start-up wrapper to unload");
    while !module_absent(WRAPPER_MODULE) {
        if Instant::now() > deadline {
            return Err("timeout waiting for start-up wrapper".to_string());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    log::info("ready: start-up wrapper absent");

    // 2. Code decrypted and stable across three polls.
    log::info("ready: waiting for code decryption");
    let mut stable = 0u32;
    let mut last: Option<Vec<u8>> = None;
    while stable < 3 {
        if Instant::now() > deadline {
            return Err("timeout waiting for code decryption".to_string());
        }
        if text_decrypted(exe_base) {
            let sections = pelive::sections(exe_base).unwrap_or_default();
            let text_rva = sections
                .iter()
                .find(|(n, _, _)| n == ".text")
                .map(|(_, r, _)| *r)
                .unwrap_or(0);
            let cur = mem::read_bytes(exe_base + text_rva as usize + 0x100, 16);
            if cur.is_some() && cur == last {
                stable += 1;
            } else {
                stable = 0;
                last = cur;
            }
        } else {
            stable = 0;
            last = None;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    log::info("ready: code decrypted and stable");

    // 3. Settle, then anchors.
    std::thread::sleep(Duration::from_millis(SETTLE_MS));
    if !anchors.is_empty() {
        if !anchors_match(exe_base, anchors) {
            return Err("anchor bytes do not match".to_string());
        }
        log::info(&format!("ready: {} anchors match", anchors.len()));
    }
    Ok(())
}
