//! The 32-bit differential runtime: checker macros plus scripted callees.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Numbered callees dispatch through a script the test
//! installs: each id maps to a stub of the matching convention and
//! arity, which records its arguments, snapshots any buffers it is
//! handed, applies scripted memory effects, and answers from a queue.
//! Virtual-slot calls land on test-owned stubs that record through
//! [`record_virtual`] and answer from [`virtual_answer`]; the token
//! fetcher pops its scripts from [`pop_fetch`]. The streaming globals
//! map to test-owned tables through [`set_globals`]. Everything is
//! guarded by one mutex; tests hold [`script_lock`] while a rewrite
//! runs, so parallel test threads cannot interleave scripts.
//!
//! Test-support code: the lifted crate itself stays
//! `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through scripted stubs. Every
// access stays inside memory the running case built.
#![allow(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

/// A recorded numbered-callee call: callee id, argument words, and the
/// byte snapshots the stub took (in argument order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberedCall {
    /// Callee id.
    pub id: u32,
    /// Argument words (frame-cell addresses are dropped by effect stubs).
    pub args: Vec<u32>,
    /// Snapshotted buffers.
    pub snaps: Vec<Vec<u8>>,
}

/// A recorded virtual-slot call: stub name, argument words, snapshots.
pub type VirtualCall = (String, Vec<u32>, Vec<Vec<u8>>);

/// Which stub shape serves a callee id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StubKind {
    /// `extern "thiscall" fn(u32) -> u32`, record only.
    Thiscall1,
    /// `extern "thiscall" fn(u32, u32) -> u32`, record only.
    Thiscall2,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`, record only.
    Thiscall3,
    /// The stream refill: writes the scripted byte to the cell, answers
    /// the scripted count.
    Refill,
    /// `extern "cdecl" fn(u32) -> u32` snapshotting a NUL-terminated
    /// string at its argument (the integer parser).
    CdeclStr,
    /// `extern "cdecl" fn(u32) -> f64` snapshotting a NUL-terminated
    /// string at its argument (the float parser).
    CdeclF64Str,
    /// `extern "cdecl" fn(u32, u32) -> u32` snapshotting both strings
    /// (the string comparer).
    CdeclStrStr,
    /// `extern "cdecl" fn(u32, u32, u32) -> u32` writing scripted bytes
    /// to the buffer (the integer formatter).
    CdeclFormat,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32` snapshotting the
    /// third argument's bytes at the second (the block writers).
    ThiscallMem,
}

/// One refill/formatter script: the bytes the stub writes and the answer
/// it returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteScript {
    /// Bytes written to the stub's buffer argument.
    pub bytes: Vec<u8>,
    /// The answer returned.
    pub ret: u32,
}

/// One token-fetch script: the length reported and the bytes left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchScript {
    /// The reported length.
    pub len: u32,
    /// The bytes left in the buffer.
    pub bytes: Vec<u8>,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, StubKind>,
    answers: HashMap<u32, VecDeque<u32>>,
    answers64: HashMap<u32, VecDeque<u64>>,
    byte_scripts: HashMap<u32, VecDeque<ByteScript>>,
    numbered: Vec<NumberedCall>,
    virtual_answers: HashMap<String, VecDeque<u32>>,
    virtual_calls: Vec<VirtualCall>,
    fetch_scripts: VecDeque<FetchScript>,
    globals: HashMap<u32, u32>,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            answers64: HashMap::new(),
            byte_scripts: HashMap::new(),
            numbered: Vec::new(),
            virtual_answers: HashMap::new(),
            virtual_calls: Vec::new(),
            fetch_scripts: VecDeque::new(),
            globals: HashMap::new(),
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
/// Clears both call logs, the fetch scripts and the globals.
pub fn set_script(spec: &[(u32, StubKind, Vec<u32>)]) {
    let mut s = script();
    *s = Script::new();
    for (id, kind, answers) in spec {
        s.kinds.insert(*id, *kind);
        s.answers.insert(*id, answers.iter().copied().collect());
    }
}

/// Installs 64-bit answers (float-parser bits) for a callee id.
pub fn set_answers64(id: u32, answers: Vec<u64>) {
    script().answers64.insert(id, answers.into_iter().collect());
}

/// Installs byte scripts (refill/formatter) for a callee id.
pub fn set_byte_scripts(id: u32, scripts: Vec<ByteScript>) {
    script()
        .byte_scripts
        .insert(id, scripts.into_iter().collect());
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

/// Queues token-fetch scripts (popped in call order).
pub fn push_fetch(scripts: Vec<FetchScript>) {
    script().fetch_scripts.extend(scripts);
}

/// Maps relocated file VAs to test-owned addresses.
pub fn set_globals(spec: &[(u32, u32)]) {
    let mut s = script();
    for (va, addr) in spec {
        s.globals.insert(*va, *addr);
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
fn record_numbered(id: u32, args: Vec<u32>, snaps: Vec<Vec<u8>>) -> u32 {
    let mut s = script();
    s.numbered.push(NumberedCall { id, args, snaps });
    s.answers
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

/// Records a virtual-slot call.
pub fn record_virtual(name: &str, args: Vec<u32>, snaps: Vec<Vec<u8>>) {
    script().virtual_calls.push((name.to_string(), args, snaps));
}

/// Pops a virtual-slot answer.
pub fn virtual_answer(name: &str) -> u32 {
    script()
        .virtual_answers
        .get_mut(name)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

/// Pops the next token-fetch script.
///
/// # Panics
///
/// When no script is queued: a case bug, never a guess.
pub fn pop_fetch() -> FetchScript {
    script()
        .fetch_scripts
        .pop_front()
        .expect("fetch stub called with no script queued")
}

/// Snapshots a NUL-terminated string, bounded.
fn snap_cstr(ptr: u32, bound: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..bound {
        let byte = unsafe { ((ptr.wrapping_add(i as u32)) as *const u8).read() };
        if byte == 0 {
            break;
        }
        out.push(byte);
    }
    out
}

/// Snapshots `len` bytes.
fn snap_bytes(ptr: u32, len: u32) -> Vec<u8> {
    let mut out = vec![0u8; len as usize];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = unsafe { ((ptr.wrapping_add(i as u32)) as *const u8).read() };
    }
    out
}

extern "thiscall" fn stub_thiscall1(a: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    record_numbered(id, vec![a], Vec::new())
}

extern "thiscall" fn stub_thiscall2(a: u32, b: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    record_numbered(id, vec![a, b], Vec::new())
}

extern "thiscall" fn stub_thiscall3(a: u32, b: u32, c: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    record_numbered(id, vec![a, b, c], Vec::new())
}

extern "thiscall" fn stub_refill(stream: u32, cell: u32, n: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let item = script()
        .byte_scripts
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .expect("refill stub called with no script queued");
    let byte = item.bytes.first().copied().unwrap_or(0);
    unsafe { (cell as *mut u8).write(byte) };
    record_numbered(id, vec![stream, n], vec![vec![byte]]);
    item.ret
}

extern "cdecl" fn stub_cdecl_str(ptr: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let snap = snap_cstr(ptr, 0x100);
    record_numbered(id, Vec::new(), vec![snap])
}

extern "cdecl" fn stub_cdecl_f64str(ptr: u32) -> f64 {
    let id = CURRENT_ID.with(|c| c.get());
    let snap = snap_cstr(ptr, 0x100);
    let bits = script()
        .answers64
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0);
    record_numbered(id, Vec::new(), vec![snap]);
    f64::from_bits(bits)
}

extern "cdecl" fn stub_cdecl_strstr(a: u32, b: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let snaps = vec![snap_cstr(a, 0x100), snap_cstr(b, 0x100)];
    record_numbered(id, Vec::new(), snaps)
}

extern "cdecl" fn stub_cdecl_format(buf: u32, _fmt: u32, value: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let item = script()
        .byte_scripts
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .expect("format stub called with no script queued");
    for (i, byte) in item.bytes.iter().enumerate() {
        unsafe { ((buf.wrapping_add(i as u32)) as *mut u8).write(*byte) };
    }
    record_numbered(id, vec![value], vec![item.bytes.clone()]);
    item.ret
}

extern "thiscall" fn stub_thiscall_mem(obj: u32, ptr: u32, len: u32) -> u32 {
    let id = CURRENT_ID.with(|c| c.get());
    let snap = snap_bytes(ptr, len);
    record_numbered(id, vec![obj, len], vec![snap])
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
        StubKind::Refill => stub_refill as *const () as usize as u32,
        StubKind::CdeclStr => stub_cdecl_str as *const () as usize as u32,
        StubKind::CdeclF64Str => stub_cdecl_f64str as *const () as usize as u32,
        StubKind::CdeclStrStr => stub_cdecl_strstr as *const () as usize as u32,
        StubKind::CdeclFormat => stub_cdecl_format as *const () as usize as u32,
        StubKind::ThiscallMem => stub_thiscall_mem as *const () as usize as u32,
    }
}

/// File VA to relocated address, through the test-installed globals.
///
/// # Panics
///
/// For any unmapped VA: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    script()
        .globals
        .get(&file_va)
        .copied()
        .unwrap_or_else(|| panic!("unexpected relocated VA {file_va:#x}"))
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
