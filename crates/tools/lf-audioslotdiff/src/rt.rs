//! The 32-bit differential runtime: checker macros, globals, callee stubs.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`,
//! `relocated`). The proof set reads seven globals (the banked table's
//! stride and base, the strided pool's base, the gated release's four
//! gate words) and two relocated addresses (the triplet table's buffer
//! and the mixer's object word); each is one atomic slot. Numbered
//! callees dispatch through a script the test installs: each id maps to
//! a stub of the matching convention and arity, which records its
//! arguments and answers from a queue. Tests hold one lock across each
//! whole test, so the rewrite's plain reads and writes through the slots
//! never race.
//! Test-support code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs and
// global slots. Every access stays inside memory the running case built.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;
#[cfg(target_arch = "x86")]
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::VecDeque;

/// The banked table's stride word.
static STRIDE_SLOT: AtomicU32 = AtomicU32::new(0);
/// The banked table's table-base word.
static TABLE_SLOT: AtomicU32 = AtomicU32::new(0);
/// The strided pool's element-store base word.
static POOL_SLOT: AtomicU32 = AtomicU32::new(0);
/// The gated release's first gate word.
static G1_SLOT: AtomicU32 = AtomicU32::new(0);
/// The gated release's second gate word.
static G2_SLOT: AtomicU32 = AtomicU32::new(0);
/// The gated release's third gate word.
static G3_SLOT: AtomicU32 = AtomicU32::new(0);
/// The gated release's fourth gate word.
static G4_SLOT: AtomicU32 = AtomicU32::new(0);
/// The triplet table's planted buffer address.
static TRIPLET_SLOT: AtomicU32 = AtomicU32::new(0);
/// The mixer object word handed to the liveness callee.
static MIXER_SLOT: AtomicU32 = AtomicU32::new(0);

/// Plants `v` into the global slot for a file VA.
///
/// # Panics
///
/// When the VA is not one of the seven globals the proof set reads: a
/// case bug, never a guess.
pub fn set_global(file_va: u32, v: u32) {
    match file_va {
        0x0115_D964 => STRIDE_SLOT.store(v, Ordering::SeqCst),
        0x0115_D988 => TABLE_SLOT.store(v, Ordering::SeqCst),
        0x0115_F810 => POOL_SLOT.store(v, Ordering::SeqCst),
        0x011F_7060 => G1_SLOT.store(v, Ordering::SeqCst),
        0x0120_88B4 => G2_SLOT.store(v, Ordering::SeqCst),
        0x00F1_C040 => G3_SLOT.store(v, Ordering::SeqCst),
        0x0103_7720 => G4_SLOT.store(v, Ordering::SeqCst),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Address of the global slot for a file VA.
///
/// # Panics
///
/// When the VA is not one of the seven globals the proof set reads.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x0115_D964 => STRIDE_SLOT.as_ptr(),
        0x0115_D988 => TABLE_SLOT.as_ptr(),
        0x0115_F810 => POOL_SLOT.as_ptr(),
        0x011F_7060 => G1_SLOT.as_ptr(),
        0x0120_88B4 => G2_SLOT.as_ptr(),
        0x00F1_C040 => G3_SLOT.as_ptr(),
        0x0103_7720 => G4_SLOT.as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the shared global at a file VA.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va).cast::<T>()
}

/// Plants `v` as the relocated address for a file VA.
///
/// # Panics
///
/// When the VA is not one of the two relocated addresses the proof set
/// reads: a case bug, never a guess.
pub fn set_relocated(file_va: u32, v: u32) {
    match file_va {
        0x0128_47D8 => TRIPLET_SLOT.store(v, Ordering::SeqCst),
        0x013B_0EB0 => MIXER_SLOT.store(v, Ordering::SeqCst),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Relocated address for a file VA.
///
/// One rewrite reads the stride/table globals through `relocated` and
/// dereferences the answer, where the others use `global`, so those two
/// VAs answer the slot addresses and every other VA answers a planted
/// address.
///
/// # Panics
///
/// When the VA is not one the proof set reads.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x0115_D964 => STRIDE_SLOT.as_ptr() as u32,
        0x0115_D988 => TABLE_SLOT.as_ptr() as u32,
        0x0128_47D8 => TRIPLET_SLOT.load(Ordering::SeqCst),
        0x013B_0EB0 => MIXER_SLOT.load(Ordering::SeqCst),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// A recorded numbered-callee call: callee id and argument words.
pub type NumberedCall = (u32, Vec<u32>);

/// Which stub shape serves a callee id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubKind {
    /// `extern "thiscall" fn(u32) -> u32` (this only).
    Thiscall1,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    Thiscall2,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`.
    Thiscall3,
    /// `extern "thiscall" fn(u32, u32, u32, u32) -> u32`.
    Thiscall4,
    /// `extern "cdecl" fn(u32) -> u32`.
    Cdecl1,
    /// `extern "stdcall" fn(u32) -> u32`.
    Stdcall1,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    numbered: Vec<NumberedCall>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            numbered: Vec::new(),
        }
    }
}

static SCRIPT: std::sync::LazyLock<std::sync::Mutex<Script>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Script::new()));

fn script() -> std::sync::MutexGuard<'static, Script> {
    SCRIPT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Installs the callee script: for each id, its stub shape and its
/// answer queue (popped in call order; exhausted queues answer 0).
/// Clears the call log.
pub fn set_script(spec: &[(u32, StubKind, Vec<u32>)]) {
    let mut s = script();
    *s = Script::new();
    for (id, kind, answers) in spec {
        s.kinds.insert(*id, *kind);
        s.answers.insert(*id, answers.iter().copied().collect());
    }
}

/// Takes the recorded numbered calls, clearing the log.
#[must_use]
pub fn take_calls() -> Vec<NumberedCall> {
    std::mem::take(&mut script().numbered)
}

/// Records a numbered call and pops its answer. Only the x86 stubs call it.
#[cfg(target_arch = "x86")]
fn record_numbered(id: u32, args: Vec<u32>) -> u32 {
    let mut s = script();
    s.numbered.push((id, args));
    s.answers
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

// The stubs name 32-bit calling conventions, which other targets
// reject outright: everything from here to `callee_addr` is x86-only.
#[cfg(target_arch = "x86")]
thread_local! {
    static CURRENT_ID: Cell<u32> = const { Cell::new(0) };
}

#[cfg(target_arch = "x86")]
extern "thiscall" fn stub_thiscall1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

#[cfg(target_arch = "x86")]
extern "thiscall" fn stub_thiscall2(a: u32, b: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b])
}

#[cfg(target_arch = "x86")]
extern "thiscall" fn stub_thiscall3(a: u32, b: u32, c: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c])
}

#[cfg(target_arch = "x86")]
extern "thiscall" fn stub_thiscall4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d])
}

#[cfg(target_arch = "x86")]
extern "cdecl" fn stub_cdecl1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

#[cfg(target_arch = "x86")]
extern "stdcall" fn stub_stdcall1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

/// Raw stub address for callee `id`, by the installed script.
///
/// # Panics
///
/// When no script entry covers `id`: a case bug, never a guess.
#[cfg(target_arch = "x86")]
#[must_use]
// Function addresses travel as words: the rewrites call through them.
pub fn callee_addr(id: u32) -> u32 {
    let kind = *script()
        .kinds
        .get(&id)
        .unwrap_or_else(|| panic!("no stub installed for callee {id}"));
    // The id travels to the stub out of band: `callee_addr` runs at the
    // call site, immediately before the indirect call through its answer,
    // with the test lock held, so no other call can intervene.
    CURRENT_ID.with(|c| c.set(id));
    match kind {
        StubKind::Thiscall1 => stub_thiscall1 as *const () as usize as u32,
        StubKind::Thiscall2 => stub_thiscall2 as *const () as usize as u32,
        StubKind::Thiscall3 => stub_thiscall3 as *const () as usize as u32,
        StubKind::Thiscall4 => stub_thiscall4 as *const () as usize as u32,
        StubKind::Cdecl1 => stub_cdecl1 as *const () as usize as u32,
        StubKind::Stdcall1 => stub_stdcall1 as *const () as usize as u32,
    }
}

/// Non-x86 placeholder: the rewrites (the only callers) never run here.
///
/// # Panics
///
/// Always: no case should reach it off the 32-bit target.
#[cfg(not(target_arch = "x86"))]
#[must_use]
pub fn callee_addr(_id: u32) -> u32 {
    panic!("callee stubs need the 32-bit target")
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

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
