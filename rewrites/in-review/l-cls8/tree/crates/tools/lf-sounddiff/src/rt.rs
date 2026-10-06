//! The 32-bit differential runtime: checker macros plus scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Numbered callees dispatch through a script the test
//! installs: each id maps to a stub of the matching convention and
//! arity, which records its arguments and answers from a queue.
//! Virtual-slot calls land on test-owned stubs that record through
//! [`record_virtual`] and answer from [`virtual_answer`]. Everything is
//! guarded by one mutex; tests hold [`script_lock`] while a rewrite
//! runs, so parallel test threads cannot interleave scripts.
//!
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs, and two
// legacy cells stand in for the vtable and manager words. Every access
// stays inside memory the running case built.
#![allow(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

/// A recorded numbered-callee call: callee id and argument words.
pub type NumberedCall = (u32, Vec<u32>);
/// A recorded virtual-slot call: stub name and argument words.
pub type VirtualCall = (String, Vec<u32>);

/// Which stub shape serves a callee id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubKind {
    /// `extern "thiscall" fn(u32) -> u32`.
    Thiscall1,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    Thiscall2,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`.
    Thiscall3,
    /// `extern "cdecl" fn(u32, u32) -> u32`.
    Cdecl2,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    numbered: Vec<NumberedCall>,
    virtual_answers: HashMap<String, VecDeque<u32>>,
    virtual_calls: Vec<VirtualCall>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
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
    script().virtual_calls.push((name.to_string(), args));
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

extern "thiscall" fn stub_thiscall3(a: u32, b: u32, c: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c])
}

extern "cdecl" fn stub_cdecl2(a: u32, b: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b])
}

use std::cell::Cell;

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
        StubKind::Thiscall3 => stub_thiscall3 as *const () as usize as u32,
        StubKind::Cdecl2 => stub_cdecl2 as *const () as usize as u32,
    }
}

/// The effect vtable cell the constructors stamp. Written by the running
/// case under the script lock.
pub static mut VTABLE_CELL: u32 = 0;

/// The audio manager cell the attach entry looks through. Written by the
/// running case under the script lock.
pub static mut MANAGER_CELL: u32 = 0;

/// File VA to relocated address. Only the vtable and manager words the
/// proof set reads are mapped.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x00E7_BC64 => core::ptr::addr_of!(VTABLE_CELL) as u32,
        0x0115_DAD4 => core::ptr::addr_of!(MANAGER_CELL) as u32,
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Pointer to the shared global at a file VA. The proof set reads no
/// globals through this entry; it exists so the included files link.
///
/// # Panics
///
/// Always: no proof case should reach it.
#[must_use]
pub fn global<T>(_file_va: u32) -> *mut T {
    panic!("unexpected global read: the proof set maps none")
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

/// Call intercepted callee `id` with the cdecl convention.
#[macro_export]
macro_rules! callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        // The block is redundant where the call site already sits in
        // unsafe code (all proof files do); the allow keeps the
        // expansion warning-free either way.
        #[allow(unused_unsafe)]
        let f: extern "cdecl" fn($( $crate::__ty!($arg) ),*) -> $ret =
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

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
