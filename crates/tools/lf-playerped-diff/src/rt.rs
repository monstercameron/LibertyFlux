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
    /// `extern "thiscall" fn(u32, u32, u32) -> u32` with two pointer arguments the stub fills.
    T3Out,
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

/// The shared squared-distance threshold the relink reads.
static DIST_THRESHOLD_CELL: AtomicU32 = AtomicU32::new(0);

/// File address of the squared-distance threshold.
pub const DIST_THRESHOLD_VA: u32 = 0x00FE_8B40;

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
pub fn begin(kinds: &[(u32, Kind)], answers: &[(u32, Vec<u32>)], effect: Option<Effect>, ped: u32) {
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
        Kind::T3Out => stub_classify as *const () as usize as u32,
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
    if file_va == DIST_THRESHOLD_VA {
        return DIST_THRESHOLD_CELL.as_ptr().cast::<T>();
    }
    if file_va == AIM_STATE_VA {
        return AIM_STATE_CELL.as_ptr().cast::<T>();
    }
    if let Some(index) = cell_index(file_va) {
        return CELLS[index].as_ptr().cast::<T>();
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

/// Sets the squared-distance threshold the relink reads.
pub fn set_dist_threshold(bits: u32) {
    DIST_THRESHOLD_CELL.store(bits, Ordering::SeqCst);
}

/// The squared-distance threshold, as the lifted side reads it.
#[must_use]
pub fn dist_threshold() -> u32 {
    DIST_THRESHOLD_CELL.load(Ordering::SeqCst)
}

// ---------------------------------------------------------------------------
// The gate block's runtime: global cells, the signed pose table, the hook
// stub reached through a global slot, and the virtual stubs of the four
// table slots the block calls.
// ---------------------------------------------------------------------------

/// The id the hook stub records under.
pub const HOOK_ID: u32 = 0x120;
/// The id the refresh slot of the pedestrian's table records under.
pub const REFRESH_ID: u32 = 0x110;
/// The id the attach slot of the outer object's inner table records under.
pub const ATTACH_ID: u32 = 0x111;
/// The id the tail slot of the pedestrian's table records under.
pub const TAIL_ID: u32 = 0x112;
/// The id the sub-object slot records under.
pub const VSUB_ID: u32 = 0x113;

/// Number of global cells the gate block reads.
pub const CELL_COUNT: usize = 15;

/// The global cells, by index: the hook argument, the hook slot (the address
/// of the hook stub), two flag bytes, two or-bytes, the guard byte, the state
/// word, two pair words, the mode word and the float constant.
static CELLS: [AtomicU32; CELL_COUNT] = [const { AtomicU32::new(0) }; CELL_COUNT];

/// The pose table: sixteen entries; the table's base is the entry at index 8,
/// so table indexes from -8 to 7 are all inside it.
static POSE_TABLE: [AtomicU32; 16] = [const { AtomicU32::new(0) }; 16];

/// The table index of the base entry.
const POSE_BASE_INDEX: i32 = 8;

/// File address of the pose table the gate block reads through its base.
pub const POSE_TABLE_VA: u32 = 0x0129_5CD8;

/// Cell index for a gate-block global at a file address.
fn cell_index(file_va: u32) -> Option<usize> {
    Some(match file_va {
        0x017A_CCD8 => 0,
        0x00E7_33DC => 1,
        0x0105_B48F => 2,
        0x017E_D8D1 => 3,
        0x0117_3590 => 4,
        0x0117_3591 => 5,
        0x0116_0C39 => 6,
        0x011F_7060 => 7,
        0x0120_88B4 => 8,
        0x00F1_C040 => 9,
        0x0103_7720 => 10,
        0x00FE_8628 => 11,
        0x0117_35BC => 12,
        0x00FE_8C58 => 13,
        0x0103_CE48 => 14,
        _ => return None,
    })
}

/// Address of a gate-block global, for the relocated address map. The pose
/// table answers with its base, so indexes reach its entries.
///
/// # Panics
///
/// For any address the gate block does not read: a case bug.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    if file_va == POSE_TABLE_VA {
        return POSE_TABLE[POSE_BASE_INDEX as usize].as_ptr() as usize as u32;
    }
    let index =
        cell_index(file_va).unwrap_or_else(|| panic!("unexpected relocated address {file_va:#x}"));
    CELLS[index].as_ptr() as usize as u32
}

/// Sets every gate-block global cell, by index.
pub fn set_cells(values: &[u32; CELL_COUNT]) {
    for (cell, value) in CELLS.iter().zip(values) {
        cell.store(*value, Ordering::SeqCst);
    }
}

/// The value of a gate-block global cell, as the lifted side reads it.
#[must_use]
pub fn cell_value(index: usize) -> u32 {
    CELLS[index].load(Ordering::SeqCst)
}

/// Sets one pose table entry, by table index (-8 to 7).
///
/// # Panics
///
/// When the index is outside the sixteen entries.
pub fn set_pose_entry(index: i32, value: u32) {
    let slot = usize::try_from(POSE_BASE_INDEX + index).expect("pose index inside the table");
    POSE_TABLE[slot].store(value, Ordering::SeqCst);
}

/// The pose table entry at a table index, as the lifted side reads it.
#[must_use]
pub fn pose_entry(index: i32) -> u32 {
    let slot = usize::try_from(POSE_BASE_INDEX + index).expect("pose index inside the table");
    POSE_TABLE[slot].load(Ordering::SeqCst)
}

/// One byte of a record the case built, at an address the record holds.
#[must_use]
pub fn read_u8(addr: u32) -> u8 {
    // SAFETY: every address the lifted side reads names a record the case
    // built, with the byte inside it.
    unsafe { (addr as *const u8).read() }
}

/// The hook stub: the stdcall target the gate block calls through its slot.
extern "stdcall" fn stub_hook(a: u32) -> u32 {
    rewrite_call(HOOK_ID, vec![a])
}

/// The refresh slot of the pedestrian's table (two arguments).
extern "thiscall" fn stub_refresh(this: u32, arg: u32) -> u32 {
    rewrite_call(REFRESH_ID, vec![this, arg])
}

/// The attach slot of the inner object's table (two arguments).
extern "thiscall" fn stub_attach(this: u32, arg: u32) -> u32 {
    rewrite_call(ATTACH_ID, vec![this, arg])
}

/// The tail slot of the pedestrian's table (one argument).
extern "thiscall" fn stub_tail(this: u32) -> u32 {
    rewrite_call(TAIL_ID, vec![this])
}

/// The sub-object slot (one argument).
extern "thiscall" fn stub_vsub(this: u32) -> u32 {
    rewrite_call(VSUB_ID, vec![this])
}

/// A trap for a one-argument slot the rewrite should not call.
extern "thiscall" fn trap_one(this: u32) -> u32 {
    rewrite_call(TRAP_ID, vec![this])
}

/// Address of the hook stub, for the hook slot.
#[must_use]
pub fn hook_stub_addr() -> u32 {
    stub_hook as *const () as usize as u32
}

/// Address of the refresh stub, for the pedestrian's table.
#[must_use]
pub fn refresh_stub_addr() -> u32 {
    stub_refresh as *const () as usize as u32
}

/// Address of the attach stub, for the inner object's table.
#[must_use]
pub fn attach_stub_addr() -> u32 {
    stub_attach as *const () as usize as u32
}

/// Address of the tail stub, for the pedestrian's table.
#[must_use]
pub fn tail_stub_addr() -> u32 {
    stub_tail as *const () as usize as u32
}

/// Address of the sub-object stub, for the sub-object's table.
#[must_use]
pub fn vsub_stub_addr() -> u32 {
    stub_vsub as *const () as usize as u32
}

/// Address of the one-argument trap, for the unused one-argument slots.
#[must_use]
pub fn trap_one_addr() -> u32 {
    trap_one as *const () as usize as u32
}

// ---------------------------------------------------------------------------
// The task-budget step's classifier: a callee that writes two words through
// pointer arguments, scripted per call.
// ---------------------------------------------------------------------------

/// The value a pointer argument is logged as: the log compares the shape of a
/// call, not the address of the caller's locals.
pub const POINTER_ARG: u32 = 0xFFFF_FFF0;

/// Records a classifier call and pops its three scripted answers: the return
/// word, the output word and the tag byte. The pointer arguments are logged
/// as [`POINTER_ARG`].
pub fn script_classify(id: u32, task: u32) -> (u32, u32, u32) {
    let mut s = script();
    s.calls
        .push((format!("#{id}"), vec![task, POINTER_ARG, POINTER_ARG]));
    let mut pop = || {
        s.answers
            .get_mut(&id)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    };
    let answer = pop();
    let out = pop();
    let tag = pop();
    (answer, out, tag)
}

/// The classifier stub: a thiscall with two pointer arguments, which it fills
/// with the scripted output word and tag byte.
extern "thiscall" fn stub_classify(task: u32, out_ptr: u32, tag_ptr: u32) -> u32 {
    let id = CURRENT_ID.with(Cell::get);
    let (answer, out, tag) = script_classify(id, task);
    // SAFETY: the rewrite passes the addresses of its own two locals, sized
    // for the words written here.
    unsafe {
        (out_ptr as *mut u32).write_unaligned(out);
        (tag_ptr as *mut u8).write(tag as u8);
    }
    answer
}

/// Address of the classifier stub, for a numbered callee of its shape.
#[must_use]
pub fn classify_stub_addr() -> u32 {
    stub_classify as *const () as usize as u32
}
