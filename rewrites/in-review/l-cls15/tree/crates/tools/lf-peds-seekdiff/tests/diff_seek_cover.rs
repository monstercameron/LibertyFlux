//! Differential tests: the lifted seek-cover task methods against their
//! verified 32-bit rewrites.
//!
//! Each case runs the rewrite and the lifted method on the same generated
//! inputs, in the same test memory layout. The comparison covers the
//! return value, every numbered callee call in order with its arguments
//! (addresses named by what they point into), a copy of the object at each
//! call, the object after the call, and every other word the method writes
//! (the worker, the subtask, the node). Floats are compared as their bits.
//!
//! The rewrites run on the 32-bit target only, so this file is compiled
//! for that target alone. On other hosts the lifted crate's own host tests
//! run instead.

#![cfg(target_arch = "x86")]

use std::slice;

use lf_core::Handle32;
use lf_peds_seekdiff::rewrites::{fn_00d1d3d0, fn_00d1d4c0, fn_00d1d650, fn_00d1d7a0, fn_00d1e060, fn_00d1f5b0};
use lf_peds_seekdiff::rt;
use lf_peds_tasks::seek_cover::{
    CoverRowArgs, CoverRoute, CoverSubTask, CoverWorker, FullArgs, OwnedSlot, RouteArgs, RouteExtra,
    SeekCoverTask, SeekCoverWorld, ShortArgs, SmallNode, Stamp, TaskNode,
};

/// Length of the seek-cover object image the proof compares.
const OBJ_LEN: usize = 0xC0;
/// Length of a subtask image.
const SUB_LEN: usize = 0x100;
/// Length of a task-node image.
const NODE_LEN: usize = 0x20;
/// Rows of the cover table the generated inputs index into.
const TABLE_ROWS: usize = 4096;
/// Bytes per cover-table row.
const ROW_STRIDE: usize = 36;

/// File address of the seek-cover class stamp.
const VTABLE_VA: u32 = 0x00EE_0A6C;
/// File address of the task-node class stamp.
const NODE_VTABLE_VA: u32 = 0x00EB_391C;
/// File address of the cell that holds the task pool object.
const POOL_VA: u32 = 0x0167_E2A0;
/// File address of the cover table.
const COVER_TABLE_VA: u32 = 0x016F_C990;

/// Trials per case.
const TRIALS: u32 = 1500;

/// A fixed-address block of test memory. The vector never reallocates
/// after construction, so the address stays valid for the arena's life.
struct Arena {
    words: Vec<u32>,
    len: usize,
}

impl Arena {
    fn zeroed(len: usize) -> Self {
        Self { words: vec![0; len.div_ceil(4).max(1)], len }
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut a = Self::zeroed(bytes.len());
        a.bytes_mut().copy_from_slice(bytes);
        a
    }

    fn from_words(words: &[u32]) -> Self {
        let mut bytes = Vec::with_capacity(words.len() * 4);
        for w in words {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
        Self::from_bytes(&bytes)
    }

    fn addr(&self) -> u32 {
        self.words.as_ptr() as usize as u32
    }

    fn bytes(&self) -> &[u8] {
        // SAFETY: the vector owns `words.len() * 4 >= len` initialised bytes.
        unsafe { slice::from_raw_parts(self.words.as_ptr() as *const u8, self.len) }
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: as in `bytes`, with exclusive access through `&mut self`.
        unsafe { slice::from_raw_parts_mut(self.words.as_mut_ptr() as *mut u8, self.len) }
    }
}

/// Small deterministic generator (xorshift64*), so every run sees the same
/// inputs.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    fn below(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }

    fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next_u32() as u8).collect()
    }

    /// A word that is often a special value: zero, one, the signed and
    /// unsigned limits, the top bit alone, the float infinities and a
    /// not-a-number with and without its sign bit.
    fn word(&mut self) -> u32 {
        const SPECIAL: [u32; 12] = [
            0,
            1,
            0xFFFF_FFFF,
            0x7FFF_FFFF,
            0x8000_0000,
            0x7FC0_0000,
            0xFFC0_0000,
            0x7F80_0000,
            0xFF80_0000,
            0x3F80_0000,
            0x0000_0007,
            0x0000_0008,
        ];
        if self.below(3) == 0 {
            SPECIAL[self.below(SPECIAL.len() as u32) as usize]
        } else {
            self.next_u32()
        }
    }

    fn words<const N: usize>(&mut self) -> [u32; N] {
        std::array::from_fn(|_| self.word())
    }
}

/// A normalised argument: a word, or the name of the thing it points into.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Norm {
    W(u32),
    S(&'static str),
}

/// A normalised callee call or lifted collaborator call.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Ev {
    name: &'static str,
    args: Vec<Norm>,
    snap: Option<Vec<u8>>,
}

/// Names the addresses a rewrite may pass, so the two logs compare by
/// meaning. An address with no name stays a word.
fn norm_call(
    raw: &rt::RawCall,
    id_names: &[(u32, &'static str)],
    addr_names: &[(u32, &'static str)],
    query_id: Option<u32>,
) -> Ev {
    let name = id_names
        .iter()
        .find(|(id, _)| *id == raw.id)
        .map_or("UNMAPPED", |(_, n)| *n);
    let args = raw
        .args
        .iter()
        .enumerate()
        .map(|(i, &w)| {
            if Some(raw.id) == query_id && i == 1 {
                // The query's out pointer is a stack word of the rewrite.
                return Norm::S("out");
            }
            addr_names
                .iter()
                .find(|(a, _)| *a == w)
                .map_or(Norm::W(w), |(_, n)| Norm::S(n))
        })
        .collect();
    Ev { name, args, snap: raw.snap.clone() }
}

fn rd32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn wr32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// Reads the owned fields of the object image.
fn load(b: &[u8]) -> SeekCoverTask {
    SeekCoverTask {
        dir: [rd32(b, 0x20), rd32(b, 0x24), rd32(b, 0x28)],
        pos: rd32(b, 0x30),
        mode_byte: b[0x5C],
        index: rd32(b, 0x74),
        slot: [rd32(b, 0x80), rd32(b, 0x84), rd32(b, 0x88), rd32(b, 0x8C)],
        flags: rd32(b, 0x90),
        latch: b[0x94],
        weight: rd32(b, 0x98),
        tail: [rd32(b, 0xA0), rd32(b, 0xA4), rd32(b, 0xA8)],
        link: rd32(b, 0xB0),
        mark: rd32(b, 0xB4),
        tail_end: [rd32(b, 0xB8), rd32(b, 0xBC)],
    }
}

/// Writes the owned fields into an object image, leaving every other byte.
fn store(t: &SeekCoverTask, b: &mut [u8]) {
    for (i, w) in t.dir.iter().enumerate() {
        wr32(b, 0x20 + 4 * i, *w);
    }
    wr32(b, 0x30, t.pos);
    b[0x5C] = t.mode_byte;
    wr32(b, 0x74, t.index);
    for (i, w) in t.slot.iter().enumerate() {
        wr32(b, 0x80 + 4 * i, *w);
    }
    wr32(b, 0x90, t.flags);
    b[0x94] = t.latch;
    wr32(b, 0x98, t.weight);
    for (i, w) in t.tail.iter().enumerate() {
        wr32(b, 0xA0 + 4 * i, *w);
    }
    wr32(b, 0xB0, t.link);
    wr32(b, 0xB4, t.mark);
    for (i, w) in t.tail_end.iter().enumerate() {
        wr32(b, 0xB8 + 4 * i, *w);
    }
}

/// The lifted side's collaborators, recording each call the way the
/// rewrite's stubs record theirs. The object image is kept alongside the
/// struct: the stamp and the bytes the struct does not own live in it.
struct Recorder {
    img: Vec<u8>,
    watch: bool,
    events: Vec<Ev>,
    table: Vec<u8>,
    query: [u32; 4],
    worker: u32,
    sub: u32,
    sub_img: Vec<u8>,
    node: u32,
    node_img: Vec<u8>,
    dtor_answer: u32,
}

impl Recorder {
    fn image(&self, t: &SeekCoverTask) -> Vec<u8> {
        let mut v = self.img.clone();
        store(t, &mut v);
        v
    }

    fn ev(&mut self, name: &'static str, args: Vec<Norm>, this: Option<&SeekCoverTask>) {
        let snap = match (self.watch, this) {
            (true, Some(t)) => Some(self.image(t)),
            _ => None,
        };
        self.events.push(Ev { name, args, snap });
    }

    fn worker_norm(&self, w: Handle32<CoverWorker>) -> Norm {
        if w.get() == self.worker { Norm::S("worker") } else { Norm::W(w.get()) }
    }

    fn node_norm(&self, n: Handle32<TaskNode>) -> Norm {
        if n.get() == self.node { Norm::S("node") } else { Norm::W(n.get()) }
    }
}

impl SeekCoverWorld for Recorder {
    fn set_stamp(&mut self, _this: &mut SeekCoverTask, stamp: Stamp) {
        match stamp {
            Stamp::SeekCover => wr32(&mut self.img, 0, rt::relocated(VTABLE_VA)),
        }
    }

    fn base_construct(&mut self, this: &mut SeekCoverTask) {
        self.ev("base", vec![Norm::S("obj")], Some(&*this));
    }

    fn init_member_60(&mut self, this: &mut SeekCoverTask) {
        self.ev("init60", vec![Norm::S("m60")], Some(&*this));
    }

    fn init_member_6c(&mut self, this: &mut SeekCoverTask) {
        self.ev("init6c", vec![Norm::S("m6c")], Some(&*this));
    }

    fn reinit_shared(&mut self, this: &mut SeekCoverTask) {
        self.ev("reinit", vec![Norm::S("obj")], Some(&*this));
    }

    fn cover_row_flags(&mut self, index: u32) -> u8 {
        self.table[index as usize * ROW_STRIDE]
    }

    fn cover_row_query(&mut self, this: &mut SeekCoverTask, _index: u32) -> [u32; 4] {
        self.ev("query", vec![Norm::S("row"), Norm::S("out"), Norm::W(0)], Some(&*this));
        self.query
    }

    fn release_owned(&mut self, this: &mut SeekCoverTask, slot: OwnedSlot, owned: u32) {
        let slot_name = match slot {
            OwnedSlot::Pos => "pos_slot",
            OwnedSlot::Link => "link_slot",
        };
        self.ev("release", vec![Norm::W(owned), Norm::S(slot_name)], Some(&*this));
    }

    fn release_member_6c(&mut self, this: &mut SeekCoverTask) {
        self.ev("rel6c", vec![Norm::S("m6c")], Some(&*this));
    }

    fn release_member_60(&mut self, this: &mut SeekCoverTask) {
        self.ev("rel60", vec![Norm::S("m60")], Some(&*this));
    }

    fn base_destruct(&mut self, this: &mut SeekCoverTask) -> u32 {
        self.ev("base_dtor", vec![Norm::S("obj")], Some(&*this));
        self.dtor_answer
    }

    fn fetch_worker(&mut self, this: &mut SeekCoverTask) -> Option<Handle32<CoverWorker>> {
        self.ev("fetch", vec![Norm::S("pool")], Some(&*this));
        Handle32::new(self.worker)
    }

    fn run_route(
        &mut self,
        this: &mut SeekCoverTask,
        route: CoverRoute,
        worker: Handle32<CoverWorker>,
        args: &RouteArgs,
    ) -> Option<Handle32<CoverSubTask>> {
        let name = match route {
            CoverRoute::Two => "route2",
            CoverRoute::One => "route1",
            CoverRoute::Zero => "route0",
        };
        let mut v = vec![
            self.worker_norm(worker),
            Norm::W(args.pos),
            Norm::S("dir"),
            Norm::W(u32::from(args.mode)),
            Norm::W(u32::from(args.latch_bit)),
        ];
        match args.extra {
            Some(RouteExtra::Index(i)) => v.push(Norm::W(i)),
            Some(RouteExtra::SlotRef) => v.push(Norm::S("slot")),
            None => {}
        }
        v.push(Norm::W(args.speed));
        v.push(Norm::W(args.lane));
        self.ev(name, v, Some(&*this));
        Handle32::new(self.sub)
    }

    fn latch_of(&mut self, _sub: Handle32<CoverSubTask>) -> u8 {
        self.sub_img[0x94]
    }

    fn set_latch_of(&mut self, _sub: Handle32<CoverSubTask>, latch: u8) {
        self.sub_img[0x94] = latch;
    }

    fn allocate_node(&mut self) -> Option<Handle32<TaskNode>> {
        self.ev("alloc", vec![Norm::S("pool")], None);
        Handle32::new(self.node)
    }

    fn construct_node(&mut self, node: Handle32<TaskNode>) {
        let n = self.node_norm(node);
        self.ev("node_ctor", vec![n], None);
    }

    fn store_small_node(&mut self, _node: Handle32<TaskNode>, fields: &SmallNode) {
        wr32(&mut self.node_img, 0, rt::relocated(NODE_VTABLE_VA));
        wr32(&mut self.node_img, 0x14, fields.arg);
        self.node_img[0x18] = fields.flag;
        wr32(&mut self.node_img, 0x1C, fields.tail);
    }
}

/// Compares the two logs and the object images.
fn compare_calls(
    raw: &[rt::RawCall],
    id_names: &[(u32, &'static str)],
    addr_names: &[(u32, &'static str)],
    query_id: Option<u32>,
    lifted: &[Ev],
) -> Result<(), String> {
    let rewritten: Vec<Ev> = raw
        .iter()
        .map(|r| norm_call(r, id_names, addr_names, query_id))
        .collect();
    if rewritten != lifted {
        return Err(format!("call logs differ:\n rewrite {rewritten:?}\n lifted  {lifted:?}"));
    }
    Ok(())
}

/// The object image after the rewrite and after the lifted method must agree.
fn compare_images(what: &str, raw: &[u8], lifted: &[u8]) -> Result<(), String> {
    if raw != lifted {
        let at = raw.iter().zip(lifted).position(|(a, b)| a != b);
        return Err(format!("{what} images differ first at {at:?}"));
    }
    Ok(())
}

/// Object-names used by every seek-cover case.
fn object_names(obj: u32) -> Vec<(u32, &'static str)> {
    vec![
        (obj, "obj"),
        (obj + 0x20, "dir"),
        (obj + 0x30, "pos_slot"),
        (obj + 0x60, "m60"),
        (obj + 0x6C, "m6c"),
        (obj + 0x80, "slot"),
        (obj + 0xB0, "link_slot"),
    ]
}

/// Fresh recorder for one trial.
fn recorder(seed: &[u8], table: &[u8], watch: bool) -> Recorder {
    Recorder {
        img: seed.to_vec(),
        watch,
        events: Vec::new(),
        table: table.to_vec(),
        query: [0; 4],
        worker: 0,
        sub: 0,
        sub_img: Vec::new(),
        node: 0,
        node_img: Vec::new(),
        dtor_answer: 0,
    }
}

/// Binds the two class stamps the rewrites write.
fn bind_stamps(vtable: &Arena, node_vtable: &Arena) {
    rt::bind(VTABLE_VA, vtable.addr());
    rt::bind(NODE_VTABLE_VA, node_vtable.addr());
}

/// Points the callee numbers at named stubs.
fn install(stubs: &[(u32, &str)]) {
    for (id, name) in stubs {
        rt::set_callee(*id, rt::stub(name));
    }
}

/// Runs trials of one case, returning how many completed with each branch
/// label the case reports.
fn run_cases_unlocked(name: &str, mut trial: impl FnMut(&mut Rng, u32) -> Result<&'static str, String>) {
    let mut rng = Rng::new(0x5EEC_C0DE ^ name.len() as u64);
    let mut branches: Vec<(&'static str, u32)> = Vec::new();
    for n in 0..TRIALS {
        match trial(&mut rng, n) {
            Ok(branch) => match branches.iter_mut().find(|(b, _)| *b == branch) {
                Some((_, c)) => *c += 1,
                None => branches.push((branch, 1)),
            },
            Err(e) => panic!("{name}: trial {n} differs: {e}"),
        }
    }
    eprintln!("{name}: {TRIALS} trials agree; branches {branches:?}");
    for (branch, count) in &branches {
        assert!(*count > 0, "{name}: branch {branch} never ran");
    }
}

// ----------------------------------------------------------------------------
// The full constructor.
// ----------------------------------------------------------------------------

const CTOR_IDS: [(u32, &str); 4] = [(1, "base"), (2, "init60"), (3, "init6c"), (4, "reinit")];

#[test]
fn full_constructor_matches_rewrite() {
    let _guard = lock();
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    bind_stamps(&vtable, &node_vtable);
    run_cases("construct_full", |rng, _| {
        let seed = rng.bytes(OBJ_LEN);
        let w = rng.word();
        let pos_a: [u32; 3] = rng.words();
        let pos_b: [u32; 3] = rng.words();
        let b1 = rng.next_u32() as u8;
        let b2 = rng.next_u32() as u8;
        let f = rng.word();
        let mode = rng.word();

        let obj = Arena::from_bytes(&seed);
        let pa = Arena::from_words(&pos_a);
        let pb = Arena::from_words(&pos_b);
        install(&[(1, "s1_0"), (2, "s2_0"), (3, "s3_0"), (4, "s4_0")]);
        rt::set_answers(1, &[]);
        let _ = rt::take_log();
        rt::watch(obj.addr(), OBJ_LEN as u32);
        let ret = fn_00d1d3d0::rw_00d1d3d0(obj.addr(), w, pa.addr(), u32::from(b1), u32::from(b2), pb.addr(), f, mode);
        let raw = rt::take_log();
        rt::unwatch();
        if ret != obj.addr() {
            return Err(format!("return {ret:#x} is not the object"));
        }

        let mut world = recorder(&seed, &[], true);
        let mut t = load(&seed);
        t.construct_full(
            &mut world,
            &FullArgs { pos: w, dir_src: &pos_a, mode_byte: b1, latch_src: b2, slot_src: &pos_b, weight: f, mode },
        );
        let lifted_img = world.image(&t);
        compare_calls(&raw, &CTOR_IDS, &object_names(obj.addr()), None, &world.events)?;
        compare_images("object", obj.bytes(), &lifted_img)?;
        Ok(if mode & 7 == 0 { "route zero mode" } else { "other mode" })
    });
}

// ----------------------------------------------------------------------------
// The constructor that takes a cover-table row.
// ----------------------------------------------------------------------------

#[test]
fn row_constructor_matches_rewrite() {
    let _guard = lock();
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    bind_stamps(&vtable, &node_vtable);
    run_cases("construct_cover_row", |rng, _| {
        let seed = rng.bytes(OBJ_LEN);
        let table_bytes = rng.bytes(TABLE_ROWS * ROW_STRIDE);
        let table = Arena::from_bytes(&table_bytes);
        rt::bind(COVER_TABLE_VA, table.addr());

        let pos = rng.word();
        let dir: [u32; 3] = rng.words();
        let flags0 = rng.word();
        let flags1 = rng.word();
        let index = rng.below(TABLE_ROWS as u32);
        let weight = rng.word();
        let mode = rng.word();
        let query: [u32; 4] = rng.words();

        let obj = Arena::from_bytes(&seed);
        let da = Arena::from_words(&dir);
        install(&[(1, "s1_0"), (2, "s2_0"), (3, "s3_0"), (4, "s4_2"), (5, "s5_0")]);
        rt::set_query_words(query);
        let _ = rt::take_log();
        rt::watch(obj.addr(), OBJ_LEN as u32);
        let ret = fn_00d1d4c0::rw_00d1d4c0(obj.addr(), pos, da.addr(), flags0, flags1, index, weight, mode);
        let raw = rt::take_log();
        rt::unwatch();
        if ret != obj.addr() {
            return Err(format!("return {ret:#x} is not the object"));
        }

        let row_flags = table_bytes[index as usize * ROW_STRIDE];
        let mut world = recorder(&seed, &table_bytes, true);
        world.query = query;
        let mut t = load(&seed);
        t.construct_cover_row(
            &mut world,
            &CoverRowArgs { pos, dir_src: &dir, flags0, flags1, index, weight, mode },
        );
        let lifted_img = world.image(&t);

        let mut names = object_names(obj.addr());
        names.push((table.addr() + index * ROW_STRIDE as u32, "row"));
        let ids = [(1, "base"), (2, "init60"), (3, "init6c"), (4, "query"), (5, "reinit")];
        compare_calls(&raw, &ids, &names, Some(4), &world.events)?;
        compare_images("object", obj.bytes(), &lifted_img)?;
        Ok(if row_flags & 7 != 0 { "populated row" } else { "empty row" })
    });
}

// ----------------------------------------------------------------------------
// The shorter constructor.
// ----------------------------------------------------------------------------

#[test]
fn short_constructor_matches_rewrite() {
    let _guard = lock();
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    bind_stamps(&vtable, &node_vtable);
    run_cases("construct_short", |rng, _| {
        let seed = rng.bytes(OBJ_LEN);
        let w = rng.word();
        let pos_a: [u32; 3] = rng.words();
        let b1 = rng.next_u32() as u8;
        let b2 = rng.next_u32() as u8;
        let f = rng.word();
        let mode = rng.word();

        let obj = Arena::from_bytes(&seed);
        let pa = Arena::from_words(&pos_a);
        install(&[(1, "s1_0"), (2, "s2_0"), (3, "s3_0"), (4, "s4_0")]);
        let _ = rt::take_log();
        rt::watch(obj.addr(), OBJ_LEN as u32);
        let ret = fn_00d1d650::rw_00d1d650(obj.addr(), w, pa.addr(), u32::from(b1), u32::from(b2), f, mode);
        let raw = rt::take_log();
        rt::unwatch();
        if ret != obj.addr() {
            return Err(format!("return {ret:#x} is not the object"));
        }

        let mut world = recorder(&seed, &[], true);
        let mut t = load(&seed);
        t.construct_short(
            &mut world,
            &ShortArgs { pos: w, dir_src: &pos_a, mode_byte: b1, latch_src: b2, weight: f, mode },
        );
        let lifted_img = world.image(&t);
        compare_calls(&raw, &CTOR_IDS, &object_names(obj.addr()), None, &world.events)?;
        compare_images("object", obj.bytes(), &lifted_img)?;
        Ok("short")
    });
}

// ----------------------------------------------------------------------------
// The destructor.
// ----------------------------------------------------------------------------

#[test]
fn destructor_matches_rewrite() {
    let _guard = lock();
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    bind_stamps(&vtable, &node_vtable);
    run_cases("destruct", |rng, _| {
        let seed = rng.bytes(OBJ_LEN);
        let obj = Arena::from_bytes(&seed);
        let answer = rng.word();
        install(&[(1, "s1_1"), (2, "s2_0"), (3, "s3_0"), (4, "s4_0")]);
        rt::set_answers(4, &[answer]);
        let _ = rt::take_log();
        rt::watch(obj.addr(), OBJ_LEN as u32);
        let ret = fn_00d1d7a0::rw_00d1d7a0(obj.addr());
        let raw = rt::take_log();
        rt::unwatch();

        let mut world = recorder(&seed, &[], true);
        world.dtor_answer = answer;
        let mut t = load(&seed);
        let lifted_ret = t.destruct(&mut world);
        let lifted_img = world.image(&t);
        if ret != lifted_ret {
            return Err(format!("return {ret:#x} vs {lifted_ret:#x}"));
        }
        compare_calls(
            &raw,
            &[(1, "release"), (2, "rel6c"), (3, "rel60"), (4, "base_dtor")],
            &object_names(obj.addr()),
            None,
            &world.events,
        )?;
        compare_images("object", obj.bytes(), &lifted_img)?;
        let released = u32::from(load(&seed).pos != 0) + u32::from(load(&seed).link != 0);
        Ok(match released {
            0 => "no owned word",
            1 => "one owned word",
            _ => "two owned words",
        })
    });
}

// ----------------------------------------------------------------------------
// The subtask router.
// ----------------------------------------------------------------------------

#[test]
fn router_matches_rewrite() {
    let _guard = lock();
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    bind_stamps(&vtable, &node_vtable);
    run_cases("route_to_sub_task", |rng, _| {
        let mut seed = rng.bytes(OBJ_LEN);
        // The route is the low three bits of the flag word; the original
        // runs routes zero to two and faults on the others, so the proof
        // draws only those.
        let flags = (rng.next_u32() & !7) | rng.below(3);
        wr32(&mut seed, 0x90, flags);

        let pool_obj = Arena::zeroed(16);
        let pool_cell = Arena::from_words(&[pool_obj.addr()]);
        rt::bind(POOL_VA, pool_cell.addr());

        let worker = Arena::from_bytes(&rng.bytes(0x20));
        let sub_bytes = rng.bytes(SUB_LEN);
        let sub = Arena::from_bytes(&sub_bytes);
        let obj = Arena::from_bytes(&seed);

        install(&[(1, "s1_0"), (2, "s2_7"), (3, "s3_7"), (4, "s4_6")]);
        rt::set_answers(1, &[worker.addr()]);
        rt::set_answers(2, &[sub.addr()]);
        rt::set_answers(3, &[sub.addr()]);
        rt::set_answers(4, &[sub.addr()]);
        let _ = rt::take_log();
        rt::watch(obj.addr(), OBJ_LEN as u32);
        let ret = {
            // The rewrite is exported under its own name; the route is run
            // through the same entry point as the checker.
            fn_00d1e060::rw_00d1e060(obj.addr())
        };
        let raw = rt::take_log();
        rt::unwatch();

        let mut world = recorder(&seed, &[], true);
        world.worker = worker.addr();
        world.sub = sub.addr();
        world.sub_img = sub_bytes.clone();
        let mut t = load(&seed);
        let lifted = t.route_to_sub_task(&mut world).map_or(0, Handle32::get);
        let lifted_img = world.image(&t);

        if ret != lifted {
            return Err(format!("return {ret:#x} vs {lifted:#x}"));
        }
        compare_calls(
            &raw,
            &[(1, "fetch"), (2, "route2"), (3, "route1"), (4, "route0")],
            &[(obj.addr(), "obj"), (obj.addr() + 0x20, "dir"), (obj.addr() + 0x80, "slot"), (pool_obj.addr(), "pool"), (worker.addr(), "worker"), (sub.addr(), "sub")],
            None,
            &world.events,
        )?;
        compare_images("object", obj.bytes(), &lifted_img)?;
        compare_images("subtask", sub.bytes(), &world.sub_img)?;
        Ok(match CoverRoute::from_flags(flags) {
            Some(CoverRoute::Zero) => "route zero",
            Some(CoverRoute::One) => "route one",
            Some(CoverRoute::Two) => "route two",
            None => "other",
        })
    });
}

// ----------------------------------------------------------------------------
// The node allocator.
// ----------------------------------------------------------------------------

#[test]
fn allocator_matches_rewrite() {
    let _guard = lock();
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    bind_stamps(&vtable, &node_vtable);
    run_cases("allocate_node_for_arg", |rng, _| {
        let arg = rng.word();
        let pool_obj = Arena::zeroed(16);
        let pool_cell = Arena::from_words(&[pool_obj.addr()]);
        rt::bind(POOL_VA, pool_cell.addr());

        let node_bytes = rng.bytes(NODE_LEN);
        let node = Arena::from_bytes(&node_bytes);
        let got_node = rng.below(4) != 0;
        install(&[(1, "s1_0"), (2, "s2_0")]);
        rt::set_answers(1, &[if got_node { node.addr() } else { 0 }]);
        let _ = rt::take_log();
        let ret = fn_00d1f5b0::rw_00d1f5b0(0, arg);
        let raw = rt::take_log();

        let mut world = recorder(&[0; OBJ_LEN], &[], false);
        world.node = if got_node { node.addr() } else { 0 };
        world.node_img = node_bytes.clone();
        let lifted = SeekCoverTask::allocate_node_for_arg(&mut world, arg).map_or(0, Handle32::get);
        if ret != lifted {
            return Err(format!("return {ret:#x} vs {lifted:#x}"));
        }
        compare_calls(
            &raw,
            &[(1, "alloc"), (2, "node_ctor")],
            &[(pool_obj.addr(), "pool"), (node.addr(), "node")],
            None,
            &world.events,
        )?;
        compare_images("node", node.bytes(), &world.node_img)?;
        Ok(if got_node { "node taken" } else { "pool empty" })
    });
}

/// The test binary's runs share the runtime's scripted state, so each test
/// holds this lock for its whole body.
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

/// Runs a case's trials, with the case's name in every report.
fn run_cases(name: &str, trial: impl FnMut(&mut Rng, u32) -> Result<&'static str, String>) {
    run_cases_unlocked(name, trial);
}
