//! The 32-bit differential runtime: checker macros plus scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Numbered callees dispatch through a script the test
//! installs: each id maps to a stub of the matching convention and
//! arity, which records its arguments, snapshots pointed-to words the
//! proof compares, performs scripted writes through out-pointers, and
//! answers from a queue. The completion virtual lands on a test-owned
//! stub that records through [`record_virtual`] and answers from
//! [`virtual_answer`]. Everything is guarded by one mutex; tests hold
//! [`script_lock`] while a rewrite runs, so parallel test threads cannot
//! interleave scripts.
//!
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs, and a few
// global cells stand in for the session globals. Every access stays
// inside memory the running case built.
#![allow(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

/// A recorded numbered-callee call: callee id and argument words.
pub type NumberedCall = (u32, Vec<u32>);
/// A recorded virtual-slot call: stub name and argument words.
pub type VirtualCall = (String, Vec<u32>);
/// A snapshot: callee id, argument index holding the pointer, words read.
pub type Snapshot = (u32, usize, Vec<u32>);

/// Which stub shape serves a callee id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubKind {
    /// `extern "thiscall" fn(u32) -> u32`.
    Thiscall0,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    Thiscall1,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`.
    Thiscall2,
    /// `extern "thiscall" fn(u32, u32, u32, u32) -> u32`.
    Thiscall3,
    /// Five words in, one out.
    Thiscall4,
    /// Six words in, one out.
    Thiscall5,
    /// Seven words in, one out.
    Thiscall6,
    /// Eight words in, one out.
    Thiscall7,
    /// `extern "cdecl" fn(u32, u32, u32) -> u32`.
    Cdecl3,
}

/// The installed script for one callee id.
pub struct Spec {
    /// Callee id.
    pub id: u32,
    /// Stub shape.
    pub kind: StubKind,
    /// Answers, popped in call order; exhausted queues answer 0.
    pub answers: Vec<u32>,
    /// Snapshots: (argument index, word count) pairs, read on entry.
    /// Argument 0 is `this` for thiscall stubs.
    pub snaps: Vec<(usize, usize)>,
    /// Out writes: one entry per call, each a list of (argument index,
    /// words to store through that pointer). Missing entries write nothing.
    pub outs: Vec<Vec<(usize, Vec<u32>)>>,
}

/// The installed script and the recorded calls.
struct CalleeScript {
    kind: StubKind,
    answers: VecDeque<u32>,
    snaps: Vec<(usize, usize)>,
    outs: VecDeque<Vec<(usize, Vec<u32>)>>,
}

struct Script {
    callees: HashMap<u32, CalleeScript>,
    numbered: Vec<NumberedCall>,
    snaps: Vec<Snapshot>,
    virtual_answers: HashMap<String, VecDeque<u32>>,
    virtual_calls: Vec<VirtualCall>,
    image_base: u32,
}

impl Script {
    fn new() -> Self {
        Self {
            callees: HashMap::new(),
            numbered: Vec::new(),
            snaps: Vec::new(),
            virtual_answers: HashMap::new(),
            virtual_calls: Vec::new(),
            image_base: 0,
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

/// Installs the callee script, clearing every log. The image base keeps
/// its last value; set it with [`set_image_base`].
pub fn set_script(spec: &[Spec]) {
    let mut s = script();
    let base = s.image_base;
    *s = Script::new();
    s.image_base = base;
    for sp in spec {
        s.callees.insert(
            sp.id,
            CalleeScript {
                kind: sp.kind,
                answers: sp.answers.iter().copied().collect(),
                snaps: sp.snaps.clone(),
                outs: sp.outs.iter().cloned().collect(),
            },
        );
    }
}

/// Sets the image base [`relocated`] adds file values to.
pub fn set_image_base(base: u32) {
    script().image_base = base;
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

/// Takes the recorded snapshots, clearing the log.
pub fn take_snaps() -> Vec<Snapshot> {
    std::mem::take(&mut script().snaps)
}

/// Takes the recorded virtual calls, clearing the log.
pub fn take_virtual() -> Vec<VirtualCall> {
    std::mem::take(&mut script().virtual_calls)
}

/// Records a numbered call, snapshots its pointer arguments, performs its
/// scripted out writes, and pops its answer.
fn record_numbered(id: u32, args: Vec<u32>) -> u32 {
    let mut s = script();
    s.numbered.push((id, args.clone()));
    let callee = s.callees.get_mut(&id);
    let mut pending_snaps = Vec::new();
    let mut pending_outs = Vec::new();
    let mut answer = 0;
    if let Some(c) = callee {
        for (idx, n) in &c.snaps {
            pending_snaps.push((*idx, *n));
        }
        if let Some(one) = c.outs.pop_front() {
            pending_outs = one;
        }
        answer = c.answers.pop_front().unwrap_or(0);
    }
    drop(s);
    // Pointer work happens outside the script lock: every address comes
    // from memory the running case built.
    let mut snaps = Vec::new();
    for (idx, n) in pending_snaps {
        let ptr = args[idx] as *const u32;
        let mut words = Vec::with_capacity(n);
        for k in 0..n {
            words.push(unsafe { ptr.add(k).read_unaligned() });
        }
        snaps.push((id, idx, words));
    }
    for (idx, words) in &pending_outs {
        let ptr = args[*idx] as *mut u32;
        for (k, w) in words.iter().enumerate() {
            unsafe {
                ptr.add(k).write_unaligned(*w);
            }
        }
    }
    if !snaps.is_empty() {
        script().snaps.extend(snaps);
    }
    answer
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

macro_rules! stub_thiscall {
    ($name:ident) => {
        extern "thiscall" fn $name(this: u32) -> u32 {
            record_numbered(CURRENT_ID.with(|c| c.get()), vec![this])
        }
    };
    ($name:ident, $( $a:ident ),+) => {
        extern "thiscall" fn $name(this: u32, $( $a: u32 ),+) -> u32 {
            record_numbered(CURRENT_ID.with(|c| c.get()), vec![this, $( $a ),+])
        }
    };
}

stub_thiscall!(stub_t0);
stub_thiscall!(stub_t1, a0);
stub_thiscall!(stub_t2, a0, a1);
stub_thiscall!(stub_t3, a0, a1, a2);
stub_thiscall!(stub_t4, a0, a1, a2, a3);
stub_thiscall!(stub_t5, a0, a1, a2, a3, a4);
stub_thiscall!(stub_t6, a0, a1, a2, a3, a4, a5);
stub_thiscall!(stub_t7, a0, a1, a2, a3, a4, a5, a6);

extern "cdecl" fn stub_c3(a0: u32, a1: u32, a2: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a0, a1, a2])
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
    let kind = script()
        .callees
        .get(&id)
        .unwrap_or_else(|| panic!("no stub installed for callee {id}"))
        .kind;
    // The id travels to the stub out of band: `callee_addr` runs at the
    // call site, immediately before the indirect call through its answer,
    // with the script lock held, so no other call can intervene.
    CURRENT_ID.with(|c| c.set(id));
    match kind {
        StubKind::Thiscall0 => stub_t0 as *const () as usize as u32,
        StubKind::Thiscall1 => stub_t1 as *const () as usize as u32,
        StubKind::Thiscall2 => stub_t2 as *const () as usize as u32,
        StubKind::Thiscall3 => stub_t3 as *const () as usize as u32,
        StubKind::Thiscall4 => stub_t4 as *const () as usize as u32,
        StubKind::Thiscall5 => stub_t5 as *const () as usize as u32,
        StubKind::Thiscall6 => stub_t6 as *const () as usize as u32,
        StubKind::Thiscall7 => stub_t7 as *const () as usize as u32,
        StubKind::Cdecl3 => stub_c3 as *const () as usize as u32,
    }
}

/// The cookie cell the completion entry checks through its global.
pub static mut COOKIE_CELL: u32 = 0;
/// The setup flag cell the advance entry tests (low byte).
pub static mut FLAG_CELL: u32 = 0;
/// The four setup key cells the advance entry hands to key resolution.
pub static mut KEY_CELLS: [u32; 4] = [0; 4];

/// Writes the global cells a case installed. Call under the script lock.
pub fn set_cells(cookie: u32, flag: u32, key: [u32; 4]) {
    unsafe {
        COOKIE_CELL = cookie;
        FLAG_CELL = flag;
        KEY_CELLS = key;
    }
}

/// File VA to relocated address: the image base plus the file value.
/// Only the tag and table values the proof set reads are mapped.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x00FE_3504 | 0x00FE_341C | 0x019F_3230 => script().image_base.wrapping_add(file_va),
        _ => panic!("unexpected relocated VA {file_va:#x}"),
    }
}

/// Pointer to the shared global at a file VA: the cookie word, the setup
/// flag word, or one setup key word.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    match file_va {
        0x0105_7FB4 => core::ptr::addr_of_mut!(COOKIE_CELL) as *mut T,
        0x019F_32AC => core::ptr::addr_of_mut!(FLAG_CELL) as *mut T,
        0x019F_3278 => core::ptr::addr_of_mut!(KEY_CELLS) as *mut T,
        0x019F_327C => unsafe { core::ptr::addr_of_mut!(KEY_CELLS).add(1) as *mut T },
        0x019F_3280 => unsafe { core::ptr::addr_of_mut!(KEY_CELLS).add(2) as *mut T },
        0x019F_3284 => unsafe { core::ptr::addr_of_mut!(KEY_CELLS).add(3) as *mut T },
        _ => panic!("unexpected global VA {file_va:#x}"),
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
        // The block is redundant where the call site already sits in
        // unsafe code (all proof files do); the allow keeps the
        // expansion warning-free either way.
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
