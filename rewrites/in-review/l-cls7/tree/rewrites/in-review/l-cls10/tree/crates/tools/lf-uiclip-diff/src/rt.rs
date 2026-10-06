//! The 32-bit differential runtime: checker macros plus scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Numbered callees dispatch through a script the test
//! installs: each id maps to a stub of the matching convention and
//! arity, which records its arguments and answers from a queue. Two
//! stubs also write: the scratch-zeroing call and the matcher, which
//! fills its two out-words from a second queue. Virtual-slot calls land
//! on test-owned stubs that record through [`record_virtual`] (or
//! [`record_virtual_snap`], which also snapshots bytes behind a pointer
//! argument) and answer from [`virtual_answer`]. Everything is guarded
//! by one mutex; tests hold [`script_lock`] while a rewrite runs, so
//! parallel test threads cannot interleave scripts.
//!
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs. Every
// access stays inside memory the running case built.
#![allow(unsafe_code)]

use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

/// A recorded numbered-callee call: callee id and argument words.
pub type NumberedCall = (u32, Vec<u32>);
/// A recorded virtual-slot call: stub name, argument words, and the
/// optional byte snapshot behind a pointer argument.
pub type VirtualCall = (String, Vec<u32>, Option<Vec<u8>>);

/// Which stub shape serves a callee id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubKind {
    /// `extern "thiscall" fn(u32) -> u32`.
    Thiscall1,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    Thiscall2,
    /// `extern "thiscall" fn(u32, u32) -> u32`, writing two scripted
    /// words through the first argument (the matcher).
    Thiscall2Out2,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`.
    Thiscall3,
    /// `extern "cdecl" fn() -> u32`.
    Cdecl0,
    /// `extern "cdecl" fn(u32) -> u32`.
    Cdecl1,
    /// `extern "cdecl" fn(u32, u32, u32) -> u32`, zeroing the third
    /// argument's bytes at the first (the scratch fill).
    Cdecl3Zero,
    /// `extern "cdecl" fn(u32, u32, u32, u32, u32) -> u32`.
    Cdecl5,
    /// `extern "stdcall" fn() -> u32`.
    Stdcall0,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    out_pairs: HashMap<u32, VecDeque<[u32; 2]>>,
    numbered: Vec<NumberedCall>,
    virtual_answers: HashMap<String, VecDeque<u32>>,
    virtual_calls: Vec<VirtualCall>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            out_pairs: HashMap::new(),
            numbered: Vec::new(),
            virtual_answers: HashMap::new(),
            virtual_calls: Vec::new(),
        }
    }
}

static SCRIPT: std::sync::LazyLock<Mutex<Script>> =
    std::sync::LazyLock::new(|| Mutex::new(Script::new()));

/// Locks the script for one rewrite run. Hold the guard from installing
/// the script until the recorded calls are collected. Poison is ignored:
/// a failed case must not wedge the remaining tests.
pub fn script_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn script() -> std::sync::MutexGuard<'static, Script> {
    SCRIPT.lock().unwrap_or_else(|e| e.into_inner())
}

/// Installs the callee script: for each id, its stub shape and its
/// answer queue (popped in call order; exhausted queues answer 0).
/// Clears both call logs.
pub fn set_script(spec: &[(u32, StubKind, Vec<u32>)]) {
    let mut s = script();
    *s = Script::new();
    for (id, kind, answers) in spec {
        s.kinds.insert(*id, *kind);
        s.answers.insert(*id, answers.iter().copied().collect());
    }
}

/// Installs the matcher's out-word pairs by callee id (popped in call
/// order; exhausted queues write zeros).
pub fn set_out_pairs(spec: &[(u32, Vec<[u32; 2]>)]) {
    let mut s = script();
    for (id, pairs) in spec {
        s.out_pairs.insert(*id, pairs.iter().copied().collect());
    }
}

/// Installs virtual-slot answers by stub name (popped in call order;
/// exhausted queues answer 0).
pub fn set_virtual(spec: &[(&str, Vec<u32>)]) {
    let mut s = script();
    for (name, answers) in spec {
        s.virtual_answers
            .insert((*name).to_string(), answers.iter().copied().collect());
    }
}

/// Takes the recorded numbered calls, clearing the log.
pub fn take_numbered() -> Vec<NumberedCall> {
    std::mem::take(&mut script().numbered)
}

/// Takes the recorded virtual calls, clearing the log.
pub fn take_virtual() -> Vec<VirtualCall> {
    std::mem::take(&mut script().virtual_calls)
}

/// Records a numbered call and pops its answer.
fn record_numbered(id: u32, args: Vec<u32>) -> u32 {
    let mut s = script();
    s.numbered.push((id, args));
    s.answers
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

/// Records a virtual-slot call.
pub fn record_virtual(name: &str, args: Vec<u32>) {
    script()
        .virtual_calls
        .push((name.to_string(), args, None));
}

/// Records a virtual-slot call, snapshotting `len` bytes behind `ptr`
/// while the caller's frame is alive.
pub fn record_virtual_snap(name: &str, args: Vec<u32>, ptr: u32, len: usize) {
    let bytes = unsafe { core::slice::from_raw_parts(ptr as *const u8, len).to_vec() };
    script()
        .virtual_calls
        .push((name.to_string(), args, Some(bytes)));
}

/// Pops a virtual-slot answer.
pub fn virtual_answer(name: &str) -> u32 {
    script()
        .virtual_answers
        .get_mut(name)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

extern "thiscall" fn stub_thiscall1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

extern "thiscall" fn stub_thiscall2(a: u32, b: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b])
}

extern "thiscall" fn stub_thiscall2_out(a: u32, b: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let pair = script()
        .out_pairs
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or([0, 0]);
    unsafe {
        (a as *mut u32).write_unaligned(pair[0]);
        ((a.wrapping_add(4)) as *mut u32).write_unaligned(pair[1]);
    }
    record_numbered(id, vec![a, b])
}

extern "thiscall" fn stub_thiscall3(a: u32, b: u32, c: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c])
}

extern "cdecl" fn stub_cdecl0() -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![])
}

extern "cdecl" fn stub_cdecl1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

extern "cdecl" fn stub_cdecl3_zero(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        core::slice::from_raw_parts_mut(a as *mut u8, c as usize).fill(0);
    }
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c])
}

extern "cdecl" fn stub_cdecl5(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d, e])
}

extern "stdcall" fn stub_stdcall0() -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![])
}

thread_local! {
    static CURRENT_ID: Cell<u32> = const { Cell::new(0) };
}

/// Raw stub address for callee `id`, by the installed script.
///
/// # Panics
///
/// When no script entry covers `id`: a case bug, never a guess.
#[must_use]
// Function addresses travel as words: the rewrites call through them.
pub fn callee_addr(id: u32) -> u32 {
    let kind = *script()
        .kinds
        .get(&id)
        .unwrap_or_else(|| panic!("no stub installed for callee {id}"));
    // The id travels to the stub out of band: `callee_addr` runs at the
    // call site, immediately before the indirect call through its answer,
    // with the script lock held, so no other call can intervene.
    CURRENT_ID.with(|c| c.set(id));
    match kind {
        StubKind::Thiscall1 => stub_thiscall1 as *const () as usize as u32,
        StubKind::Thiscall2 => stub_thiscall2 as *const () as usize as u32,
        StubKind::Thiscall2Out2 => stub_thiscall2_out as *const () as usize as u32,
        StubKind::Thiscall3 => stub_thiscall3 as *const () as usize as u32,
        StubKind::Cdecl0 => stub_cdecl0 as *const () as usize as u32,
        StubKind::Cdecl1 => stub_cdecl1 as *const () as usize as u32,
        StubKind::Cdecl3Zero => stub_cdecl3_zero as *const () as usize as u32,
        StubKind::Cdecl5 => stub_cdecl5 as *const () as usize as u32,
        StubKind::Stdcall0 => stub_stdcall0 as *const () as usize as u32,
    }
}

/// The triple splitter's first multiplier, measured from the
/// executable's data.
static TRIPLE_C0_BITS: u32 = 0x3C88_8889;
/// The triple splitter's back-multiplier, measured the same way.
static TRIPLE_C1_BITS: u32 = 0x4270_0000;
/// The triple splitter's second multiplier, measured the same way.
static TRIPLE_C2_BITS: u32 = 0x42C8_0000;
/// The float-push multiplier, measured the same way.
static MEASURE_SCALE_BITS: u32 = 0x3F70_A3D7;
/// The float-push offset, measured the same way.
static MEASURE_BIAS_BITS: u32 = 0x40C0_0000;

/// The shared constant at a file VA, if it is one of the five the proof
/// set reads.
fn const_bits(file_va: u32) -> Option<*const u32> {
    match file_va {
        0xFE8724 => Some(&TRIPLE_C0_BITS),
        0xFE8B80 => Some(&TRIPLE_C1_BITS),
        0xFE8BB0 => Some(&TRIPLE_C2_BITS),
        0xFE88C0 => Some(&MEASURE_SCALE_BITS),
        0xFE8AE0 => Some(&MEASURE_BIAS_BITS),
        _ => None,
    }
}

/// Pointer to the shared constant at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// When the address is not one of the five constants the proof set
/// reads: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let word =
        const_bits(file_va).unwrap_or_else(|| panic!("unexpected shared constant VA {file_va:#x}"));
    word as *mut T
}

/// File VA to relocated address. The two engine tables the proof set
/// passes by address answer as themselves: stable, distinct words.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0xEFB9A4 | 0xEFB9B4 => file_va,
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (fastcall).
        #[unsafe(no_mangle)]
        pub extern "fastcall" fn $name($($arg : $ty),*) -> $ret $body
    };
}

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
}

/// Call intercepted callee `id` with the stdcall convention.
#[macro_export]
macro_rules! callee_stdcall {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "stdcall" fn($( $crate::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($( $arg ),*)
    }};
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

/// Call intercepted callee `id` with the fastcall convention.
#[macro_export]
macro_rules! callee_fastcall {
    ($id:expr, $ret:ty, $ecx_arg:expr, $edx_arg:expr $(, $arg:expr)* $(,)?) => {{
        #[allow(unused_unsafe)]
        let f: extern "fastcall" fn(u32, u32 $(, $crate::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute($crate::callee_addr($id) as usize) };
        f($ecx_arg, $edx_arg $(, $arg )*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
