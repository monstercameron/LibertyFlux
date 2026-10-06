//! The 32-bit differential runtime: checker macros plus scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Numbered callees dispatch through a script the test
//! installs: each id maps to a stub of the matching convention and
//! arity, which records its arguments and answers from a queue. Two
//! callees also write words through a pointer argument (the placement
//! matrix, the pushed corner); their words come from a second queue.
//! Virtual-slot calls land on test-owned stubs that record through
//! [`record_virtual`] and answer from [`virtual_answer`]. Everything is
//! guarded by one mutex; tests hold [`script_lock`] while a rewrite
//! runs, so parallel test threads cannot interleave scripts.
//!
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs, and four
// legacy global cells stand in for the shared words. Every access stays
// inside memory the running case built.
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
    /// `extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32`.
    Thiscall5,
    /// `extern "cdecl" fn(u32, u32, u32) -> u32`.
    Cdecl3,
    /// `extern "cdecl" fn(u32) -> u32`.
    Cdecl1,
    /// `extern "cdecl" fn() -> u32`.
    Cdecl0,
    /// `extern "thiscall" fn(u32 nine times) -> u32`.
    Thiscall9,
    /// `extern "thiscall" fn(u32 eleven times) -> u32`.
    Thiscall11,
    /// The pose-submit callee: cdecl with nine words, snapshotting four
    /// words through each of its second, third and fourth arguments.
    SubmitSnap,
    /// The placement-matrix callee: thiscall shape, writes sixteen queued
    /// words through its second argument.
    MatrixWrite,
    /// The corner-push callee: cdecl shape, reads three words through its
    /// third argument into the corner log, writes three queued words
    /// through its first.
    CornerWrite,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    writes: HashMap<u32, VecDeque<Vec<u32>>>,
    numbered: Vec<NumberedCall>,
    corners: Vec<[u32; 3]>,
    virtual_answers: HashMap<String, VecDeque<u32>>,
    virtual_calls: Vec<VirtualCall>,
    relocs: HashMap<u32, u32>,
    snaps: Vec<Vec<u32>>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            writes: HashMap::new(),
            numbered: Vec::new(),
            corners: Vec::new(),
            virtual_answers: HashMap::new(),
            virtual_calls: Vec::new(),
            relocs: HashMap::new(),
            snaps: Vec::new(),
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
/// Clears both call logs, the write queues and the corner log.
pub fn set_script(spec: &[(u32, StubKind, Vec<u32>)]) {
    let mut s = script();
    *s = Script::new();
    for (id, kind, answers) in spec {
        s.kinds.insert(*id, *kind);
        s.answers.insert(*id, answers.iter().copied().collect());
    }
}

/// Queues the word blocks a writing callee stores per call, in call
/// order. A call with no block queued is a case bug and panics.
pub fn set_writes(id: u32, blocks: Vec<Vec<u32>>) {
    script()
        .writes
        .insert(id, blocks.into_iter().collect());
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

/// Takes the recorded corner inputs, clearing the log.
pub fn take_corners() -> Vec<[u32; 3]> {
    std::mem::take(&mut script().corners)
}

/// Takes the recorded virtual calls, clearing the log.
pub fn take_virtual() -> Vec<VirtualCall> {
    std::mem::take(&mut script().virtual_calls)
}

/// Installs the relocated-address map: file VA to test address. Only
/// mapped VAs answer; anything else panics.
pub fn set_relocated(pairs: &[(u32, u32)]) {
    script().relocs = pairs.iter().copied().collect();
}

/// Takes the recorded submit snapshots, clearing the log.
pub fn take_snaps() -> Vec<Vec<u32>> {
    std::mem::take(&mut script().snaps)
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

/// Pops the next write block for a writing callee.
fn pop_block(id: u32) -> Vec<u32> {
    script()
        .writes
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or_else(|| panic!("no write block queued for callee {id}"))
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

extern "thiscall" fn stub_thiscall5(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d, e])
}

extern "cdecl" fn stub_cdecl3(a: u32, b: u32, c: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c])
}

extern "cdecl" fn stub_cdecl1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

extern "cdecl" fn stub_cdecl0() -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![])
}

extern "thiscall" fn stub_thiscall9(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
    i: u32,
) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d, e, f, g, h, i])
}

extern "thiscall" fn stub_thiscall11(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
    i: u32,
    j: u32,
    k: u32,
) -> u32 {
    record_numbered(
        CURRENT_ID.with(|c| c.get()),
        vec![a, b, c, d, e, f, g, h, i, j, k],
    )
}

extern "cdecl" fn stub_submit(
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let mut snap = Vec::with_capacity(12);
    for bp in [a1, a2, a3] {
        for k in 0..4 {
            snap.push(unsafe { ((bp as *const u32).add(k)).read_unaligned() });
        }
    }
    script().snaps.push(snap);
    record_numbered(id, vec![a0, a1, a2, a3, a4, a5, a6, a7, a8])
}

extern "thiscall" fn stub_matrix(obj: u32, buf: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let block = pop_block(id);
    assert_eq!(block.len(), 16, "matrix block must hold sixteen words");
    for (i, w) in block.iter().enumerate() {
        unsafe {
            ((buf as *mut u32).add(i)).write_unaligned(*w);
        }
    }
    record_numbered(id, vec![obj, buf])
}

extern "cdecl" fn stub_corner(out: u32, obj: u32, vec: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let corner = unsafe {
        [
            ((vec as *const u32).read_unaligned()),
            ((vec as *const u32).add(1).read_unaligned()),
            ((vec as *const u32).add(2).read_unaligned()),
        ]
    };
    script().corners.push(corner);
    let block = pop_block(id);
    assert_eq!(block.len(), 3, "corner block must hold three words");
    for (i, w) in block.iter().enumerate() {
        unsafe {
            ((out as *mut u32).add(i)).write_unaligned(*w);
        }
    }
    record_numbered(id, vec![out, obj, vec])
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
        StubKind::Thiscall5 => stub_thiscall5 as *const () as usize as u32,
        StubKind::Cdecl3 => stub_cdecl3 as *const () as usize as u32,
        StubKind::Cdecl1 => stub_cdecl1 as *const () as usize as u32,
        StubKind::Cdecl0 => stub_cdecl0 as *const () as usize as u32,
        StubKind::Thiscall9 => stub_thiscall9 as *const () as usize as u32,
        StubKind::Thiscall11 => stub_thiscall11 as *const () as usize as u32,
        StubKind::SubmitSnap => stub_submit as *const () as usize as u32,
        StubKind::MatrixWrite => stub_matrix as *const () as usize as u32,
        StubKind::CornerWrite => stub_corner as *const () as usize as u32,
    }
}

/// The four shared words the box computation reads: three scales and the
/// mask. Written by the running case under the script lock.
pub static mut SCALE_X: u32 = 0;
/// The second shared scale word (see [`SCALE_X`]).
pub static mut SCALE_Y: u32 = 0;
/// The third shared scale word (see [`SCALE_X`]).
pub static mut SCALE_Z: u32 = 0;
/// The shared mask word (see [`SCALE_X`]).
pub static mut ABS_MASK: u32 = 0;

/// The thirty shared words the per-frame update reads (four of them
/// written back). Written by the running case under the script lock, in
/// slot order: entry sequence, the table slot (unused: the table lives in
/// [`UTABLE`]), the four maze counters, the float window triple, the
/// setup flag and value, the accumulator flag and triple, the blend
/// factors, and the submit factors.
pub static mut UG: [u32; 30] = [0; 30];

/// The update's four-entry table: the global at the table VA is indexed
/// in place, so it needs array backing, not one value cell. Written by
/// the running case under the script lock.
pub static mut UTABLE: [u32; 4] = [0; 4];

/// Slot of an update shared word, if it is one of the thirty.
fn update_slot(file_va: u32) -> Option<usize> {
    match file_va {
        0x0159_3310 => Some(0),
        0x0129_5854 => Some(2),
        0x0129_5848 => Some(3),
        0x0129_5858 => Some(4),
        0x0129_584C => Some(5),
        0x012D_DEA0 => Some(6),
        0x012D_DEAC => Some(7),
        0x00E9_D0D8 => Some(8),
        0x0129_577C => Some(9),
        0x012E_22A8 => Some(10),
        0x016D_CEB0 => Some(11),
        0x016D_CEA0 => Some(12),
        0x016D_CEA4 => Some(13),
        0x016D_CEA8 => Some(14),
        0x0104_9694 => Some(15),
        0x00FE_88E8 => Some(16),
        0x0104_9698 => Some(17),
        0x00FE_8628 => Some(18),
        0x00FE_8830 => Some(19),
        0x00FE_8A24 => Some(20),
        0x0104_969C => Some(21),
        0x0104_96A0 => Some(22),
        0x0104_96A4 => Some(23),
        0x0104_96A8 => Some(24),
        0x0104_96AC => Some(25),
        0x0104_96B0 => Some(26),
        0x0104_96B4 => Some(27),
        0x016D_CEB4 => Some(28),
        0x00FE_87E4 => Some(29),
        _ => None,
    }
}

/// Pointer to the shared word at a file VA, mirroring
/// `lf-checker-rt::global`. Only the words the proof set reads are mapped.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    match file_va {
        0x0110_DB60 => core::ptr::addr_of_mut!(SCALE_X) as *mut T,
        0x0110_DB64 => core::ptr::addr_of_mut!(SCALE_Y) as *mut T,
        0x0110_DB68 => core::ptr::addr_of_mut!(SCALE_Z) as *mut T,
        0x00FE_8F80 => core::ptr::addr_of_mut!(ABS_MASK) as *mut T,
        0x0129_5CD8 => unsafe { core::ptr::addr_of_mut!(UTABLE) as *mut T },
        va => match update_slot(va) {
            Some(i) => unsafe { core::ptr::addr_of_mut!(UG[i]) as *mut T },
            None => panic!("unexpected shared word VA {file_va:#x}"),
        },
    }
}

/// File VA to relocated address, by the installed map (see
/// [`set_relocated`]).
///
/// # Panics
///
/// For an unmapped VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    *script()
        .relocs
        .get(&file_va)
        .unwrap_or_else(|| panic!("unexpected relocated VA {file_va:#x}"))
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
