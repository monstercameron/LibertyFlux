//! Differential tests: the lifted shooting-task methods against their
//! verified 32-bit rewrites, and deliberately wrong lifts of each that the
//! same comparison must catch.
//!
//! The shooting task is a seek-cover task with the subject word at its
//! offset eight. Its virtual calls go through vtables the test plants in
//! its own memory: each slot the rewrite reaches points at a recording stub,
//! and the lifted side records the same call by name. The rewrites run on
//! the 32-bit target only.

#![cfg(target_arch = "x86")]
// The test memory is a buffer of words that the rewrite is given by address and the support code views as
// bytes. This is test-only code: the lifted module forbids unsafe code.
#![allow(unsafe_code)]

#[path = "support/mod.rs"]
mod support;

use lf_core::Handle32;
use lf_peds_seekdiff::rewrites::{fn_00d1d810, fn_00d1efd0, fn_00d1f5f0, fn_00d20580, fn_00d20800};
use lf_peds_seekdiff::rt;
use lf_peds_tasks::seek_cover::{
    InitArg, PursuitNode, PursuitTarget, SeekCoverTask, ShootingTask, ShootingWorld, ShotNodeArgs,
    ShotSubject, TaskNode,
};
use support::{
    Arena, Ev, Norm, Rng, compare_bytes, compare_calls_ov, count_disagreements, install, lock,
    rd32, run_cases, wr32,
};

/// Length of the shooting object image the proof compares.
const OBJ_LEN: usize = 0xC0;
/// Length of a subject image.
const SUBJECT_LEN: usize = 0x10;
/// Length of a target image: the target's flag byte is at offset 0x26C.
const TARGET_LEN: usize = 0x270;
/// Length of a pursuit or firing node image.
const NODE_LEN: usize = 0x20;
/// Words in a planted vtable (enough for the highest slot used).
const VTABLE_WORDS: usize = 32;

/// File address of the pool object cell.
const POOL_VA: u32 = 0x0167_E2A0;
/// File address of the firing-solution scale factor.
const FACTOR_VA: u32 = 0x00FE_86B4;
/// File address of the pursuit node class stamp.
const PURSUIT_NODE_VA: u32 = 0x00E8_9044;

/// Callee numbers the shooting rewrites use, and the virtual-slot ids the
/// stubs record under.
const VIRT_DISPATCH: u32 = 6;
const VIRT_ASK: u32 = 7;
const VIRT_TAG: u32 = 8;

/// Which lift a case runs: the lifted method, or a deliberately wrong one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Lift {
    Real,
    Wrong,
}

/// Reads the shooting object image.
fn load(b: &[u8]) -> ShootingTask {
    ShootingTask {
        subject: Handle32::new(rd32(b, 0x08)),
        base: SeekCoverTask {
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
        },
    }
}

/// Writes the shooting object's owned words into an image, leaving every other byte.
fn store(t: &ShootingTask, b: &mut [u8]) {
    wr32(b, 0x08, Handle32::raw_or_zero(t.subject));
    let s = &t.base;
    for (i, w) in s.dir.iter().enumerate() {
        wr32(b, 0x20 + 4 * i, *w);
    }
    wr32(b, 0x30, s.pos);
    b[0x5C] = s.mode_byte;
    wr32(b, 0x74, s.index);
    for (i, w) in s.slot.iter().enumerate() {
        wr32(b, 0x80 + 4 * i, *w);
    }
    wr32(b, 0x90, s.flags);
    b[0x94] = s.latch;
    wr32(b, 0x98, s.weight);
    for (i, w) in s.tail.iter().enumerate() {
        wr32(b, 0xA0 + 4 * i, *w);
    }
    wr32(b, 0xB0, s.link);
    wr32(b, 0xB4, s.mark);
    for (i, w) in s.tail_end.iter().enumerate() {
        wr32(b, 0xB8 + 4 * i, *w);
    }
}

/// A planted vtable: the given slot words, all others zero.
fn vtable(slots: &[(usize, u32)]) -> Arena {
    let mut words = vec![0u32; VTABLE_WORDS];
    for (slot, addr) in slots {
        words[*slot] = *addr;
    }
    Arena::from_words(&words)
}

/// The lifted side's collaborators, recording calls the way the rewrite's
/// stubs record theirs.
struct Recorder {
    img: Vec<u8>,
    watch: bool,
    events: Vec<Ev>,
    /// The target object's image, read for its flag byte.
    target: Vec<u8>,
    /// The pool's node: the address and the lifted copy of its image.
    node: u32,
    node_img: Vec<u8>,
    /// The subject: the address and the lifted copy of its image.
    subject: u32,
    subject_img: Vec<u8>,
    /// The first argument of the initialiser, by address.
    a0: u32,
    /// The scale factor's bits.
    scale: u32,
    /// Answers the lifted side returns, in call order.
    dispatch_answer: u32,
    ask_answer: u32,
    base_answer: u32,
    tags: std::collections::VecDeque<u32>,
    /// The chain: each link's next link, by address.
    chain: Vec<(u32, u32)>,
    /// The chain's links, named as the call logs name them.
    links: Vec<(u32, &'static str)>,
}

impl Recorder {
    fn image(&self, t: &ShootingTask) -> Vec<u8> {
        let mut v = self.img.clone();
        store(t, &mut v);
        v
    }

    fn ev(&mut self, name: &'static str, args: Vec<Norm>, this: Option<&ShootingTask>) {
        let snap = match (self.watch, this) {
            (true, Some(t)) => Some(self.image(t)),
            _ => None,
        };
        self.events.push(Ev { name, args, snap });
    }

    /// A call made while the object is unchanged: the object's image is the
    /// image it had when the method began, which this method never changes
    /// before the call (the struct's owned words are only written after it).
    fn ev_unchanged(&mut self, name: &'static str, args: Vec<Norm>) {
        let snap = if self.watch {
            Some(self.img.clone())
        } else {
            None
        };
        self.events.push(Ev { name, args, snap });
    }

    fn node_norm(&self, n: Handle32<TaskNode>) -> Norm {
        if n.get() == self.node {
            Norm::S("node")
        } else {
            Norm::W(n.get())
        }
    }

    fn subject_norm(&self, s: Handle32<ShotSubject>) -> Norm {
        if s.get() == self.subject {
            Norm::S("subject")
        } else {
            Norm::W(s.get())
        }
    }
}

impl ShootingWorld for Recorder {
    fn base_destruct(&mut self, this: &mut ShootingTask) -> u32 {
        self.ev("base_dtor", vec![Norm::S("obj")], Some(&*this));
        self.base_answer
    }

    fn free_to_pool(&mut self, this: &mut ShootingTask) {
        self.ev("free", vec![Norm::S("pool"), Norm::S("obj")], Some(&*this));
    }

    fn fetch_node(&mut self) -> Option<Handle32<TaskNode>> {
        self.ev_unchanged("alloc", vec![Norm::S("pool")]);
        Handle32::new(self.node)
    }

    fn construct_node(&mut self, node: Handle32<TaskNode>) {
        let n = self.node_norm(node);
        self.ev_unchanged("node_ctor", vec![n]);
    }

    fn store_pursuit_node(&mut self, _node: Handle32<TaskNode>, fields: &PursuitNode) {
        wr32(&mut self.node_img, 0, rt::relocated(PURSUIT_NODE_VA));
        wr32(&mut self.node_img, 0x14, fields.arg);
        self.node_img[0x18..0x1A].copy_from_slice(&fields.flag16.to_le_bytes());
        self.node_img[0x1A] = fields.flag8;
    }

    fn construct_shot_node(&mut self, node: Handle32<TaskNode>, args: &ShotNodeArgs) -> u32 {
        let n = self.node_norm(node);
        self.ev(
            "shot_ctor",
            vec![
                n,
                Norm::W(args.kind),
                Norm::W(args.pos),
                Norm::S("dir"),
                Norm::W(args.scaled_bits),
                Norm::W(args.tail),
            ],
            None,
        );
        self.base_answer
    }

    fn target_flags(&mut self, _target: Handle32<PursuitTarget>) -> u8 {
        self.target[0x26C]
    }

    fn solve(&mut self, this: &mut ShootingTask, target: Handle32<PursuitTarget>) {
        let _ = target;
        self.ev(
            "solve",
            vec![Norm::S("obj"), Norm::S("target"), Norm::S("scratch")],
            Some(&*this),
        );
    }

    fn dispatch_virtual(&mut self, this: &mut ShootingTask, arg: u32) -> u32 {
        self.ev(
            "dispatch54",
            vec![Norm::S("obj"), Norm::W(arg)],
            Some(&*this),
        );
        self.dispatch_answer
    }

    fn scale_factor(&mut self) -> u32 {
        self.scale
    }

    fn subject_flags(&mut self, _subject: Handle32<ShotSubject>) -> u32 {
        rd32(&self.subject_img, 0xC)
    }

    fn set_subject_flags(&mut self, _subject: Handle32<ShotSubject>, flags: u32) {
        wr32(&mut self.subject_img, 0xC, flags);
    }

    fn ask_subject(
        &mut self,
        subject: Handle32<ShotSubject>,
        a0: Handle32<InitArg>,
        a1: u32,
        a2: u32,
    ) -> u32 {
        let s = self.subject_norm(subject);
        let a = if a0.get() == self.a0 {
            Norm::S("a0")
        } else {
            Norm::W(a0.get())
        };
        self.ev_unchanged("ask14", vec![s, a, Norm::W(a1), Norm::W(a2)]);
        self.ask_answer
    }

    fn init_member_60(&mut self, this: &mut ShootingTask) {
        self.ev("init60", vec![Norm::S("m60")], Some(&*this));
    }

    fn init_member_6c(&mut self, this: &mut ShootingTask) {
        self.ev("init6c", vec![Norm::S("m6c")], Some(&*this));
    }

    fn finish_arg(&mut self, a0: Handle32<InitArg>) {
        let a = if a0.get() == self.a0 {
            Norm::S("a0")
        } else {
            Norm::W(a0.get())
        };
        self.ev_unchanged("finish", vec![a]);
    }

    fn chain_tag(&mut self, link: Handle32<ShotSubject>) -> u32 {
        let l = match self.links.iter().find(|(a, _)| *a == link.get()) {
            Some((_, n)) => Norm::S(n),
            None => Norm::W(link.get()),
        };
        self.ev("tag", vec![l], None);
        self.tags.pop_front().unwrap_or(0)
    }

    fn chain_next(&mut self, link: Handle32<ShotSubject>) -> Option<Handle32<ShotSubject>> {
        self.chain
            .iter()
            .find(|(from, _)| *from == link.get())
            .and_then(|(_, to)| Handle32::new(*to))
    }
}

/// Object names the shooting cases use in their call logs.
fn object_names(obj: u32) -> Vec<(u32, &'static str)> {
    vec![
        (obj, "obj"),
        (obj + 0x20, "dir"),
        (obj + 0x60, "m60"),
        (obj + 0x6C, "m6c"),
        (obj + 0x80, "slot"),
        (obj + 0xB0, "link_slot"),
    ]
}

/// Binds the global cells the shooting rewrites read: the pool cell (to a
/// pool object), the scale factor cell and the pursuit node class stamp.
struct Globals {
    pool_obj: Arena,
    _pool_cell: Arena,
    _factor: Arena,
    _stamp: Arena,
}

impl Globals {
    /// The address of the pool object the pool cell names.
    fn pool_addr(&self) -> u32 {
        self.pool_obj.addr()
    }
}

fn bind_globals(scale: u32) -> Globals {
    let pool_obj = Arena::zeroed(16);
    let pool_cell = Arena::from_words(&[pool_obj.addr()]);
    let factor = Arena::from_words(&[scale]);
    let stamp = Arena::zeroed(4);
    rt::bind(POOL_VA, pool_cell.addr());
    rt::bind(FACTOR_VA, factor.addr());
    rt::bind(PURSUIT_NODE_VA, stamp.addr());
    Globals {
        pool_obj,
        _pool_cell: pool_cell,
        _factor: factor,
        _stamp: stamp,
    }
}

/// A fresh recorder seeded like the rewrite's memory.
fn recorder(seed: &[u8], target: &[u8], node: u32, node_img: &[u8], scale: u32) -> Recorder {
    Recorder {
        img: seed.to_vec(),
        watch: true,
        events: Vec::new(),
        target: target.to_vec(),
        node,
        node_img: node_img.to_vec(),
        subject: 0,
        subject_img: Vec::new(),
        a0: 0,
        scale,
        dispatch_answer: 0,
        ask_answer: 0,
        base_answer: 0,
        tags: std::collections::VecDeque::new(),
        chain: Vec::new(),
        links: Vec::new(),
    }
}

// ----------------------------------------------------------------------------
// The scalar deleting destructor.
// ----------------------------------------------------------------------------

fn delete_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let g = bind_globals(0);
    let seed = rng.bytes(OBJ_LEN);
    let free_flag = rng.word();
    let obj = Arena::from_bytes(&seed);
    install(&[(1, "s1_0"), (2, "s2_1")]);
    let _ = rt::take_log();
    rt::watch(obj.addr(), OBJ_LEN as u32);
    let ret = fn_00d1d810::rw_00d1d810(obj.addr(), free_flag);
    let raw = rt::take_log();
    rt::unwatch();
    if ret != obj.addr() {
        return Err(format!("return {ret:#x} is not the object"));
    }

    let mut world = recorder(&seed, &[], 0, &[], 0);
    let mut t = load(&seed);
    match lift {
        Lift::Real => t.scalar_delete(&mut world, free_flag),
        Lift::Wrong => wrong::scalar_delete(&mut t, &mut world, free_flag),
    }
    let lifted_img = world.image(&t);
    compare_calls_ov(
        &raw,
        &[(1, "base_dtor"), (2, "free")],
        &[object_names(obj.addr()), vec![(g.pool_addr(), "pool")]].concat(),
        &[],
        &world.events,
    )?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    Ok(if free_flag & 1 != 0 {
        "pool free"
    } else {
        "no pool free"
    })
}

#[test]
fn scalar_delete_matches_rewrite() {
    let _guard = lock();
    run_cases("scalar_delete", &["pool free", "no pool free"], |rng| {
        delete_case(rng, Lift::Real)
    });
}

#[test]
fn scalar_delete_wrong_lift_is_caught() {
    let _guard = lock();
    let bad = count_disagreements("scalar_delete (wrong)", |rng| delete_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong deleting destructor was not caught");
}

// ----------------------------------------------------------------------------
// The follow-up action for a target.
// ----------------------------------------------------------------------------

fn pursuit_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let g = bind_globals(0);
    let node_bytes = rng.bytes(NODE_LEN);
    let node = Arena::from_bytes(&node_bytes);
    let target_bytes = {
        let mut t = rng.bytes(TARGET_LEN);
        // The node bit is set about half the time.
        if rng.chance(2) {
            t[0x26C] |= 4;
        } else {
            t[0x26C] &= !4;
        }
        t
    };
    let target = Arena::from_bytes(&target_bytes);
    let mut seed = rng.bytes(OBJ_LEN);
    let vt = vtable(&[(0x54 / 4, rt::stub("v6_1"))]);
    wr32(&mut seed, 0, vt.addr());
    let obj = Arena::from_bytes(&seed);
    let dispatch_answer = rng.word();
    let got_node = !rng.chance(4);

    install(&[(1, "s1_0"), (2, "s2_0"), (3, "s3_2")]);
    rt::set_answers(1, &[if got_node { node.addr() } else { 0 }]);
    rt::set_answers(VIRT_DISPATCH, &[dispatch_answer]);
    let _ = rt::take_log();
    rt::watch(obj.addr(), OBJ_LEN as u32);
    let ret = fn_00d1efd0::rw_00d1efd0(obj.addr(), target.addr());
    let raw = rt::take_log();
    rt::unwatch();

    let mut world = recorder(
        &seed,
        &target_bytes,
        if got_node { node.addr() } else { 0 },
        &node_bytes,
        0,
    );
    world.dispatch_answer = dispatch_answer;
    let mut t = load(&seed);
    let lifted = match lift {
        Lift::Real => t.pursuit_action(
            &mut world,
            Handle32::new(target.addr()).unwrap_or_else(|| unreachable!()),
        ),
        Lift::Wrong => wrong::pursuit_action(
            &mut t,
            &mut world,
            Handle32::new(target.addr()).unwrap_or_else(|| unreachable!()),
        ),
    };
    let lifted_img = world.image(&t);
    if ret != lifted {
        return Err(format!("return {ret:#x} vs {lifted:#x}"));
    }
    compare_calls_ov(
        &raw,
        &[
            (1, "alloc"),
            (2, "node_ctor"),
            (3, "solve"),
            (VIRT_DISPATCH, "dispatch54"),
        ],
        &[
            (obj.addr(), "obj"),
            (target.addr(), "target"),
            (node.addr(), "node"),
            (g.pool_addr(), "pool"),
            (vt.addr(), "vt"),
        ],
        &[(3, 2, "scratch")],
        &world.events,
    )?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    compare_bytes("node", node.bytes(), &world.node_img)?;
    Ok(if target_bytes[0x26C] & 4 != 0 {
        "node path"
    } else {
        "solver path"
    })
}

#[test]
fn pursuit_action_matches_rewrite() {
    let _guard = lock();
    run_cases("pursuit_action", &["node path", "solver path"], |rng| {
        pursuit_case(rng, Lift::Real)
    });
}

#[test]
fn pursuit_action_wrong_lift_is_caught() {
    let _guard = lock();
    let bad = count_disagreements("pursuit_action (wrong)", |rng| {
        pursuit_case(rng, Lift::Wrong)
    });
    assert!(bad > 0, "the wrong pursuit action was not caught");
}

// ----------------------------------------------------------------------------
// The firing-node builder.
// ----------------------------------------------------------------------------

fn shot_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let scale = rng.word();
    let g = bind_globals(scale);
    let node_bytes = rng.bytes(NODE_LEN);
    let node = Arena::from_bytes(&node_bytes);
    let got_node = !rng.chance(4);
    let seed = rng.bytes(OBJ_LEN);
    let obj = Arena::from_bytes(&seed);
    let answer = rng.word();
    let arg = rng.word();

    install(&[(1, "s1_0"), (2, "s2_5")]);
    rt::set_answers(1, &[if got_node { node.addr() } else { 0 }]);
    rt::set_answers(2, &[answer]);
    let _ = rt::take_log();
    let ret = fn_00d1f5f0::rw_00d1f5f0(obj.addr(), arg);
    let raw = rt::take_log();

    let mut world = recorder(
        &seed,
        &[],
        if got_node { node.addr() } else { 0 },
        &node_bytes,
        scale,
    );
    world.base_answer = answer;
    world.watch = false;
    let t = load(&seed);
    let lifted = match lift {
        Lift::Real => t.shot_node(&mut world, arg),
        Lift::Wrong => wrong::shot_node(&t, &mut world, arg),
    };
    if ret != lifted {
        return Err(format!("return {ret:#x} vs {lifted:#x}"));
    }
    compare_calls_ov(
        &raw,
        &[(1, "alloc"), (2, "shot_ctor")],
        &[
            (node.addr(), "node"),
            (obj.addr() + 0x20, "dir"),
            (g.pool_addr(), "pool"),
        ],
        &[],
        &world.events,
    )?;
    compare_bytes("node", node.bytes(), &world.node_img)?;
    Ok(if got_node { "node taken" } else { "pool empty" })
}

#[test]
fn shot_node_matches_rewrite() {
    let _guard = lock();
    run_cases("shot_node", &["node taken", "pool empty"], |rng| {
        shot_case(rng, Lift::Real)
    });
}

#[test]
fn shot_node_wrong_lift_is_caught() {
    let _guard = lock();
    let bad = count_disagreements("shot_node (wrong)", |rng| shot_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong firing-node builder was not caught");
}

// ----------------------------------------------------------------------------
// The subject initialiser.
// ----------------------------------------------------------------------------

fn subject_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let _g = bind_globals(0);
    let mut subject_bytes = rng.bytes(SUBJECT_LEN);
    // The ready bit is set about a third of the time.
    if rng.chance(3) {
        subject_bytes[0xC] |= 1;
    } else {
        subject_bytes[0xC] &= !1;
    }
    let vt = vtable(&[(0x14 / 4, rt::stub("v7_3"))]);
    wr32(&mut subject_bytes, 0, vt.addr());
    let subject = Arena::from_bytes(&subject_bytes);
    let a0_bytes = rng.bytes(0x40);
    let a0 = Arena::from_bytes(&a0_bytes);
    let a1 = rng.word();
    let a2 = rng.word();
    let mut ask = rng.word();
    if rng.chance(4) {
        ask &= !0xFF;
    }

    let mut seed = rng.bytes(OBJ_LEN);
    wr32(&mut seed, 0x08, subject.addr());
    let obj = Arena::from_bytes(&seed);

    install(&[(2, "s2_0"), (3, "s3_0"), (4, "s4_0")]);
    rt::set_answers(VIRT_ASK, &[ask]);
    let _ = rt::take_log();
    rt::watch(obj.addr(), OBJ_LEN as u32);
    let ret = fn_00d20580::rw_00d20580(obj.addr(), a0.addr(), a1, a2);
    let raw = rt::take_log();
    rt::unwatch();

    let mut world = recorder(&seed, &[], 0, &[], 0);
    world.subject = subject.addr();
    world.subject_img = subject_bytes.clone();
    world.a0 = a0.addr();
    world.ask_answer = ask;
    let mut t = load(&seed);
    let a0_h = Handle32::new(a0.addr()).unwrap_or_else(|| unreachable!());
    let lifted = match lift {
        Lift::Real => t.initialise_against_subject(&mut world, a0_h, a1, a2),
        Lift::Wrong => wrong::initialise_against_subject(&mut t, &mut world, a0_h, a1, a2),
    };
    let lifted_img = world.image(&t);
    if ret != lifted {
        return Err(format!("return {ret:#x} vs {lifted:#x}"));
    }
    compare_calls_ov(
        &raw,
        &[
            (2, "init60"),
            (3, "init6c"),
            (4, "finish"),
            (VIRT_ASK, "ask14"),
        ],
        &[
            (obj.addr(), "obj"),
            (subject.addr(), "subject"),
            (a0.addr(), "a0"),
            (obj.addr() + 0x60, "m60"),
            (obj.addr() + 0x6C, "m6c"),
        ],
        &[],
        &world.events,
    )?;
    compare_bytes("object", obj.bytes(), &lifted_img)?;
    compare_bytes("subject", subject.bytes(), &world.subject_img)?;
    let accepted = ret == 1;
    let ready = subject_bytes[0xC] & 1 != 0;
    Ok(match (ready, accepted) {
        (true, _) => "subject ready",
        (false, true) => "subject accepts",
        (false, false) => "subject declines",
    })
}

#[test]
fn subject_initialiser_matches_rewrite() {
    let _guard = lock();
    run_cases(
        "initialise_against_subject",
        &["subject ready", "subject accepts", "subject declines"],
        |rng| subject_case(rng, Lift::Real),
    );
}

#[test]
fn subject_initialiser_wrong_lift_is_caught() {
    let _guard = lock();
    let bad = count_disagreements("initialise_against_subject (wrong)", |rng| {
        subject_case(rng, Lift::Wrong)
    });
    assert!(bad > 0, "the wrong subject initialiser was not caught");
}

// ----------------------------------------------------------------------------
// The three-level type-tag check.
// ----------------------------------------------------------------------------

fn chain_case(rng: &mut Rng, lift: Lift) -> Result<&'static str, String> {
    let _g = bind_globals(0);
    let vt = vtable(&[(0xC / 4, rt::stub("v8_0"))]);
    let o3 = Arena::from_words(&[vt.addr(), 0, rng.word(), 0]);
    let o2 = Arena::from_words(&[vt.addr(), 0, o3.addr(), 0]);
    let o1 = Arena::from_words(&[vt.addr(), 0, o2.addr(), 0]);
    let mut seed = rng.bytes(OBJ_LEN);
    wr32(&mut seed, 0x08, o1.addr());
    if rng.chance(3) {
        wr32(&mut seed, 0xB0, 0);
    }
    let obj = Arena::from_bytes(&seed);

    let mut tags = [0u32; 3];
    for (i, want) in [0x11D, 0x41A, 0x76D].iter().enumerate() {
        tags[i] = if rng.chance(4) { rng.word() } else { *want };
    }
    install(&[]);
    rt::set_answers(VIRT_TAG, &tags);
    let _ = rt::take_log();
    let ret = fn_00d20800::rw_00d20800(obj.addr());
    let raw = rt::take_log();

    let mut world = recorder(&seed, &[], 0, &[], 0);
    world.tags = tags.iter().copied().collect();
    world.chain = vec![(o1.addr(), o2.addr()), (o2.addr(), o3.addr())];
    world.links = vec![(o1.addr(), "o1"), (o2.addr(), "o2"), (o3.addr(), "o3")];
    let t = load(&seed);
    let lifted = match lift {
        Lift::Real => t.has_full_chain(&mut world),
        Lift::Wrong => wrong::has_full_chain(&t, &mut world),
    };
    if ret != lifted {
        return Err(format!("return {ret:#x} vs {lifted:#x}"));
    }
    compare_calls_ov(
        &raw,
        &[(VIRT_TAG, "tag")],
        &[(o1.addr(), "o1"), (o2.addr(), "o2"), (o3.addr(), "o3")],
        &[],
        &world.events,
    )?;
    Ok(match ret {
        1 => "chain complete",
        _ => "chain broken",
    })
}

#[test]
fn chain_check_matches_rewrite() {
    let _guard = lock();
    run_cases(
        "has_full_chain",
        &["chain complete", "chain broken"],
        |rng| chain_case(rng, Lift::Real),
    );
}

#[test]
fn chain_check_wrong_lift_is_caught() {
    let _guard = lock();
    let bad = count_disagreements("has_full_chain (wrong)", |rng| chain_case(rng, Lift::Wrong));
    assert!(bad > 0, "the wrong chain check was not caught");
}

// ----------------------------------------------------------------------------
// Wrong lifts. Each copies one lifted method and makes one deliberate fault.
// ----------------------------------------------------------------------------

mod wrong {
    use super::*;

    /// Fault: the free flag is read at bit one, not bit zero.
    pub fn scalar_delete<W: ShootingWorld>(t: &mut ShootingTask, w: &mut W, free_flag: u32) {
        w.base_destruct(t);
        if free_flag & 2 != 0 {
            w.free_to_pool(t);
        }
    }

    /// Fault: the node bit is read at bit two.
    pub fn pursuit_action<W: ShootingWorld>(
        t: &mut ShootingTask,
        w: &mut W,
        target: Handle32<PursuitTarget>,
    ) -> u32 {
        if w.target_flags(target) & 2 != 0 {
            let Some(node) = w.fetch_node() else {
                return 0;
            };
            w.construct_node(node);
            w.store_pursuit_node(
                node,
                &PursuitNode {
                    arg: 0,
                    flag16: 1,
                    flag8: 1,
                },
            );
            return node.get();
        }
        w.solve(t, target);
        w.dispatch_virtual(t, 0x3E8)
    }

    /// Fault: the argument is converted as unsigned.
    pub fn shot_node<W: ShootingWorld>(t: &ShootingTask, w: &mut W, arg: u32) -> u32 {
        let Some(node) = w.fetch_node() else {
            return 0;
        };
        let scaled = (arg as f32) * f32::from_bits(w.scale_factor());
        w.construct_shot_node(
            node,
            &ShotNodeArgs {
                kind: 0xF,
                pos: t.base.pos,
                scaled_bits: scaled.to_bits(),
                tail: 0xFFFF_FFFF,
            },
        )
    }

    /// Fault: the prepared mark is never stored on an accepting subject.
    pub fn initialise_against_subject<W: ShootingWorld>(
        t: &mut ShootingTask,
        w: &mut W,
        a0: Handle32<InitArg>,
        a1: u32,
        a2: u32,
    ) -> u32 {
        let Some(subject) = t.subject else {
            return 0;
        };
        if w.subject_flags(subject) & 1 == 0 {
            if w.ask_subject(subject, a0, a1, a2) & 0xFF == 0 {
                return 0;
            }
        }
        w.init_member_60(t);
        w.init_member_6c(t);
        w.finish_arg(a0);
        1
    }

    /// Fault: the tags are checked second before first.
    pub fn has_full_chain<W: ShootingWorld>(t: &ShootingTask, w: &mut W) -> u32 {
        let Some(o1) = t.subject else {
            return 0;
        };
        let Some(o2) = w.chain_next(o1) else {
            return 0;
        };
        if w.chain_tag(o2) != 0x41A {
            return 0;
        }
        if w.chain_tag(o1) != 0x11D {
            return 0;
        }
        let Some(o3) = w.chain_next(o2) else {
            return 0;
        };
        if w.chain_tag(o3) != 0x76D {
            return 0;
        }
        u32::from(t.base.link != 0)
    }
}
