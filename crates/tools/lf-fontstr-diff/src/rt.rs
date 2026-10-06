//! The 32-bit differential runtime: checker macros plus scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Numbered callees dispatch through a script the test
//! installs: each id maps to a stub of the matching convention and
//! arity, which records its arguments and answers from a queue. Stubs
//! whose calls carry pointers also snapshot the words behind them
//! while the caller's frame is alive (the scratch buffers die with the
//! call); the converter stub writes its scripted words instead.
//! Virtual-slot calls land on test-owned stubs that record through
//! [`record_virtual`] and answer from [`virtual_answer`]. Shared
//! globals answer from cells the test writes per case. Everything is
//! guarded by one mutex; tests hold [`script_lock`] while a rewrite
//! runs, so parallel test threads cannot interleave scripts.
//!
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs, and the
// shared globals are plain cells. Every access stays inside memory the
// running case built.
#![allow(unsafe_code)]

use std::cell::Cell;
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
    /// `extern "cdecl" fn() -> u32`.
    Cdecl0,
    /// `extern "cdecl" fn() -> f32`.
    Cdecl0F32,
    /// `extern "cdecl" fn(u32) -> u32`.
    Cdecl1,
    /// `extern "cdecl" fn(u32, u32) -> u32`.
    Cdecl2,
    /// `extern "cdecl" fn(u32, u32) -> u32`, snapshotting two words
    /// behind the first argument (the zeroed registry scratch).
    Cdecl2Snap2,
    /// `extern "cdecl" fn(u32, u32) -> f32`, snapshotting eight words
    /// behind the first argument (the height query).
    Cdecl2F32Snap8,
    /// `extern "cdecl" fn(u32, u32) -> u32`, writing eight scripted
    /// words through the second argument (the text converter).
    Cdecl2Write8,
    /// `extern "cdecl" fn(u32, u32, u32) -> u32`.
    Cdecl3,
    /// `extern "cdecl" fn(u32, u32, u32, u32) -> u32`, snapshotting four
    /// words behind the third argument (the position notify).
    Cdecl4Snap,
    /// `extern "cdecl" fn(u32, u32, u32, u32, u32) -> u32`, snapshotting
    /// eight words behind the third argument (the row submission).
    Cdecl5Snap8,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    out_words: HashMap<u32, VecDeque<[u32; 8]>>,
    numbered: Vec<NumberedCall>,
    snaps: Vec<Vec<u32>>,
    virtual_answers: HashMap<String, VecDeque<u32>>,
    virtual_calls: Vec<VirtualCall>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            out_words: HashMap::new(),
            numbered: Vec::new(),
            snaps: Vec::new(),
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
/// Clears all call logs.
pub fn set_script(spec: &[(u32, StubKind, Vec<u32>)]) {
    let mut s = script();
    *s = Script::new();
    for (id, kind, answers) in spec {
        s.kinds.insert(*id, *kind);
        s.answers.insert(*id, answers.iter().copied().collect());
    }
}

/// Installs the converter's out-word groups by callee id (popped in
/// call order; exhausted queues write zeros).
pub fn set_out_words(spec: &[(u32, Vec<[u32; 8]>)]) {
    let mut s = script();
    for (id, groups) in spec {
        s.out_words.insert(*id, groups.iter().copied().collect());
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

/// Takes the recorded pointer snapshots, clearing the log. One entry
/// per snap/write stub call, in call order.
pub fn take_snaps() -> Vec<Vec<u32>> {
    std::mem::take(&mut script().snaps)
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

/// Records a pointer snapshot alongside the numbered call.
fn record_snap(words: Vec<u32>) {
    script().snaps.push(words);
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

fn read_words(ptr: u32, n: usize) -> Vec<u32> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(unsafe { ((ptr.wrapping_add(i as u32 * 4)) as *const u32).read_unaligned() });
    }
    out
}

extern "thiscall" fn stub_thiscall1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

extern "thiscall" fn stub_thiscall2(a: u32, b: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b])
}

extern "cdecl" fn stub_cdecl0() -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![])
}

extern "cdecl" fn stub_cdecl0_f32() -> f32 {
    f32::from_bits(record_numbered(CURRENT_ID.with(|c| c.get()), vec![]))
}

extern "cdecl" fn stub_cdecl1(a: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a])
}

extern "cdecl" fn stub_cdecl2(a: u32, b: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b])
}

extern "cdecl" fn stub_cdecl2_snap2(a: u32, b: u32) -> u32 {
    let snap = read_words(a, 2);
    let ans = record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b]);
    record_snap(snap);
    ans
}

extern "cdecl" fn stub_cdecl2_f32_snap8(a: u32, b: u32) -> f32 {
    let snap = read_words(a, 8);
    let ans = record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b]);
    record_snap(snap);
    f32::from_bits(ans)
}

extern "cdecl" fn stub_cdecl2_write8(a: u32, b: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let group = script()
        .out_words
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or([0; 8]);
    for (i, w) in group.iter().enumerate() {
        unsafe { ((b.wrapping_add(i as u32 * 4)) as *mut u32).write_unaligned(*w) };
    }
    let ans = record_numbered(id, vec![a, b]);
    record_snap(group.to_vec());
    ans
}

extern "cdecl" fn stub_cdecl3(a: u32, b: u32, c: u32) -> u32 {
    record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c])
}

extern "cdecl" fn stub_cdecl4_snap(a: u32, b: u32, c: u32, d: u32) -> u32 {
    let snap = read_words(c, 4);
    let ans = record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d]);
    record_snap(snap);
    ans
}

extern "cdecl" fn stub_cdecl5_snap8(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let snap = read_words(c, 8);
    let ans = record_numbered(CURRENT_ID.with(|c| c.get()), vec![a, b, c, d, e]);
    record_snap(snap);
    ans
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
        StubKind::Cdecl0 => stub_cdecl0 as *const () as usize as u32,
        StubKind::Cdecl0F32 => stub_cdecl0_f32 as *const () as usize as u32,
        StubKind::Cdecl1 => stub_cdecl1 as *const () as usize as u32,
        StubKind::Cdecl2 => stub_cdecl2 as *const () as usize as u32,
        StubKind::Cdecl2Snap2 => stub_cdecl2_snap2 as *const () as usize as u32,
        StubKind::Cdecl2F32Snap8 => stub_cdecl2_f32_snap8 as *const () as usize as u32,
        StubKind::Cdecl2Write8 => stub_cdecl2_write8 as *const () as usize as u32,
        StubKind::Cdecl3 => stub_cdecl3 as *const () as usize as u32,
        StubKind::Cdecl4Snap => stub_cdecl4_snap as *const () as usize as u32,
        StubKind::Cdecl5Snap8 => stub_cdecl5_snap8 as *const () as usize as u32,
    }
}

// Shared-global cells, one per file VA the proof set reads. Written by
// the running case under the script lock.
static mut CELL_FE8830: u32 = 0;
static mut CELL_576F8: u32 = 0;
static mut CELL_576FC: u32 = 0;
static mut CELL_5C880: u32 = 0;
static mut CELL_5C87C: u32 = 0;
static mut CELL_5C884: u32 = 0;
static mut CELL_5C888: u32 = 0;
static mut CELL_6A668C: u32 = 0;
static mut CELL_7A65A8: u32 = 0;
static mut CELL_7A65AC: u32 = 0;
static mut CELL_UIMODE: u32 = 0;
static mut CELL_UIALT: u32 = 0;
/// The planted shared-cache object address behind `relocated`.
static mut CACHE_ADDR: u32 = 0;

/// Writes a shared-global cell. Only the VAs the proof set reads are
/// mapped; anything else is a case bug.
///
/// # Panics
///
/// When the VA is not one the proof set reads.
pub fn set_global(file_va: u32, value: u32) {
    unsafe {
        let cell = match file_va {
            0x00FE_8830 => core::ptr::addr_of_mut!(CELL_FE8830),
            0x0105_76F8 => core::ptr::addr_of_mut!(CELL_576F8),
            0x0105_76FC => core::ptr::addr_of_mut!(CELL_576FC),
            0x0105_C880 => core::ptr::addr_of_mut!(CELL_5C880),
            0x0105_C87C => core::ptr::addr_of_mut!(CELL_5C87C),
            0x0105_C884 => core::ptr::addr_of_mut!(CELL_5C884),
            0x0105_C888 => core::ptr::addr_of_mut!(CELL_5C888),
            0x017A_668C => core::ptr::addr_of_mut!(CELL_6A668C),
            0x017A_65A8 => core::ptr::addr_of_mut!(CELL_7A65A8),
            0x017A_65AC => core::ptr::addr_of_mut!(CELL_7A65AC),
            0x0116_C250 => core::ptr::addr_of_mut!(CELL_UIMODE),
            0x0116_C253 => core::ptr::addr_of_mut!(CELL_UIALT),
            _ => panic!("unexpected shared global VA {file_va:#x}"),
        };
        cell.write(value);
    }
}

/// Plants the shared-cache object address behind `relocated`.
pub fn set_cache_addr(addr: u32) {
    unsafe { core::ptr::addr_of_mut!(CACHE_ADDR).write(addr) };
}

/// Pointer to the shared global at a file VA, mirroring
/// `lf-checker-rt::global`. Byte reads take the cell's low byte.
///
/// # Panics
///
/// When the address is not one of the globals the proof set reads: a
/// case bug, never a guess.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let word: *mut u32 = match file_va {
        0x00FE_8830 => unsafe { core::ptr::addr_of_mut!(CELL_FE8830) },
        0x0105_76F8 => unsafe { core::ptr::addr_of_mut!(CELL_576F8) },
        0x0105_76FC => unsafe { core::ptr::addr_of_mut!(CELL_576FC) },
        0x0105_C880 => unsafe { core::ptr::addr_of_mut!(CELL_5C880) },
        0x0105_C87C => unsafe { core::ptr::addr_of_mut!(CELL_5C87C) },
        0x0105_C884 => unsafe { core::ptr::addr_of_mut!(CELL_5C884) },
        0x0105_C888 => unsafe { core::ptr::addr_of_mut!(CELL_5C888) },
        0x017A_668C => unsafe { core::ptr::addr_of_mut!(CELL_6A668C) },
        0x017A_65A8 => unsafe { core::ptr::addr_of_mut!(CELL_7A65A8) },
        0x017A_65AC => unsafe { core::ptr::addr_of_mut!(CELL_7A65AC) },
        0x0116_C250 => unsafe { core::ptr::addr_of_mut!(CELL_UIMODE) },
        0x0116_C253 => unsafe { core::ptr::addr_of_mut!(CELL_UIALT) },
        _ => panic!("unexpected shared global VA {file_va:#x}"),
    };
    word as *mut T
}

/// File VA to relocated address. Only the shared text cache the proof
/// set reads is mapped, to the planted address.
///
/// # Panics
///
/// For any other VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    match file_va {
        0x0116_BFF0 => unsafe { core::ptr::addr_of!(CACHE_ADDR).read() },
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
