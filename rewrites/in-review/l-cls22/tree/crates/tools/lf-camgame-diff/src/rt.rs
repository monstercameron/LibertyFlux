//! The 32-bit differential runtime for the game camera rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`). Numbered
//! callees dispatch through a script the test installs: each id maps to a
//! stub of the matching convention and arity, which records its arguments
//! and answers from a queue. A stub of the effect kind also applies the
//! trial's scripted writes to the camera object it is given, so a callee
//! that changes the camera does so the same way on both sides. Virtual
//! slot calls land on test-owned stubs that record through the same log.
//! Everything is guarded by one mutex; tests hold [`script_lock`] while a
//! rewrite runs, so parallel test threads cannot interleave scripts.
//!
//! Test-support code: the lifted crate itself forbids `unsafe`. Raw
//! addresses flow through scripted stubs, and the two globals the rewrites
//! read are cells this module owns. Every access stays inside memory the
//! running case built.

// Test-only runtime: raw addresses flow through scripted stubs, and the
// rewrites' globals are cells this module owns. Every access stays inside
// memory the running case built.
#![allow(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

/// A recorded call: the rewrite's callee as `#id`, or a virtual stub's
/// name, with its argument words.
pub type Call = (String, Vec<u32>);

/// Which stub shape serves a callee id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubKind {
    /// `extern "thiscall" fn(u32) -> u32`, records its argument.
    Thiscall1,
    /// As [`StubKind::Thiscall1`], and first records the camera's words
    /// (see [`SNAPSHOT`]) and then applies the trial's effects to it.
    Thiscall1Effect,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    Thiscall2,
    /// `extern "thiscall" fn(u32, u32, u32, u32) -> u32`.
    Thiscall4,
}

/// One scripted write a callee makes to the camera object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Effect {
    /// Offset within the camera object.
    pub off: u32,
    /// Width in bytes: 1 or 4.
    pub width: u8,
    /// The value written (truncated to the width).
    pub value: u32,
}

/// Camera words the callees record, as (offset, width in bytes). The
/// order is the order of the recorded words.
pub const SNAPSHOT: [(u32, u8); 19] = [
    (0x114, 4),
    (0x140, 4),
    (0x144, 1),
    (0x148, 4),
    (0x14C, 4),
    (0x150, 1),
    (0x154, 4),
    (0x1BC, 4),
    (0x1C4, 1),
    (0x1C5, 1),
    (0x1C8, 4),
    (0x1CC, 4),
    (0x1D0, 4),
    (0x1D4, 4),
    (0x1D8, 1),
    (0x1DC, 4),
    (0x1E0, 4),
    (0x1E4, 4),
    (0x1E8, 4),
];

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    effects: Vec<Effect>,
    calls: Vec<Call>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            effects: Vec::new(),
            calls: Vec::new(),
        }
    }
}

static SCRIPT: std::sync::LazyLock<Mutex<Script>> =
    std::sync::LazyLock::new(|| Mutex::new(Script::new()));

/// The shared global the deleting destructor reads (the pool manager).
static MANAGER_CELL: AtomicU32 = AtomicU32::new(0);

/// The shared global the idle reset copies (the default word).
static DEFAULT_CELL: AtomicU32 = AtomicU32::new(0);

thread_local! {
    static CURRENT_ID: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Locks the script for one rewrite run. Poison is ignored: a failed case
/// must not wedge the remaining tests.
pub fn script_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn script() -> MutexGuard<'static, Script> {
    SCRIPT.lock().unwrap_or_else(|e| e.into_inner())
}

/// Installs one case: each callee id's stub shape and answer queue (popped
/// in call order; exhausted queues answer 0), the effects the effect stubs
/// apply, and the globals. Clears the call log.
pub fn set_case(spec: &[(u32, StubKind, Vec<u32>)], effects: &[Effect], manager: u32, default: u32) {
    let mut s = script();
    *s = Script::new();
    for (id, kind, answers) in spec {
        s.kinds.insert(*id, *kind);
        s.answers.insert(*id, answers.iter().copied().collect());
    }
    s.effects = effects.to_vec();
    drop(s);
    MANAGER_CELL.store(manager, Ordering::SeqCst);
    DEFAULT_CELL.store(default, Ordering::SeqCst);
}

/// Takes the recorded calls in order, clearing the log.
pub fn take_calls() -> Vec<Call> {
    std::mem::take(&mut script().calls)
}

fn record(name: String, args: Vec<u32>) {
    script().calls.push((name, args));
}

/// Records a numbered call and pops its answer.
fn record_numbered(id: u32, args: Vec<u32>) -> u32 {
    let mut s = script();
    s.calls.push((format!("#{id}"), args));
    s.answers
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

/// Reads the camera words named by [`SNAPSHOT`] from raw memory.
///
/// # Safety
///
/// `this` must address a camera image of at least the largest offset plus
/// its width, as the running case builds it.
unsafe fn snapshot_at(this: u32) -> Vec<u32> {
    SNAPSHOT
        .iter()
        .map(|&(off, width)| {
            let p = (this as usize + off as usize) as *const u8;
            if width == 1 {
                // SAFETY: the case's camera image covers this offset.
                unsafe { p.read() as u32 }
            } else {
                // SAFETY: the case's camera image covers this offset.
                unsafe { (p as *const u32).read_unaligned() }
            }
        })
        .collect()
}

/// Applies one scripted write at `this`.
///
/// # Safety
///
/// As [`snapshot_at`], for the effect's offset and width.
unsafe fn write_effect(this: u32, e: Effect) {
    let p = (this as usize + e.off as usize) as *mut u8;
    if e.width == 1 {
        // SAFETY: the case's camera image covers this offset.
        unsafe { p.write(e.value as u8) };
    } else {
        // SAFETY: the case's camera image covers this offset.
        unsafe { (p as *mut u32).write_unaligned(e.value) };
    }
}

fn run_effect_stub(id: u32, this: u32) -> u32 {
    let mut args = vec![this];
    // SAFETY: the rewrite runs on a camera image the case built.
    args.extend(unsafe { snapshot_at(this) });
    let effects = script().effects.clone();
    for e in effects {
        // SAFETY: as above.
        unsafe { write_effect(this, e) };
    }
    record_numbered(id, args)
}

extern "thiscall" fn stub_thiscall1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

extern "thiscall" fn stub_thiscall1_effect(a: u32) -> u32 {
    run_effect_stub(CURRENT_ID.with(|c| c.get()), a)
}

extern "thiscall" fn stub_thiscall2(a: u32, b: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b])
}

extern "thiscall" fn stub_thiscall4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d])
}

/// Raw stub address for callee `id`, by the installed script.
///
/// # Panics
///
/// When no script entry covers `id`: a case bug, never a guess.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let kind = *script()
        .kinds
        .get(&id)
        .unwrap_or_else(|| panic!("no stub installed for callee {id}"));
    // The id travels to the stub out of band: `callee_addr` runs at the
    // call site, immediately before the indirect call through its answer.
    CURRENT_ID.with(|c| c.set(id));
    match kind {
        StubKind::Thiscall1 => stub_thiscall1 as *const () as usize as u32,
        StubKind::Thiscall1Effect => stub_thiscall1_effect as *const () as usize as u32,
        StubKind::Thiscall2 => stub_thiscall2 as *const () as usize as u32,
        StubKind::Thiscall4 => stub_thiscall4 as *const () as usize as u32,
    }
}

extern "thiscall" fn stub_teardown(this: u32) -> u32 {
    record("teardown".to_string(), vec![this]);
    0
}

extern "thiscall" fn stub_release_worker(worker: u32) -> u32 {
    record("release_worker".to_string(), vec![worker]);
    0
}

/// Address of the stub that answers a camera's dispatched teardown slot.
#[must_use]
pub fn teardown_stub_addr() -> u32 {
    stub_teardown as *const () as usize as u32
}

/// Address of the stub that answers a worker's release slot.
#[must_use]
pub fn release_worker_stub_addr() -> u32 {
    stub_release_worker as *const () as usize as u32
}

/// Pointer to a shared global at a file address. Only the two globals the
/// verified rewrites read are mapped; any other address is a case bug.
///
/// # Panics
///
/// For any other address.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    match file_va {
        0x012F_B1A0 => MANAGER_CELL.as_ptr().cast::<T>(),
        0x0117_35C4 => DEFAULT_CELL.as_ptr().cast::<T>(),
        _ => panic!("unexpected global address {file_va:#x}"),
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
        #[allow(unused_unsafe)]
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
