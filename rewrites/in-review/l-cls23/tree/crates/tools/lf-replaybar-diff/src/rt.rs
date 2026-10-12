//! The 32-bit differential runtime: checker macros and recording callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). A numbered callee dispatches through the innermost active
//! script: each stub records its raw argument words, writes the scripted
//! words through the call's out-pointer arguments, and returns the scripted
//! return word. One callee of the damage-free nested case runs another
//! verified rewrite under a second script, so its calls are recorded in the
//! same log, tagged with the script they belong to.
//!
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through the stubs, and the stubs
// write through the out-pointers the rewrites pass. Every access stays
// inside stack words the rewrite itself passed.
#![allow(unsafe_code)]

use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

use crate::fake::{Answer, CallSpec, Script};

/// Which stub shape serves a callee number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Thiscall with the object word only.
    T1,
    /// Thiscall with the object word and one argument.
    T2,
    /// Thiscall with the object word and two arguments.
    T3,
    /// Thiscall with the object word and three arguments.
    T4,
    /// Cdecl with no arguments.
    C0,
    /// Cdecl with one argument.
    C1,
    /// Cdecl with two arguments.
    C2,
    /// Thiscall that runs the verified teardown rewrite under scope 1.
    NestedTeardown,
}

/// One scope's callee table: the stub shape, the scripted answers and the
/// out-pointer argument positions of each callee number.
#[derive(Default)]
pub struct Scope {
    kinds: HashMap<u32, Kind>,
    answers: HashMap<u32, std::collections::VecDeque<Answer>>,
    outs: HashMap<u32, &'static [usize]>,
}

/// A recorded call: the scope it ran in, the callee number, and the words.
pub type Recorded = (usize, u32, Vec<u32>);

#[derive(Default)]
struct State {
    scopes: [Scope; 2],
    stack: Vec<usize>,
    log: Vec<Recorded>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// Serialises the cases: one rewrite runs at a time.
pub fn case_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    f(guard.get_or_insert_with(State::default))
}

/// Installs a scope: each call of the table gets its stub kind, its
/// out-pointer positions and its answers from `script` by name.
pub fn install(scope: usize, table: &[(CallSpec, Kind)], script: &Script) {
    with_state(|state| {
        let mut fresh = Scope::default();
        for (spec, kind) in table {
            fresh.kinds.insert(spec.id, *kind);
            fresh.outs.insert(spec.id, spec.outs);
            let queue = script.get(spec.name).cloned().unwrap_or_default();
            fresh.answers.insert(spec.id, queue);
        }
        state.scopes[scope] = fresh;
        state.stack.clear();
        state.stack.push(scope);
        state.log.clear();
    });
}

/// Installs a scope that only dispatches nested calls, with no answers.
pub fn install_nested_only(scope: usize, id: u32, kind: Kind) {
    with_state(|state| {
        let mut fresh = Scope::default();
        fresh.kinds.insert(id, kind);
        fresh.outs.insert(id, &[]);
        state.scopes[scope] = fresh;
    });
}

/// Takes the recorded calls, clearing the log.
pub fn take_log() -> Vec<Recorded> {
    with_state(|state| std::mem::take(&mut state.log))
}

thread_local! {
    static CURRENT_ID: Cell<u32> = const { Cell::new(0) };
}

fn current_id() -> u32 {
    CURRENT_ID.with(Cell::get)
}

fn top_scope() -> usize {
    with_state(|state| *state.stack.last().expect("a scope is active"))
}

/// Raw stub address for callee number `id`, in the innermost scope.
///
/// # Panics
///
/// When no scope entry covers `id`: a case bug, never a guess.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let kind = with_state(|state| {
        let scope = *state.stack.last().expect("a scope is active");
        *state.scopes[scope]
            .kinds
            .get(&id)
            .unwrap_or_else(|| panic!("no stub installed for callee {id}"))
    });
    CURRENT_ID.with(|c| c.set(id));
    let addr = match kind {
        Kind::T1 => stub_t1 as *const () as usize,
        Kind::T2 => stub_t2 as *const () as usize,
        Kind::T3 => stub_t3 as *const () as usize,
        Kind::T4 => stub_t4 as *const () as usize,
        Kind::C0 => stub_c0 as *const () as usize,
        Kind::C1 => stub_c1 as *const () as usize,
        Kind::C2 => stub_c2 as *const () as usize,
        Kind::NestedTeardown => stub_nested_teardown as *const () as usize,
    };
    addr as u32
}

/// Records a call in the innermost scope, writes its scripted out-words and
/// returns its scripted return word.
fn dispatch(id: u32, args: Vec<u32>) -> u32 {
    let (scope, ret, outs, positions) = with_state(|state| {
        let scope = *state.stack.last().expect("a scope is active");
        state.log.push((scope, id, args.clone()));
        let answer = state.scopes[scope]
            .answers
            .get_mut(&id)
            .and_then(std::collections::VecDeque::pop_front)
            .unwrap_or_default();
        let positions = state.scopes[scope].outs.get(&id).copied().unwrap_or(&[]);
        (scope, answer.ret, answer.outs, positions)
    });
    let _ = scope;
    for (k, &position) in positions.iter().enumerate() {
        let pointer = args[position] as *mut u32;
        // SAFETY: the rewrite passed this address as an out-pointer to a
        // stack word of its own frame, which stays live for the call.
        unsafe { pointer.write_unaligned(outs[k]) };
    }
    ret
}

extern "thiscall" fn stub_t1(a: u32) -> u32 {
    dispatch(current_id(), vec![a])
}

extern "thiscall" fn stub_t2(a: u32, b: u32) -> u32 {
    dispatch(current_id(), vec![a, b])
}

extern "thiscall" fn stub_t3(a: u32, b: u32, c: u32) -> u32 {
    dispatch(current_id(), vec![a, b, c])
}

extern "thiscall" fn stub_t4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    dispatch(current_id(), vec![a, b, c, d])
}

extern "cdecl" fn stub_c0() -> u32 {
    dispatch(current_id(), vec![])
}

extern "cdecl" fn stub_c1(a: u32) -> u32 {
    dispatch(current_id(), vec![a])
}

extern "cdecl" fn stub_c2(a: u32, b: u32) -> u32 {
    dispatch(current_id(), vec![a, b])
}

/// Runs the verified teardown rewrite on the object, with its calls
/// recorded under scope 1. Nothing is recorded in the caller's scope.
extern "thiscall" fn stub_nested_teardown(this: u32) -> u32 {
    with_state(|state| state.stack.push(1));
    let result = crate::rewrites::rw_00d6bcf0(this);
    with_state(|state| {
        state.stack.pop();
    });
    result
}

/// The live-table word cell, the dead-table word cell and the token argument
/// cell: the three constants the teardown reads through `relocated`.
static LIVE_CELL: AtomicU32 = AtomicU32::new(0);
static DEAD_CELL: AtomicU32 = AtomicU32::new(0);
static TOKEN_CELL: AtomicU32 = AtomicU32::new(0);

/// The mode word, the flag byte (read as a byte), the or-word, the saved
/// word and the gate byte, each in its own cell.
static MODE_CELL: AtomicU32 = AtomicU32::new(0);
static FLAG_CELL: AtomicU32 = AtomicU32::new(0);
static OR_CELL: AtomicU32 = AtomicU32::new(0);
static SAVE_CELL: AtomicU32 = AtomicU32::new(0);
static GATE_CELL: AtomicU32 = AtomicU32::new(0);

fn cell_addr(cell: &AtomicU32) -> u32 {
    cell.as_ptr() as usize as u32
}

/// Address of the token argument cell, for the case that builds the record.
#[must_use]
pub fn token_cell_addr() -> u32 {
    cell_addr(&TOKEN_CELL)
}

/// Address of the live-table cell, for the case that checks the object.
#[must_use]
pub fn live_cell_addr() -> u32 {
    cell_addr(&LIVE_CELL)
}

/// Address of the dead-table cell, for the case that checks the object.
#[must_use]
pub fn dead_cell_addr() -> u32 {
    cell_addr(&DEAD_CELL)
}

/// Sets the shared globals the update reads, before a case runs.
pub fn set_globals(mode: u32, flag: u32, gate: u32, or_word: u32, save_word: u32) {
    MODE_CELL.store(mode, Ordering::SeqCst);
    FLAG_CELL.store(flag, Ordering::SeqCst);
    GATE_CELL.store(gate, Ordering::SeqCst);
    OR_CELL.store(or_word, Ordering::SeqCst);
    SAVE_CELL.store(save_word, Ordering::SeqCst);
}

/// Reads back the or-word and the saved word after a case.
#[must_use]
pub fn read_or_save() -> (u32, u32) {
    (OR_CELL.load(Ordering::SeqCst), SAVE_CELL.load(Ordering::SeqCst))
}

/// File VA to relocated address. Only the constants the proof set reads
/// are mapped.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x00EE_AF58 => cell_addr(&LIVE_CELL),
        0x00EE_AEF0 => cell_addr(&TOKEN_CELL),
        0x00EE_A88C => cell_addr(&DEAD_CELL),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Pointer to the shared global at a file VA.
///
/// # Panics
///
/// For any VA the proof set does not map: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let cell: &AtomicU32 = match file_va {
        0x0103_7720 => &MODE_CELL,
        0x011F_701E => &FLAG_CELL,
        0x011F_70E0 => &OR_CELL,
        0x011F_70E4 => &SAVE_CELL,
        0x0103_F690 => &GATE_CELL,
        _ => panic!("unexpected global VA {file_va:#x}"),
    };
    cell.as_ptr().cast::<T>()
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
