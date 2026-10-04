//! `lf-checker-worker`: the checker's 32-bit comparison worker.
//! Maps the original executable once, then runs many trials on stdin/stdout
//! as JSON lines. Driven by `scripts/checker/checker2.py`, whose module
//! documentation describes the protocol and the contract format.
//!
//! Usage (from the repository root):
//! `cargo build --release --target i686-pc-windows-msvc -p lf-checker-worker`
//! then run contracts with the driver; never run this binary by hand.
//! Only the i686 build ever runs: it executes original 32-bit code
//! natively. Commands: setup, trial, teardown, ping. One JSON object per
//! line on stdin, one per line on stdout; stderr is diagnostics (never JSON).
//!
//! v2 abilities: per-callee script slots, stub out-param writes, stub
//! addresses in the setup response (for driver-side vtable planting, which
//! is the indirect-call interception), E9 tail-patch sites, pointer
//! normalization with pointed-to snapshots in call keys, TLS fabrication,
//! XMM entry values, XMM0 call-arg logging with stack transport on the
//! rewrite side, defined stack fills, and below-ESP scratch reset.

// The worker maps and executes original machine code, plants stubs in it,
// and catches faults in-process: raw pointers and FFI are its whole job.
#![allow(unsafe_code)]
// Integrated lane code, proven by the checker's regression suite: address
// casts, long decoders and shared match arms are inherent to the domain,
// so pedantic style lints stay off here. Correctness lints (clippy::all)
// still apply; narrow this to per-lint allows if the code is reworked.
#![allow(clippy::pedantic)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{BufRead, Write};
use std::sync::mpsc;
use std::time::Duration;

mod protocol;
mod state;
mod image;
mod execution;
mod interception;
mod trial;

use crate::protocol::*;
use crate::state::*;
use crate::image::*;
use crate::execution::*;
use crate::interception::*;
use crate::trial::*;


fn main() {
    unsafe {
        let boxed = Box::new(State::new());
        ST = Box::into_raw(boxed);
        AddVectoredExceptionHandler(1, veh);
    }
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut out = std::io::stdout();
    // ready banner (driver waits for this)
    writeln!(out, "{{\"ready\":true,\"checker_version\":\"checker4\"}}").unwrap();
    out.flush().unwrap();
    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let q = match parse_json(&line) {
            Ok(J::Obj(m)) => J::Obj(m),
            Ok(_) => {
                writeln!(out, "{{\"ok\":false,\"error\":\"not an object\"}}").unwrap();
                out.flush().unwrap();
                continue;
            }
            Err(e) => {
                writeln!(out, "{{\"ok\":false,\"error\":\"json: {}\"}}", esc(&e)).unwrap();
                out.flush().unwrap();
                continue;
            }
        };
        let cmd = q.get("cmd").map(|v| v.as_str()).unwrap_or("");
        match cmd {
            "ping" => {
                writeln!(out, "{{\"ok\":true,\"mapped\":{}}}", st().mapped).unwrap();
                out.flush().unwrap();
            }
            "setup" => {
                let r = cmd_setup(st(), &q);
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "teardown" => {
                let r = cmd_teardown(st());
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "dump_bytes" => {
                // k-split debug-only: byte dump for split equivalence.
                let r = cmd_dump_bytes(st());
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "trial" => {
                let timeout = q.get("timeout_ms").map(|v| v.as_u32()).unwrap_or(10000);
                // explicit scripted values (driver-resolved per callee scripts)
                let sv: Vec<(u32, u32, u32, Vec<u32>, Vec<(u32, u32)>)> = q
                    .get("script_vals")
                    .map(|v| {
                        v.as_arr()
                            .iter()
                            .map(|e| {
                                (
                                    e.get("id").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("lo").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("hi").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("w")
                                        .map(|w| w.as_arr().iter().map(|x| x.as_u32()).collect())
                                        .unwrap_or_default(),
                                    // v3: per-call answer steps [[lo,hi],...]
                                    e.get("seq")
                                        .map(|s| {
                                            s.as_arr()
                                                .iter()
                                                .map(|st| match st {
                                                    J::Arr(a) => (
                                                        a.first().map(|x| x.as_u32()).unwrap_or(0),
                                                        a.get(1).map(|x| x.as_u32()).unwrap_or(0),
                                                    ),
                                                    x => (x.as_u32(), 0),
                                                })
                                                .collect()
                                        })
                                        .unwrap_or_default(),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let parsed = parse_trial(st(), &q);
                match parsed {
                    Err(e) => {
                        writeln!(out, "{{\"ok\":false,\"error\":\"{}\"}}", esc(&e)).unwrap();
                        out.flush().unwrap();
                    }
                    Ok((req, checks)) => {
                        let (tx, rx) = mpsc::channel();
                        std::thread::Builder::new()
                            .name("trial".to_string())
                            .stack_size(8 * 1024 * 1024)
                            .spawn(move || {
                                let r = trial_body(req, checks, sv);
                                let _ = tx.send(r);
                            })
                            .unwrap();
                        match rx.recv_timeout(Duration::from_millis(timeout as u64)) {
                            Ok(r) => {
                                writeln!(out, "{}", r).unwrap();
                                out.flush().unwrap();
                            }
                            Err(_) => {
                                // hang: the trial thread is abandoned (tainted worker).
                                // Report, then exit so the driver starts a fresh worker.
                                writeln!(
                                    out,
                                    "{{\"ok\":true,\"pass\":false,\"hang\":true,\"checks\":[],\"first_mismatch\":\"watchdog timeout\",\"fp_exact\":false,\"trial_us\":{}}}",
                                    (timeout as u64) * 1000
                                )
                                .unwrap();
                                out.flush().unwrap();
                                diag("watchdog: trial timed out, exiting");
                                unsafe {
                                    ExitProcess(0);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                writeln!(out, "{{\"ok\":false,\"error\":\"unknown cmd\"}}").unwrap();
                out.flush().unwrap();
            }
        }
    }
}
