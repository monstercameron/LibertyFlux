//! The differential harness: test memory, the lifted host, and one runner
//! per verified method.
//!
//! A case builds the render phase's object, its parameter block, the scene
//! snapshot, the player and target objects, the query blocks, the float and
//! matrix blocks, a virtual table and the globals arena in one word array.
//! It runs the verified rewrite on that memory and records its calls, then
//! restores the memory and runs the lifted method on the same bytes, with a
//! host that records the same calls. The results, the two call logs and the
//! two final memory images must agree.

// Test-only harness: the rewrites and the lifted host read and write the
// memory this module allocated; every address is one it computed inside it.
#![allow(unsafe_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use lf_renderer::mirror_phase::registry;
use lf_renderer::mirror_phase::{MirrorGlobals, MirrorHost, MirrorReflectionPhase, Ref, Word};

use crate::rt::{self, ARENA_WORDS, CalleeSpec, Call, Conv};

/// Word offsets of the regions of the case memory.
const PHASE_AT: usize = 0;
const PHASE_WORDS: usize = 1024;
const SNAP_AT: usize = 1024;
const BLOCK_AT: usize = 1408;
const OBJECT_AT: usize = 1472;
const VTABLE_AT: usize = 1488;
const FLOATS_AT: usize = 1504;
const CURRENT_AT: usize = 1508;
const FRESH_AT: usize = 1512;
const QUERY_AT: usize = 1520;
const QUERY_COUNT: usize = 8;
const QUERY_WORDS: usize = 16;
const MEM_WORDS: usize = QUERY_AT + QUERY_COUNT * QUERY_WORDS;

/// Byte offset of the slot-30 entry in a virtual table.
const VTABLE_SLOT30_WORD: usize = 0x30 / 4;
/// Length of the parameter table the globals arena holds.
const TABLE_WORDS: usize = 64;
/// Largest swap index the generator uses: three words per entry, so the
/// entry and the one after it stay inside the table.
const MAX_SWAP_INDEX: u32 = 20;

/// Byte offsets inside the render-phase object and its parts (for the
/// memory the harness fills; the lifted code keeps its own copies).
const OBJ_FLAGS: usize = 0x24;
const PARAM_SELECTOR: usize = 0x30;
const PARAM_COUNT: usize = 0x38;
const SNAP_SELECTOR: usize = 0x538;
const SNAP_STATUS: usize = 0x558;
const PHASE_TARGET: usize = 0x938;
const PHASE_PARAMS: usize = 0x940;
const PHASE_SEEDS: [usize; 3] = [0x510, 0x514, 0x518];
const NO_TARGET: u32 = 0xFFFF_FFFF;

/// Serialises the cases: the arena and the thread-local runtime are shared
/// by everything the test binary runs.
static CASE_LOCK: Mutex<()> = Mutex::new(());

/// Edge float patterns: zeros, ones, infinities, quiet and signalling
/// not-a-number with several payloads, a subnormal, the largest finite.
const EDGE_WORDS: [u32; 16] = [
    0x0000_0000,
    0x8000_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x3F00_0000,
    0x7F80_0000,
    0xFF80_0000,
    0x7FC0_0000,
    0xFFC0_0000,
    0x7F80_0001,
    0xFF80_0001,
    0x7FC0_1234,
    0xFFC0_4321,
    0x0000_0001,
    0x7F7F_FFFF,
    0x4F00_0000,
];

/// The verified methods the harness runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    /// The deleting destructor (slot 0).
    DeletingDestructor,
    /// The refresh (slot 3).
    Refresh,
    /// The reflection (slot 4).
    Reflect,
    /// The render (slot 8).
    Render,
}

/// Small deterministic generator (splitmix64).
pub struct Rng(pub u64);

impl Rng {
    /// The next 64 bits.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// The next 32 bits.
    pub fn u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// A float pattern: an edge pattern four times in ten, otherwise any bits.
    pub fn float_bits(&mut self) -> u32 {
        if self.u32() % 10 < 4 {
            EDGE_WORDS[(self.u32() as usize) % EDGE_WORDS.len()]
        } else {
            self.u32()
        }
    }

    /// A value in `0..n`.
    pub fn below(&mut self, n: u32) -> u32 {
        self.u32() % n
    }

    /// True in `percent` cases out of a hundred.
    pub fn chance(&mut self, percent: u32) -> bool {
        self.below(100) < percent
    }
}

/// Coverage of one run: how many cases ran, and how many of them made each
/// recorded call at least once on the rewrite's side.
#[derive(Debug, Default)]
pub struct Coverage {
    /// Cases that ran and agreed.
    pub cases: usize,
    /// Cases in which each named call occurred.
    pub calls: HashMap<&'static str, usize>,
}

/// The case memory: one word array holding every region, plus the arena.
pub struct Scene {
    mem: Vec<u32>,
}

impl Scene {
    /// A zeroed scene. Its word array never grows, so its addresses hold.
    fn new() -> Self {
        Self {
            mem: vec![0; MEM_WORDS],
        }
    }

    /// The address of a word of the scene.
    fn addr(&self, word: usize) -> u32 {
        (self.mem.as_ptr() as usize as u32).wrapping_add((word * 4) as u32)
    }

    /// The whole observable state: the scene words and the arena words.
    fn image(&self) -> Vec<u32> {
        let mut image = self.mem.clone();
        image.extend_from_slice(&arena_words());
        image
    }

    /// Puts an image back, as it was when it was taken.
    fn restore(&mut self, image: &[u32]) {
        self.mem.copy_from_slice(&image[..MEM_WORDS]);
        arena_write_words(&image[MEM_WORDS..]);
    }

    /// Fills the scene with random words, then sets the fields the methods
    /// depend on to values that reach their branches.
    fn randomize(&mut self, rng: &mut Rng, method: Method) {
        for word in self.mem.iter_mut() {
            *word = rng.u32();
        }
        // Virtual table: slot 0x30 answers the recording stub.
        self.mem[VTABLE_AT + VTABLE_SLOT30_WORD] = rt::slot30_addr();
        // Objects: the first word is the virtual table.
        let vtable = self.addr(VTABLE_AT);
        self.mem[OBJECT_AT] = vtable;
        self.mem[OBJECT_AT + OBJ_FLAGS / 4] = rng.u32();
        // Floats and matrices: any patterns, not-a-number included.
        for k in 0..4 {
            self.mem[FLOATS_AT + k] = rng.float_bits();
        }
        // The parameter block: selector, count and latch words.
        let selector = if rng.chance(20) { 0 } else { rng.u32() };
        self.mem[BLOCK_AT + PARAM_SELECTOR / 4] = selector;
        // Counts at the values the comparisons turn on: zero, one, the byte
        // and word edges, the signed limits, and the words either side.
        let count = match rng.below(12) {
            0 => 0,
            1 => 1,
            2 => 255,
            3 => 256,
            4 => 257,
            5 => 0x8000_0000,
            6 => 0x7FFF_FFFF,
            7 => 0xFFFF_FFFF,
            8 => rng.u32(),
            9 => (rng.below(4) as i32).wrapping_neg() as u32,
            _ => rng.below(40),
        };
        self.mem[BLOCK_AT + PARAM_COUNT / 4] = count;
        // The phase object.
        self.mem[PHASE_AT + PHASE_TARGET / 4] = if rng.chance(20) {
            NO_TARGET
        } else {
            rng.u32()
        };
        let params = match method {
            Method::Refresh => true,
            _ => rng.chance(85),
        };
        self.mem[PHASE_AT + PHASE_PARAMS / 4] = if params { self.addr(BLOCK_AT) } else { 0 };
        for offset in PHASE_SEEDS {
            self.mem[PHASE_AT + offset / 4] = rng.float_bits();
        }
        // The snapshot: selector, and a status word whose bit two is random.
        self.mem[SNAP_AT + SNAP_SELECTOR / 4] = if rng.chance(25) { 0 } else { rng.u32() };
        self.mem[SNAP_AT + SNAP_STATUS / 4] = rng.u32();
        // Globals: gates and flags as bytes, the rest as words.
        for offset in 0..6 {
            let value = if offset == 0 {
                u8::from(rng.chance(80))
            } else if offset == 1 {
                u8::from(rng.chance(20))
            } else {
                u8::from(rng.chance(50))
            };
            arena_write_byte(offset, value);
        }
        arena_write_word(0x30, if rng.chance(80) { 0x3F80_0000 } else { rng.u32() });
        arena_write_word(0x34, if rng.chance(80) { 0x3F80_0000 } else { rng.u32() });
        arena_write_word(0x10, rng.below(2));
        arena_write_word(0x14, rng.below(MAX_SWAP_INDEX + 1));
        for k in 0..TABLE_WORDS {
            arena_write_word(0x100 + 4 * k, rng.u32());
        }
    }
}

/// The answers the callees return in one case, and the words a callee
/// writes into a frame.
#[derive(Clone, Debug)]
pub struct Plan {
    /// The answer queues, by callee name.
    pub answers: HashMap<&'static str, VecDeque<u32>>,
    /// The four words the reducer writes into the frame's second half.
    pub reduce_out: [u32; 4],
}

/// Builds the answers for one case.
fn build_plan(scene: &Scene, rng: &mut Rng) -> Plan {
    let mut answers: HashMap<&'static str, VecDeque<u32>> = HashMap::new();
    let queries: VecDeque<u32> = (0..QUERY_COUNT)
        .map(|i| {
            if rng.chance(25) {
                0
            } else {
                scene.addr(QUERY_AT + i * QUERY_WORDS)
            }
        })
        .collect();
    answers.insert("query", queries);
    let player = if rng.chance(50) {
        scene.addr(OBJECT_AT)
    } else {
        0
    };
    answers.insert("find_player", VecDeque::from([player]));
    let lookup = if rng.chance(50) {
        scene.addr(OBJECT_AT)
    } else {
        0
    };
    answers.insert("lookup_object", VecDeque::from([lookup]));
    answers.insert("fetch_floats", VecDeque::from([scene.addr(FLOATS_AT)]));
    answers.insert("current_matrix", VecDeque::from([scene.addr(CURRENT_AT)]));
    answers.insert("fresh_matrix", VecDeque::from([scene.addr(FRESH_AT)]));
    for name in [
        "probe_gate",
        "apply_six",
        "release",
        "feed_four",
        "toggle",
        "pair",
        "select",
        "commit",
        "slot30",
        "fetch_target",
        "footer",
        "bind",
        "reduce",
        "describe",
        "describe_with",
        "combine",
        "finish",
        "fallback",
        "project",
        "set_six",
        "scale_project",
        "rebuild_target",
        "destruct",
        "free_storage",
    ] {
        let mut queue: VecDeque<u32> = (0..4).map(|_| rng.u32()).collect();
        if name == "probe_gate" && rng.chance(50) {
            // A probe whose low byte is set, so the staged float is zero.
            queue[0] |= 0x40;
        }
        answers.insert(name, queue);
    }
    let reduce_out = [rng.u32(), rng.u32(), rng.u32(), rng.u32()];
    Plan {
        answers,
        reduce_out,
    }
}

/// The callee specifications of one method's rewrite, by its own numbers.
fn specs(method: Method, reduce_out: [u32; 4]) -> Vec<CalleeSpec> {
    let s = |id, name, conv, nargs, ptrs: Vec<(usize, usize)>, writes| CalleeSpec {
        id,
        name,
        conv,
        nargs,
        ptrs,
        writes,
    };
    use Conv::{Cdecl, This};
    match method {
        Method::DeletingDestructor => vec![
            s(1, "destruct", This, 1, vec![], vec![]),
            s(2, "free_storage", Cdecl, 1, vec![], vec![]),
        ],
        Method::Refresh => vec![
            s(1, "fetch_target", This, 1, vec![], vec![]),
            s(2, "rebuild_target", Cdecl, 6, vec![], vec![]),
            s(3, "lookup_object", Cdecl, 1, vec![], vec![]),
            s(5, "commit", Cdecl, 1, vec![], vec![]),
            s(6, "footer", Cdecl, 0, vec![], vec![]),
        ],
        Method::Reflect => vec![
            s(1, "bind", This, 2, vec![], vec![]),
            s(2, "reduce", This, 3, vec![(2, 8)], vec![(2, 4, reduce_out.to_vec())]),
            s(3, "describe", This, 1, vec![(0, 4)], vec![]),
            s(4, "describe_with", This, 2, vec![(0, 4)], vec![]),
            s(5, "combine", Cdecl, 5, vec![(2, 4)], vec![]),
            s(6, "finish", This, 1, vec![(0, 4)], vec![]),
            s(7, "fallback", Cdecl, 1, vec![], vec![]),
            s(8, "current_matrix", Cdecl, 0, vec![], vec![]),
            s(9, "project", This, 2, vec![(1, 4)], vec![]),
            s(10, "set_six", This, 7, vec![], vec![]),
            s(11, "scale_project", This, 2, vec![(1, 4)], vec![]),
            s(12, "fresh_matrix", Cdecl, 0, vec![], vec![]),
        ],
        Method::Render => vec![
            s(1, "query", Cdecl, 2, vec![], vec![]),
            s(2, "query", Cdecl, 2, vec![], vec![]),
            s(3, "query", Cdecl, 2, vec![], vec![]),
            s(4, "query", Cdecl, 2, vec![], vec![]),
            s(5, "probe_gate", This, 1, vec![], vec![]),
            s(6, "apply_six", This, 7, vec![], vec![]),
            s(7, "release", Cdecl, 1, vec![], vec![]),
            s(8, "fetch_floats", Cdecl, 0, vec![], vec![]),
            s(9, "feed_four", This, 3, vec![(2, 4)], vec![]),
            s(10, "toggle", This, 2, vec![], vec![]),
            s(11, "pair", This, 3, vec![], vec![]),
            s(12, "find_player", Cdecl, 1, vec![], vec![]),
            s(14, "select", Cdecl, 3, vec![], vec![]),
            s(15, "commit", Cdecl, 1, vec![], vec![]),
        ],
    }
}

/// Reads a word of the arena.
fn arena_words() -> Vec<u32> {
    // SAFETY: the arena is a static array of ARENA_WORDS words, read whole.
    unsafe { std::slice::from_raw_parts(rt::arena_ptr(), ARENA_WORDS).to_vec() }
}

/// Writes a whole arena image back.
fn arena_write_words(words: &[u32]) {
    // SAFETY: the image has the arena's length (taken by `arena_words`), and
    // the arena is written only under the case lock.
    unsafe { std::slice::from_raw_parts_mut(rt::arena_ptr(), ARENA_WORDS).copy_from_slice(words) };
}

/// Writes one arena word at a byte offset.
fn arena_write_word(offset: usize, value: u32) {
    // SAFETY: the offset lies inside the arena (every offset here is below
    // its size), and the word is written under the case lock.
    unsafe { rt::arena_ptr().cast::<u8>().add(offset).cast::<u32>().write_unaligned(value) };
}

/// Writes one arena byte at a byte offset.
fn arena_write_byte(offset: usize, value: u8) {
    // SAFETY: as for `arena_write_word`.
    unsafe { rt::arena_ptr().cast::<u8>().add(offset).write(value) };
}

/// Reads one arena byte.
fn arena_read_byte(offset: u32) -> u8 {
    // SAFETY: as for `arena_write_word`.
    unsafe { rt::arena_ptr().cast::<u8>().add(offset as usize).read() }
}

/// Reads one arena word.
fn arena_read_word(offset: u32) -> u32 {
    // SAFETY: as for `arena_write_word`.
    unsafe {
        rt::arena_ptr()
            .cast::<u8>()
            .add(offset as usize)
            .cast::<u32>()
            .read_unaligned()
    }
}

/// Writes one arena word, by its file address.
fn arena_store_at(file_va: u32, value: u32) {
    arena_write_word(rt::arena_offset(file_va) as usize, value);
}

/// Loads the phase's process-wide values from the arena, as the caller of
/// the lifted methods would.
fn load_globals() -> MirrorGlobals {
    let table_at = rt::arena_offset(0x0166_DA3C);
    MirrorGlobals {
        render_enabled: arena_read_byte(rt::arena_offset(0x0166_DA20)),
        render_skipped: arena_read_byte(rt::arena_offset(0x0166_DA21)),
        select_flag_a: arena_read_byte(rt::arena_offset(0x0166_DA22)),
        select_flag_b: arena_read_byte(rt::arena_offset(0x0179_7695)),
        float_block_enabled: arena_read_byte(rt::arena_offset(0x0105_6B50)),
        scale_enabled: arena_read_byte(rt::arena_offset(0x0179_7694)),
        scale_bits: arena_read_word(rt::arena_offset(0x00FE_8D94)),
        one_bits: arena_read_word(rt::arena_offset(0x00FE_88E8)),
        swap_enabled: arena_read_word(rt::arena_offset(0x0166_DA68)),
        swap_index: arena_read_word(rt::arena_offset(0x0166_DA6C)),
        table: (0..TABLE_WORDS as u32)
            .map(|k| arena_read_word(table_at + 4 * k))
            .collect(),
        published_selector: arena_read_word(rt::arena_offset(0x0166_DA70)),
        published_count: arena_read_word(rt::arena_offset(0x0166_DA74)),
        session: rt::relocated(0x0161_4C90),
        stamp_counter: arena_read_word(rt::arena_offset(0x0103_27A0)),
        stamp_tag_first: rt::relocated(0x00E7_E048),
        stamp_tag_final: rt::relocated(0x00E7_E080),
        trailer_first: rt::relocated(0x009C_B6A0),
        trailer_second: rt::relocated(0x00C1_06F0),
    }
}

/// Writes the globals a lifted method may change back to the arena.
fn store_globals(g: &MirrorGlobals) {
    arena_store_at(0x0166_DA70, g.published_selector);
    arena_store_at(0x0166_DA74, g.published_count);
    arena_store_at(0x0103_27A0, g.stamp_counter);
}

/// The lifted host: records each call as the rewrite's side records it, and
/// performs the memory accesses on the case memory.
pub struct LiftHost {
    this: u32,
    snap: u32,
    answers: HashMap<&'static str, VecDeque<u32>>,
    reduce_out: [u32; 4],
    /// The calls, in order.
    pub log: Vec<Call>,
}

impl LiftHost {
    fn expand(&self, object: Ref) -> u32 {
        match object {
            Ref::Phase(offset) => self.this.wrapping_add(offset),
            Ref::Snapshot(offset) => self.snap.wrapping_add(offset),
            Ref::Handle(handle) => handle,
        }
    }

    fn expand_word(&self, word: Word) -> u32 {
        match word {
            Word::Value(value) => value,
            Word::PhaseTagged(byte) => (self.this & !0xFF) | u32::from(byte),
        }
    }

    fn record(&mut self, name: &'static str, args: Vec<u32>, data: Vec<u32>) {
        self.log.push(Call { name, args, data });
    }

    fn pop(&mut self, name: &'static str) -> u32 {
        self.answers
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or(0)
    }

    fn read_words(address: u32, count: usize) -> Vec<u32> {
        (0..count)
            .map(|k| {
                // SAFETY: the address is a word the case memory holds (a
                // matrix or a float block the plan named), read in full.
                unsafe { (address as usize as *const u32).add(k).read_unaligned() }
            })
            .collect()
    }
}

impl MirrorHost for LiftHost {
    fn word(&mut self, object: Ref, offset: u32) -> u32 {
        let address = self.expand(object).wrapping_add(offset);
        // SAFETY: the object is one the case memory holds at this offset.
        unsafe { (address as usize as *const u32).read_unaligned() }
    }

    fn byte(&mut self, object: Ref, offset: u32) -> u8 {
        let address = self.expand(object).wrapping_add(offset);
        // SAFETY: as for `word`.
        unsafe { (address as usize as *const u8).read() }
    }

    fn set_word(&mut self, object: Ref, offset: u32, value: u32) {
        let address = self.expand(object).wrapping_add(offset);
        // SAFETY: as for `word`; the write is one the rewrite also makes.
        unsafe { (address as usize as *mut u32).write_unaligned(value) }
    }

    fn destruct(&mut self, this: Ref) {
        let a = self.expand(this);
        self.record("destruct", vec![a], vec![]);
        self.pop("destruct");
    }

    fn free_storage(&mut self, this: Ref) {
        let a = self.expand(this);
        self.record("free_storage", vec![a], vec![]);
        self.pop("free_storage");
    }

    fn probe_gate(&mut self, descriptor: Ref) -> u32 {
        let a = self.expand(descriptor);
        self.record("probe_gate", vec![a], vec![]);
        self.pop("probe_gate")
    }

    fn query(&mut self, kind: u32, sub: u32) -> u32 {
        self.record("query", vec![kind, sub], vec![]);
        self.pop("query")
    }

    fn apply_six(&mut self, query: Ref, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
        let q = self.expand(query);
        self.record("apply_six", vec![q, a1, a2, a3, a4, a5, a6], vec![]);
        self.pop("apply_six")
    }

    fn release(&mut self, handle: Ref) {
        let h = self.expand(handle);
        self.record("release", vec![h], vec![]);
        self.pop("release");
    }

    fn fetch_floats(&mut self) -> u32 {
        self.record("fetch_floats", vec![], vec![]);
        self.pop("fetch_floats")
    }

    fn feed_four(&mut self, query: Ref, flag: u32, block: &mut [u32; 4]) -> u32 {
        let q = self.expand(query);
        self.record("feed_four", vec![q, flag], block.to_vec());
        self.pop("feed_four")
    }

    fn toggle(&mut self, query: Ref, on: u32) -> u32 {
        let q = self.expand(query);
        self.record("toggle", vec![q, on], vec![]);
        self.pop("toggle")
    }

    fn pair(&mut self, query: Ref, a: u32, b: u32) -> u32 {
        let q = self.expand(query);
        self.record("pair", vec![q, a, b], vec![]);
        self.pop("pair")
    }

    fn find_player(&mut self, zero: u32) -> u32 {
        self.record("find_player", vec![zero], vec![]);
        self.pop("find_player")
    }

    fn select(&mut self, slot: Ref, a: u32, selector: u32) -> u32 {
        let s = self.expand(slot);
        self.record("select", vec![s, a, selector], vec![]);
        self.pop("select")
    }

    fn commit(&mut self, target: Ref) -> u32 {
        let t = self.expand(target);
        self.record("commit", vec![t], vec![]);
        self.pop("commit")
    }

    fn slot30(&mut self, object: Ref, arg: Word) -> u32 {
        let o = self.expand(object);
        let a = self.expand_word(arg);
        self.record("slot30", vec![o, a], vec![]);
        self.pop("slot30")
    }

    fn bind(&mut self, descriptor: Ref, view: Ref) -> u32 {
        let d = self.expand(descriptor);
        let v = self.expand(view);
        self.record("bind", vec![d, v], vec![]);
        self.pop("bind")
    }

    fn reduce(&mut self, params: Ref, selector: u32, frame: &mut [u32; 8]) -> u32 {
        let p = self.expand(params);
        self.record("reduce", vec![p, selector], frame.to_vec());
        frame[4..8].copy_from_slice(&self.reduce_out);
        self.pop("reduce")
    }

    fn describe(&mut self, half: &mut [u32; 4]) -> u32 {
        self.record("describe", vec![], half.to_vec());
        self.pop("describe")
    }

    fn describe_with(&mut self, half: &mut [u32; 4], descriptor: Ref) -> u32 {
        let d = self.expand(descriptor);
        self.record("describe_with", vec![d], half.to_vec());
        self.pop("describe_with")
    }

    fn combine(&mut self, selector: u32, block: Ref, half: &mut [u32; 4], phase: Ref, zero: u32) -> u32 {
        let b = self.expand(block);
        let p = self.expand(phase);
        self.record("combine", vec![selector, b, p, zero], half.to_vec());
        self.pop("combine")
    }

    fn finish(&mut self, half: &mut [u32; 4]) -> u32 {
        self.record("finish", vec![], half.to_vec());
        self.pop("finish")
    }

    fn fallback(&mut self, zero: u32) -> u32 {
        self.record("fallback", vec![zero], vec![]);
        self.pop("fallback")
    }

    fn current_matrix(&mut self) -> [u32; 4] {
        self.record("current_matrix", vec![], vec![]);
        let at = self.pop("current_matrix");
        let words = Self::read_words(at, 4);
        [words[0], words[1], words[2], words[3]]
    }

    fn project(&mut self, phase: Ref, matrix: &mut [u32; 4]) -> u32 {
        let p = self.expand(phase);
        self.record("project", vec![p], matrix.to_vec());
        self.pop("project")
    }

    fn set_six(&mut self, descriptor: Ref, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
        let d = self.expand(descriptor);
        self.record("set_six", vec![d, a1, a2, a3, a4, a5, a6], vec![]);
        self.pop("set_six")
    }

    fn fresh_matrix(&mut self) -> [u32; 4] {
        self.record("fresh_matrix", vec![], vec![]);
        let at = self.pop("fresh_matrix");
        let words = Self::read_words(at, 4);
        [words[0], words[1], words[2], words[3]]
    }

    fn scale_project(&mut self, descriptor: Ref, matrix: &mut [u32; 4]) -> u32 {
        let d = self.expand(descriptor);
        self.record("scale_project", vec![d], matrix.to_vec());
        self.pop("scale_project")
    }

    fn rebuild_target(&mut self, desc_a: Ref, desc_b: Ref, desc_c: Ref, phase: Ref, one: u32, zero: u32) {
        let a = self.expand(desc_a);
        let b = self.expand(desc_b);
        let c = self.expand(desc_c);
        let p = self.expand(phase);
        self.record("rebuild_target", vec![a, b, c, p, one, zero], vec![]);
        self.pop("rebuild_target");
    }

    fn fetch_target(&mut self, session: Ref) -> u32 {
        let s = self.expand(session);
        self.record("fetch_target", vec![s], vec![]);
        self.pop("fetch_target")
    }

    fn lookup_object(&mut self, zero: u32) -> u32 {
        self.record("lookup_object", vec![zero], vec![]);
        self.pop("lookup_object")
    }

    fn footer(&mut self) -> u32 {
        self.record("footer", vec![], vec![]);
        self.pop("footer")
    }
}

/// The outcome of one case: the two results and the rewrite's calls.
struct Outcome {
    ret_rewrite: u32,
    ret_lift: u32,
    log_rewrite: Vec<Call>,
    log_lift: Vec<Call>,
    image_rewrite: Vec<u32>,
    image_lift: Vec<u32>,
    this: u32,
    snap: u32,
}

/// Runs the rewrite on the scene and records its calls and result.
fn run_rewrite(method: Method, this: u32, snap: u32, flags: u32) -> u32 {
    use crate::rewrites::{fn_00d76910, fn_00d76950, fn_00d77430, fn_00d77600};
    match method {
        Method::DeletingDestructor => fn_00d76910::rw_00d76910(this, flags),
        Method::Refresh => fn_00d77430::rw_00d77430(this),
        Method::Reflect => fn_00d77600::rw_00d77600(this, snap),
        Method::Render => fn_00d76950::rw_00d76950(this),
    }
}

/// Runs a lifted method (or a mutant copy of it) over the scene's phase
/// words, stores the phase back, and returns the method's result word.
macro_rules! lift_run {
    ($phase:path, $method:expr, $scene:ident, $host:ident, $globals:ident, $flags:expr) => {{
        const RANGE: std::ops::Range<usize> = PHASE_AT..PHASE_AT + PHASE_WORDS;
        match $method {
            Method::DeletingDestructor => {
                <$phase>::deleting_destructor(&mut $host, $flags);
                0
            }
            Method::Refresh => {
                let mut phase = <$phase>::load(&$scene.mem[RANGE]);
                let ret = phase.refresh(&mut $host, &$globals);
                phase.store(&mut $scene.mem[RANGE]);
                ret
            }
            Method::Reflect => {
                let mut phase = <$phase>::load(&$scene.mem[RANGE]);
                let status = phase.reflect(&mut $host, &mut $globals);
                phase.store(&mut $scene.mem[RANGE]);
                u32::from(status)
            }
            Method::Render => {
                let phase = <$phase>::load(&$scene.mem[RANGE]);
                phase.render(&mut $host, &mut $globals);
                0
            }
        }
    }};
}

/// Runs one case of a method, the rewrite side first. With `mutant` set, the
/// lifted side is the deliberately wrong copy of that method instead.
fn run_case(method: Method, seed: u64, mutant: bool) -> Result<Outcome, String> {
    let mut rng = Rng(seed);
    let mut scene = Scene::new();
    scene.randomize(&mut rng, method);
    let plan = build_plan(&scene, &mut rng);
    let flags = rng.below(4);
    let this = scene.addr(PHASE_AT);
    let snap = scene.addr(SNAP_AT);
    let before = scene.image();

    rt::install(specs(method, plan.reduce_out), plan.answers.clone());
    let ret_rewrite = run_rewrite(method, this, snap, flags);
    let log_rewrite = rt::take_log();
    let image_rewrite = scene.image();

    scene.restore(&before);
    let mut host = LiftHost {
        this,
        snap,
        answers: plan.answers.clone(),
        reduce_out: plan.reduce_out,
        log: Vec::new(),
    };
    let mut globals = load_globals();
    let ret_lift = if mutant {
        match method {
            Method::DeletingDestructor => lift_run!(crate::mutants::m1::MutantPhase, method, scene, host, globals, flags),
            Method::Refresh => lift_run!(crate::mutants::m2::MutantPhase, method, scene, host, globals, flags),
            Method::Reflect => lift_run!(crate::mutants::m3::MutantPhase, method, scene, host, globals, flags),
            Method::Render => lift_run!(crate::mutants::m4::MutantPhase, method, scene, host, globals, flags),
        }
    } else {
        lift_run!(MirrorReflectionPhase, method, scene, host, globals, flags)
    };
    store_globals(&globals);
    let image_lift = scene.image();
    Ok(Outcome {
        ret_rewrite,
        ret_lift,
        log_rewrite,
        log_lift: host.log,
        image_rewrite,
        image_lift,
        this,
        snap,
    })
}

/// Checks one outcome against the rule for the method's result.
fn check(method: Method, o: &Outcome) -> Result<(), String> {
    match method {
        Method::DeletingDestructor => {
            if o.ret_rewrite != o.this {
                return Err(format!("rewrite did not return its object ({:#x})", o.ret_rewrite));
            }
        }
        Method::Render => {
            if o.ret_rewrite != 0 {
                return Err(format!("rewrite did not return zero ({:#x})", o.ret_rewrite));
            }
        }
        Method::Refresh => {
            if o.ret_rewrite != o.ret_lift {
                return Err(format!("results differ: rewrite {:#x}, lift {:#x}", o.ret_rewrite, o.ret_lift));
            }
        }
        Method::Reflect => {
            let low = o.ret_rewrite & 0xFF;
            if low != o.ret_lift {
                return Err(format!("status bit differs: rewrite {low:#x}, lift {:#x}", o.ret_lift));
            }
            if (o.ret_rewrite & !0xFF) != (o.snap & !0xFF) {
                return Err("high bits of the returned snapshot pointer differ from the argument".to_string());
            }
        }
    }
    if o.log_rewrite != o.log_lift {
        let index = o
            .log_rewrite
            .iter()
            .zip(o.log_lift.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(o.log_rewrite.len().min(o.log_lift.len()));
        return Err(format!(
            "call logs differ at call {index} (rewrite {} calls, lift {} calls): rewrite {:?} / lift {:?}",
            o.log_rewrite.len(),
            o.log_lift.len(),
            o.log_rewrite.get(index),
            o.log_lift.get(index)
        ));
    }
    if o.image_rewrite != o.image_lift {
        let index = o
            .image_rewrite
            .iter()
            .zip(o.image_lift.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        return Err(format!(
            "memory differs at word {index}: rewrite {:#x}, lift {:#x}",
            o.image_rewrite[index], o.image_lift[index]
        ));
    }
    Ok(())
}

/// Runs `cases` seeded cases of one method, and returns the coverage.
///
/// # Errors
///
/// The first case where the lift and the rewrite differ, with its seed.
pub fn run_method(method: Method, cases: usize, seed_base: u64) -> Result<Coverage, String> {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut coverage = Coverage::default();
    for index in 0..cases {
        let seed = seed_base.wrapping_add((index as u64).wrapping_mul(0x2545_F491_4F6C_DD1D));
        let outcome = run_case(method, seed, false).map_err(|e| format!("seed {seed:#x}: {e}"))?;
        check(method, &outcome).map_err(|e| format!("{method:?} seed {seed:#x}: {e}"))?;
        for call in &outcome.log_rewrite {
            *coverage.calls.entry(call.name).or_insert(0) += 1;
        }
        coverage.cases += 1;
    }
    Ok(coverage)
}

/// The number of methods the registry lists as proven (the harness covers
/// each of them).
#[must_use]
pub fn proven_methods() -> usize {
    registry::proven_count()
}


/// Runs `cases` seeded cases of one method against its deliberately wrong
/// copy, and returns how many cases the harness caught (the wrong copy must
/// be caught in at least one).
pub fn run_mutant(method: Method, cases: usize, seed_base: u64) -> usize {
    let _guard = CASE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut caught = 0;
    for index in 0..cases {
        let seed = seed_base.wrapping_add((index as u64).wrapping_mul(0x2545_F491_4F6C_DD1D));
        match run_case(method, seed, true) {
            Ok(outcome) => {
                if check(method, &outcome).is_err() {
                    caught += 1;
                }
            }
            Err(_) => caught += 1,
        }
    }
    caught
}

