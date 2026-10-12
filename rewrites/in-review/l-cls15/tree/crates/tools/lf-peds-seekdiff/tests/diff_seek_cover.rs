//! Differential tests: the lifted seek-cover task methods against their
//! verified 32-bit rewrites, and deliberately wrong lifts of each that the
//! same comparison must catch.
//!
//! Each case runs the rewrite and the lifted method on the same generated
//! inputs, in the same test memory layout. The comparison covers the
//! return value, every numbered callee call in order with its arguments, a
//! copy of the object at each call, the object after the call, and every
//! other word the method writes (the subtask, the node). Floats are
//! compared as their bits.
//!
//! The rewrites run on the 32-bit target only, so this file is compiled for
//! that target alone. On other hosts the lifted crate's host tests run.

#![cfg(target_arch = "x86")]

#[path = "support/mod.rs"]
mod support;

use lf_core::Handle32;
use lf_peds_seekdiff::rewrites::{fn_00d1d3d0, fn_00d1d4c0, fn_00d1d650, fn_00d1d7a0, fn_00d1e060, fn_00d1f5b0};
use lf_peds_seekdiff::rt;
use lf_peds_tasks::seek_cover::{
    CoverRowArgs, CoverRoute, CoverSubTask, CoverWorker, FullArgs, OwnedSlot, RouteArgs, RouteExtra,
    SeekCoverTask, SeekCoverWorld, ShortArgs, SmallNode, Stamp, TaskNode,
};
use support::{Arena, Ev, Norm, Rng, compare_bytes, compare_calls, count_disagreements, install, lock, rd32, run_cases, wr32};

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

/// Which lift a case runs: the lifted method, or a deliberately wrong one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Lift {
    Real,
    Wrong,
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
    fn new(seed: &[u8], table: &[u8], watch: bool) -> Self {
        Self {
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

/// Object names every seek-cover case uses in its call logs.
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

/// The class stamps the rewrites write, bound to test memory. The arenas
/// stay alive as long as the returned value.
struct Stamps {
    _vtable: Arena,
    _node_vtable: Arena,
}

fn bind_stamps() -> Stamps {
    let vtable = Arena::zeroed(4);
    let node_vtable = Arena::zeroed(4);
    rt::bind(VTABLE_VA, vtable.addr());
    rt::bind(NODE_VTABLE_VA, node_vtable.addr());
    Stamps { _vtable: vtable, _node_vtable: node_vtable }
}

const CTOR_IDS: [(u32, &str); 4] = [(1, "base"), (2, "init60"), (3, "init6c"), (4, "reinit")];

// ----------------------------------------------------------------------------
// The full constructor.
// ----------------------------------------------------------------------------

fn full_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
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
    let _ = rt::take_log();
    rt::watch(obj.addr(), OBJ_LEN as u32);
    let ret = fn_00d1d3d0::rw_00d1d3d0(
        obj.addr(), w, pa.addr(), u32::from(b1), u32::from(b2), pb.addr(), f, mode,
    );
    let raw = rt::take_log();
    rt::unwatch();
    if ret != obj.addr() {
        return Err(format!("return {ret:#x} is not the object"));
    }

    let mut world = Recorder::new(&seed, &[], true);
    let mut t = load(&seed);
    let args = FullArgs { pos: w, dir_src: &pos_a, mode_byte: b1, latch_src: b2, slot_src: &pos_b, weight: f, mode };
    match lift {
        Lift::Real => t.construct_full(&mut world, &args),
        Lift::Wrong => wrong::construct_full(&mut t, &mut world, &args),
    }
    let lifted_img = world.image(&t);
    compare_calls(&raw, &CTOR_IDS, &object_names(obj.addr()), None, &world.events)?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    Ok(if mode & 7 == 0 { "mode zero" } else { "mode nonzero" })
}

#[test]
fn full_constructor_matches_rewrite() {
    let _guard = lock();
    let _stamps = bind_stamps();
    run_cases("construct_full", &["mode zero", "mode nonzero"], |rng| full_case(rng, Lift::Real));
}

#[test]
fn full_constructor_wrong_lift_is_caught() {
    let _guard = lock();
    let _stamps = bind_stamps();
    let bad = count_disagreements("construct_full (wrong)", |rng| full_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong full constructor was not caught");
}

// ----------------------------------------------------------------------------
// The constructor that takes a cover-table row.
// ----------------------------------------------------------------------------

fn row_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
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
    let mut world = Recorder::new(&seed, &table_bytes, true);
    world.query = query;
    let mut t = load(&seed);
    let args = CoverRowArgs { pos, dir_src: &dir, flags0, flags1, index, weight, mode };
    match lift {
        Lift::Real => t.construct_cover_row(&mut world, &args),
        Lift::Wrong => wrong::construct_cover_row(&mut t, &mut world, &args),
    }
    let lifted_img = world.image(&t);

    let mut names = object_names(obj.addr());
    names.push((table.addr() + index * ROW_STRIDE as u32, "row"));
    let ids = [(1, "base"), (2, "init60"), (3, "init6c"), (4, "query"), (5, "reinit")];
    compare_calls(&raw, &ids, &names, Some(4), &world.events)?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    Ok(if row_flags & 7 != 0 { "populated row" } else { "empty row" })
}

#[test]
fn row_constructor_matches_rewrite() {
    let _guard = lock();
    let _stamps = bind_stamps();
    run_cases("construct_cover_row", &["populated row", "empty row"], |rng| row_case(rng, Lift::Real));
}

#[test]
fn row_constructor_wrong_lift_is_caught() {
    let _guard = lock();
    let _stamps = bind_stamps();
    let bad = count_disagreements("construct_cover_row (wrong)", |rng| row_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong row constructor was not caught");
}

// ----------------------------------------------------------------------------
// The shorter constructor.
// ----------------------------------------------------------------------------

fn short_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
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

    let mut world = Recorder::new(&seed, &[], true);
    let mut t = load(&seed);
    let args = ShortArgs { pos: w, dir_src: &pos_a, mode_byte: b1, latch_src: b2, weight: f, mode };
    match lift {
        Lift::Real => t.construct_short(&mut world, &args),
        Lift::Wrong => wrong::construct_short(&mut t, &mut world, &args),
    }
    let lifted_img = world.image(&t);
    compare_calls(&raw, &CTOR_IDS, &object_names(obj.addr()), None, &world.events)?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    Ok("short")
}

#[test]
fn short_constructor_matches_rewrite() {
    let _guard = lock();
    let _stamps = bind_stamps();
    run_cases("construct_short", &["short"], |rng| short_case(rng, Lift::Real));
}

#[test]
fn short_constructor_wrong_lift_is_caught() {
    let _guard = lock();
    let _stamps = bind_stamps();
    let bad = count_disagreements("construct_short (wrong)", |rng| short_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong short constructor was not caught");
}

// ----------------------------------------------------------------------------
// The destructor.
// ----------------------------------------------------------------------------

fn dtor_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let mut seed = rng.bytes(OBJ_LEN);
    // The owned words are zero about a third of the time each, so every
    // branch of the destructor runs.
    if rng.chance(3) {
        wr32(&mut seed, 0x30, 0);
    }
    if rng.chance(3) {
        wr32(&mut seed, 0xB0, 0);
    }
    let answer = rng.word();
    let obj = Arena::from_bytes(&seed);
    install(&[(1, "s1_1"), (2, "s2_0"), (3, "s3_0"), (4, "s4_0")]);
    rt::set_answers(4, &[answer]);
    let _ = rt::take_log();
    rt::watch(obj.addr(), OBJ_LEN as u32);
    let ret = fn_00d1d7a0::rw_00d1d7a0(obj.addr());
    let raw = rt::take_log();
    rt::unwatch();

    let mut world = Recorder::new(&seed, &[], true);
    world.dtor_answer = answer;
    let mut t = load(&seed);
    let lifted_ret = match lift {
        Lift::Real => t.destruct(&mut world),
        Lift::Wrong => wrong::destruct(&mut t, &mut world),
    };
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
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    let owned = u32::from(rd32(&seed, 0x30) != 0) + u32::from(rd32(&seed, 0xB0) != 0);
    Ok(match owned {
        0 => "no owned word",
        1 => "one owned word",
        _ => "two owned words",
    })
}

#[test]
fn destructor_matches_rewrite() {
    let _guard = lock();
    let _stamps = bind_stamps();
    run_cases("destruct", &["no owned word", "one owned word", "two owned words"], |rng| {
        dtor_case(rng, Lift::Real)
    });
}

#[test]
fn destructor_wrong_lift_is_caught() {
    let _guard = lock();
    let _stamps = bind_stamps();
    let bad = count_disagreements("destruct (wrong)", |rng| dtor_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong destructor was not caught");
}

// ----------------------------------------------------------------------------
// The subtask router.
// ----------------------------------------------------------------------------

fn route_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let mut seed = rng.bytes(OBJ_LEN);
    // The route is the low three bits of the flag word. The original runs
    // routes zero to two and faults on the others, so the proof draws only
    // those, with a non-null worker and subtask.
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
    let ret = fn_00d1e060::rw_00d1e060(obj.addr());
    let raw = rt::take_log();
    rt::unwatch();

    let mut world = Recorder::new(&seed, &[], true);
    world.worker = worker.addr();
    world.sub = sub.addr();
    world.sub_img = sub_bytes.clone();
    let mut t = load(&seed);
    let lifted = match lift {
        Lift::Real => t.route_to_sub_task(&mut world),
        Lift::Wrong => wrong::route_to_sub_task(&mut t, &mut world),
    }
    .map_or(0, Handle32::get);
    let lifted_img = world.image(&t);

    if ret != lifted {
        return Err(format!("return {ret:#x} vs {lifted:#x}"));
    }
    compare_calls(
        &raw,
        &[(1, "fetch"), (2, "route2"), (3, "route1"), (4, "route0")],
        &[
            (obj.addr(), "obj"),
            (obj.addr() + 0x20, "dir"),
            (obj.addr() + 0x80, "slot"),
            (pool_obj.addr(), "pool"),
            (worker.addr(), "worker"),
            (sub.addr(), "sub"),
        ],
        None,
        &world.events,
    )?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    compare_bytes("subtask", sub.bytes(), &world.sub_img)?;
    Ok(match CoverRoute::from_flags(flags) {
        Some(CoverRoute::Zero) => "route zero",
        Some(CoverRoute::One) => "route one",
        Some(CoverRoute::Two) => "route two",
        None => "other",
    })
}

#[test]
fn router_matches_rewrite() {
    let _guard = lock();
    let _stamps = bind_stamps();
    run_cases("route_to_sub_task", &["route zero", "route one", "route two"], |rng| {
        route_case(rng, Lift::Real)
    });
}

#[test]
fn router_wrong_lift_is_caught() {
    let _guard = lock();
    let _stamps = bind_stamps();
    let bad = count_disagreements("route_to_sub_task (wrong)", |rng| route_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong router was not caught");
}

// ----------------------------------------------------------------------------
// The node allocator.
// ----------------------------------------------------------------------------

fn alloc_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let arg = rng.word();
    let pool_obj = Arena::zeroed(16);
    let pool_cell = Arena::from_words(&[pool_obj.addr()]);
    rt::bind(POOL_VA, pool_cell.addr());

    let node_bytes = rng.bytes(NODE_LEN);
    let node = Arena::from_bytes(&node_bytes);
    let got_node = !rng.chance(4);
    install(&[(1, "s1_0"), (2, "s2_0")]);
    rt::set_answers(1, &[if got_node { node.addr() } else { 0 }]);
    let _ = rt::take_log();
    let ret = fn_00d1f5b0::rw_00d1f5b0(0, arg);
    let raw = rt::take_log();

    let mut world = Recorder::new(&[0; OBJ_LEN], &[], false);
    world.node = if got_node { node.addr() } else { 0 };
    world.node_img = node_bytes.clone();
    let lifted = match lift {
        Lift::Real => SeekCoverTask::allocate_node_for_arg(&mut world, arg),
        Lift::Wrong => wrong::allocate_node_for_arg(&mut world, arg),
    }
    .map_or(0, Handle32::get);
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
    compare_bytes("node", node.bytes(), &world.node_img)?;
    Ok(if got_node { "node taken" } else { "pool empty" })
}

#[test]
fn allocator_matches_rewrite() {
    let _guard = lock();
    let _stamps = bind_stamps();
    run_cases("allocate_node_for_arg", &["node taken", "pool empty"], |rng| alloc_case(rng, Lift::Real));
}

#[test]
fn allocator_wrong_lift_is_caught() {
    let _guard = lock();
    let _stamps = bind_stamps();
    let bad = count_disagreements("allocate_node_for_arg (wrong)", |rng| alloc_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong allocator was not caught");
}

// ----------------------------------------------------------------------------
// Wrong lifts. Each copies one lifted method and makes one deliberate fault
// that a reader could plausibly make from the rewrite. The same comparison
// must report it.
// ----------------------------------------------------------------------------

mod wrong {
    use super::*;

    /// Fault: the flag word keeps bit one, which the constructor clears.
    pub fn construct_full<W: SeekCoverWorld>(t: &mut SeekCoverTask, w: &mut W, a: &FullArgs<'_>) {
        w.base_construct(t);
        w.set_stamp(t, Stamp::SeekCover);
        t.dir = *a.dir_src;
        t.pos = a.pos;
        t.mode_byte = a.mode_byte;
        w.init_member_60(t);
        w.init_member_6c(t);
        t.index = 0xFFFF_FFFF;
        t.slot[0] = a.slot_src[0];
        t.slot[1] = a.slot_src[1];
        t.slot[2] = a.slot_src[2];
        t.flags = ((a.mode & 7) << 3) | (t.flags & 0xFFFF_FFC3) | 1;
        t.latch ^= (t.latch ^ a.latch_src) & 1;
        t.weight = a.weight;
        t.tail = [0, 0, 0];
        t.mark = 0xFFFF_FFFF;
        t.tail_end = [0, 0];
        w.reinit_shared(t);
    }

    /// Fault: the empty-row path never runs the shared re-initialiser.
    pub fn construct_cover_row<W: SeekCoverWorld>(t: &mut SeekCoverTask, w: &mut W, a: &CoverRowArgs<'_>) {
        w.base_construct(t);
        w.set_stamp(t, Stamp::SeekCover);
        t.dir = *a.dir_src;
        t.pos = a.pos;
        t.mode_byte = a.flags0 as u8;
        w.init_member_60(t);
        w.init_member_6c(t);
        t.index = a.index;
        t.slot[0] = 0;
        t.slot[1] = 0;
        t.slot[2] = 0;
        t.flags = (t.flags & 0xFFFF_FFC2) | ((a.mode & 7) << 3);
        t.latch ^= (t.latch ^ a.flags1 as u8) & 1;
        t.flags |= 2;
        t.weight = a.weight;
        t.tail = [0, 0, 0];
        t.mark = 0xFFFF_FFFF;
        t.tail_end = [0, 0];
        let row_flags = w.cover_row_flags(a.index);
        if row_flags & 7 != 0 {
            t.slot = w.cover_row_query(t, a.index);
            w.reinit_shared(t);
        } else {
            t.slot[2] = 0;
            t.slot[1] = 0;
            t.slot[0] = 0;
            t.flags &= 0xFFFF_FFF8;
            t.index = 0xFFFF_FFFF;
        }
    }

    /// Fault: the two member initialisers run in the wrong order.
    pub fn construct_short<W: SeekCoverWorld>(t: &mut SeekCoverTask, w: &mut W, a: &ShortArgs<'_>) {
        w.base_construct(t);
        w.set_stamp(t, Stamp::SeekCover);
        t.dir = *a.dir_src;
        t.pos = a.pos;
        t.mode_byte = a.mode_byte;
        w.init_member_6c(t);
        w.init_member_60(t);
        t.index = 0xFFFF_FFFF;
        t.slot[0] = 0;
        t.slot[1] = 0;
        t.slot[2] = 0;
        t.flags = ((a.mode & 7) << 3) | (t.flags & 0xFFFF_FFC0);
        t.latch ^= (t.latch ^ a.latch_src) & 1;
        t.weight = a.weight;
        t.tail = [0, 0, 0];
        t.mark = 0xFFFF_FFFF;
        t.tail_end = [0, 0];
        w.reinit_shared(t);
    }

    /// Fault: the owned word is released but never cleared.
    pub fn destruct<W: SeekCoverWorld>(t: &mut SeekCoverTask, w: &mut W) -> u32 {
        w.set_stamp(t, Stamp::SeekCover);
        let owned_pos = t.pos;
        if owned_pos != 0 {
            w.release_owned(t, OwnedSlot::Pos, owned_pos);
            t.pos = 0;
        }
        let owned_link = t.link;
        if owned_link != 0 {
            w.release_owned(t, OwnedSlot::Link, owned_link);
        }
        w.release_member_6c(t);
        w.release_member_60(t);
        w.base_destruct(t)
    }

    /// Fault: the lane is read without sign extension.
    pub fn route_to_sub_task<W: SeekCoverWorld>(t: &mut SeekCoverTask, w: &mut W) -> Option<Handle32<CoverSubTask>> {
        let route = CoverRoute::from_flags(t.flags)?;
        let worker = w.fetch_worker(t)?;
        let extra = match route {
            CoverRoute::Zero => None,
            CoverRoute::One => Some(RouteExtra::SlotRef),
            CoverRoute::Two => Some(RouteExtra::Index(t.index)),
        };
        let args = RouteArgs {
            pos: t.pos,
            mode: t.mode_byte,
            latch_bit: t.latch & 1,
            extra,
            speed: t.weight,
            lane: (t.flags >> 3) & 7,
        };
        let sub = w.run_route(t, route, worker, &args)?;
        let mine = t.latch;
        let theirs = w.latch_of(sub);
        let flip = (mine ^ theirs) & 8;
        w.set_latch_of(sub, theirs ^ flip);
        Some(sub)
    }

    /// Fault: the argument goes into the tail word, not the argument word.
    pub fn allocate_node_for_arg<W: SeekCoverWorld>(w: &mut W, arg: u32) -> Option<Handle32<TaskNode>> {
        let node = w.allocate_node()?;
        w.construct_node(node);
        w.store_small_node(node, &SmallNode { arg: 0, flag: 0, tail: arg });
        Some(node)
    }
}
