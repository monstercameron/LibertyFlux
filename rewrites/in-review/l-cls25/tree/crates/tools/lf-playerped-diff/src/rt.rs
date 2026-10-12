//! The 32-bit differential runtime for the player pedestrian rewrites.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `global`). Numbered
//! callees dispatch through a script the test installs: each id maps to a
//! stub of the matching convention and arity, which records its arguments
//! and answers from a queue. Virtual slot calls land on test-owned stubs
//! that record through the same log. A scripted effect, when one is set,
//! changes one word of the pedestrian each time its callee runs: the
//! rewrite side writes the word in memory, and the lifted side's test
//! world writes the same word into the lifted fields, so a callee that
//! changes the pedestrian does so the same way on both sides.
//!
//! Everything is guarded by one mutex; tests hold [`lock`] while a rewrite
//! or a lifted method runs, so parallel test threads cannot interleave
//! scripts.
//!
//! Test-support code: the lifted crate itself forbids `unsafe`. Raw
//! addresses flow through scripted stubs and the pedestrian image the test
//! built; every access stays inside memory the running case built.

// Test-only runtime: raw addresses flow through scripted stubs, and the
// rewrites read and write the pedestrian image and sub-object records the
// case built. Every access stays inside that memory.
#![allow(unsafe_code)]

use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex, MutexGuard};

/// A recorded call: the callee as `#id`, with its argument words.
pub type Call = (String, Vec<u32>);

/// The stub shape a numbered callee id is served by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `extern "thiscall" fn(u32) -> u32`.
    T1,
    /// `extern "thiscall" fn(u32, u32) -> u32`.
    T2,
    /// `extern "thiscall" fn(u32, u32, u32) -> u32`.
    T3,
    /// `extern "thiscall" fn(u32, u32, u32, u32) -> u32`.
    T4,
    /// `extern "thiscall" fn(u32 x 6) -> u32`.
    T6,
    /// `extern "thiscall" fn(u32 x 7) -> u32`.
    T7,
    /// `extern "cdecl" fn() -> u32`.
    C0,
    /// `extern "cdecl" fn(u32) -> u32`.
    C1,
    /// `extern "cdecl" fn(u32, u32) -> u32`.
    C2,
    /// `extern "cdecl" fn(u32, u32, u32) -> u32`.
    C3,
}

/// The id the virtual slot-8 stub records under.
pub const SLOT8_ID: u32 = 0x100;
/// The id the trap stubs record under: a call to any other virtual slot.
pub const TRAP_ID: u32 = 0x101;

/// One scripted write to the pedestrian, made each time a callee runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Effect {
    /// The callee id that makes the write.
    pub callee: u32,
    /// Byte offset of the word within the pedestrian image.
    pub off: u32,
    /// The value written.
    pub value: u32,
}

/// The installed script and the recorded calls.
struct Script {
    kinds: HashMap<u32, Kind>,
    answers: HashMap<u32, VecDeque<u32>>,
    calls: Vec<Call>,
    effect: Option<Effect>,
    ped: u32,
}

impl Script {
    fn new() -> Self {
        Self {
            kinds: HashMap::new(),
            answers: HashMap::new(),
            calls: Vec::new(),
            effect: None,
            ped: 0,
        }
    }
}

static SCRIPT: LazyLock<Mutex<Script>> = LazyLock::new(|| Mutex::new(Script::new()));

/// The shared word the context lookup reads (the player context global).
static PLAYER_CONTEXT_CELL: AtomicU32 = AtomicU32::new(0);

/// The shared model table: one pointer per model number.
static MODEL_TABLE: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];

/// The shared aim state word the aim acquisition reads.
static AIM_STATE_CELL: AtomicU32 = AtomicU32::new(0);

/// File address of the aim state word.
pub const AIM_STATE_VA: u32 = 0x011D_6FD4;

/// File address of the player context global.
pub const PLAYER_CONTEXT_VA: u32 = 0x018B_62DC;
/// File address of the first entry of the model table.
pub const MODEL_TABLE_VA: u32 = 0x0129_5CD8;

thread_local! {
    static CURRENT_ID: Cell<u32> = const { Cell::new(0) };
}

/// Locks the script for one run. Poison is ignored: a failed case must not
/// wedge the remaining tests.
pub fn lock() -> MutexGuard<'static, ()> {
    static GUARD: Mutex<()> = Mutex::new(());
    GUARD.lock().unwrap_or_else(|e| e.into_inner())
}

fn script() -> MutexGuard<'static, Script> {
    SCRIPT.lock().unwrap_or_else(|e| e.into_inner())
}

/// Installs one run: each callee id's stub shape and answer queue (popped in
/// call order; exhausted queues answer 0), the optional effect, and the
/// pedestrian's identity. Clears the call log.
pub fn begin(
    kinds: &[(u32, Kind)],
    answers: &[(u32, Vec<u32>)],
    effect: Option<Effect>,
    ped: u32,
) {
    let mut s = script();
    *s = Script::new();
    for (id, kind) in kinds {
        s.kinds.insert(*id, *kind);
    }
    for (id, queue) in answers {
        s.answers.insert(*id, queue.iter().copied().collect());
    }
    s.effect = effect;
    s.ped = ped;
}

/// Takes the recorded calls in order, clearing the log.
pub fn take_calls() -> Vec<Call> {
    std::mem::take(&mut script().calls)
}

/// Records one call and pops its scripted answer.
pub fn script_call(id: u32, args: Vec<u32>) -> u32 {
    let mut s = script();
    s.calls.push((format!("#{id}"), args));
    s.answers
        .get_mut(&id)
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

/// The scripted effect a callee id makes, if the installed run has one for it.
#[must_use]
pub fn effect_for(id: u32) -> Option<Effect> {
    script().effect.filter(|e| e.callee == id)
}

/// The pedestrian's identity the installed run uses.
#[must_use]
pub fn ped_identity() -> u32 {
    script().ped
}

/// Records a call from a rewrite-side stub: applies the scripted effect to
/// the pedestrian image in memory when one is due, then records the call.
fn rewrite_call(id: u32, args: Vec<u32>) -> u32 {
    if let Some(e) = effect_for(id) {
        let ped = ped_identity();
        // SAFETY: the case's pedestrian image covers the effect's offset,
        // and the identity names that image.
        unsafe { ((ped.wrapping_add(e.off)) as *mut u32).write_unaligned(e.value) };
    }
    script_call(id, args)
}

fn numbered(args: Vec<u32>) -> u32 {
    rewrite_call(CURRENT_ID.with(Cell::get), args)
}

extern "thiscall" fn stub_t1(a: u32) -> u32 {
    numbered(vec![a])
}

extern "thiscall" fn stub_t2(a: u32, b: u32) -> u32 {
    numbered(vec![a, b])
}

extern "thiscall" fn stub_t3(a: u32, b: u32, c: u32) -> u32 {
    numbered(vec![a, b, c])
}

extern "thiscall" fn stub_t4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    numbered(vec![a, b, c, d])
}

extern "thiscall" fn stub_t6(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> u32 {
    numbered(vec![a, b, c, d, e, f])
}

extern "thiscall" fn stub_t7(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    numbered(vec![a, b, c, d, e, f, g])
}

extern "cdecl" fn stub_c0() -> u32 {
    numbered(Vec::new())
}

extern "cdecl" fn stub_c1(a: u32) -> u32 {
    numbered(vec![a])
}

extern "cdecl" fn stub_c2(a: u32, b: u32) -> u32 {
    numbered(vec![a, b])
}

extern "cdecl" fn stub_c3(a: u32, b: u32, c: u32) -> u32 {
    numbered(vec![a, b, c])
}

/// The virtual slot-8 stub a sub-object's table holds.
extern "thiscall" fn stub_slot8(this: u32, arg: u32) -> u32 {
    rewrite_call(SLOT8_ID, vec![this, arg])
}

/// The stub the other virtual slots of a sub-object hold: a call to it is
/// recorded and would fail the comparison.
extern "thiscall" fn stub_trap(this: u32, arg: u32) -> u32 {
    rewrite_call(TRAP_ID, vec![this, arg])
}

/// Address of the slot-8 stub (for a test-owned table).
#[must_use]
pub fn slot8_stub_addr() -> u32 {
    stub_slot8 as *const () as usize as u32
}

/// Address of the trap stub (for the other table slots).
#[must_use]
pub fn trap_stub_addr() -> u32 {
    stub_trap as *const () as usize as u32
}

/// Installs the numbered-callee shapes a run uses; the ids not installed
/// here fault the case.
pub fn kind_for(id: u32) -> Kind {
    *script()
        .kinds
        .get(&id)
        .unwrap_or_else(|| panic!("no stub installed for callee {id}"))
}

/// Raw stub address for callee `id`, by the installed run.
///
/// # Panics
///
/// When the run installs no stub for `id`: a case bug, never a guess.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let kind = kind_for(id);
    // The id travels to the stub out of band: `callee_addr` runs at the call
    // site, immediately before the indirect call through its address.
    CURRENT_ID.with(|c| c.set(id));
    match kind {
        Kind::T1 => stub_t1 as *const () as usize as u32,
        Kind::T2 => stub_t2 as *const () as usize as u32,
        Kind::T3 => stub_t3 as *const () as usize as u32,
        Kind::T4 => stub_t4 as *const () as usize as u32,
        Kind::T6 => stub_t6 as *const () as usize as u32,
        Kind::T7 => stub_t7 as *const () as usize as u32,
        Kind::C0 => stub_c0 as *const () as usize as u32,
        Kind::C1 => stub_c1 as *const () as usize as u32,
        Kind::C2 => stub_c2 as *const () as usize as u32,
        Kind::C3 => stub_c3 as *const () as usize as u32,
    }
}

/// Sets the player context global the context lookup reads.
pub fn set_player_context(bits: u32) {
    PLAYER_CONTEXT_CELL.store(bits, Ordering::SeqCst);
}

/// The player context global, as the lifted side reads it.
#[must_use]
pub fn player_context() -> u32 {
    PLAYER_CONTEXT_CELL.load(Ordering::SeqCst)
}

/// Sets one model table entry to the address of a model record.
///
/// # Panics
///
/// When the model number is outside the eight entries the proof covers.
pub fn set_model_entry(model_id: usize, record: u32) {
    MODEL_TABLE[model_id].store(record, Ordering::SeqCst);
}

/// Sets the aim state word the aim acquisition reads.
pub fn set_aim_state(bits: u32) {
    AIM_STATE_CELL.store(bits, Ordering::SeqCst);
}

/// The aim state word, as the lifted side reads it.
#[must_use]
pub fn aim_state() -> u32 {
    AIM_STATE_CELL.load(Ordering::SeqCst)
}

/// Reads one word of a record the case built, at an address the record
/// holds: the lifted side's read of memory outside the pedestrian.
#[must_use]
pub fn read_u32(addr: u32) -> u32 {
    // SAFETY: every address the lifted side reads names a record the case
    // built, with the word inside it.
    unsafe { (addr as *const u32).read_unaligned() }
}

/// Writes one byte of a record the case built, at an address the record
/// holds: the lifted side's write to memory outside the pedestrian.
pub fn write_u8(addr: u32, value: u8) {
    // SAFETY: as for [`read_u32`], for the byte written.
    unsafe { (addr as *mut u8).write(value) };
}

/// The state byte of a model entry, read from the entry's record: the lifted
/// side's answer for the table read.
///
/// # Panics
///
/// When the model number is outside the eight entries the proof covers.
#[must_use]
pub fn model_state_byte(model_id: u32) -> u8 {
    let entry = MODEL_TABLE[model_id as usize].load(Ordering::SeqCst);
    // SAFETY: every entry the proof reads names a record the case built,
    // with the state byte inside it.
    unsafe { ((entry.wrapping_add(0x8C)) as *const u8).read() }
}

/// Pointer to a shared global at a file address. Only the player context
/// and the model table entries are mapped; any other address is a case bug.
///
/// # Panics
///
/// For any other address.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    if file_va == PLAYER_CONTEXT_VA {
        return PLAYER_CONTEXT_CELL.as_ptr().cast::<T>();
    }
    if file_va == AIM_STATE_VA {
        return AIM_STATE_CELL.as_ptr().cast::<T>();
    }
    let end = MODEL_TABLE_VA + 4 * MODEL_TABLE.len() as u32;
    if (MODEL_TABLE_VA..end).contains(&file_va) && (file_va - MODEL_TABLE_VA) % 4 == 0 {
        let index = ((file_va - MODEL_TABLE_VA) / 4) as usize;
        return MODEL_TABLE[index].as_ptr().cast::<T>();
    }
    panic!("unexpected global address {file_va:#x}");
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

#[doc(hidden)]
#[macro_export]
macro_rules! __ty {
    ($e:expr) => {
        u32
    };
}
