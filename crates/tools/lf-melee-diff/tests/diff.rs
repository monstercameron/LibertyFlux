//! Differential cases: the lifted complex melee task methods against their
//! verified 32-bit rewrites.
//!
//! For every method the same generated inputs run through the rewrite and
//! through the lifted method. A case compares every byte of the task
//! object's memory (except the vtable word the constructor installs, which
//! the lifted type has no equivalent of) and the ordered log of every call
//! the method made (with its arguments). A deliberately wrong lift of each
//! method runs through the same cases and must be caught.
//!
//! The rewrites take real addresses, so this file runs on the 32-bit target
//! only. The callees and the globals are answered by the test: the checker
//! runtime's callee table and base address point at test memory for the
//! length of one case, and the table is guarded by a lock because the
//! runtime's statics are process-wide.

#![cfg(target_arch = "x86")]
// The rewrites take raw addresses through the checker runtime and the test
// memory is read and written through them; every access stays inside the
// buffers the case builds.
#![allow(unsafe_code)]

use core::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

use lf_core::boundary::Handle32;
use lf_melee_diff::rewrites::{fn_00ccefb0, fn_00ccf840, fn_00cd0d00, fn_00cd0d40, fn_00cd0450};
use lf_peds_tasks::melee::{
    CtorArgs, InitWords, MeleeHost, MeleeTask, SLOT_OFFSETS, StampWords, StartArgs,
    StarterHandleTag, SubTask, TaskManagerTag,
};

// Number of generated cases per method.
const CASES: usize = 20_000;
// Bytes of the task object the rewrites may touch (the highest field ends
// before offset 0xF2).
const OBJ_LEN: usize = 0x100;

// Field offsets inside the task object, as the verified rewrites use them.
const O_SUB: usize = 0x08;
const O_WORD_20: usize = 0x20;
const O_WORD_24: usize = 0x24;
const O_WORD_28: usize = 0x28;
const O_MODE: usize = 0x30;
const O_WORD_34: usize = 0x34;
const O_WORD_38: usize = 0x38;
const O_WORD_3C: usize = 0x3C;
const O_FLAG_40: usize = 0x40;
const O_FILLER_41: usize = 0x41;
const O_WORD_44: usize = 0x44;
const O_WORD_48: usize = 0x48;
const O_HALF_4C: usize = 0x4C;
const O_WORD_50: usize = 0x50;
const O_WORD_CC: usize = 0xCC;
const O_WORD_D0: usize = 0xD0;
const O_WORD_D4: usize = 0xD4;
const O_WORD_D8: usize = 0xD8;
const O_WORD_DC: usize = 0xDC;
const O_WORD_E0: usize = 0xE0;
const O_FLAG_E4: usize = 0xE4;
const O_FILLER_E5: usize = 0xE5;
const O_WORD_E8: usize = 0xE8;
const O_WORD_EC: usize = 0xEC;
const O_FLAGS_F0: usize = 0xF0;
const O_FLAGS_F1: usize = 0xF1;

// The executable's data addresses the rewrites read, as file VAs.
const MANAGER_VA: u32 = 0x0167_E2A0;
const TIMING_FIRST_VA: u32 = 0x0105_194C;
const CTOR_SECOND_VA: u32 = 0x0105_19BC;
const STAMP_VA: u32 = 0x0117_35B4;
// The class table the constructor installs, as a file VA.
const VTABLE_VA: u32 = 0x00ED_A884;
// The lowest data address read, which the test area starts at.
const AREA_LO_VA: u32 = 0x0105_194C;
// The file image base the checker runtime subtracts.
const IMAGE_BASE: u32 = 0x40_0000;

// The kind tag and the second accepted follow-up answer.
const KIND: u32 = 0x1B1;
const ALT: u32 = 0x11D;

/// Serialises the cases: the callee table and the base address are process
/// statics, and each method needs its own entries.
static CASE_LOCK: Mutex<()> = Mutex::new(());

/// The callee table the rewrites index.
static CALLEE_TABLE: [AtomicU32; 256] = [const { AtomicU32::new(0) }; 256];

/// The leaked test memory standing in for the executable's data.
static GLOBAL_AREA: OnceLock<u32> = OnceLock::new();

thread_local! {
    /// The calls the rewrite side made during one case, in order.
    static LOG: RefCell<Vec<Ev>> = const { RefCell::new(Vec::new()) };
    /// The address of the task object under test on the rewrite side.
    static SRC: Cell<u32> = const { Cell::new(0) };
    /// What the handle stub answers.
    static HANDLE_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// What the mode stub answers.
    static MODE_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// What the flag-helper stub answers.
    static FLAG_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// What the flag-helper stub writes to the mode word.
    static MODE_WORD_WRITE: Cell<u32> = const { Cell::new(0) };
    /// What the cleanup stub writes to the flag byte.
    static FLAG_BYTE_WRITE: Cell<u8> = const { Cell::new(0) };
    /// What the sub-task query stub answers.
    static QUERY_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// What the follow-up sink stub answers.
    static SINK_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// The base word of the slot-reset pattern.
    static RESET_BASE: Cell<u32> = const { Cell::new(0) };
}

/// One call a method made, in the form both sides log it. Receivers on the
/// task are stored as offsets from the task (always zero when both sides
/// agree); the sub-task query stores the sub-task address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ev {
    BaseCall {
        recv: u32,
    },
    Reset {
        offset: u32,
    },
    SetTarget {
        recv: u32,
        first: u32,
        second: u32,
    },
    Destroy {
        recv: u32,
    },
    Free {
        manager: u32,
        recv: u32,
    },
    Handle {
        manager: u32,
    },
    Mode {
        recv: u32,
    },
    FlagVal {
        recv: u32,
        mode_byte: u32,
    },
    Starter {
        handle: u32,
        mode: u32,
        b1: u32,
        b2: u32,
        b0: u32,
        b3: u32,
        computed: u32,
    },
    Setup {
        recv: u32,
        arg: u32,
    },
    Register {
        recv: u32,
        arg: u32,
        kind: u32,
        one: u32,
    },
    Cleanup {
        recv: u32,
    },
    Query {
        sub: u32,
    },
    Sink {
        recv: u32,
        arg: u32,
        kind: u32,
        zero: u32,
    },
}

/// The slot-reset pattern: three words from the script base and the slot
/// offset, so every slot's bytes differ. Both sides compute it.
fn reset_pattern(base: u32, offset: u32) -> [u32; 3] {
    [
        base ^ offset,
        base.wrapping_add(offset),
        base.rotate_left(offset & 31),
    ]
}

// The rewrite side's callee stubs. They answer from the thread-local script
// and log what the rewrite passed them. A stub writes only when its address
// arithmetic lands inside the object, so a misread rewrite fails the case
// instead of scribbling wild memory.

extern "thiscall" fn stub_base(this: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::BaseCall {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
        })
    });
    0
}

extern "thiscall" fn stub_reset(addr: u32) -> u32 {
    let src = SRC.with(Cell::get);
    let offset = addr.wrapping_sub(src);
    LOG.with(|l| l.borrow_mut().push(Ev::Reset { offset }));
    if offset as usize + 12 <= OBJ_LEN && addr.wrapping_sub(src) == offset {
        let words = reset_pattern(RESET_BASE.with(Cell::get), offset);
        // SAFETY: the offset was just bounds-checked against the live object.
        unsafe {
            (addr as *mut u32).write_unaligned(words[0]);
            ((addr + 4) as *mut u32).write_unaligned(words[1]);
            ((addr + 8) as *mut u32).write_unaligned(words[2]);
        }
    }
    0
}

extern "thiscall" fn stub_set_target(this: u32, first: u32, second: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::SetTarget {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
            first,
            second,
        })
    });
    0
}

extern "thiscall" fn stub_dtor(this: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Destroy {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
        })
    });
    0
}

extern "thiscall" fn stub_free(manager: u32, freed: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Free {
            manager,
            recv: freed.wrapping_sub(SRC.with(Cell::get)),
        })
    });
    0
}

extern "thiscall" fn stub_handle(manager: u32) -> u32 {
    LOG.with(|l| l.borrow_mut().push(Ev::Handle { manager }));
    HANDLE_ANSWER.with(Cell::get)
}

extern "thiscall" fn stub_mode(this: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Mode {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
        })
    });
    MODE_ANSWER.with(Cell::get)
}

extern "thiscall" fn stub_flagval(this: u32, mode_byte: u32) -> u32 {
    let src = SRC.with(Cell::get);
    LOG.with(|l| {
        l.borrow_mut().push(Ev::FlagVal {
            recv: this.wrapping_sub(src),
            mode_byte,
        })
    });
    // The callee may write the task; the script overwrites the mode word,
    // which the start method reads after this call returns.
    if this == src {
        let word = MODE_WORD_WRITE.with(Cell::get);
        // SAFETY: the case passes the address of a live object of OBJ_LEN bytes.
        unsafe { ((this + O_MODE as u32) as *mut u32).write_unaligned(word) };
    }
    FLAG_ANSWER.with(Cell::get)
}

extern "thiscall" fn stub_starter(
    handle: u32,
    mode: u32,
    b1: u32,
    b2: u32,
    b0: u32,
    b3: u32,
    computed: u32,
) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Starter {
            handle,
            mode,
            b1,
            b2,
            b0,
            b3,
            computed,
        })
    });
    0
}

extern "thiscall" fn stub_setup(this: u32, arg: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Setup {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
            arg,
        })
    });
    0
}

extern "thiscall" fn stub_register(this: u32, arg: u32, kind: u32, one: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Register {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
            arg,
            kind,
            one,
        })
    });
    0
}

extern "thiscall" fn stub_cleanup(this: u32) -> u32 {
    let src = SRC.with(Cell::get);
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Cleanup {
            recv: this.wrapping_sub(src),
        })
    });
    // The callee may write the task; the script overwrites the flag byte,
    // which the follow-up method reads after this call returns.
    if this == src {
        let byte = FLAG_BYTE_WRITE.with(Cell::get);
        // SAFETY: as above.
        unsafe { ((this + O_FLAGS_F0 as u32) as *mut u8).write(byte) };
    }
    0
}

/// The sub-task query stub: called through a fake virtual table, like the
/// original's indirect call.
extern "thiscall" fn stub_query(sub: u32) -> u32 {
    LOG.with(|l| l.borrow_mut().push(Ev::Query { sub }));
    QUERY_ANSWER.with(Cell::get)
}

extern "thiscall" fn stub_sink(this: u32, arg: u32, kind: u32, zero: u32) -> u32 {
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Sink {
            recv: this.wrapping_sub(SRC.with(Cell::get)),
            arg,
            kind,
            zero,
        })
    });
    SINK_ANSWER.with(Cell::get)
}

/// Points the checker runtime at the test memory: the callee table holds
/// the stubs in the slots the rewrites use, cleared first so a rewrite
/// that calls an unexpected slot faults loudly, and the base address is set
/// so that the data addresses resolve to the leaked area.
fn setup_runtime(slots: &[(usize, u32)]) {
    let area = *GLOBAL_AREA.get_or_init(|| {
        let len = (MANAGER_VA + 4 - AREA_LO_VA) as usize;
        let v: &'static mut [u8] = Box::leak(vec![0u8; len].into_boxed_slice());
        v.as_mut_ptr() as usize as u32
    });
    for slot in &CALLEE_TABLE {
        slot.store(0, Ordering::SeqCst);
    }
    for (id, addr) in slots {
        CALLEE_TABLE[*id].store(*addr, Ordering::SeqCst);
    }
    // SAFETY: the statics are written only under CASE_LOCK, before any
    // rewrite runs; the table and the area outlive the process.
    unsafe {
        lf_checker_rt::CHECKER_CTABLE = CALLEE_TABLE.as_ptr() as *const u32;
        lf_checker_rt::CHECKER_XBASE = area.wrapping_sub(AREA_LO_VA - IMAGE_BASE);
    }
}

/// Writes a test-memory word at a file VA.
fn write_global(va: u32, v: u32) {
    let p = lf_checker_rt::global::<u32>(va);
    // SAFETY: as above.
    unsafe { p.write_unaligned(v) };
}

/// A stub's address as the callee table holds it.
macro_rules! stub {
    ($f:expr) => {
        $f as *const () as usize as u32
    };
}

// Generator.

/// Small deterministic generator (splitmix64).
struct Gen(u64);

impl Gen {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }

    fn below(&mut self, n: u32) -> u32 {
        self.u32() % n
    }
}

// Flag bytes the bit tests treat differently: each low bit alone and in
// pairs, the disable bit with and without company, the constructor's base
// bit, and the extremes. Written decimally on purpose (see the crate root).
const FLAG_EDGE: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 32, 64, 128, 223, 255];

fn pick_flag_byte(g: &mut Gen) -> u8 {
    if g.below(10) < 6 {
        FLAG_EDGE[g.below(FLAG_EDGE.len() as u32) as usize]
    } else {
        g.u32() as u8
    }
}

/// A word whose low bit decides: small counts, the low-byte edges and the
/// high patterns a full-word test would wrongly pick up.
const WORD_EDGE: [u32; 12] = [
    0,
    1,
    2,
    3,
    4,
    5,
    0x100,
    0x101,
    0xFFFF_FFFE,
    0xFFFF_FFFF,
    0x8000_0000,
    0x8000_0001,
];

fn pick_word(g: &mut Gen) -> u32 {
    if g.below(10) < 5 {
        WORD_EDGE[g.below(WORD_EDGE.len() as u32) as usize]
    } else {
        g.u32()
    }
}

/// Sub-task query answers: the two accepted answers, their neighbours, and
/// the extremes.
const QUERY_EDGE: [u32; 10] = [0, 1, 7, 0x11C, ALT, 0x11E, 0x1B0, KIND, 0x1B2, 0xFFFF_FFFF];

fn pick_query(g: &mut Gen) -> u32 {
    // The two accepted answers arrive often, so both sink paths run.
    match g.below(10) {
        0..=2 => ALT,
        3..=5 => KIND,
        6..=7 => QUERY_EDGE[g.below(QUERY_EDGE.len() as u32) as usize],
        _ => g.u32(),
    }
}

// Inputs.

/// One generated case: the task object's bytes, the arguments, the words
/// of test memory the globals resolve to, and the scripted stub answers.
#[derive(Clone, Copy)]
struct Input {
    init: [u8; OBJ_LEN],
    initial_word: u32,
    flag_words: [u32; 4],
    target: [u32; 2],
    destroy_flags: u32,
    arg: u32,
    manager: u32,
    stamp: [u32; 2],
    timing: [u32; 2],
    handle_answer: u32,
    mode_answer: u32,
    flag_answer: u32,
    mode_word_write: u32,
    flag_byte_write: u8,
    query_answer: u32,
    sink_answer: u32,
    reset_base: u32,
    /// Whether the follow-up case runs with a null sub-task (only with
    /// the disable bit set, where the original never dereferences it).
    null_sub: bool,
}

fn put_u32(b: &mut [u8; OBJ_LEN], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn read_u32(b: &[u8; OBJ_LEN], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn read_u16(b: &[u8; OBJ_LEN], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn put_u16(b: &mut [u8; OBJ_LEN], off: usize, v: u16) {
    b[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

fn make_input(g: &mut Gen, m: Method) -> Input {
    let mut init = [0u8; OBJ_LEN];
    for b in init.iter_mut() {
        *b = g.u32() as u8;
    }
    let mut inp = Input {
        init,
        initial_word: g.u32(),
        flag_words: [pick_word(g), pick_word(g), pick_word(g), pick_word(g)],
        target: [g.u32(), g.u32()],
        destroy_flags: pick_word(g),
        arg: g.u32(),
        manager: g.u32(),
        stamp: [g.u32(), g.u32()],
        timing: [g.u32(), g.u32()],
        handle_answer: if g.below(3) == 0 { 0 } else { g.u32() | 1 },
        mode_answer: g.u32(),
        flag_answer: g.u32(),
        mode_word_write: g.u32(),
        flag_byte_write: pick_flag_byte(g),
        query_answer: pick_query(g),
        sink_answer: g.u32(),
        reset_base: g.u32(),
        null_sub: false,
    };
    // The flag byte of the stored task.
    inp.init[O_FLAGS_F0] = pick_flag_byte(g);
    match m {
        Method::Ctor => {
            // The sub-task word is any address or null; the constructor
            // carries it over untouched.
            put_u32(&mut inp.init, O_SUB, g.u32());
            // Equal stamp pairs sometimes, so both the swap-catch and the
            // equal path run.
            if g.below(8) == 0 {
                inp.stamp[1] = inp.stamp[0];
            }
        }
        Method::Destroy => {}
        Method::Start => {
            put_u32(&mut inp.init, O_MODE, g.u32());
        }
        Method::Init => {
            // Equal timing pairs sometimes.
            if g.below(8) == 0 {
                inp.timing[1] = inp.timing[0];
            }
        }
        Method::FollowUp => {
            // The cleanup call overwrites the flag byte before the method
            // tests it, so correlate the sub-task fixture with that write.
            if g.below(2) == 0 {
                inp.flag_byte_write |= 0x20;
                // A null sub-task is safe only when cleanup disables the query.
                inp.null_sub = g.below(2) == 0;
            } else {
                inp.flag_byte_write &= !0x20;
            }
        }
    }
    inp
}

/// The methods under test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Method {
    Ctor,
    Destroy,
    Start,
    Init,
    FollowUp,
}

/// Which side of the comparison a run exercises: the lift itself, or the
/// deliberately wrong lift that must be caught.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Variant {
    Lift,
    Wrong,
}

/// What a case leaves behind, in the form both sides produce.
#[derive(Debug, PartialEq)]
struct Outcome {
    ret: Option<u32>,
    mem: [u8; OBJ_LEN],
    log: Vec<Ev>,
}

// The fields of the task object, as the lifted type reads them.

fn decode(b: &[u8; OBJ_LEN]) -> MeleeTask {
    let mut slots = [lf_peds_tasks::melee::ResetSlot([0; 12]); 10];
    for (i, slot) in slots.iter_mut().enumerate() {
        let off = SLOT_OFFSETS[i] as usize;
        slot.0.copy_from_slice(&b[off..off + 12]);
    }
    MeleeTask {
        sub_task: Handle32::new(read_u32(b, O_SUB)),
        word_20: read_u32(b, O_WORD_20),
        word_24: read_u32(b, O_WORD_24),
        word_28: read_u32(b, O_WORD_28),
        mode_word: read_u32(b, O_MODE),
        word_34: read_u32(b, O_WORD_34),
        word_38: read_u32(b, O_WORD_38),
        word_3c: read_u32(b, O_WORD_3C),
        flag_40: b[O_FLAG_40],
        filler_41: b[O_FILLER_41],
        word_44: read_u32(b, O_WORD_44),
        word_48: read_u32(b, O_WORD_48),
        half_4c: read_u16(b, O_HALF_4C),
        word_50: read_u32(b, O_WORD_50),
        slots,
        word_cc: read_u32(b, O_WORD_CC),
        word_d0: read_u32(b, O_WORD_D0),
        word_d4: read_u32(b, O_WORD_D4),
        word_d8: read_u32(b, O_WORD_D8),
        word_dc: read_u32(b, O_WORD_DC),
        word_e0: read_u32(b, O_WORD_E0),
        flag_e4: b[O_FLAG_E4],
        filler_e5: b[O_FILLER_E5],
        word_e8: read_u32(b, O_WORD_E8),
        word_ec: read_u32(b, O_WORD_EC),
        flags_f0: b[O_FLAGS_F0],
        flags_f1: b[O_FLAGS_F1],
    }
}

fn encode(t: &MeleeTask, b: &mut [u8; OBJ_LEN]) {
    put_u32(b, O_SUB, Handle32::raw_or_zero(t.sub_task));
    put_u32(b, O_WORD_20, t.word_20);
    put_u32(b, O_WORD_24, t.word_24);
    put_u32(b, O_WORD_28, t.word_28);
    put_u32(b, O_MODE, t.mode_word);
    put_u32(b, O_WORD_34, t.word_34);
    put_u32(b, O_WORD_38, t.word_38);
    put_u32(b, O_WORD_3C, t.word_3c);
    b[O_FLAG_40] = t.flag_40;
    b[O_FILLER_41] = t.filler_41;
    put_u32(b, O_WORD_44, t.word_44);
    put_u32(b, O_WORD_48, t.word_48);
    put_u16(b, O_HALF_4C, t.half_4c);
    put_u32(b, O_WORD_50, t.word_50);
    for (i, slot) in t.slots.iter().enumerate() {
        let off = SLOT_OFFSETS[i] as usize;
        b[off..off + 12].copy_from_slice(&slot.0);
    }
    put_u32(b, O_WORD_CC, t.word_cc);
    put_u32(b, O_WORD_D0, t.word_d0);
    put_u32(b, O_WORD_D4, t.word_d4);
    put_u32(b, O_WORD_D8, t.word_d8);
    put_u32(b, O_WORD_DC, t.word_dc);
    put_u32(b, O_WORD_E0, t.word_e0);
    b[O_FLAG_E4] = t.flag_e4;
    b[O_FILLER_E5] = t.filler_e5;
    put_u32(b, O_WORD_E8, t.word_e8);
    put_u32(b, O_WORD_EC, t.word_ec);
    b[O_FLAGS_F0] = t.flags_f0;
    b[O_FLAGS_F1] = t.flags_f1;
}

// The rewrite side.

/// A sub-task object with a fake virtual table whose slot at `+0xC` answers
/// the query stub, as the follow-up contract builds it.
struct SubTaskObjects {
    _sub: Box<[u32; 2]>,
    _vtable: Box<[u32; 4]>,
    addr: u32,
}

fn make_subtask() -> SubTaskObjects {
    let mut vtable = Box::new([0u32; 4]);
    vtable[3] = stub!(stub_query);
    let vtable_addr = vtable.as_mut_ptr() as usize as u32;
    let mut sub = Box::new([0u32; 2]);
    sub[0] = vtable_addr;
    let addr = sub.as_mut_ptr() as usize as u32;
    SubTaskObjects {
        _sub: sub,
        _vtable: vtable,
        addr,
    }
}

fn run_rewrite(m: Method, inp: &Input, sub_addr: u32) -> Outcome {
    let mut obj = Box::new(inp.init);
    if m == Method::FollowUp && !inp.null_sub {
        put_u32(&mut obj, O_SUB, sub_addr);
    }
    if m == Method::FollowUp && inp.null_sub {
        put_u32(&mut obj, O_SUB, 0);
    }
    let addr = obj.as_mut_ptr() as usize as u32;

    write_global(MANAGER_VA, inp.manager);
    // The stamp global serves both the constructor (with the second stamp
    // global) and the initialise method (with the first timing global).
    if m == Method::Ctor {
        write_global(STAMP_VA, inp.stamp[0]);
        write_global(CTOR_SECOND_VA, inp.stamp[1]);
    }
    if m == Method::Init {
        write_global(STAMP_VA, inp.timing[0]);
        write_global(TIMING_FIRST_VA, inp.timing[1]);
    }
    SRC.with(|s| s.set(addr));
    HANDLE_ANSWER.with(|h| h.set(inp.handle_answer));
    MODE_ANSWER.with(|h| h.set(inp.mode_answer));
    FLAG_ANSWER.with(|h| h.set(inp.flag_answer));
    MODE_WORD_WRITE.with(|h| h.set(inp.mode_word_write));
    FLAG_BYTE_WRITE.with(|h| h.set(inp.flag_byte_write));
    QUERY_ANSWER.with(|h| h.set(inp.query_answer));
    SINK_ANSWER.with(|h| h.set(inp.sink_answer));
    RESET_BASE.with(|h| h.set(inp.reset_base));
    LOG.with(|l| l.borrow_mut().clear());

    let ret = match m {
        Method::Ctor => {
            let out = fn_00ccefb0::rw_ctask_complex_melee_ctor(
                addr,
                inp.initial_word,
                inp.flag_words[0],
                inp.flag_words[1],
                inp.flag_words[2],
                inp.flag_words[3],
                inp.target[0],
                inp.target[1],
            );
            assert_eq!(out, addr, "the constructor answers its object");
            // The vtable word the lift drops is pinned here instead.
            let vtable = read_u32(&obj, 0);
            let base = lf_checker_rt::xbase();
            assert_eq!(
                vtable,
                VTABLE_VA.wrapping_sub(IMAGE_BASE).wrapping_add(base)
            );
            None
        }
        Method::Destroy => {
            let out = fn_00ccf840::rw_00ccf840(addr, inp.destroy_flags);
            assert_eq!(out, addr, "the destructor answers its object");
            None
        }
        Method::Start => {
            let out = fn_00cd0450::rw_00cd0450(addr);
            assert_eq!(out, 0, "the start method always answers 0");
            None
        }
        Method::Init => {
            let out = fn_00cd0d00::rw_00cd0d00(addr, inp.arg);
            assert_eq!(out, 0, "the initialise method always answers 0");
            None
        }
        Method::FollowUp => Some(fn_00cd0d40::rw_00cd0d40(addr, inp.arg)),
    };
    Outcome {
        ret,
        mem: *obj,
        log: LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
    }
}

// The lifted side: the host records the calls the lifted method makes and
// mirrors the stubs' scripted writes.

struct Host {
    log: Vec<Ev>,
    manager_word: u32,
    handle_answer: u32,
    mode_answer: u32,
    flag_answer: u32,
    mode_word_write: u32,
    flag_byte_write: u8,
    query_answer: u32,
    sink_answer: u32,
    reset_base: u32,
    stamp: [u32; 2],
    timing: [u32; 2],
}

impl MeleeHost for Host {
    fn construct_base(&mut self, _task: &mut MeleeTask) {
        self.log.push(Ev::BaseCall { recv: 0 });
    }

    fn reset_slot(&mut self, task: &mut MeleeTask, offset: u32) {
        self.log.push(Ev::Reset { offset });
        let index = SLOT_OFFSETS
            .iter()
            .position(|o| *o == offset)
            .expect("the lift resets a slot outside its table");
        let words = reset_pattern(self.reset_base, offset);
        let bytes = task.slots[index].0.as_mut_slice();
        for (i, w) in words.iter().enumerate() {
            bytes[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
        }
    }

    fn set_target_node(&mut self, _task: &mut MeleeTask, first: u32, second: u32) {
        self.log.push(Ev::SetTarget {
            recv: 0,
            first,
            second,
        });
    }

    fn stamp_words(&mut self) -> StampWords {
        StampWords {
            word_38: self.stamp[0],
            word_3c: self.stamp[1],
        }
    }

    fn destroy_task(&mut self, _task: &mut MeleeTask) {
        self.log.push(Ev::Destroy { recv: 0 });
    }

    fn manager(&mut self) -> Option<Handle32<TaskManagerTag>> {
        Handle32::new(self.manager_word)
    }

    fn free_task(&mut self, manager: Option<Handle32<TaskManagerTag>>) {
        self.log.push(Ev::Free {
            manager: Handle32::raw_or_zero(manager),
            recv: 0,
        });
    }

    fn task_handle(
        &mut self,
        manager: Option<Handle32<TaskManagerTag>>,
    ) -> Option<Handle32<StarterHandleTag>> {
        self.log.push(Ev::Handle {
            manager: Handle32::raw_or_zero(manager),
        });
        Handle32::new(self.handle_answer)
    }

    fn mode_value(&mut self, _task: &mut MeleeTask) -> u32 {
        self.log.push(Ev::Mode { recv: 0 });
        self.mode_answer
    }

    fn flag_value(&mut self, task: &mut MeleeTask, mode_byte: u32) -> u32 {
        self.log.push(Ev::FlagVal { recv: 0, mode_byte });
        task.mode_word = self.mode_word_write;
        self.flag_answer
    }

    fn start_melee(&mut self, args: &StartArgs) {
        self.log.push(Ev::Starter {
            handle: args.handle.get(),
            mode: args.mode_word,
            b1: args.flag_bit1,
            b2: args.flag_bit2,
            b0: args.flag_bit0,
            b3: args.flag_bit3,
            computed: args.computed,
        });
    }

    fn timing_words(&mut self) -> InitWords {
        InitWords {
            word_dc: self.timing[0],
            word_e0: self.timing[1],
        }
    }

    fn setup_target(&mut self, _task: &mut MeleeTask, arg: u32) {
        self.log.push(Ev::Setup { recv: 0, arg });
    }

    fn register(&mut self, _task: &mut MeleeTask, arg: u32, kind: u32, flag: u32) {
        self.log.push(Ev::Register {
            recv: 0,
            arg,
            kind,
            one: flag,
        });
    }

    fn release_refs(&mut self, task: &mut MeleeTask) {
        self.log.push(Ev::Cleanup { recv: 0 });
        task.flags_f0 = self.flag_byte_write;
    }

    fn subtask_query(&mut self, sub: Option<Handle32<SubTask>>) -> u32 {
        self.log.push(Ev::Query {
            sub: Handle32::raw_or_zero(sub),
        });
        self.query_answer
    }

    fn follow_up_sink(&mut self, _task: &mut MeleeTask, arg: u32, kind: u32, zero: u32) -> u32 {
        self.log.push(Ev::Sink {
            recv: 0,
            arg,
            kind,
            zero,
        });
        self.sink_answer
    }
}

fn make_host(inp: &Input) -> Host {
    Host {
        log: Vec::new(),
        manager_word: inp.manager,
        handle_answer: inp.handle_answer,
        mode_answer: inp.mode_answer,
        flag_answer: inp.flag_answer,
        mode_word_write: inp.mode_word_write,
        flag_byte_write: inp.flag_byte_write,
        query_answer: inp.query_answer,
        sink_answer: inp.sink_answer,
        reset_base: inp.reset_base,
        stamp: inp.stamp,
        timing: inp.timing,
    }
}

fn ctor_args(inp: &Input, mem: &[u8; OBJ_LEN]) -> CtorArgs {
    CtorArgs {
        initial_word: inp.initial_word,
        flag_two: inp.flag_words[0],
        flag_three: inp.flag_words[1],
        flag_four: inp.flag_words[2],
        flag_five: inp.flag_words[3],
        target_first: inp.target[0],
        target_second: inp.target[1],
        sub_task: Handle32::new(read_u32(mem, O_SUB)),
        incoming_flags_high: mem[O_FLAGS_F1],
    }
}

/// The wrong lifts. Each is the lifted method with one change, written here
/// so that the library code stays the one under review.
fn wrong_new(host: &mut Host, args: &CtorArgs) -> MeleeTask {
    // The crossed flag bits are uncrossed: bit 0 from the second word and
    // bit 1 from the fourth, instead of the other way round.
    let mut fixed = *args;
    core::mem::swap(&mut fixed.flag_two, &mut fixed.flag_four);
    MeleeTask::new(host, &fixed)
}

fn wrong_destroy(task: &mut MeleeTask, host: &mut Host, flags: u32) {
    // The neighbour bit is tested instead of the low bit.
    host.destroy_task(task);
    if flags & 2 != 0 {
        let manager = host.manager();
        host.free_task(manager);
    }
}

fn wrong_start(task: &mut MeleeTask, host: &mut Host) {
    // Bits 1 and 2 are passed to the starter in swapped order.
    let manager = host.manager();
    let Some(handle) = host.task_handle(manager) else {
        return;
    };
    let flags = task.flags_f0;
    let mode = host.mode_value(task);
    let computed = host.flag_value(task, mode & 0xFF);
    host.start_melee(&StartArgs {
        handle,
        mode_word: task.mode_word,
        flag_bit1: u32::from((flags >> 2) & 1),
        flag_bit2: u32::from((flags >> 1) & 1),
        flag_bit0: u32::from(flags & 1),
        flag_bit3: u32::from((flags >> 3) & 1),
        computed,
    });
}

fn wrong_initialize(task: &mut MeleeTask, host: &mut Host, arg: u32) {
    // The two timing words are stored in swapped offsets.
    let words = host.timing_words();
    task.word_dc = words.word_e0;
    task.word_e0 = words.word_dc;
    task.flag_e4 = 1;
    host.setup_target(task, arg);
    host.register(task, arg, KIND, 1);
}

fn wrong_follow_up(task: &mut MeleeTask, host: &mut Host, arg: u32) -> u32 {
    // Only the kind tag is accepted; the second answer is dropped.
    host.release_refs(task);
    if task.flags_f0 & 0x20 != 0 {
        return 0;
    }
    let answer = host.subtask_query(task.sub_task);
    if answer != KIND {
        return 0;
    }
    host.follow_up_sink(task, arg, KIND, 0)
}

fn run_lift(m: Method, v: Variant, inp: &Input, sub_addr: u32) -> Outcome {
    let mut mem = inp.init;
    if m == Method::FollowUp && !inp.null_sub {
        put_u32(&mut mem, O_SUB, sub_addr);
    }
    if m == Method::FollowUp && inp.null_sub {
        put_u32(&mut mem, O_SUB, 0);
    }
    let mut host = make_host(inp);
    let mut ret = None;
    if m == Method::Ctor {
        let args = ctor_args(inp, &mem);
        let task = match v {
            Variant::Lift => MeleeTask::new(&mut host, &args),
            Variant::Wrong => wrong_new(&mut host, &args),
        };
        encode(&task, &mut mem);
    } else {
        let mut task = decode(&mem);
        match (m, v) {
            (Method::Destroy, Variant::Lift) => task.destroy(&mut host, inp.destroy_flags),
            (Method::Destroy, Variant::Wrong) => {
                wrong_destroy(&mut task, &mut host, inp.destroy_flags)
            }
            (Method::Start, Variant::Lift) => task.start(&mut host),
            (Method::Start, Variant::Wrong) => wrong_start(&mut task, &mut host),
            (Method::Init, Variant::Lift) => task.initialize(&mut host, inp.arg),
            (Method::Init, Variant::Wrong) => wrong_initialize(&mut task, &mut host, inp.arg),
            (Method::FollowUp, Variant::Lift) => {
                ret = Some(task.follow_up(&mut host, inp.arg));
            }
            (Method::FollowUp, Variant::Wrong) => {
                ret = Some(wrong_follow_up(&mut task, &mut host, inp.arg));
            }
            (Method::Ctor, _) => unreachable!("handled above"),
        }
        encode(&task, &mut mem);
    }
    Outcome {
        ret,
        mem,
        log: host.log,
    }
}

/// Runs one case through both sides. The sub-task objects live in the
/// caller so their addresses stay valid for both runs.
fn run_case(m: Method, v: Variant, inp: &Input) -> (Outcome, Outcome) {
    let sub = make_subtask();
    let rw = run_rewrite(m, inp, sub.addr);
    let lf = run_lift(m, v, inp, sub.addr);
    (rw, lf)
}

/// Installs the callee stubs the method's rewrite uses.
fn install(m: Method) {
    match m {
        Method::Ctor => setup_runtime(&[
            (1, stub!(stub_base)),
            (2, stub!(stub_reset)),
            (3, stub!(stub_set_target)),
        ]),
        Method::Destroy => setup_runtime(&[(1, stub!(stub_dtor)), (2, stub!(stub_free))]),
        Method::Start => setup_runtime(&[
            (1, stub!(stub_handle)),
            (2, stub!(stub_mode)),
            (3, stub!(stub_flagval)),
            (4, stub!(stub_starter)),
        ]),
        Method::Init => setup_runtime(&[(1, stub!(stub_setup)), (2, stub!(stub_register))]),
        Method::FollowUp => setup_runtime(&[(1, stub!(stub_cleanup)), (3, stub!(stub_sink))]),
    }
}

/// Compares the two sides for a method. Returns the number of cases that
/// differ, and the first difference found, described for a reader.
fn compare(m: Method, v: Variant, seed: u64) -> (usize, Option<String>) {
    let mut g = Gen(seed);
    let mut bad = 0usize;
    let mut first = None;
    for i in 0..CASES {
        let inp = make_input(&mut g, m);
        let (rw, lf) = run_case(m, v, &inp);
        let mut mem_equal = true;
        for (o, (a, b)) in rw.mem.iter().zip(lf.mem.iter()).enumerate() {
            // The constructor's vtable word has no lifted equivalent; the
            // rewrite's value is asserted where it runs.
            if m == Method::Ctor && o < 4 {
                continue;
            }
            if a != b {
                mem_equal = false;
                break;
            }
        }
        let same = rw.ret == lf.ret && mem_equal && rw.log == lf.log;
        if !same {
            bad += 1;
            if first.is_none() {
                let mut diff_off = None;
                for (o, (a, b)) in rw.mem.iter().zip(lf.mem.iter()).enumerate() {
                    if a != b {
                        diff_off = Some((o, a, b));
                        break;
                    }
                }
                first = Some(format!(
                    "case {i}: ret {:?} vs {:?}; first memory diff {diff_off:?}; log {:?} vs {:?}",
                    rw.ret, lf.ret, rw.log, lf.log
                ));
            }
        }
    }
    (bad, first)
}

fn check(m: Method, seed: u64) {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    install(m);
    let (bad, first) = compare(m, Variant::Lift, seed);
    assert_eq!(
        bad, 0,
        "{m:?}: {bad} of {CASES} cases differ; first: {first:?}"
    );
}

fn check_wrong_caught(m: Method, seed: u64) {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    install(m);
    let (bad, _) = compare(m, Variant::Wrong, seed);
    println!("{m:?} wrong version: {bad} of {CASES} cases differ");
    assert!(bad > 0, "{m:?}: the deliberately wrong lift was not caught");
}

// The method tests. Each method has its own seed and its wrong version.

#[test]
fn constructor_matches_rewrite() {
    check(Method::Ctor, 0xC701);
}

#[test]
fn constructor_wrong_is_caught() {
    check_wrong_caught(Method::Ctor, 0xC701);
}

#[test]
fn destroy_matches_rewrite() {
    check(Method::Destroy, 0xC702);
}

#[test]
fn destroy_wrong_is_caught() {
    check_wrong_caught(Method::Destroy, 0xC702);
}

#[test]
fn start_matches_rewrite() {
    check(Method::Start, 0xC703);
}

#[test]
fn start_wrong_is_caught() {
    check_wrong_caught(Method::Start, 0xC703);
}

#[test]
fn initialise_matches_rewrite() {
    check(Method::Init, 0xC704);
}

#[test]
fn initialise_wrong_is_caught() {
    check_wrong_caught(Method::Init, 0xC704);
}

#[test]
fn follow_up_matches_rewrite() {
    check(Method::FollowUp, 0xC705);
}

#[test]
fn follow_up_wrong_is_caught() {
    check_wrong_caught(Method::FollowUp, 0xC705);
}

/// The branch a generated case takes, named as the reader would name it.
/// Used to show that the generator reaches every branch of each method.
fn branch(m: Method, inp: &Input) -> &'static str {
    match m {
        Method::Ctor => {
            // Each combination of the four low flag bits must run.
            let bits = (inp.flag_words[0] & 1)
                | ((inp.flag_words[1] & 1) << 1)
                | ((inp.flag_words[2] & 1) << 2)
                | ((inp.flag_words[3] & 1) << 3);
            match bits {
                0 => "0000",
                1 => "0001",
                2 => "0010",
                3 => "0011",
                4 => "0100",
                5 => "0101",
                6 => "0110",
                7 => "0111",
                8 => "1000",
                9 => "1001",
                10 => "1010",
                11 => "1011",
                12 => "1100",
                13 => "1101",
                14 => "1110",
                _ => "1111",
            }
        }
        Method::Destroy => {
            if inp.destroy_flags & 1 == 0 {
                "kept"
            } else {
                "freed"
            }
        }
        Method::Start => {
            if inp.handle_answer == 0 {
                "null-handle"
            } else {
                "started"
            }
        }
        Method::Init => {
            if inp.timing[0] == inp.timing[1] {
                "timing-equal"
            } else {
                "timing-distinct"
            }
        }
        Method::FollowUp => {
            // The flag the method tests is the scripted cleanup write.
            if inp.flag_byte_write & 0x20 != 0 {
                "disabled"
            } else if inp.query_answer == ALT || inp.query_answer == KIND {
                "accepted"
            } else {
                "rejected"
            }
        }
    }
}

#[test]
fn every_branch_is_reached_by_generated_cases() {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let methods = [
        Method::Ctor,
        Method::Destroy,
        Method::Start,
        Method::Init,
        Method::FollowUp,
    ];
    let ctor_branches: [&str; 16] = [
        "0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010",
        "1011", "1100", "1101", "1110", "1111",
    ];
    let expected: [(Method, &[&str]); 5] = [
        (Method::Ctor, &ctor_branches),
        (Method::Destroy, &["kept", "freed"]),
        (Method::Start, &["null-handle", "started"]),
        (Method::Init, &["timing-equal", "timing-distinct"]),
        (Method::FollowUp, &["disabled", "accepted", "rejected"]),
    ];
    for (m, want) in expected {
        install(m);
        let pos = methods.iter().position(|x| *x == m).unwrap_or(0) as u64;
        let mut g = Gen(0xC710 + pos);
        let mut seen: std::collections::BTreeMap<&'static str, usize> = Default::default();
        let mut changed = 0usize;
        let mut logged = 0usize;
        for _ in 0..CASES {
            let inp = make_input(&mut g, m);
            *seen.entry(branch(m, &inp)).or_default() += 1;
            let sub = make_subtask();
            let rw = run_rewrite(m, &inp, sub.addr);
            if rw.mem != inp.init {
                changed += 1;
            }
            if !rw.log.is_empty() {
                logged += 1;
            }
        }
        println!("{m:?}: {seen:?}; memory changed in {changed}; calls logged in {logged}");
        for name in want {
            assert!(
                seen.get(name).copied().unwrap_or(0) > CASES / 100,
                "{m:?}: branch {name} is reached in fewer than one case in a hundred"
            );
        }
    }
}
