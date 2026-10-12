//! Differential proof of the lifted base sound class against its verified rewrites.
//!
//! Each lifted method runs on the same generated inputs as its verified
//! rewrite. Both run in the same test arena: the arena is restored to one
//! image before each side, the scripted stubs answer the same queues, and
//! the case compares three things: the return value, every call the method
//! made into the engine (callee or virtual slot, with its arguments, in
//! order), and every byte of the arena afterwards. Floats are not involved
//! in these methods: the float bits travel as words and are compared as
//! words.
//!
//! Each method also has a deliberately wrong lift (a copy with one change),
//! which must be caught on some case. The wrong lifts live in this file, not
//! in the lifted crate.
//!
//! Runs on the 32-bit target only (the rewrites take real addresses).

#![cfg(target_arch = "x86")]
// The cases call the rewrites and the lifted world through raw addresses in
// the test arena. Every access is checked against the arena's bounds by the
// runtime's helpers, and the rewrites' own accesses stay inside the arena
// the case builder laid out.
#![allow(unsafe_code)]

use lf_audio::sound_base::{
    AudSound, FallbackTag, ObjectTag, RegistryTag, SlotTag, SoundWorld, FAST_BIT, KIND_FAST,
    NO_VOICE, ROW_BIAS_INIT, ROW_BIAS_LOOKUP, ROW_STRIDE,
};
use lf_core::Handle32;
use lf_soundbase_diff::rewrites::{fn_00890800, fn_00890870, fn_00891680, fn_008916A0, fn_00891E90, fn_0088F700, fn_0088F7C0};
use lf_soundbase_diff::rt;

/// Cases per method in the proof run.
const CASES: u32 = 300;
/// Init-table sub-ids the test maps. The init table sits at a file address
/// whose later entries alias other globals in the test's mapping, so sub-ids
/// at and above 44 are not generated (see the registry narrowing).
const SUB_IDS: u32 = 44;
/// Arena offset of the object's virtual table pointer, class-side.
/// Size of the sound object in bytes (the fields and the bytes around them).
const OBJECT_SIZE: u32 = 0xB0;

/// A tiny deterministic generator (xorshift), so every run is the same.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    fn below(&mut self, n: u32) -> u32 {
        self.next() % n
    }
    fn chance(&mut self, one_in: u32) -> bool {
        self.below(one_in) == 0
    }
}

/// The seven methods under test, one per verified rewrite that is lifted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Method {
    Find,
    FindByName,
    StoreAtKey,
    HashForward,
    ForwardLink,
    Dtor,
    Construct2,
}

const ALL: [Method; 7] = [
    Method::Find,
    Method::FindByName,
    Method::StoreAtKey,
    Method::HashForward,
    Method::ForwardLink,
    Method::Dtor,
    Method::Construct2,
];

/// One generated case: the object, the globals, the inputs and the scripts.
#[derive(Clone)]
struct Case {
    kind: Method,
    sound: AudSound,
    scalar_a: u32,
    scalar_b: u32,
    lookup_rows: [u32; 32],
    init_rows: [u32; 32],
    filler: Vec<u8>,
    hash: u32,
    key: u32,
    bits: u32,
    arg: u32,
    free: u32,
    name: u32,
    answers: Vec<(&'static str, Vec<u32>)>,
    callees: Vec<(u32, u32)>,
}

/// What one side of a case produced.
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    ret: u32,
    log: Vec<String>,
    image: Vec<u8>,
}

fn obj_addr() -> u32 {
    rt::arena_base() + rt::SOUND_OFF
}

fn handle(raw: u32) -> Handle32<ObjectTag> {
    Handle32::new(raw).expect("nonzero object address")
}

fn aligned_arena_address(rng: &mut Rng) -> u32 {
    let span = rt::ARENA_LEN as u32 - 8;
    (rt::arena_base() + (rng.below(span) & !3)) & !3
}

fn arena_address(rng: &mut Rng) -> u32 {
    rt::arena_base() + rng.below(rt::ARENA_LEN as u32 - 8)
}

fn gen_case(rng: &mut Rng, kind: Method) -> Case {
    let scalars = [0x40u32, 0x10, 0x80];
    let scalar_a = scalars[rng.below(3) as usize];
    let scalar_b = scalars[rng.below(3) as usize];
    let base = rt::arena_base();
    let mut lookup_rows = [0u32; 32];
    let mut init_rows = [0u32; 32];
    for v in 0..32 {
        lookup_rows[v] = base + rt::OBJECTS_OFF + 0x40 * rng.below(8);
        init_rows[v] = rng.next();
    }
    let mut slot_ids = [NO_VOICE; 8];
    for id in &mut slot_ids {
        if !rng.chance(4) {
            *id = rng.next() as u8;
        }
    }
    let sound = AudSound {
        voice_column: if rng.chance(4) { NO_VOICE } else { rng.next() as u8 },
        link: if rng.chance(6) { NO_VOICE } else { rng.next() as u8 },
        kind: if rng.chance(3) { KIND_FAST } else { rng.next() as u16 },
        fast_flags: if rng.chance(2) { FAST_BIT | rng.next() as u8 } else { rng.next() as u8 & !FAST_BIT },
        init_flags: rng.next() as u8,
        sub_id: rng.below(SUB_IDS) as u8,
        voice_id: rng.below(32) as u8,
        slot_ids,
        sequence: rng.next(),
        slot_a: if rng.chance(3) { 0 } else { arena_address(rng) },
        slot_b: if rng.chance(3) { 0 } else { arena_address(rng) },
    };
    let filler: Vec<u8> = (0..OBJECT_SIZE).map(|_| rng.next() as u8).collect();
    let answers = vec![
        (
            "v0",
            (0..12)
                .map(|_| if rng.chance(3) { 0 } else { aligned_arena_address(rng) })
                .collect(),
        ),
        ("v2", (0..4).map(|_| rng.next()).collect()),
        ("v4", (0..4).map(|_| rng.next()).collect()),
        ("joaat", (0..4).map(|_| rng.next()).collect()),
        ("fallback", (0..4).map(|_| rng.next()).collect()),
        ("rebuild", (0..4).map(|_| rng.next()).collect()),
        ("unregister", (0..4).map(|_| rng.next()).collect()),
        ("set_cursor", (0..4).map(|_| rng.next()).collect()),
        ("register", (0..4).map(|_| rng.next()).collect()),
        ("init", (0..4).map(|_| rng.next()).collect()),
    ];
    let callees: Vec<(u32, u32)> = match kind {
        Method::Find | Method::StoreAtKey => Vec::new(),
        Method::FindByName | Method::HashForward => vec![(1, rt::joaat as *const () as usize as u32)],
        Method::ForwardLink => vec![(1, rt::fallback as *const () as usize as u32)],
        Method::Dtor => vec![
            (1, rt::rebuild as *const () as usize as u32),
            (2, rt::unregister as *const () as usize as u32),
        ],
        Method::Construct2 => vec![
            (2, rt::set_cursor as *const () as usize as u32),
            (3, rt::register as *const () as usize as u32),
        ],
    };
    Case {
        kind,
        sound,
        scalar_a,
        scalar_b,
        lookup_rows,
        init_rows,
        filler,
        hash: rng.next(),
        key: rng.next(),
        bits: rng.next(),
        arg: if rng.chance(8) { 0 } else { rng.next() },
        free: rng.next(),
        name: rng.next(),
        answers,
        callees,
    }
}

/// Lays the case's image into the arena: the object, the class and voice
/// virtual tables, the globals, the init table, the voice table rows and the
/// voice objects that the cursors and link targets reach.
fn build_image(c: &Case) {
    rt::arena_clear();
    let obj = obj_addr();
    for (i, b) in c.filler.iter().enumerate() {
        rt::write_u8(obj + i as u32, *b);
    }
    let class_vt = rt::arena_base() + rt::CLASS_VTABLE_OFF;
    let voice_vt = rt::arena_base() + rt::VOICE_VTABLE_OFF;
    rt::write_u32(class_vt, rt::v0 as *const () as usize as u32);
    rt::write_u32(class_vt + 8, rt::v2 as *const () as usize as u32);
    rt::write_u32(class_vt + 0x10, rt::v4 as *const () as usize as u32);
    rt::write_u32(voice_vt, rt::v0 as *const () as usize as u32);
    rt::write_u32(voice_vt + 0x10, rt::v4 as *const () as usize as u32);
    rt::write_u32(obj, class_vt);
    encode(&c.sound, obj);

    rt::write_u32(rt::relocated(rt::GLOBAL_SCALAR_A), c.scalar_a);
    rt::write_u32(rt::relocated(rt::GLOBAL_SCALAR_B), c.scalar_b);
    rt::write_u32(rt::relocated(rt::GLOBAL_VOICE_TABLE), rt::arena_base() + rt::TABLE_OFF);

    let init = rt::relocated(rt::GLOBAL_INIT_TABLE);
    for sub in 0..SUB_IDS {
        rt::write_u32(init + 4 * sub, rt::INIT_STUBS[sub as usize] as usize as u32);
    }

    let table = rt::arena_base() + rt::TABLE_OFF;
    for v in 0..32u32 {
        let row = table + v * ROW_STRIDE;
        rt::write_u32(row + ROW_BIAS_LOOKUP, c.lookup_rows[v as usize]);
        rt::write_u32(row + ROW_BIAS_INIT, c.init_rows[v as usize]);
    }
    let mut rows_done: Vec<u32> = Vec::new();
    for v in 0..32usize {
        let r = c.lookup_rows[v];
        if rows_done.contains(&r) {
            continue;
        }
        rows_done.push(r);
        for id in 0..256u32 {
            rt::write_u32(r.wrapping_add(c.scalar_a.wrapping_mul(id)), voice_vt);
        }
    }
}

/// Reads the lifted fields of the object from the arena.
fn decode(obj: u32) -> AudSound {
    let mut slot_ids = [0u8; 8];
    for (i, id) in slot_ids.iter_mut().enumerate() {
        *id = rt::read_u8(obj + 0x48 + i as u32);
    }
    AudSound {
        voice_column: rt::read_u8(obj + 0x04),
        link: rt::read_u8(obj + 0x05),
        kind: u16::from(rt::read_u8(obj + 0x06)) | (u16::from(rt::read_u8(obj + 0x07)) << 8),
        fast_flags: rt::read_u8(obj + 0x39),
        init_flags: rt::read_u8(obj + 0x3A),
        sub_id: rt::read_u8(obj + 0x3B),
        voice_id: rt::read_u8(obj + 0x40),
        slot_ids,
        sequence: rt::read_u32(obj + 0xA4),
        slot_a: rt::read_u32(obj + 0xA8),
        slot_b: rt::read_u32(obj + 0xAC),
    }
}

/// Writes the lifted fields back into the arena. Bytes outside them are not
/// touched, so the whole-arena comparison sees every other effect.
fn encode(s: &AudSound, obj: u32) {
    rt::write_u8(obj + 0x04, s.voice_column);
    rt::write_u8(obj + 0x05, s.link);
    rt::write_u8(obj + 0x06, s.kind as u8);
    rt::write_u8(obj + 0x07, (s.kind >> 8) as u8);
    rt::write_u8(obj + 0x39, s.fast_flags);
    rt::write_u8(obj + 0x3A, s.init_flags);
    rt::write_u8(obj + 0x3B, s.sub_id);
    rt::write_u8(obj + 0x40, s.voice_id);
    for (i, id) in s.slot_ids.iter().enumerate() {
        rt::write_u8(obj + 0x48 + i as u32, *id);
    }
    rt::write_u32(obj + 0xA4, s.sequence);
    rt::write_u32(obj + 0xA8, s.slot_a);
    rt::write_u32(obj + 0xAC, s.slot_b);
}

/// The engine side of the lifted class, implemented over the arena. Every
/// method performs the call the verified rewrite performs, through the same
/// address, so the stubs record it.
struct ArenaWorld;

impl SoundWorld for ArenaWorld {
    fn scalar_a(&self) -> u32 {
        rt::read_u32(rt::relocated(rt::GLOBAL_SCALAR_A))
    }
    fn scalar_b(&self) -> u32 {
        rt::read_u32(rt::relocated(rt::GLOBAL_SCALAR_B))
    }
    fn voice_table_word(&self, row: u8, offset: u32) -> u32 {
        let table = rt::read_u32(rt::relocated(rt::GLOBAL_VOICE_TABLE));
        rt::read_u32(
            table
                .wrapping_add(u32::from(row).wrapping_mul(ROW_STRIDE))
                .wrapping_add(offset),
        )
    }
    fn registry(&self) -> Handle32<RegistryTag> {
        Handle32::new(rt::relocated(rt::GLOBAL_REGISTRY)).expect("registry cell")
    }
    fn fallback_cell(&self) -> Handle32<FallbackTag> {
        Handle32::new(rt::relocated(rt::GLOBAL_FALLBACK)).expect("fallback cell")
    }
    fn install_vtable(&mut self, this: Handle32<ObjectTag>) {
        rt::write_u32(this.get(), rt::relocated(rt::CLASS_VTABLE_VA));
    }
    fn run_init(&mut self, this: Handle32<ObjectTag>, sub_id: u8) -> u32 {
        let table = rt::relocated(rt::GLOBAL_INIT_TABLE);
        let entry = rt::read_u32(table.wrapping_add(u32::from(sub_id).wrapping_mul(4)));
        let f: extern "cdecl" fn(u32) -> u32 = unsafe { core::mem::transmute(entry as usize) };
        f(this.get())
    }
    fn set_cursor(&mut self, cursor: u32) -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rt::callee_addr(2) as usize) };
        f(cursor)
    }
    fn register(&mut self, registry: Handle32<RegistryTag>, voice_id: u32, kind: u32) -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::callee_addr(3) as usize) };
        f(registry.get(), voice_id, kind)
    }
    fn joaat(&mut self, name: u32, seed: u32) -> u32 {
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::callee_addr(1) as usize) };
        f(name, seed)
    }
    fn find_slot0(&mut self, object: Handle32<ObjectTag>, hash: u32) -> u32 {
        let vt = rt::read_u32(object.get());
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::read_u32(vt) as usize) };
        f(object.get(), hash)
    }
    fn slot2(&mut self, object: Handle32<ObjectTag>, hash: u32, value_bits: u32) -> u32 {
        let vt = rt::read_u32(object.get());
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::read_u32(vt.wrapping_add(8)) as usize) };
        f(object.get(), hash, value_bits)
    }
    fn slot4(&mut self, target: u32, arg: u32) -> u32 {
        let vt = rt::read_u32(target);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::read_u32(vt.wrapping_add(0x10)) as usize) };
        f(target, arg)
    }
    fn forward_fallback(&mut self, cell: Handle32<FallbackTag>, arg: u32) -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::callee_addr(1) as usize) };
        f(cell.get(), arg)
    }
    fn store_word(&mut self, slot: Handle32<SlotTag>, value: u32) {
        rt::write_u32(slot.get(), value);
    }
    fn zero_byte(&mut self, slot: Handle32<SlotTag>) {
        rt::write_u8(slot.get(), 0);
    }
    fn zero_word(&mut self, slot: Handle32<SlotTag>) {
        rt::write_u32(slot.get(), 0);
    }
    fn rebuild(&mut self, this: Handle32<ObjectTag>) -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rt::callee_addr(1) as usize) };
        f(this.get())
    }
    fn unregister(
        &mut self,
        registry: Handle32<RegistryTag>,
        this: Handle32<ObjectTag>,
        voice_id: u32,
    ) -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rt::callee_addr(2) as usize) };
        f(registry.get(), this.get(), voice_id)
    }
}

/// Runs the verified rewrite of the case's method.
fn run_rewrite(c: &Case) -> u32 {
    let obj = obj_addr();
    match c.kind {
        Method::Find => fn_00890800::rw_00890800(obj, c.hash),
        Method::FindByName => fn_00890870::rw_00890870(obj, c.name),
        Method::StoreAtKey => fn_00891680::rw_00891680(obj as usize as *mut u8, c.key, c.bits),
        Method::HashForward => fn_008916A0::rw_008916a0(obj as usize as *mut u8, c.key, c.bits),
        Method::ForwardLink => fn_00891E90::rw_00891e90(obj as usize as *mut u8, c.arg),
        Method::Dtor => fn_0088F7C0::rw_0088F7C0(obj, c.free),
        Method::Construct2 => {
            fn_0088F700::rw_0088F700(obj);
            0
        }
    }
}

/// Runs the lifted method of the case on the decoded object.
fn run_lifted(c: &Case) -> u32 {
    let obj = obj_addr();
    let mut w = ArenaWorld;
    let ret = match c.kind {
        Method::Find => {
            let s = decode(obj);
            let r = s.find_by_hash(&mut w, c.hash);
            encode(&s, obj);
            r
        }
        Method::FindByName => AudSound::find_by_name(handle(obj), c.name, &mut w),
        Method::StoreAtKey => AudSound::store_word_at_key(handle(obj), c.key, c.bits, &mut w),
        Method::HashForward => AudSound::hash_and_forward(handle(obj), c.key, c.bits, &mut w),
        Method::ForwardLink => {
            let s = decode(obj);
            let r = s.forward_link(c.arg, &mut w);
            encode(&s, obj);
            r
        }
        Method::Dtor => {
            let s = decode(obj);
            let r = s.deleting_destructor(handle(obj), c.free, &mut w).get();
            encode(&s, obj);
            r
        }
        Method::Construct2 => {
            let mut s = decode(obj);
            s.construct_second(handle(obj), &mut w);
            encode(&s, obj);
            0
        }
    };
    ret
}

/// Runs the deliberately wrong lift of the case's method. Each differs from
/// the lifted method by one change, named in its comment.
fn run_wrong(c: &Case) -> u32 {
    let obj = obj_addr();
    let mut w = ArenaWorld;
    match c.kind {
        Method::Find => {
            // WRONG: the scan stops one slot early and never looks at the last id.
            let s = decode(obj);
            let scalar = w.scalar_a();
            let row = w.voice_table_word(s.voice_id, ROW_BIAS_LOOKUP);
            if s.fast_flags & FAST_BIT != 0 && s.kind == KIND_FAST {
                return 0;
            }
            if s.init_flags & lf_audio::sound_base::DISABLE_BIT != 0 {
                return 0;
            }
            for id in &s.slot_ids[..7] {
                if *id == NO_VOICE {
                    continue;
                }
                let cursor = scalar.wrapping_mul(u32::from(*id)).wrapping_add(row);
                if let Some(object) = Handle32::<ObjectTag>::new(cursor) {
                    let found = w.find_slot0(object, c.hash);
                    if found != 0 {
                        return found;
                    }
                }
            }
            0
        }
        Method::FindByName => {
            // WRONG: the name and the seed are passed in the other order.
            let hash = w.joaat(0, c.name);
            w.find_slot0(handle(obj), hash)
        }
        Method::StoreAtKey => {
            // WRONG: the word stored is the key, not the float bits.
            let slot = w.find_slot0(handle(obj), c.key);
            if let Some(slot) = Handle32::<SlotTag>::new(slot) {
                w.store_word(slot, c.key);
            }
            slot
        }
        Method::HashForward => {
            // WRONG: slot 2 receives the float bits where the hash belongs.
            let hashed = w.joaat(c.key, 0);
            w.slot2(handle(obj), c.bits, hashed)
        }
        Method::ForwardLink => {
            // WRONG: the target is scaled by the voice id rather than the link.
            let s = decode(obj);
            if c.arg == 0 {
                return 0;
            }
            if s.link == NO_VOICE {
                let cell = w.fallback_cell();
                return w.forward_fallback(cell, c.arg);
            }
            let scalar = w.scalar_a();
            let entry = w.voice_table_word(s.voice_id, ROW_BIAS_LOOKUP);
            let target = scalar
                .wrapping_mul(u32::from(s.voice_id))
                .wrapping_add(entry);
            w.slot4(target, c.arg)
        }
        Method::Dtor => {
            // WRONG: the registry call is made on bit 1 of the free word.
            let s = decode(obj);
            let vid = u32::from(s.voice_id);
            w.rebuild(handle(obj));
            if c.free & 2 != 0 {
                let registry = w.registry();
                w.unregister(registry, handle(obj), vid);
            }
            obj
        }
        Method::Construct2 => {
            // WRONG: the cursor is scaled by the first scalar instead of the second.
            let mut s = decode(obj);
            w.install_vtable(handle(obj));
            let flags = s.init_flags;
            if flags & lf_audio::sound_base::INIT_DONE == 0 {
                s.init_flags = flags | lf_audio::sound_base::INIT_DONE;
                w.run_init(handle(obj), s.sub_id);
            } else if flags & lf_audio::sound_base::NEEDS_INIT != 0 && s.voice_column != NO_VOICE {
                let voice_id = s.voice_id;
                let scalar = w.scalar_a();
                let word = w.voice_table_word(voice_id, ROW_BIAS_INIT);
                let cursor = scalar
                    .wrapping_mul(u32::from(s.voice_column))
                    .wrapping_add(word);
                w.set_cursor(cursor);
                let registry = w.registry();
                w.register(registry, u32::from(voice_id), u32::from(s.voice_column));
            }
            s.sequence = lf_audio::sound_base::SEQ_UNSET;
            if let Some(slot) = Handle32::<SlotTag>::new(s.slot_a) {
                w.zero_byte(slot);
                s.slot_a = 0;
            }
            if let Some(slot) = Handle32::<SlotTag>::new(s.slot_b) {
                w.zero_word(slot);
                s.slot_b = 0;
            }
            encode(&s, obj);
            0
        }
    }
}

/// Runs one side of a case from the same arena image and returns its outcome.
fn outcome(image: &[u8], c: &Case, side: fn(&Case) -> u32) -> Outcome {
    rt::arena_restore(image);
    rt::begin_run(&c.answers);
    for (id, addr) in &c.callees {
        rt::set_callee(*id, *addr);
    }
    let ret = side(c);
    Outcome {
        ret,
        log: rt::take_log(),
        image: rt::arena_copy(),
    }
}

fn first_difference(a: &[u8], b: &[u8]) -> Option<usize> {
    a.iter().zip(b).position(|(x, y)| x != y)
}

/// Compares the proof's three views of a case. Returns the first mismatch.
fn compare(rw: &Outcome, lf: &Outcome) -> Option<String> {
    if rw.ret != lf.ret {
        return Some(format!("return: rewrite {:#x}, lift {:#x}", rw.ret, lf.ret));
    }
    if rw.log != lf.log {
        return Some(format!("calls: rewrite {:?}, lift {:?}", rw.log, lf.log));
    }
    if let Some(at) = first_difference(&rw.image, &lf.image) {
        return Some(format!(
            "arena byte at offset {at:#x}: rewrite {:#x}, lift {:#x}",
            rw.image[at], lf.image[at]
        ));
    }
    None
}

/// Runs the proof for one method over generated cases; returns the count of
/// cases on which the two sides differed (zero for a passing proof).
fn prove(kind: Method, seed: u64, side_lift: fn(&Case) -> u32) -> (u32, Option<String>) {
    let mut rng = Rng(seed);
    let mut first = None;
    let mut mismatches = 0;
    for _ in 0..CASES {
        let case = gen_case(&mut rng, kind);
        build_image(&case);
        let image = rt::arena_copy();
        let rw = outcome(&image, &case, run_rewrite);
        let lf = outcome(&image, &case, side_lift);
        if let Some(why) = compare(&rw, &lf) {
            mismatches += 1;
            if first.is_none() {
                first = Some(format!("{kind:?}: {why}"));
            }
        }
    }
    (mismatches, first)
}

fn run_proof(kind: Method, seed: u64) {
    let _guard = rt::lock();
    let (mismatches, first) = prove(kind, seed, run_lifted);
    assert_eq!(mismatches, 0, "lift differs from the rewrite: {first:?}");
}

fn run_wrong_caught(kind: Method, seed: u64) {
    let _guard = rt::lock();
    let (mismatches, _) = prove(kind, seed, run_wrong);
    assert!(mismatches > 0, "the wrong lift of {kind:?} was not caught");
}

#[test]
fn find_by_hash_matches_rewrite() {
    run_proof(Method::Find, 0x1001);
}

#[test]
fn find_by_name_matches_rewrite() {
    run_proof(Method::FindByName, 0x2002);
}

#[test]
fn store_word_at_key_matches_rewrite() {
    run_proof(Method::StoreAtKey, 0x3003);
}

#[test]
fn hash_and_forward_matches_rewrite() {
    run_proof(Method::HashForward, 0x4004);
}

#[test]
fn forward_link_matches_rewrite() {
    run_proof(Method::ForwardLink, 0x5005);
}

#[test]
fn deleting_destructor_matches_rewrite() {
    run_proof(Method::Dtor, 0x6006);
}

#[test]
fn construct_second_matches_rewrite() {
    run_proof(Method::Construct2, 0x7007);
}

#[test]
fn wrong_lift_of_find_by_hash_is_caught() {
    run_wrong_caught(Method::Find, 0x1001);
}

#[test]
fn wrong_lift_of_find_by_name_is_caught() {
    run_wrong_caught(Method::FindByName, 0x2002);
}

#[test]
fn wrong_lift_of_store_word_at_key_is_caught() {
    run_wrong_caught(Method::StoreAtKey, 0x3003);
}

#[test]
fn wrong_lift_of_hash_and_forward_is_caught() {
    run_wrong_caught(Method::HashForward, 0x4004);
}

#[test]
fn wrong_lift_of_forward_link_is_caught() {
    run_wrong_caught(Method::ForwardLink, 0x5005);
}

#[test]
fn wrong_lift_of_deleting_destructor_is_caught() {
    run_wrong_caught(Method::Dtor, 0x6006);
}

#[test]
fn wrong_lift_of_construct_second_is_caught() {
    run_wrong_caught(Method::Construct2, 0x7007);
}

#[test]
fn every_method_is_listed() {
    assert_eq!(ALL.len(), 7);
}
