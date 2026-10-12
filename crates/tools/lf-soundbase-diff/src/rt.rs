//! The 32-bit differential runtime for the base sound class.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, the `callee_*` macros, `callee_addr`, `relocated`,
//! `global`). Memory is one test arena at a real 32-bit address: the
//! rewrites read and write it through the same pointer arithmetic they use
//! on the game's image. The global cells, the class's virtual table and the
//! init table sit at the file addresses the rewrites name, mapped into the
//! arena. Numbered callees and virtual slots are stubs of the matching
//! convention that record their arguments in order and answer from a queue.
//!
//! Test-only code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses flow through the test arena and the scripted stubs.
#![allow(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

/// Length of the test arena in bytes: globals, objects, tables and stubs.
pub const ARENA_LEN: usize = 0x3_D000;
/// Arena offset of the voice records (the attach method marks their flag bytes).
pub const REC_OFF: u32 = 0x3_4000;
/// Arena offset of the attach descriptor, voice and extra word (aligned).
pub const DESC_OFF: u32 = 0x3_C400;
/// Arena offset of the constant cell whose address the category-key callee receives.
pub const CONST_OFF: u32 = 0x3_C800;
/// Arena offset of the scratch frame handle the attach callees receive.
pub const FRAME_OFF: u32 = 0x3_C900;
/// The file address of the divisor table (eight lanes of two words).
pub const DIVISOR_VA: u32 = 0x115_F808;
/// Arena offset of the divisor table.
pub const DIV_OFF: u32 = 0x3_CA00;
/// The file address of the category-key constant (mapped to `CONST_OFF`).
pub const CATEGORY_KEY_VA: u32 = 0x00E7_84F8;
/// The file address at which the mapped global window begins.
pub const GLOBALS_VA: u32 = 0x115_D000;
/// Length of the global window; it sits at the start of the arena.
pub const GLOBALS_LEN: u32 = 0x4000;
/// The class's virtual table file address (mapped to the arena below).
pub const CLASS_VTABLE_VA: u32 = 0x00E7_854C;
/// Arena offset of the class's virtual table.
pub const CLASS_VTABLE_OFF: u32 = 0x5000;
/// Arena offset of the test sound object.
pub const SOUND_OFF: u32 = 0x4000;
/// Arena offset of the voice object's virtual table.
pub const VOICE_VTABLE_OFF: u32 = 0x5100;
/// Arena offset of the voice table (rows of `0x6f40` bytes; the test maps the first five rows).
pub const TABLE_OFF: u32 = 0x8000;
/// Arena offset of the voice objects (cursor and link targets).
pub const OBJECTS_OFF: u32 = 0x2_B000;

/// The global cell of the scalar used by the lookup and the link.
pub const GLOBAL_SCALAR_A: u32 = 0x115_D964;
/// The global cell of the scalar used by the initialisation cursor.
pub const GLOBAL_SCALAR_B: u32 = 0x115_D968;
/// The global cell holding the voice table's base address.
pub const GLOBAL_VOICE_TABLE: u32 = 0x115_D988;
/// The init table of per-sub-id entries.
pub const GLOBAL_INIT_TABLE: u32 = 0x115_D774;
/// The sound registry cell.
pub const GLOBAL_REGISTRY: u32 = 0x115_D8A0;
/// The fallback object cell.
pub const GLOBAL_FALLBACK: u32 = 0x115_DB1C;
/// The memoised category-hash flag word.
pub const GLOBAL_MEMO_FLAG: u32 = 0x115_D994;
/// The memoised category hash.
pub const GLOBAL_MEMO_VALUE: u32 = 0x115_D990;
/// The category registry cell.
pub const GLOBAL_CATEGORY_CELL: u32 = 0x115_D9A0;

#[repr(C, align(16))]
struct Arena([u8; ARENA_LEN]);

static mut ARENA: Arena = Arena([0; ARENA_LEN]);

/// Callee address per numbered id (index 0 unused).
static CALLEES: [AtomicU32; 8] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// The recorded calls of the running case, in order.
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// Scripted answers per stub name, popped in call order.
static ANSWERS: Mutex<Option<HashMap<&'static str, VecDeque<u32>>>> = Mutex::new(None);
/// Serialises the cases: the arena and the logs are shared.
static LOCK: Mutex<()> = Mutex::new(());

/// Locks the runtime for one case. Hold the guard for the whole case.
pub fn lock() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// The arena's address on this machine.
#[must_use]
pub fn arena_base() -> u32 {
    core::ptr::addr_of_mut!(ARENA) as usize as u32
}

/// Maps a file address to the arena: the global window, the class's
/// virtual table, and nothing else.
///
/// # Panics
///
/// For any other address: a case bug, never a guess.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    if (GLOBALS_VA..GLOBALS_VA + GLOBALS_LEN).contains(&file_va) {
        return arena_base() + (file_va - GLOBALS_VA);
    }
    if file_va == CLASS_VTABLE_VA {
        return arena_base() + CLASS_VTABLE_OFF;
    }
    if file_va == CATEGORY_KEY_VA {
        return arena_base() + CONST_OFF;
    }
    if (DIVISOR_VA..DIVISOR_VA + 0x40).contains(&file_va) {
        return arena_base() + DIV_OFF + (file_va - DIVISOR_VA);
    }
    panic!("unexpected relocated address {file_va:#x}");
}

/// Pointer to the shared global at a file address.
///
/// # Panics
///
/// As [`relocated`].
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    relocated(file_va) as usize as *mut T
}

/// Raw stub address for callee `id`, as installed for the running case.
///
/// # Panics
///
/// For an id outside the table: a case bug.
#[must_use]
pub fn callee_addr(id: u32) -> u32 {
    let i = usize::try_from(id).expect("callee id");
    assert!(i < CALLEES.len(), "callee id {id} out of range");
    CALLEES[i].load(Ordering::SeqCst)
}

/// Installs the stub address for numbered callee `id`.
pub fn set_callee(id: u32, addr: u32) {
    CALLEES[id as usize].store(addr, Ordering::SeqCst);
}

/// Reads a 32-bit word from the arena at an absolute address.
#[must_use]
pub fn read_u32(addr: u32) -> u32 {
    check(addr, 4);
    // SAFETY: `check` keeps the access inside the arena.
    unsafe { core::ptr::read_unaligned(addr as usize as *const u32) }
}

/// Writes a 32-bit word into the arena at an absolute address.
pub fn write_u32(addr: u32, value: u32) {
    check(addr, 4);
    // SAFETY: `check` keeps the access inside the arena.
    unsafe { core::ptr::write_unaligned(addr as usize as *mut u32, value) }
}

/// Reads a byte from the arena at an absolute address.
#[must_use]
pub fn read_u8(addr: u32) -> u8 {
    check(addr, 1);
    // SAFETY: `check` keeps the access inside the arena.
    unsafe { core::ptr::read(addr as usize as *const u8) }
}

/// Writes a byte into the arena at an absolute address.
pub fn write_u8(addr: u32, value: u8) {
    check(addr, 1);
    // SAFETY: `check` keeps the access inside the arena.
    unsafe { core::ptr::write(addr as usize as *mut u8, value) }
}

fn check(addr: u32, len: u32) {
    let base = arena_base();
    let end = base + ARENA_LEN as u32;
    assert!(
        addr >= base && addr.checked_add(len).is_some_and(|e| e <= end),
        "access {addr:#x} outside the test arena [{base:#x}, {end:#x})"
    );
}

/// Copies the whole arena.
#[must_use]
pub fn arena_copy() -> Vec<u8> {
    // SAFETY: the arena is a plain static byte array; no other reference
    // is alive while the case runs under the lock.
    unsafe {
        core::slice::from_raw_parts(core::ptr::addr_of!(ARENA) as *const u8, ARENA_LEN).to_vec()
    }
}

/// Restores the whole arena from a copy taken with [`arena_copy`].
pub fn arena_restore(image: &[u8]) {
    assert_eq!(image.len(), ARENA_LEN, "arena image size");
    // SAFETY: as in `arena_copy`; the lengths match.
    unsafe {
        core::slice::from_raw_parts_mut(core::ptr::addr_of_mut!(ARENA) as *mut u8, ARENA_LEN)
            .copy_from_slice(image);
    }
}

/// Zeroes the whole arena.
pub fn arena_clear() {
    arena_restore(&vec![0u8; ARENA_LEN]);
}

/// Clears the call log and installs the scripted answers for a run. Every
/// run installs the same script, so the rewrite and the lift consume the
/// same answers in the same order.
pub fn begin_run(answers: &[(&'static str, Vec<u32>)]) {
    LOG.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let map: HashMap<&'static str, VecDeque<u32>> = answers
        .iter()
        .map(|(name, values)| (*name, values.iter().copied().collect()))
        .collect();
    *ANSWERS.lock().unwrap_or_else(|e| e.into_inner()) = Some(map);
}

/// Takes the call log of the run.
#[must_use]
pub fn take_log() -> Vec<String> {
    std::mem::take(&mut *LOG.lock().unwrap_or_else(|e| e.into_inner()))
}

/// Records one call and pops its scripted answer (zero when exhausted).
fn record(name: &'static str, args: &[u32]) -> u32 {
    let mut line = String::from(name);
    for a in args {
        line.push_str(&format!(" {a:#x}"));
    }
    LOG.lock().unwrap_or_else(|e| e.into_inner()).push(line);
    ANSWERS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_mut()
        .and_then(|m| m.get_mut(name))
        .and_then(VecDeque::pop_front)
        .unwrap_or(0)
}

/// Init-table stub for sub-id `N`: records the sub-id and the object.
pub extern "cdecl" fn init_stub<const N: u32>(this: u32) -> u32 {
    record("init", &[N, this])
}

/// Virtual slot 0 stub: records the object and the hash.
pub extern "thiscall" fn v0(this: u32, hash: u32) -> u32 {
    record("v0", &[this, hash])
}

/// Virtual slot 2 stub: records the object, the hash and the float bits.
pub extern "thiscall" fn v2(this: u32, hash: u32, bits: u32) -> u32 {
    record("v2", &[this, hash, bits])
}

/// Virtual slot 4 stub: records the target and the argument.
pub extern "thiscall" fn v4(target: u32, arg: u32) -> u32 {
    record("v4", &[target, arg])
}

/// Numbered callee 1 of the hashing routines: the name hash stub.
pub extern "cdecl" fn joaat(name: u32, seed: u32) -> u32 {
    record("joaat", &[name, seed])
}

/// Numbered callee 1 of the forwarding method: the fallback stub.
pub extern "thiscall" fn fallback(cell: u32, arg: u32) -> u32 {
    record("fallback", &[cell, arg])
}

/// Numbered callee 1 of the deleting destructor: the rebuild stub.
pub extern "thiscall" fn rebuild(this: u32) -> u32 {
    record("rebuild", &[this])
}

/// Numbered callee 2 of the deleting destructor: the unregister stub.
pub extern "thiscall" fn unregister(registry: u32, this: u32, voice_id: u32) -> u32 {
    record("unregister", &[registry, this, voice_id])
}

/// Numbered callee 2 of the second constructor: the cursor stub.
pub extern "thiscall" fn set_cursor(cursor: u32) -> u32 {
    record("set_cursor", &[cursor])
}

/// Numbered callee 3 of the second constructor: the register stub.
pub extern "thiscall" fn register(registry: u32, voice_id: u32, kind: u32) -> u32 {
    record("register", &[registry, voice_id, kind])
}

/// The init table: one stub per sub-id, indexed by the sub-id.
pub static INIT_STUBS: [extern "cdecl" fn(u32) -> u32; 256] = [
    init_stub::<0>,
    init_stub::<1>,
    init_stub::<2>,
    init_stub::<3>,
    init_stub::<4>,
    init_stub::<5>,
    init_stub::<6>,
    init_stub::<7>,
    init_stub::<8>,
    init_stub::<9>,
    init_stub::<10>,
    init_stub::<11>,
    init_stub::<12>,
    init_stub::<13>,
    init_stub::<14>,
    init_stub::<15>,
    init_stub::<16>,
    init_stub::<17>,
    init_stub::<18>,
    init_stub::<19>,
    init_stub::<20>,
    init_stub::<21>,
    init_stub::<22>,
    init_stub::<23>,
    init_stub::<24>,
    init_stub::<25>,
    init_stub::<26>,
    init_stub::<27>,
    init_stub::<28>,
    init_stub::<29>,
    init_stub::<30>,
    init_stub::<31>,
    init_stub::<32>,
    init_stub::<33>,
    init_stub::<34>,
    init_stub::<35>,
    init_stub::<36>,
    init_stub::<37>,
    init_stub::<38>,
    init_stub::<39>,
    init_stub::<40>,
    init_stub::<41>,
    init_stub::<42>,
    init_stub::<43>,
    init_stub::<44>,
    init_stub::<45>,
    init_stub::<46>,
    init_stub::<47>,
    init_stub::<48>,
    init_stub::<49>,
    init_stub::<50>,
    init_stub::<51>,
    init_stub::<52>,
    init_stub::<53>,
    init_stub::<54>,
    init_stub::<55>,
    init_stub::<56>,
    init_stub::<57>,
    init_stub::<58>,
    init_stub::<59>,
    init_stub::<60>,
    init_stub::<61>,
    init_stub::<62>,
    init_stub::<63>,
    init_stub::<64>,
    init_stub::<65>,
    init_stub::<66>,
    init_stub::<67>,
    init_stub::<68>,
    init_stub::<69>,
    init_stub::<70>,
    init_stub::<71>,
    init_stub::<72>,
    init_stub::<73>,
    init_stub::<74>,
    init_stub::<75>,
    init_stub::<76>,
    init_stub::<77>,
    init_stub::<78>,
    init_stub::<79>,
    init_stub::<80>,
    init_stub::<81>,
    init_stub::<82>,
    init_stub::<83>,
    init_stub::<84>,
    init_stub::<85>,
    init_stub::<86>,
    init_stub::<87>,
    init_stub::<88>,
    init_stub::<89>,
    init_stub::<90>,
    init_stub::<91>,
    init_stub::<92>,
    init_stub::<93>,
    init_stub::<94>,
    init_stub::<95>,
    init_stub::<96>,
    init_stub::<97>,
    init_stub::<98>,
    init_stub::<99>,
    init_stub::<100>,
    init_stub::<101>,
    init_stub::<102>,
    init_stub::<103>,
    init_stub::<104>,
    init_stub::<105>,
    init_stub::<106>,
    init_stub::<107>,
    init_stub::<108>,
    init_stub::<109>,
    init_stub::<110>,
    init_stub::<111>,
    init_stub::<112>,
    init_stub::<113>,
    init_stub::<114>,
    init_stub::<115>,
    init_stub::<116>,
    init_stub::<117>,
    init_stub::<118>,
    init_stub::<119>,
    init_stub::<120>,
    init_stub::<121>,
    init_stub::<122>,
    init_stub::<123>,
    init_stub::<124>,
    init_stub::<125>,
    init_stub::<126>,
    init_stub::<127>,
    init_stub::<128>,
    init_stub::<129>,
    init_stub::<130>,
    init_stub::<131>,
    init_stub::<132>,
    init_stub::<133>,
    init_stub::<134>,
    init_stub::<135>,
    init_stub::<136>,
    init_stub::<137>,
    init_stub::<138>,
    init_stub::<139>,
    init_stub::<140>,
    init_stub::<141>,
    init_stub::<142>,
    init_stub::<143>,
    init_stub::<144>,
    init_stub::<145>,
    init_stub::<146>,
    init_stub::<147>,
    init_stub::<148>,
    init_stub::<149>,
    init_stub::<150>,
    init_stub::<151>,
    init_stub::<152>,
    init_stub::<153>,
    init_stub::<154>,
    init_stub::<155>,
    init_stub::<156>,
    init_stub::<157>,
    init_stub::<158>,
    init_stub::<159>,
    init_stub::<160>,
    init_stub::<161>,
    init_stub::<162>,
    init_stub::<163>,
    init_stub::<164>,
    init_stub::<165>,
    init_stub::<166>,
    init_stub::<167>,
    init_stub::<168>,
    init_stub::<169>,
    init_stub::<170>,
    init_stub::<171>,
    init_stub::<172>,
    init_stub::<173>,
    init_stub::<174>,
    init_stub::<175>,
    init_stub::<176>,
    init_stub::<177>,
    init_stub::<178>,
    init_stub::<179>,
    init_stub::<180>,
    init_stub::<181>,
    init_stub::<182>,
    init_stub::<183>,
    init_stub::<184>,
    init_stub::<185>,
    init_stub::<186>,
    init_stub::<187>,
    init_stub::<188>,
    init_stub::<189>,
    init_stub::<190>,
    init_stub::<191>,
    init_stub::<192>,
    init_stub::<193>,
    init_stub::<194>,
    init_stub::<195>,
    init_stub::<196>,
    init_stub::<197>,
    init_stub::<198>,
    init_stub::<199>,
    init_stub::<200>,
    init_stub::<201>,
    init_stub::<202>,
    init_stub::<203>,
    init_stub::<204>,
    init_stub::<205>,
    init_stub::<206>,
    init_stub::<207>,
    init_stub::<208>,
    init_stub::<209>,
    init_stub::<210>,
    init_stub::<211>,
    init_stub::<212>,
    init_stub::<213>,
    init_stub::<214>,
    init_stub::<215>,
    init_stub::<216>,
    init_stub::<217>,
    init_stub::<218>,
    init_stub::<219>,
    init_stub::<220>,
    init_stub::<221>,
    init_stub::<222>,
    init_stub::<223>,
    init_stub::<224>,
    init_stub::<225>,
    init_stub::<226>,
    init_stub::<227>,
    init_stub::<228>,
    init_stub::<229>,
    init_stub::<230>,
    init_stub::<231>,
    init_stub::<232>,
    init_stub::<233>,
    init_stub::<234>,
    init_stub::<235>,
    init_stub::<236>,
    init_stub::<237>,
    init_stub::<238>,
    init_stub::<239>,
    init_stub::<240>,
    init_stub::<241>,
    init_stub::<242>,
    init_stub::<243>,
    init_stub::<244>,
    init_stub::<245>,
    init_stub::<246>,
    init_stub::<247>,
    init_stub::<248>,
    init_stub::<249>,
    init_stub::<250>,
    init_stub::<251>,
    init_stub::<252>,
    init_stub::<253>,
    init_stub::<254>,
    init_stub::<255>,
];

/// The marker a stub records in place of the scratch frame argument: the
/// frame is a stack address in the rewrite and an arena address in the lift,
/// so only its position is compared.
pub const FRAME_MARK: u32 = 0xF7A0_0000;

/// Attach callee 1: initialises the scratch frame (thiscall, the frame).
pub extern "thiscall" fn attach_init(frame: u32) -> u32 {
    let _ = frame;
    record("c1", &[FRAME_MARK])
}

/// Attach callee 2: binds the voice (thiscall: sound, voice, zero, frame).
pub extern "thiscall" fn attach_bind(this: u32, voice: u32, zero: u32, frame: u32) -> u32 {
    let _ = frame;
    record("c2", &[this, voice, zero, FRAME_MARK])
}

/// Attach callee 3: binds the extra word (thiscall: sound, extra, frame).
pub extern "thiscall" fn attach_extra(this: u32, extra: u32, frame: u32) -> u32 {
    let _ = frame;
    record("c3", &[this, extra, FRAME_MARK])
}

/// Attach callee 4: resets the sound (thiscall: sound).
pub extern "thiscall" fn attach_reset(this: u32) -> u32 {
    record("c4", &[this])
}

/// Attach callee 5: the category-key hash (cdecl: key constant, seed).
pub extern "cdecl" fn attach_key_hash(key: u32, seed: u32) -> u32 {
    record("c5", &[key, seed])
}

/// Attach callee 6: the category lookup (thiscall: registry cell, category).
pub extern "thiscall" fn attach_lookup(cell: u32, category: u32) -> u32 {
    record("c6", &[cell, category])
}

/// Attach callee 7: the category verdict (thiscall: found entry).
pub extern "thiscall" fn attach_verdict(found: u32) -> u32 {
    record("c7", &[found])
}
