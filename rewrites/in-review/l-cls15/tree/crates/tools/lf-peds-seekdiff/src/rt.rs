//! The 32-bit differential runtime: checker macros and scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). A numbered callee dispatches to a Rust stub the test
//! installs for the rewrite under test. Each stub records its arguments
//! and a copy of the watched object, then answers from a queue. The
//! constants a rewrite reads through `relocated` and `global` are bound
//! by the test to memory it owns.
//!
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs, and the
// stubs read and write the test's own memory.
#![allow(unsafe_code)]

use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

/// Callee slots a rewrite may name.
const CALLEE_SLOTS: usize = 8;

const UNSET: AtomicU32 = AtomicU32::new(0);
static CALLEES: [AtomicU32; CALLEE_SLOTS] = [UNSET; CALLEE_SLOTS];

/// Points numbered callee `id` at a stub.
pub fn set_callee(id: u32, addr: u32) {
    CALLEES[id as usize].store(addr, Ordering::SeqCst);
}

/// The stub address for numbered callee `id`, mirroring the checker.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    CALLEES.get(id as usize).map_or(0, |a| a.load(Ordering::SeqCst))
}

/// File-address bindings: a file address the rewrites read through
/// `relocated` or `global` maps to memory the test owns.
static BINDINGS: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

/// Binds a file address to the test memory that stands for it.
pub fn bind(file_va: u32, addr: u32) {
    let mut b = BINDINGS.lock().unwrap_or_else(|e| e.into_inner());
    b.retain(|(va, _)| *va != file_va);
    b.push((file_va, addr));
}

/// The bound address for a file address.
///
/// # Panics
///
/// When the test did not bind the address: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    let b = BINDINGS.lock().unwrap_or_else(|e| e.into_inner());
    b.iter()
        .find(|(va, _)| *va == file_va)
        .map(|(_, addr)| *addr)
        .unwrap_or_else(|| panic!("unbound file address {file_va:#x}"))
}

/// Pointer to the bound cell for a file address, mirroring the checker's
/// `global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as usize as *mut T
}

/// One recorded numbered-callee call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawCall {
    /// The callee number the rewrite used.
    pub id: u32,
    /// The argument words, `this` first.
    pub args: Vec<u32>,
    /// A copy of the watched object at the time of the call, when watched.
    pub snap: Option<Vec<u8>>,
}

/// The recorded calls of the rewrite under test.
static LOG: Mutex<Vec<RawCall>> = Mutex::new(Vec::new());

/// The watched object: its address and length (zero when unwatched).
static WATCH_ADDR: AtomicU32 = AtomicU32::new(0);
static WATCH_LEN: AtomicU32 = AtomicU32::new(0);

/// Watches an object for snapshots at each callee call.
pub fn watch(addr: u32, len: u32) {
    WATCH_ADDR.store(addr, Ordering::SeqCst);
    WATCH_LEN.store(len, Ordering::SeqCst);
}

/// Stops watching.
pub fn unwatch() {
    watch(0, 0);
}

/// Takes the recorded calls, clearing the log.
pub fn take_log() -> Vec<RawCall> {
    std::mem::take(&mut *LOG.lock().unwrap_or_else(|e| e.into_inner()))
}

fn record(id: u32, args: Vec<u32>) {
    let addr = WATCH_ADDR.load(Ordering::SeqCst);
    let len = WATCH_LEN.load(Ordering::SeqCst) as usize;
    let snap = if addr != 0 {
        // SAFETY: the test watches an arena it owns for the whole call and
        // passes its exact length.
        Some(unsafe { std::slice::from_raw_parts(addr as usize as *const u8, len).to_vec() })
    } else {
        None
    };
    LOG.lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(RawCall { id, args, snap });
}

/// Answers queued per callee number (popped in call order; an empty queue
/// answers zero).
static ANSWERS: Mutex<Vec<(u32, VecDeque<u32>)>> = Mutex::new(Vec::new());

/// Queues the answers of callee `id`, replacing any queue for it.
pub fn set_answers(id: u32, values: &[u32]) {
    let mut a = ANSWERS.lock().unwrap_or_else(|e| e.into_inner());
    a.retain(|(i, _)| *i != id);
    a.push((id, values.iter().copied().collect()));
}

fn answer(id: u32) -> u32 {
    let mut a = ANSWERS.lock().unwrap_or_else(|e| e.into_inner());
    a.iter_mut()
        .find(|(i, _)| *i == id)
        .and_then(|(_, q)| q.pop_front())
        .unwrap_or(0)
}

/// The four words the cover-row query writes to its out pointer.
static QUERY_WORDS: Mutex<[u32; 4]> = Mutex::new([0; 4]);

/// Sets the words the cover-row query writes.
pub fn set_query_words(words: [u32; 4]) {
    *QUERY_WORDS.lock().unwrap_or_else(|e| e.into_inner()) = words;
}

macro_rules! stub {
    ($name:ident, $id:literal, [$($a:ident),*]) => {
        /// Scripted numbered callee: records its arguments and answers.
        extern "thiscall" fn $name(this: u32 $(, $a: u32)*) -> u32 {
            record($id, vec![this $(, $a)*]);
            answer($id)
        }
    };
}

stub!(s1_0, 1, []);
stub!(s1_1, 1, [a]);
stub!(s2_0, 2, []);
stub!(s2_1, 2, [a]);
stub!(s2_5, 2, [a, b, c, d, e]);
stub!(s2_7, 2, [a, b, c, d, e, f, g]);
stub!(s3_0, 3, []);
stub!(s3_7, 3, [a, b, c, d, e, f, g]);
stub!(s4_0, 4, []);
stub!(s4_6, 4, [a, b, c, d, e, f]);
stub!(s5_0, 5, []);

/// The cover-row query: records its arguments, writes the queued words
/// through its out pointer, and answers.
extern "thiscall" fn s4_2(this: u32, out: u32, zero: u32) -> u32 {
    record(4, vec![this, out, zero]);
    let words = *QUERY_WORDS.lock().unwrap_or_else(|e| e.into_inner());
    // SAFETY: the rewrite passes the address of a four-word array it owns.
    unsafe {
        for (i, w) in words.iter().enumerate() {
            (out as usize as *mut u32).add(i).write_unaligned(*w);
        }
    }
    answer(4)
}

/// Address of a named stub, for installing it as a callee.
///
/// # Panics
///
/// On an unknown name: a case bug.
#[must_use]
pub fn stub(name: &str) -> u32 {
    match name {
        "s1_0" => s1_0 as *const () as usize as u32,
        "s1_1" => s1_1 as *const () as usize as u32,
        "s2_0" => s2_0 as *const () as usize as u32,
        "s2_1" => s2_1 as *const () as usize as u32,
        "s2_5" => s2_5 as *const () as usize as u32,
        "s2_7" => s2_7 as *const () as usize as u32,
        "s3_0" => s3_0 as *const () as usize as u32,
        "s3_7" => s3_7 as *const () as usize as u32,
        "s4_0" => s4_0 as *const () as usize as u32,
        "s4_2" => s4_2 as *const () as usize as u32,
        "s4_6" => s4_6 as *const () as usize as u32,
        "s5_0" => s5_0 as *const () as usize as u32,
        other => panic!("unknown stub {other}"),
    }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the thiscall convention.
#[macro_export]
macro_rules! callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
