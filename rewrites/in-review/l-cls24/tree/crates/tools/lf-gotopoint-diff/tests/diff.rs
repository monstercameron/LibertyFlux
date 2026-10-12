//! Differential cases: the lifted go-to-point-on-route methods against their
//! verified 32-bit rewrites.
//!
//! For every method the same generated inputs run through the rewrite and
//! through the lifted method. A case compares the return value, every byte of
//! the task object's memory, the words an output block receives, and the
//! ordered log of every call the method made (with its arguments). Floats are
//! compared as bits. A deliberately wrong lift of each method runs through
//! the same cases and must be caught.
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

use lf_gotopoint_diff::rewrites::{
    fn_00B791F0, fn_00CAF720, fn_00CAFC10, fn_00CB7480, fn_00CB8AC0, fn_00CBBB90, fn_00CBC000,
    fn_00CBC0D0,
};
use lf_peds_tasks::go_to_point_on_route::{
    CopyArgs, GoToPointOnRouteHost, GoToPointOnRouteTask, ProgressCache,
};

// Number of generated cases per method.
const CASES: usize = 20_000;
// Bytes of the task object the rewrites may touch (the highest field ends
// before offset 0xE1).
const OBJ_LEN: usize = 0x100;

// Field offsets inside the task object, as the verified rewrites use them.
const O_SPEED: usize = 0x10;
const O_PROGRESS: usize = 0x18;
const O_POSITION: usize = 0x2C;
const O_VALUE_8C: usize = 0x8C;
const O_WORD_A0: usize = 0xA0;
const O_FLAGS_B0: usize = 0xB0;
const O_FLAGS_C4: usize = 0xC4;
const O_ENTRY_VALUE: usize = 0x24;
const O_ANCHOR: usize = 0xD0;
const O_ACTIVE: usize = 0xE0;

// The advance's shared cache words, as the executable's data addresses.
const CACHE_WEIGHT_A_VA: u32 = 0x0171_BF64;
const CACHE_FLAGS_VA: u32 = 0x0171_BF68;
const CACHE_WEIGHT_B_VA: u32 = 0x0171_BF6C;
const CACHE_SCALE_VA: u32 = 0x0171_BF70;

// The allocator manager word in the executable's data, and the offset that
// places it in test memory (see `setup_globals`).
const ALLOCATOR_GLOBAL: u32 = 0x0167_E2A0;
const IMAGE_BASE: u32 = 0x40_0000;

/// Serialises the cases: the callee table and the base address are process
/// statics, and each method needs its own entries.
static CASE_LOCK: Mutex<()> = Mutex::new(());

/// The callee table the rewrites index: slot 1 and slot 2.
static CALLEE_TABLE: [AtomicU32; 256] = [const { AtomicU32::new(0) }; 256];

/// The leaked test memory standing in for the executable's data.
static GLOBAL_AREA: OnceLock<u32> = OnceLock::new();

thread_local! {
    /// The calls the rewrite side made during one case, in order.
    static LOG: RefCell<Vec<Ev>> = const { RefCell::new(Vec::new()) };
    /// The address of the task object under test on the rewrite side.
    static SRC: Cell<u32> = const { Cell::new(0) };
    /// What the allocator stub answers.
    static ALLOC_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// What the constructor stub answers.
    static CTOR_ANSWER: Cell<u32> = const { Cell::new(0) };
    /// The bits the route refresh stub sets in the flag word, as a script.
    static REFRESH_MASK: Cell<u32> = const { Cell::new(0) };
    /// What the scale helper stub answers.
    static HELPER_ANSWER: Cell<u64> = const { Cell::new(0) };
}

/// One call a method made, in the form both sides log it. Block addresses
/// are stored as offsets from the task, so the two sides compare equal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ev {
    Refresh { recv: u32, arg: u32 },
    Alloc { manager: u32 },
    Helper { lo: u32, hi: u32 },
    Construct {
        new_task: u32,
        progress: u32,
        block_40: u32,
        block_d0: u32,
        active: u32,
        word_a0: u32,
        bit0: u32,
        bit1: u32,
    },
}

// The rewrite side's callee stubs. They answer from the thread-local script
// and log what the rewrite passed them.

extern "thiscall" fn stub_refresh(this: u32, arg: u32) -> u32 {
    let src = SRC.with(Cell::get);
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Refresh {
            recv: this.wrapping_sub(src),
            arg,
        })
    });
    let mask = REFRESH_MASK.with(Cell::get);
    if mask != 0 {
        let flags = src.wrapping_add(O_FLAGS_C4 as u32) as *mut u32;
        // SAFETY: the case passes the address of a live object of OBJ_LEN bytes.
        unsafe { flags.write_unaligned(flags.read_unaligned() | mask) };
    }
    0
}

extern "thiscall" fn stub_alloc(manager: u32) -> u32 {
    LOG.with(|l| l.borrow_mut().push(Ev::Alloc { manager }));
    ALLOC_ANSWER.with(Cell::get)
}

extern "thiscall" fn stub_ctor(
    obj: u32,
    progress: u32,
    block_40: u32,
    block_d0: u32,
    active: u32,
    word_a0: u32,
    bit0: u32,
    bit1: u32,
) -> u32 {
    let src = SRC.with(Cell::get);
    LOG.with(|l| {
        l.borrow_mut().push(Ev::Construct {
            new_task: obj,
            progress,
            block_40: block_40.wrapping_sub(src),
            block_d0: block_d0.wrapping_sub(src),
            active,
            word_a0,
            bit0,
            bit1,
        })
    });
    CTOR_ANSWER.with(Cell::get)
}

/// The scale helper's stub: called with the two words of its double argument
/// and answers with the script's double.
extern "cdecl" fn stub_helper(lo: u32, hi: u32) -> u64 {
    LOG.with(|l| l.borrow_mut().push(Ev::Helper { lo, hi }));
    HELPER_ANSWER.with(Cell::get)
}

/// Points the checker runtime at the test memory: the callee table holds the
/// stubs in the slots the rewrites use, and the base address is set so that
/// the allocator global resolves to the leaked area.
fn setup_runtime(stub1: u32, stub2: u32) -> u32 {
    let area = *GLOBAL_AREA.get_or_init(|| {
        let v: &'static mut [u32] = Box::leak(vec![0u32; 0x2_8000].into_boxed_slice());
        v.as_mut_ptr() as usize as u32
    });
    CALLEE_TABLE[1].store(stub1, Ordering::SeqCst);
    CALLEE_TABLE[2].store(stub2, Ordering::SeqCst);
    // SAFETY: the statics are written only under CASE_LOCK, before any
    // rewrite runs; the table outlives the process.
    unsafe {
        lf_checker_rt::CHECKER_CTABLE = CALLEE_TABLE.as_ptr() as *const u32;
        lf_checker_rt::CHECKER_XBASE = area
            .wrapping_sub(ALLOCATOR_GLOBAL.wrapping_sub(IMAGE_BASE));
    }
    area
}

/// The allocator manager word the rewrite and the host both read.
fn manager_word() -> u32 {
    let p = lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL);
    // SAFETY: the global resolves into the leaked area set up by `setup_runtime`.
    unsafe { p.read_unaligned() }
}

fn set_manager_word(v: u32) {
    let p = lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL);
    // SAFETY: as above.
    unsafe { p.write_unaligned(v) }
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

/// Float bit patterns that the comparisons and clamps treat differently:
/// signed zeros, the clamp bounds and their neighbours, infinities, quiet and
/// signalling NaNs with payloads, subnormals and the extremes.
const F32_EDGE: [u32; 26] = [
    0x0000_0000,
    0x8000_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x3F00_0000,
    0x4040_0000,
    0x4040_0001,
    0x403F_FFFF,
    0x4080_0000,
    0x4000_0000,
    0x3FC0_0000,
    0x3F8C_CCCD,
    0x7F80_0000,
    0xFF80_0000,
    0x7FC0_0000,
    0xFFC0_0000,
    0x7F80_0001,
    0x7FC0_1234,
    0xFFC0_ABCD,
    0x0000_0001,
    0x8000_0001,
    0x7F7F_FFFF,
    0xFF7F_FFFF,
    0x0080_0000,
    0xC040_0000,
    0x3F7F_FFFF,
];

/// A float bit pattern: an edge value, a value in `[-1, 4]`, or any bits.
fn pick_f32(g: &mut Gen) -> u32 {
    match g.below(10) {
        0..=3 => F32_EDGE[g.below(F32_EDGE.len() as u32) as usize],
        4..=6 => {
            let v = (g.below(5_000_001) as f32) / 1_000_000.0 - 1.0;
            v.to_bits()
        }
        _ => g.u32(),
    }
}

/// A word the mode and force tests read: small counts, the low-byte edges and
/// the high-byte patterns a full-word test would wrongly pick up.
const WORD_EDGE: [u32; 13] = [
    0,
    1,
    2,
    3,
    0x100,
    0x101,
    0x102,
    0xFF,
    0x1FF,
    0xFFFF_FF00,
    0xFFFF_FFFF,
    0xFF00,
    0x8000_0000,
];

fn pick_word(g: &mut Gen) -> u32 {
    if g.below(10) < 6 {
        WORD_EDGE[g.below(WORD_EDGE.len() as u32) as usize]
    } else {
        g.u32()
    }
}

// Inputs.

/// One generated case: the task object's bytes, the arguments, the source
/// block and the scripted answers of the callees.
#[derive(Clone, Copy)]
struct Input {
    init: [u8; OBJ_LEN],
    word1: u32,
    word2: u32,
    src: [u32; 4],
    manager: u32,
    alloc_answer: u32,
    ctor_answer: u32,
    refresh_mask: u32,
    /// The advance's target point, as two float words.
    target: [u32; 2],
    /// The flag word of the ped's link, as the advance reads it.
    ped_link: u32,
    /// The ped's position block, as two float words.
    ped_pos: [u32; 2],
    /// The scale helper's answer.
    helper_answer: u64,
    /// The shared cache: flags, first weight, second weight, scale.
    cache: [u32; 4],
}

/// The fields of the task object, as the lifted type reads them.
fn put_u32(b: &mut [u8; OBJ_LEN], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn make_input(g: &mut Gen, m: Method) -> Input {
    let mut init = [0u8; OBJ_LEN];
    for b in init.iter_mut() {
        *b = g.u32() as u8;
    }
    let mut inp = Input {
        init,
        word1: pick_f32(g),
        word2: pick_word(g),
        src: [0; 4],
        manager: pick_word(g),
        alloc_answer: if g.below(10) < 3 { 0 } else { g.u32() | 1 },
        ctor_answer: g.u32(),
        refresh_mask: [0, 0x1, 0x2, 0x3, 0x10, 0x20, 0x8000_0000, g.u32() & 0xFF][g.below(8) as usize],
        target: [pick_f32(g), pick_f32(g)],
        ped_link: g.u32(),
        ped_pos: [pick_f32(g), pick_f32(g)],
        helper_answer: g.next(),
        cache: [pick_f32(g), g.u32() & 0x7, pick_f32(g), pick_f32(g)],
    };
    // Position and progress words.
    for k in 0..4 {
        put_u32(&mut inp.init, O_POSITION + 4 * k, pick_f32(g));
    }
    put_u32(&mut inp.init, O_PROGRESS, pick_f32(g));
    put_u32(&mut inp.init, O_VALUE_8C, pick_f32(g));
    put_u32(&mut inp.init, O_WORD_A0, g.u32());
    put_u32(&mut inp.init, O_FLAGS_B0, g.u32());
    put_u32(&mut inp.init, O_FLAGS_C4, g.u32());
    inp.init[O_ACTIVE] = if g.below(2) == 0 { 0 } else { g.u32() as u8 };
    match m {
        Method::Positive | Method::Clone => {}
        Method::Clamp => {}
        Method::Copy3 => {}
        Method::SetPosition => {
            // Source: equal to the stored position, equal up to zero signs,
            // or unrelated; the first three words decide replacement.
            for k in 0..4 {
                let stored_word = read_u32(&inp.init, O_POSITION + 4 * k);
                inp.src[k] = match g.below(10) {
                    0..=3 => stored_word,
                    4 => stored_word ^ 0x8000_0000,
                    _ => pick_f32(g),
                };
            }
            inp.word2 = match g.below(10) {
                0..=3 => pick_word(g),
                _ => WORD_EDGE[g.below(WORD_EDGE.len() as u32) as usize],
            };
        }
        Method::Set8c => {
            // A zero is stored in a quarter of the cases, with either sign,
            // so that the sign-flipped incoming value compares equal.
            if g.below(4) == 0 {
                put_u32(&mut inp.init, O_VALUE_8C, if g.below(2) == 0 { 0 } else { 0x8000_0000 });
            }
            // Incoming: the stored bits, the stored value with its sign
            // flipped (equal when zero), or any value.
            let stored = read_u32(&inp.init, O_VALUE_8C);
            inp.word1 = match g.below(10) {
                0..=3 => stored,
                4 => stored ^ 0x8000_0000,
                _ => pick_f32(g),
            };
        }
        Method::Advance => {
            // Mostly active, mostly not held, and an entry value above the
            // idle bound, so that the blend runs in most cases.
            inp.init[O_ACTIVE] = if g.below(10) < 9 { 1 + (g.u32() as u8 % 255) } else { 0 };
            inp.ped_link = if g.below(10) < 8 { inp.ped_link & !0b10 } else { inp.ped_link | 0b10 };
            let entry = if g.below(10) < 6 {
                (1.0f32 + (g.below(3_000_001) as f32) / 1_000_000.0).to_bits()
            } else {
                pick_f32(g)
            };
            put_u32(&mut inp.init, O_ENTRY_VALUE, entry);
            put_u32(&mut inp.init, O_ANCHOR, pick_f32(g));
            put_u32(&mut inp.init, O_ANCHOR + 4, pick_f32(g));
            // Points near the origin, so that the distances fall on both
            // sides of the thresholds.
            inp.target = [small_f32(g), small_f32(g)];
            inp.ped_pos = [small_f32(g), small_f32(g)];
            // Cache flags: any combination of fill bits, sometimes a zero scale.
            inp.cache = [
                pick_f32(g),
                g.u32() & 0x7,
                pick_f32(g),
                if g.below(5) == 0 { 0 } else { pick_f32(g) },
            ];
        }
        Method::Refresh => {
            inp.word2 = match g.below(10) {
                0..=6 => WORD_EDGE[g.below(WORD_EDGE.len() as u32) as usize],
                _ => g.u32(),
            };
        }
    }
    inp
}

/// A float in `[-2, 4)` with a quarter-step grid, or an edge value one in
/// eight times.
fn small_f32(g: &mut Gen) -> u32 {
    if g.below(8) == 0 {
        return pick_f32(g);
    }
    ((g.below(25) as f32) * 0.25 - 2.0).to_bits()
}

fn read_u32(b: &[u8; OBJ_LEN], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

/// The methods under test.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Method {
    Positive,
    Clamp,
    Copy3,
    SetPosition,
    Set8c,
    Refresh,
    Clone,
    Advance,
}

/// Which side of the comparison a run exercises: the lift itself, or one of
/// the deliberately wrong lifts that must be caught.
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
    out: Option<[u32; 3]>,
    cache: Option<[u32; 4]>,
    log: Vec<Ev>,
}

/// Reads the shared cache words from the test memory.
fn read_cache() -> [u32; 4] {
    [CACHE_WEIGHT_A_VA, CACHE_FLAGS_VA, CACHE_WEIGHT_B_VA, CACHE_SCALE_VA].map(|va| {
        let p = lf_checker_rt::global::<u32>(va);
        // SAFETY: the address resolves into the leaked area set up by `setup_runtime`.
        unsafe { p.read_unaligned() }
    })
}

/// Writes the shared cache words into the test memory.
fn write_cache(words: [u32; 4]) {
    for (va, w) in [CACHE_WEIGHT_A_VA, CACHE_FLAGS_VA, CACHE_WEIGHT_B_VA, CACHE_SCALE_VA]
        .into_iter()
        .zip(words)
    {
        let p = lf_checker_rt::global::<u32>(va);
        // SAFETY: as above.
        unsafe { p.write_unaligned(w) }
    }
}

// The rewrite side.

fn run_rewrite(m: Method, inp: &Input) -> Outcome {
    let mut obj = Box::new(inp.init);
    let addr = obj.as_mut_ptr() as usize as u32;
    let mut srcbuf = Box::new(inp.src);
    let src_addr = srcbuf.as_mut_ptr() as usize as u32;
    let mut outbuf = Box::new([0u32; 3]);
    let out_addr = outbuf.as_mut_ptr() as usize as u32;

    // The advance's ped: a link object and a position block, placed in test
    // memory where the rewrite's pointer reads find them.
    let mut ped = Box::new([0u8; 0xB00]);
    let mut link = Box::new([0u8; 0x60]);
    let mut posblk = Box::new([0u8; 0x40]);
    put_u32_slice(&mut ped[..], 0xA80, link.as_mut_ptr() as usize as u32);
    put_u32_slice(&mut ped[..], 0x20, posblk.as_mut_ptr() as usize as u32);
    put_u32_slice(&mut link[..], 0x50, inp.ped_link);
    put_u32_slice(&mut posblk[..], 0x30, inp.ped_pos[0]);
    put_u32_slice(&mut posblk[..], 0x34, inp.ped_pos[1]);
    let mut target = Box::new(inp.target);
    let target_addr = target.as_mut_ptr() as usize as u32;
    let ped_addr = ped.as_mut_ptr() as usize as u32;
    write_cache(inp.cache);

    set_manager_word(inp.manager);
    SRC.with(|s| s.set(addr));
    HELPER_ANSWER.with(|h| h.set(inp.helper_answer));
    ALLOC_ANSWER.with(|a| a.set(inp.alloc_answer));
    CTOR_ANSWER.with(|c| c.set(inp.ctor_answer));
    REFRESH_MASK.with(|r| r.set(inp.refresh_mask));
    LOG.with(|l| l.borrow_mut().clear());

    let ret = match m {
        Method::Positive => Some(fn_00B791F0::rw_00b791f0(addr)),
        Method::Clamp => {
            fn_00CBBB90::rw_00cbbb90(addr, inp.word1, 0);
            None
        }
        Method::Copy3 => {
            fn_00CB7480::rw_00cb7480(addr, out_addr);
            None
        }
        Method::SetPosition => {
            fn_00CBC000::rw_00cbc000(addr, 0xDEAD_BEEF, src_addr, inp.word2);
            None
        }
        Method::Set8c => {
            fn_00CBC0D0::rw_00cbc0d0(addr, f32::from_bits(inp.word1));
            None
        }
        Method::Refresh => Some(fn_00CB8AC0::rw_00cb8ac0(addr, inp.word1, inp.word2, 0x5A5A_5A5A)),
        Method::Clone => Some(fn_00CAF720::rw_00caf720(addr)),
        Method::Advance => {
            // The return register is not defined on every path of the
            // original (the rewrite's comment says so): not compared.
            fn_00CAFC10::rw_00CAFC10(addr, ped_addr, target_addr);
            None
        }
    };
    let out = if m == Method::Copy3 { Some(*outbuf) } else { None };
    let cache = if m == Method::Advance { Some(read_cache()) } else { None };
    Outcome {
        ret,
        mem: *obj,
        out,
        cache,
        log: LOG.with(|l| std::mem::take(&mut *l.borrow_mut())),
    }
}

/// Writes a word into a byte buffer at an offset.
fn put_u32_slice(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

// The lifted side: the host records the calls the lifted method makes.

struct Host {
    log: Vec<Ev>,
    manager: u32,
    alloc_answer: u32,
    ctor_answer: u32,
    refresh_mask: u32,
    ped_link: u32,
    ped_pos: [f32; 2],
    helper_answer: u64,
}

impl GoToPointOnRouteHost for Host {
    fn refresh_route(&mut self, task: &mut GoToPointOnRouteTask, arg: u32) {
        self.log.push(Ev::Refresh { recv: 0, arg });
        task.flags_c4 |= self.refresh_mask;
    }

    fn allocate_task(&mut self) -> u32 {
        self.log.push(Ev::Alloc {
            manager: self.manager,
        });
        self.alloc_answer
    }

    fn construct_copy(&mut self, new_task: u32, args: &CopyArgs) -> u32 {
        let (b40, bd0) = (args.block_40.0, args.block_d0.0);
        self.log.push(Ev::Construct {
            new_task,
            progress: args.progress_bits,
            block_40: b40,
            block_d0: bd0,
            active: args.active,
            word_a0: args.word_a0,
            bit0: args.flag_bit_0,
            bit1: args.flag_bit_1,
        });
        self.ctor_answer
    }

    fn ped_link_flags(&mut self, _ped: u32) -> u32 {
        self.ped_link
    }

    fn ped_position(&mut self, _ped: u32) -> [f32; 2] {
        self.ped_pos
    }

    fn double_helper(&mut self, arg_bits: u64) -> u64 {
        self.log.push(Ev::Helper {
            lo: arg_bits as u32,
            hi: (arg_bits >> 32) as u32,
        });
        self.helper_answer
    }
}

fn decode(b: &[u8; OBJ_LEN]) -> GoToPointOnRouteTask {
    GoToPointOnRouteTask {
        speed: f32::from_bits(read_u32(b, O_SPEED)),
        progress: f32::from_bits(read_u32(b, O_PROGRESS)),
        position: [
            f32::from_bits(read_u32(b, O_POSITION)),
            f32::from_bits(read_u32(b, O_POSITION + 4)),
            f32::from_bits(read_u32(b, O_POSITION + 8)),
            f32::from_bits(read_u32(b, O_POSITION + 12)),
        ],
        value_8c: f32::from_bits(read_u32(b, O_VALUE_8C)),
        entry_value: f32::from_bits(read_u32(b, O_ENTRY_VALUE)),
        anchor: [
            f32::from_bits(read_u32(b, O_ANCHOR)),
            f32::from_bits(read_u32(b, O_ANCHOR + 4)),
        ],
        word_a0: read_u32(b, O_WORD_A0),
        flags_b0: read_u32(b, O_FLAGS_B0),
        flags_c4: read_u32(b, O_FLAGS_C4),
        active: b[O_ACTIVE],
    }
}

fn encode(t: &GoToPointOnRouteTask, b: &mut [u8; OBJ_LEN]) {
    put_u32(b, O_SPEED, t.speed.to_bits());
    put_u32(b, O_PROGRESS, t.progress.to_bits());
    for (k, w) in t.position.iter().enumerate() {
        put_u32(b, O_POSITION + 4 * k, w.to_bits());
    }
    put_u32(b, O_VALUE_8C, t.value_8c.to_bits());
    put_u32(b, O_ENTRY_VALUE, t.entry_value.to_bits());
    put_u32(b, O_ANCHOR, t.anchor[0].to_bits());
    put_u32(b, O_ANCHOR + 4, t.anchor[1].to_bits());
    put_u32(b, O_WORD_A0, t.word_a0);
    put_u32(b, O_FLAGS_B0, t.flags_b0);
    put_u32(b, O_FLAGS_C4, t.flags_c4);
    b[O_ACTIVE] = t.active;
}

/// The wrong lifts. Each is the lifted method with one change, written here
/// so that the library code stays the one under review.
fn wrong_positive(t: &GoToPointOnRouteTask) -> u32 {
    // Neighbour predicate: zero counts as positive.
    u32::from(t.progress >= 0.0)
}

fn wrong_clamp(t: &mut GoToPointOnRouteTask, requested: f32) {
    // Negated test: a NaN request is clamped to zero.
    t.speed = if !(requested >= 0.0) {
        0.0
    } else if requested > 3.0 {
        3.0
    } else {
        requested
    };
}

fn wrong_copy3(t: &GoToPointOnRouteTask) -> [f32; 3] {
    // Second and first coordinates swapped.
    [t.position[1], t.position[0], t.position[2]]
}

fn wrong_set_position(t: &mut GoToPointOnRouteTask, source: [f32; 4], force: u32) {
    // Full-word force test instead of the low byte.
    let differs = t.position[0] != source[0]
        || t.position[1] != source[1]
        || t.position[2] != source[2];
    if force != 0 || differs {
        t.position = source;
        t.flags_b0 = (t.flags_b0 & 0xFFFF_87FF) | 0x100;
    }
}

fn wrong_set_8c(t: &mut GoToPointOnRouteTask, incoming: f32) {
    // Unconditional store: equal values with different signs are rewritten.
    t.value_8c = incoming;
}

fn wrong_refresh<H: GoToPointOnRouteHost>(
    t: &mut GoToPointOnRouteTask,
    host: &mut H,
    requested: u32,
    mode: u32,
) -> u32 {
    // The route refresh is handed the mode instead of the requested word.
    let _ = requested;
    host.refresh_route(t, mode);
    if mode == 1 || mode == 2 {
        (mode & 0xFFFF_FF00) | 1
    } else {
        t.flags_c4 |= 0x10;
        mode & 0xFFFF_FF00
    }
}

fn wrong_clone<H: GoToPointOnRouteHost>(t: &GoToPointOnRouteTask, host: &mut H) -> u32 {
    // The two block operands are passed in swapped order.
    let new_task = host.allocate_task();
    if new_task == 0 {
        return 0;
    }
    host.construct_copy(
        new_task,
        &CopyArgs {
            progress_bits: t.progress.to_bits(),
            block_40: lf_peds_tasks::go_to_point_on_route::TaskField(0xD0),
            block_d0: lf_peds_tasks::go_to_point_on_route::TaskField(0x40),
            active: u32::from(t.active),
            word_a0: t.word_a0,
            flag_bit_0: t.flags_c4 & 1,
            flag_bit_1: (t.flags_c4 >> 1) & 1,
        },
    )
}

fn wrong_advance<H: GoToPointOnRouteHost>(
    t: &mut GoToPointOnRouteTask,
    host: &mut H,
    cache: &mut ProgressCache,
    target: [f32; 2],
) {
    // The idle test is negated: a NaN entry value takes the idle store and
    // leaves the cache unfilled.
    if !(t.entry_value > 1.0) && t.active != 0 {
        t.progress = t.entry_value;
        return;
    }
    t.advance_progress(host, cache, 0x1234, target);
}

fn run_lift(m: Method, v: Variant, inp: &Input) -> Outcome {
    let mut mem = inp.init;
    let mut t = decode(&mem);
    let mut host = Host {
        log: Vec::new(),
        manager: manager_word(),
        alloc_answer: inp.alloc_answer,
        ctor_answer: inp.ctor_answer,
        refresh_mask: inp.refresh_mask,
        ped_link: inp.ped_link,
        ped_pos: inp.ped_pos.map(f32::from_bits),
        helper_answer: inp.helper_answer,
    };
    let mut cache = ProgressCache {
        flags: inp.cache[1],
        weight_a: inp.cache[0],
        weight_b: inp.cache[2],
        scale: inp.cache[3],
    };
    let mut out = None;
    let ret = match (m, v) {
        (Method::Positive, Variant::Lift) => Some(u32::from(t.progress_is_positive())),
        (Method::Positive, Variant::Wrong) => Some(wrong_positive(&t)),
        (Method::Clamp, Variant::Lift) => {
            t.set_speed(f32::from_bits(inp.word1));
            None
        }
        (Method::Clamp, Variant::Wrong) => {
            wrong_clamp(&mut t, f32::from_bits(inp.word1));
            None
        }
        (Method::Copy3, Variant::Lift) => {
            out = Some(t.position_point().map(f32::to_bits));
            None
        }
        (Method::Copy3, Variant::Wrong) => {
            out = Some(wrong_copy3(&t).map(f32::to_bits));
            None
        }
        (Method::SetPosition, Variant::Lift) => {
            let src = inp.src.map(f32::from_bits);
            t.set_position(src, inp.word2);
            None
        }
        (Method::SetPosition, Variant::Wrong) => {
            let src = inp.src.map(f32::from_bits);
            wrong_set_position(&mut t, src, inp.word2);
            None
        }
        (Method::Set8c, Variant::Lift) => {
            t.set_value_8c(f32::from_bits(inp.word1));
            None
        }
        (Method::Set8c, Variant::Wrong) => {
            wrong_set_8c(&mut t, f32::from_bits(inp.word1));
            None
        }
        (Method::Refresh, Variant::Lift) => Some(t.refresh_mode(&mut host, inp.word1, inp.word2)),
        (Method::Refresh, Variant::Wrong) => Some(wrong_refresh(&mut t, &mut host, inp.word1, inp.word2)),
        (Method::Clone, Variant::Lift) => Some(t.clone_task(&mut host)),
        (Method::Clone, Variant::Wrong) => Some(wrong_clone(&t, &mut host)),
        (Method::Advance, Variant::Lift) => {
            t.advance_progress(&mut host, &mut cache, 0x1234, inp.target.map(f32::from_bits));
            None
        }
        (Method::Advance, Variant::Wrong) => {
            wrong_advance(&mut t, &mut host, &mut cache, inp.target.map(f32::from_bits));
            None
        }
    };
    encode(&t, &mut mem);
    let cache_out = if m == Method::Advance {
        Some([cache.weight_a, cache.flags, cache.weight_b, cache.scale])
    } else {
        None
    };
    Outcome {
        ret,
        mem,
        out,
        cache: cache_out,
        log: host.log,
    }
}

/// Runs one case through both sides under the test's runtime script.
fn run_case(m: Method, v: Variant, inp: &Input) -> (Outcome, Outcome) {
    set_manager_word(inp.manager);
    let rw = run_rewrite(m, inp);
    let lf = run_lift(m, v, inp);
    (rw, lf)
}

/// Installs the callee stubs the method's rewrite uses.
fn install(m: Method) {
    match m {
        Method::Refresh => {
            setup_runtime(stub_refresh as *const () as usize as u32, 0);
        }
        Method::Clone => {
            setup_runtime(stub_alloc as *const () as usize as u32, stub_ctor as *const () as usize as u32);
        }
        Method::Advance => {
            setup_runtime(stub_helper as *const () as usize as u32, 0);
        }
        _ => {
            setup_runtime(0, 0);
        }
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
        let ret_ok = match (rw.ret, lf.ret) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };
        let same = ret_ok
            && rw.mem == lf.mem
            && rw.out == lf.out
            && rw.cache == lf.cache
            && rw.log == lf.log;
        if !same {
            bad += 1;
            if first.is_none() {
                first = Some(format!(
                    "case {i}: ret {:?} vs {:?}; memory equal {}; out {:?} vs {:?}; log {:?} vs {:?}",
                    rw.ret,
                    lf.ret,
                    rw.mem == lf.mem,
                    rw.out,
                    lf.out,
                    rw.log,
                    lf.log
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
    assert_eq!(bad, 0, "{m:?}: {bad} of {CASES} cases differ; first: {first:?}");
}

fn check_wrong_caught(m: Method, seed: u64) {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    install(m);
    let (bad, _) = compare(m, Variant::Wrong, seed);
    assert!(bad > 0, "{m:?}: the deliberately wrong lift was not caught");
}

// The method tests. Each method has its own seed and its wrong version.

#[test]
fn positive_progress_matches_rewrite() {
    check(Method::Positive, 0x6001);
}

#[test]
fn positive_progress_wrong_is_caught() {
    check_wrong_caught(Method::Positive, 0x6001);
}

#[test]
fn speed_clamp_matches_rewrite() {
    check(Method::Clamp, 0x6002);
}

#[test]
fn speed_clamp_wrong_is_caught() {
    check_wrong_caught(Method::Clamp, 0x6002);
}

#[test]
fn position_point_copy_matches_rewrite() {
    check(Method::Copy3, 0x6003);
}

#[test]
fn position_point_copy_wrong_is_caught() {
    check_wrong_caught(Method::Copy3, 0x6003);
}

#[test]
fn position_set_matches_rewrite() {
    check(Method::SetPosition, 0x6004);
}

#[test]
fn position_set_wrong_is_caught() {
    check_wrong_caught(Method::SetPosition, 0x6004);
}

#[test]
fn value_8c_set_matches_rewrite() {
    check(Method::Set8c, 0x6005);
}

#[test]
fn value_8c_set_wrong_is_caught() {
    check_wrong_caught(Method::Set8c, 0x6005);
}

#[test]
fn refresh_mode_matches_rewrite() {
    check(Method::Refresh, 0x6006);
}

#[test]
fn refresh_mode_wrong_is_caught() {
    check_wrong_caught(Method::Refresh, 0x6006);
}

#[test]
fn clone_matches_rewrite() {
    check(Method::Clone, 0x6007);
}

#[test]
fn clone_wrong_is_caught() {
    check_wrong_caught(Method::Clone, 0x6007);
}

/// The branch a generated case takes, named as the reader would name it.
/// Used to show that the generator reaches every branch of each method.
fn branch(m: Method, inp: &Input) -> &'static str {
    let f = |off: usize| f32::from_bits(read_u32(&inp.init, off));
    match m {
        Method::Positive => {
            let p = f(O_PROGRESS);
            if p.is_nan() {
                "nan"
            } else if p > 0.0 {
                "positive"
            } else if p == 0.0 {
                "zero"
            } else {
                "negative"
            }
        }
        Method::Clamp => {
            let x = f32::from_bits(inp.word1);
            if x.is_nan() {
                "nan"
            } else if x < 0.0 {
                "below"
            } else if x > 3.0 {
                "above"
            } else {
                "inside"
            }
        }
        Method::Copy3 => "copy",
        Method::SetPosition => {
            let differs = (0..3).any(|k| {
                f32::from_bits(read_u32(&inp.init, O_POSITION + 4 * k))
                    != f32::from_bits(inp.src[k])
            });
            let forced = (inp.word2 as u8) != 0;
            match (forced, differs) {
                (true, true) => "forced-and-differs",
                (true, false) => "forced-equal",
                (false, true) => "differs-unforced",
                (false, false) => "equal-unforced",
            }
        }
        Method::Set8c => {
            let stored = f(O_VALUE_8C);
            let incoming = f32::from_bits(inp.word1);
            if stored.is_nan() || incoming.is_nan() {
                "nan"
            } else if stored == incoming && stored.to_bits() != incoming.to_bits() {
                "equal-different-sign"
            } else if stored == incoming {
                "equal-same-bits"
            } else {
                "different"
            }
        }
        Method::Refresh => {
            if inp.word2 == 1 || inp.word2 == 2 {
                "mode-one-or-two"
            } else {
                "other-mode"
            }
        }
        Method::Clone => {
            if inp.alloc_answer == 0 {
                "null-allocation"
            } else {
                "allocated"
            }
        }
        Method::Advance => {
            let entry = f(O_ENTRY_VALUE);
            if inp.init[O_ACTIVE] == 0 {
                "idle"
            } else if (inp.ped_link & 0b10) != 0 {
                "held"
            } else if entry.is_nan() {
                "nan-entry"
            } else if entry <= 1.0 {
                "entry-at-or-below-one"
            } else if inp.cache[3] == 0 {
                "zero-scale"
            } else {
                "blend"
            }
        }
    }
}

#[test]
fn every_branch_is_reached_by_generated_cases() {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let methods = [
        Method::Positive,
        Method::Clamp,
        Method::Copy3,
        Method::SetPosition,
        Method::Set8c,
        Method::Refresh,
        Method::Clone,
        Method::Advance,
    ];
    let expected: [(Method, &[&str]); 8] = [
        (Method::Positive, &["nan", "positive", "zero", "negative"]),
        (Method::Clamp, &["nan", "below", "above", "inside"]),
        (Method::Copy3, &["copy"]),
        (
            Method::SetPosition,
            &["forced-and-differs", "forced-equal", "differs-unforced", "equal-unforced"],
        ),
        (
            Method::Set8c,
            &["nan", "equal-different-sign", "equal-same-bits", "different"],
        ),
        (Method::Refresh, &["mode-one-or-two", "other-mode"]),
        (Method::Clone, &["null-allocation", "allocated"]),
        (
            Method::Advance,
            &["idle", "held", "nan-entry", "entry-at-or-below-one", "zero-scale", "blend"],
        ),
    ];
    for (m, want) in expected {
        install(m);
        let mut g = Gen(0x6100 + methods.iter().position(|x| *x == m).unwrap_or(0) as u64);
        let mut seen: std::collections::BTreeMap<&'static str, usize> = Default::default();
        let mut changed = 0usize;
        let mut logged = 0usize;
        for _ in 0..CASES {
            let inp = make_input(&mut g, m);
            *seen.entry(branch(m, &inp)).or_default() += 1;
            let rw = run_rewrite(m, &inp);
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

#[test]
fn advance_progress_matches_rewrite() {
    check(Method::Advance, 0x6008);
}

#[test]
fn advance_progress_wrong_is_caught() {
    check_wrong_caught(Method::Advance, 0x6008);
}
