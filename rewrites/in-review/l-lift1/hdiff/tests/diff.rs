//! Differential tests: 32-bit-form rewrites vs the lifted crate.
//! i686 only. Each test builds both states from one seed, runs both sides,
//! and compares returns, full state, and callback event logs.

use lf_lift_diff::*;
use lf_lift_pilot::{chain, misc, pool, slots};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Shared<T> = Arc<Mutex<T>>;
type Log = Shared<Vec<String>>;

fn new_log() -> Log {
    Arc::new(Mutex::new(Vec::new()))
}

// ---------------- chain family ----------------
// Node block: 0x140 bytes. Offsets: 78 slot w, 114 container w, 118 fwd w,
// 11C back w, 120 link w, 124 anchor w, 130 claim w, 13C flags b.
const NODE_SZ: usize = 0x140;

struct ChainFx {
    arena: Arena32,
    addrs: Vec<u32>,
    lifted: chain::ChainArena,
    id_of: HashMap<u32, u32>,
}

fn build_chain(rng: &mut Rng, n: usize, full_flags: bool) -> ChainFx {
    let mut arena = Arena32::new();
    let mut addrs = Vec::new();
    for _ in 0..n {
        addrs.push(arena.alloc(NODE_SZ));
    }
    let mut nodes = Vec::new();
    for i in 0..n {
        // Acyclic forward spine with random extra links.
        let fwd = if i + 1 < n && rng.below(4) != 0 { Some(i + 1) } else { None };
        let back = if i > 0 && rng.below(2) == 0 { Some(i - 1) } else { None };
        let link = if i + 1 < n && rng.below(2) == 0 { Some(i + 1) } else { None };
        let anchor = if rng.below(3) == 0 { Some(rng.below(n as u32) as usize) } else { None };
        let flags =
            if full_flags { 0x0C } else { [0x00, 0x04, 0x08, 0x0C, 0x0E, 0xFF][rng.below(6) as usize] as u8 };
        let container = if rng.below(2) == 0 { 0 } else { 0xC000_0000 | rng.any() >> 2 };
        let claim = if rng.below(2) == 0 { 0 } else { rng.any() };
        let slot78 = rng.any();
        let a = addrs[i];
        let addr_of = |l: Option<usize>| l.map_or(0, |j| addrs[j]);
        arena.set(a + 0x78, slot78);
        arena.set(a + 0x114, container);
        arena.set(a + 0x118, addr_of(fwd));
        arena.set(a + 0x11C, addr_of(back));
        arena.set(a + 0x120, addr_of(link));
        arena.set(a + 0x124, addr_of(anchor));
        arena.set(a + 0x130, claim);
        arena.setb(a + 0x13C, flags);
        let id = |l: Option<usize>| l.map(|j| chain::NodeId(j as u32));
        nodes.push(chain::ChainNode {
            slot78,
            container,
            fwd: id(fwd),
            back: id(back),
            link: id(link),
            anchor: id(anchor),
            claim,
            flags,
        });
    }
    let id_of = addrs.iter().enumerate().map(|(i, a)| (*a, i as u32)).collect();
    ChainFx { arena, addrs, lifted: chain::ChainArena { nodes }, id_of }
}

fn assert_chain_eq(fx: &ChainFx) {
    for (i, a) in fx.addrs.iter().enumerate() {
        let nd = &fx.lifted.nodes[i];
        let link = |o: u32| {
            let v = fx.arena.w(a + o);
            if v == 0 { None } else { Some(chain::NodeId(fx.id_of[&v])) }
        };
        assert_eq!(fx.arena.w(a + 0x78), nd.slot78, "node {i} slot78");
        assert_eq!(fx.arena.w(a + 0x114), nd.container, "node {i} container");
        assert_eq!(link(0x118), nd.fwd, "node {i} fwd");
        assert_eq!(link(0x11C), nd.back, "node {i} back");
        assert_eq!(link(0x120), nd.link, "node {i} link");
        assert_eq!(link(0x124), nd.anchor, "node {i} anchor");
        assert_eq!(fx.arena.w(a + 0x130), nd.claim, "node {i} claim");
        assert_eq!(fx.arena.b(a + 0x13C), nd.flags, "node {i} flags");
    }
}

#[test]
fn d_chain_contains() {
    let _g = test_guard();
    let mut rng = Rng(0xC01);
    for _ in 0..300 {
        let n = 1 + rng.below(6) as usize;
        let fx = build_chain(&mut rng, n, false);
        let s = rng.below(n as u32) as usize;
        let t = rng.below(n as u32) as usize;
        let r32 = unsafe { rw_s13_00a7c6d0(fx.addrs[s] as *const u8, fx.addrs[t]) };
        let rl = chain::contains(&fx.lifted, chain::NodeId(s as u32), chain::NodeId(t as u32));
        assert_eq!(r32, rl as u8, "contains s={s} t={t}");
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_chain_all_flagged() {
    let _g = test_guard();
    let mut rng = Rng(0xC02);
    for _ in 0..300 {
        let n = 1 + rng.below(6) as usize;
        let fx = build_chain(&mut rng, n, false);
        let s = rng.below(n as u32) as usize;
        let r32 = unsafe { rw_s13_00a7c700(fx.addrs[s] as *const u8) };
        let rl = chain::all_flagged(&fx.lifted, chain::NodeId(s as u32));
        assert_eq!(r32, rl as u8, "all_flagged s={s}");
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_chain_set_flags() {
    let _g = test_guard();
    let mut rng = Rng(0xC03);
    for _ in 0..200 {
        let n = 1 + rng.below(6) as usize;
        let mut fx = build_chain(&mut rng, n, false);
        let s = rng.below(n as u32) as usize;
        unsafe { rw_s13_00a7c720(fx.addrs[s] as *mut u8) };
        chain::set_flags(&mut fx.lifted, chain::NodeId(s as u32));
        assert_chain_eq(&fx);
    }
}

// Shared scripted-callback setup for find_or_fallback: scripted search hit
// or miss plus a scripted fallback answer, on both sides.
fn run_find_or_fallback(rw_hit: bool, second_site: bool) {
    let mut rng = Rng(if second_site { 0xCE11 } else { 0xC8D0 });
    for _ in 0..120 {
        let n = 1 + rng.below(5) as usize;
        let mut fx = build_chain(&mut rng, n, false);
        let s = rng.below(n as u32) as usize;
        let (a1, a2) = (rng.any(), rng.any());
        let hit_addr = if rw_hit { fx.addrs[rng.below(n as u32) as usize] } else { 0 };
        let fb_addr = fx.addrs[rng.below(n as u32) as usize];
        let log32 = new_log();
        let logl = new_log();
        let id_of = Arc::new(fx.id_of.clone());
        // 32-bit scripts.
        {
            let (log, map) = (log32.clone(), id_of.clone());
            set_callee(1, Sig::This2, Box::new(move |a| {
                log.lock().unwrap().push(format!("search:{}:{}:{}", map[&a[0]], a[1], a[2]));
                hit_addr
            }));
            let (log, map) = (log32.clone(), id_of.clone());
            set_callee(2, Sig::This3, Box::new(move |a| {
                log.lock().unwrap().push(format!("fallback:{}:{}:{}:{}", a[0], a[1], a[2], map[&a[3]]));
                fb_addr
            }));
        }
        let r32 = unsafe {
            if second_site {
                rw_s13_00a7ce10(fx.addrs[s], a1, a2)
            } else {
                rw_s13_00a7c8d0(fx.addrs[s], a1, a2)
            }
        };
        // Lifted side with mirrored scripts.
        let hit_id = if hit_addr == 0 { None } else { Some(chain::NodeId(id_of[&hit_addr])) };
        let fb_id = chain::NodeId(id_of[&fb_addr]);
        let mut search = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, t: chain::NodeId, x1: u32, x2: u32| {
                log.lock().unwrap().push(format!("search:{}:{}:{}", t.0, x1, x2));
                hit_id
            }
        };
        let mut fallback = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, c: u32, x1: u32, x2: u32, t: chain::NodeId| {
                log.lock().unwrap().push(format!("fallback:{c}:{x1}:{x2}:{}", t.0));
                Some(fb_id)
            }
        };
        let rl = chain::find_or_fallback(&mut fx.lifted, chain::NodeId(s as u32), a1, a2, &mut search, &mut fallback);
        let expect = if rw_hit { hit_addr } else { fb_addr };
        assert_eq!(r32, expect);
        assert_eq!(rl.map(|id| fx.addrs[id.0 as usize]), Some(expect));
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_find_or_fallback_a() {
    let _g = test_guard();
    run_find_or_fallback(true, false);
    run_find_or_fallback(false, false);
}

#[test]
fn d_find_or_fallback_b() {
    let _g = test_guard();
    run_find_or_fallback(true, true);
    run_find_or_fallback(false, true);
}

#[test]
fn d_update_flags_notify() {
    let _g = test_guard();
    let mut rng = Rng(0xC90);
    for _ in 0..200 {
        let n = 1 + rng.below(4) as usize;
        let mut fx = build_chain(&mut rng, n, false);
        let s = rng.below(n as u32) as usize;
        let arg = [0u32, 1, 0xFF, 0x100, rng.any()][rng.below(5) as usize];
        let answer = rng.any();
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, map) = (log32.clone(), fx.id_of.clone());
            set_callee(1, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("notify:{}", map[&a[0]]));
                answer
            }));
        }
        let r32 = unsafe { rw_s13_00a7c900(fx.addrs[s] as *mut u8, arg) };
        let mut notify = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, t: chain::NodeId| {
                log.lock().unwrap().push(format!("notify:{}", t.0));
                answer
            }
        };
        let rl = chain::update_flags_notify(&mut fx.lifted, chain::NodeId(s as u32), arg, &mut notify);
        assert_eq!(r32, rl, "arg={arg:#x}");
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_find_flagged_node() {
    let _g = test_guard();
    let mut rng = Rng(0xC94);
    for _ in 0..200 {
        let n = 1 + rng.below(5) as usize;
        let mut fx = build_chain(&mut rng, n, false);
        let s = rng.below(n as u32) as usize;
        // Scripted head: null or a node.
        let head_addr = if rng.below(4) == 0 { 0 } else { fx.addrs[rng.below(n as u32) as usize] };
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, map) = (log32.clone(), fx.id_of.clone());
            set_callee(1, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("head:{}", map[&a[0]]));
                head_addr
            }));
        }
        let r32 = unsafe { rw_s13_00a7c940(fx.addrs[s]) };
        let head_id = if head_addr == 0 { None } else { Some(chain::NodeId(fx.id_of[&head_addr])) };
        let mut head = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, t: chain::NodeId| {
                log.lock().unwrap().push(format!("head:{}", t.0));
                head_id
            }
        };
        let rl = chain::find_flagged_node(&mut fx.lifted, chain::NodeId(s as u32), &mut head);
        assert_eq!(r32, rl.map_or(0, |id| fx.addrs[id.0 as usize]));
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_last_link_in_chain() {
    let _g = test_guard();
    let mut rng = Rng(0xC9C);
    for _ in 0..300 {
        let n = 1 + rng.below(6) as usize;
        let fx = build_chain(&mut rng, n, true);
        let s = rng.below(n as u32) as usize;
        let r32 = unsafe { rw_s13_00a7c9c0(fx.addrs[s] as *const u8) };
        let rl = chain::last_link(&fx.lifted, chain::NodeId(s as u32));
        assert_eq!(r32, rl.map_or(0, |id| fx.addrs[id.0 as usize]), "s={s}");
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_sweep_touch_and_link() {
    let _g = test_guard();
    let mut rng = Rng(0xCA0);
    for _ in 0..150 {
        let n = 1 + rng.below(5) as usize;
        let mut fx = build_chain(&mut rng, n, false);
        // Guarantee the sweep anchor (if any) has a non-null sibling: the
        // original faults otherwise, and so does the lift (by design).
        for i in 0..n {
            if fx.lifted.nodes[i].fwd.is_none() {
                let j = rng.below(n as u32) as usize;
                fx.lifted.nodes[i].fwd = Some(chain::NodeId(j as u32));
                fx.arena.set(fx.addrs[i] + 0x118, fx.addrs[j]);
            }
        }
        let s = rng.below(n as u32) as usize;
        let head_addr = if rng.below(5) == 0 { 0 } else { fx.addrs[rng.below(n as u32) as usize] };
        let log32 = new_log();
        let logl = new_log();
        let id_of = Arc::new(fx.id_of.clone());
        {
            let (log, map) = (log32.clone(), id_of.clone());
            set_callee(1, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("head:{}", map[&a[0]]));
                head_addr
            }));
            // Touch mutates through the callback (sets claim), on both sides.
            let (log, map) = (log32.clone(), id_of.clone());
            set_callee(2, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("touch:{}", map[&a[0]]));
                unsafe { *((a[0] + 0x130) as *mut u32) = 0xAA; }
                0
            }));
            let (log, map) = (log32.clone(), id_of.clone());
            set_callee(3, Sig::This1, Box::new(move |a| {
                // Link receives sub-record views (+0x10); map back.
                log.lock().unwrap().push(format!("link:{}:{}", map[&(a[0] - 0x10)], map[&(a[1] - 0x10)]));
                0
            }));
        }
        let r32 = unsafe { rw_s13_00a7ca00(fx.addrs[s]) };
        let head_id = if head_addr == 0 { None } else { Some(chain::NodeId(id_of[&head_addr])) };
        let mut head = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, t: chain::NodeId| {
                log.lock().unwrap().push(format!("head:{}", t.0));
                head_id
            }
        };
        let mut touch = {
            let log = logl.clone();
            move |ar: &mut chain::ChainArena, t: chain::NodeId| {
                log.lock().unwrap().push(format!("touch:{}", t.0));
                ar.get_mut(t).claim = 0xAA;
                0
            }
        };
        let mut link = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, sib: chain::NodeId, a: chain::NodeId| {
                log.lock().unwrap().push(format!("link:{}:{}", sib.0, a.0));
                0
            }
        };
        let rl = chain::sweep_touch_and_link(&mut fx.lifted, chain::NodeId(s as u32), &mut head, &mut touch, &mut link);
        assert_eq!(r32, rl as u8);
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_splice_node() {
    let _g = test_guard();
    let mut rng = Rng(0xCAC);
    for _ in 0..200 {
        let n = 2 + rng.below(4) as usize;
        let mut fx = build_chain(&mut rng, n, true);
        let s = rng.below(n as u32) as usize;
        let m = rng.below(n as u32) as usize;
        let r32 = unsafe { rw_s13_00a7cac0(fx.addrs[s] as *mut u8, fx.addrs[m]) };
        chain::splice_node(&mut fx.lifted, chain::NodeId(s as u32), chain::NodeId(m as u32));
        assert_eq!(r32, fx.addrs[m]);
        assert_chain_eq(&fx);
    }
}

#[test]
fn d_touch_chain() {
    let _g = test_guard();
    let mut rng = Rng(0xCD0);
    for _ in 0..150 {
        let n = 1 + rng.below(5) as usize;
        let mut fx = build_chain(&mut rng, n, true);
        let s = rng.below(n as u32) as usize;
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, map) = (log32.clone(), fx.id_of.clone());
            set_callee(1, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("touch:{}", map[&a[0]]));
                0
            }));
        }
        let r32 = unsafe { rw_s13_00a7cd00(fx.addrs[s]) };
        let mut touch = {
            let log = logl.clone();
            move |_: &mut chain::ChainArena, t: chain::NodeId| {
                log.lock().unwrap().push(format!("touch:{}", t.0));
                0
            }
        };
        chain::touch_chain(&mut fx.lifted, chain::NodeId(s as u32), &mut touch);
        // The original always returns 1; the lift returns ().
        assert_eq!(r32, 1);
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_chain_eq(&fx);
    }
}

// ---------------- slot-record family ----------------
// Slot block 0xA8, inner block 0x100, shared block 0xE9C, holder 4 bytes.
const SLOT_SZ: usize = 0xA8;
const INNER_SZ: usize = 0x100;
const SHARED_SZ: usize = 0xE9C;

struct SlotsFx {
    arena: Arena32,
    slot_addrs: Vec<u32>,
    inner_addrs: Vec<u32>,
    shared_addrs: Vec<u32>,
    holder_addrs: Vec<u32>,
    table_addrs: Vec<u32>,
    lifted: slots::Slots,
    slot_of: HashMap<u32, u32>,
    inner_of: HashMap<u32, u32>,
    shared_of: HashMap<u32, u32>,
    holder_of: HashMap<u32, u32>,
}

fn build_slots(rng: &mut Rng) -> SlotsFx {
    let mut arena = Arena32::new();
    let ns = 1 + rng.below(4) as usize;
    let ni = 1 + rng.below(4) as usize;
    let nh = 1 + rng.below(2) as usize;
    let nt = 1 + rng.below(2) as usize;
    let nsh = 1 + rng.below(2) as usize;
    let slot_addrs: Vec<u32> = (0..ns).map(|_| arena.alloc(SLOT_SZ)).collect();
    let inner_addrs: Vec<u32> = (0..ni).map(|_| arena.alloc(INNER_SZ)).collect();
    let shared_addrs: Vec<u32> = (0..nsh).map(|_| arena.alloc(SHARED_SZ)).collect();
    let holder_addrs: Vec<u32> = (0..nh).map(|_| arena.alloc(4)).collect();
    // Tables: 1..5 payload words each.
    let mut tables = Vec::new();
    let mut table_addrs = Vec::new();
    for _ in 0..nt {
        let len = 1 + rng.below(5) as usize;
        let base = arena.alloc(len * 4);
        let mut payload = Vec::new();
        for k in 0..len {
            let v = rng.any();
            arena.set(base + (k * 4) as u32, v);
            payload.push(v);
        }
        tables.push(payload);
        table_addrs.push(base);
    }
    let holders: Vec<slots::TableId> =
        (0..nh).map(|_| slots::TableId(rng.below(nt as u32))).collect();
    for (h, t) in holders.iter().enumerate() {
        arena.set(holder_addrs[h], table_addrs[t.0 as usize]);
    }
    let pick = |rng: &mut Rng, n: usize| {
        if rng.below(3) == 0 { None } else { Some(rng.below(n as u32) as usize) }
    };
    // Inners.
    let mut inners = Vec::new();
    for i in 0..ni {
        let a = inner_addrs[i];
        let holder = pick(rng, nh).map(|h| slots::HolderId(h as u32));
        // Index: in-range, zero, or negative (never upper-OOB: uncomparable).
        let t = holders[holder.map_or(0, |h| h.0 as usize)].0 as usize;
        let index = match rng.below(5) {
            0 => 0,
            1 => 0xFFFF_FFFF,
            2 => rng.below(tables[t].len() as u32),
            3 => rng.below(tables[t].len() as u32),
            _ => rng.below(tables[t].len() as u32 + 1),
            // Note: index == len is upper-OOB for indexed_entry (panics in
            // the lift, garbage in 32-bit). Trials using indexed fetches
            // clamp below; see those tests.
        };
        let (counter18, aux_input, f8, status) =
            (rng.below(65536) as u16, rng.any(), rng.any(), rng.any());
        arena.set(a + 0x18, (arena.w(a + 0x18) & 0xFFFF_0000) | counter18 as u32);
        arena.set(a + 0x58, aux_input);
        arena.set(a + 0x9C, holder.map_or(0, |h| holder_addrs[h.0 as usize]));
        arena.set(a + 0xA0, index);
        arena.set(a + 0xF8, f8);
        arena.set(a + 0xFC, status);
        inners.push(slots::Inner { counter18, aux_input, holder, index, f8, status });
    }
    // Slots. Head cookies follow the lift rule: cookie k names slot k - 1.
    let mut lifted_slots = Vec::new();
    for i in 0..ns {
        let a = slot_addrs[i];
        let head_target = pick(rng, ns);
        let head: u32 = head_target.map_or(0, |t| t as u32 + 1);
        let child = pick(rng, ni).map(|c| slots::InnerId(c as u32));
        let slot = pick(rng, ns).map(|s| slots::SlotId(s as u32));
        let linked = pick(rng, ni).map(|c| slots::InnerId(c as u32));
        let aux = pick(rng, ns).map(|s| slots::SlotId(s as u32));
        let probe_child = if rng.below(3) == 0 { 0 } else { rng.any() | 1 };
        let counter = rng.below(65536) as u16;
        let (flag_1c, mode, status29, bit2a, flag48) =
            (rng.any() as u8, rng.any() as u8, rng.any() as u8, rng.any() as u8, rng.any() as u8);
        let words30 = [0u32, rng.any(), rng.any(), rng.any()];
        let holder = pick(rng, nh).map(|h| slots::HolderId(h as u32));
        let t = holders[holder.map_or(0, |h| h.0 as usize)].0 as usize;
        let index = match rng.below(4) {
            0 => 0,
            1 => 0xFFFF_FFFF,
            _ => rng.below(tables[t].len() as u32 + 1),
        };
        arena.set(a, head_target.map_or(0, |t| slot_addrs[t]));
        arena.set(a + 0x04, child.map_or(0, |c| inner_addrs[c.0 as usize]));
        arena.set(a + 0x08, slot.map_or(0, |s| slot_addrs[s.0 as usize]));
        arena.set(a + 0x0C, linked.map_or(0, |c| inner_addrs[c.0 as usize]));
        arena.set(a + 0x10, aux.map_or(0, |s| slot_addrs[s.0 as usize]));
        arena.set(a + 0x14, probe_child);
        arena.set(a + 0x18, (arena.w(a + 0x18) & 0xFFFF_0000) | counter as u32);
        arena.setb(a + 0x1C, flag_1c);
        arena.setb(a + 0x25, mode);
        arena.setb(a + 0x29, status29);
        arena.setb(a + 0x2A, bit2a);
        for (k, wv) in words30.iter().enumerate() {
            arena.set(a + 0x30 + (k * 4) as u32, *wv);
        }
        arena.setb(a + 0x48, flag48);
        arena.set(a + 0x9C, holder.map_or(0, |h| holder_addrs[h.0 as usize]));
        arena.set(a + 0xA0, index);
        lifted_slots.push(slots::Slot {
            head,
            child,
            slot,
            linked,
            aux,
            probe_child,
            counter,
            flag_1c,
            mode,
            status29,
            bit2a,
            words30,
            flag48,
            holder,
            index,
        });
    }
    let mut shareds = Vec::new();
    for i in 0..nsh {
        let record = pick(rng, ni).map(|c| slots::InnerId(c as u32));
        arena.set(shared_addrs[i] + 0xE98, record.map_or(0, |c| inner_addrs[c.0 as usize]));
        shareds.push(slots::Shared { record });
    }
    let slot_of = slot_addrs.iter().enumerate().map(|(i, a)| (*a, i as u32)).collect();
    let inner_of = inner_addrs.iter().enumerate().map(|(i, a)| (*a, i as u32)).collect();
    let shared_of = shared_addrs.iter().enumerate().map(|(i, a)| (*a, i as u32)).collect();
    let holder_of = holder_addrs.iter().enumerate().map(|(i, a)| (*a, i as u32)).collect();
    SlotsFx {
        arena,
        slot_addrs,
        inner_addrs,
        shared_addrs,
        holder_addrs,
        table_addrs,
        lifted: slots::Slots { slots: lifted_slots, inners, holders, tables, shareds },
        slot_of,
        inner_of,
        shared_of,
        holder_of,
    }
}

fn assert_slots_eq(fx: &SlotsFx) {
    for (i, a) in fx.slot_addrs.iter().enumerate() {
        let s = &fx.lifted.slots[i];
        let opt_slot = |o: u32| {
            let v = fx.arena.w(a + o);
            if v == 0 { None } else { Some(slots::SlotId(fx.slot_of[&v])) }
        };
        let opt_inner = |o: u32| {
            let v = fx.arena.w(a + o);
            if v == 0 { None } else { Some(slots::InnerId(fx.inner_of[&v])) }
        };
        // Head: cookie k names slot k-1.
        let hw = fx.arena.w(*a);
        if hw == 0 {
            assert_eq!(s.head, 0, "slot {i} head");
        } else {
            assert_eq!(s.head, fx.slot_of[&hw] + 1, "slot {i} head");
        }
        assert_eq!(opt_inner(0x04), s.child, "slot {i} child");
        assert_eq!(opt_slot(0x08), s.slot, "slot {i} slot");
        assert_eq!(opt_inner(0x0C), s.linked, "slot {i} linked");
        assert_eq!(opt_slot(0x10), s.aux, "slot {i} aux");
        assert_eq!(fx.arena.w(a + 0x14), s.probe_child, "slot {i} probe");
        assert_eq!(fx.arena.h(a + 0x18), s.counter, "slot {i} counter");
        assert_eq!(fx.arena.b(a + 0x1C), s.flag_1c, "slot {i} flag1c");
        assert_eq!(fx.arena.b(a + 0x25), s.mode, "slot {i} mode");
        assert_eq!(fx.arena.b(a + 0x29), s.status29, "slot {i} status29");
        assert_eq!(fx.arena.b(a + 0x2A), s.bit2a, "slot {i} bit2a");
        // words30[0]: bound form (inner addr vs inner index) or raw form.
        let w0 = fx.arena.w(a + 0x30);
        let l0 = s.words30[0];
        let bound = (l0 as usize) < fx.inner_addrs.len() && fx.inner_addrs[l0 as usize] == w0;
        assert!(bound || w0 == l0, "slot {i} words30[0]: 32={w0:#x} lift={l0}");
        for k in 1..4 {
            assert_eq!(fx.arena.w(a + 0x30 + (k * 4) as u32), s.words30[k as usize], "slot {i} w{k}");
        }
        assert_eq!(fx.arena.b(a + 0x48), s.flag48, "slot {i} flag48");
        let hw = fx.arena.w(a + 0x9C);
        assert_eq!(
            if hw == 0 { None } else { Some(slots::HolderId(fx.holder_of[&hw])) },
            s.holder,
            "slot {i} holder"
        );
        assert_eq!(fx.arena.w(a + 0xA0), s.index, "slot {i} index");
    }
    for (i, a) in fx.inner_addrs.iter().enumerate() {
        let n = &fx.lifted.inners[i];
        assert_eq!(fx.arena.h(a + 0x18), n.counter18, "inner {i} counter18");
        assert_eq!(fx.arena.w(a + 0x58), n.aux_input, "inner {i} aux_input");
        let hw = fx.arena.w(a + 0x9C);
        assert_eq!(
            if hw == 0 { None } else { Some(slots::HolderId(fx.holder_of[&hw])) },
            n.holder,
            "inner {i} holder"
        );
        assert_eq!(fx.arena.w(a + 0xA0), n.index, "inner {i} index");
        assert_eq!(fx.arena.w(a + 0xF8), n.f8, "inner {i} f8");
        assert_eq!(fx.arena.w(a + 0xFC), n.status, "inner {i} status");
    }
    for (i, a) in fx.shared_addrs.iter().enumerate() {
        let v = fx.arena.w(a + 0xE98);
        assert_eq!(
            if v == 0 { None } else { Some(slots::InnerId(fx.inner_of[&v])) },
            fx.lifted.shareds[i].record,
            "shared {i}"
        );
    }
    for (h, a) in fx.holder_addrs.iter().enumerate() {
        assert_eq!(fx.arena.w(*a), fx.table_addrs[fx.lifted.holders[h].0 as usize], "holder {h}");
    }
}

#[test]
fn d_bind_aux_and_combine() {
    let _g = test_guard();
    let mut rng = Rng(0xD00);
    for _ in 0..100 {
        let mut fx = build_slots(&mut rng);
        // Lookup must hit a shared with a non-null record on both sides.
        let sh = rng.below(fx.shared_addrs.len() as u32) as usize;
        if fx.lifted.shareds[sh].record.is_none() {
            fx.lifted.shareds[sh].record = Some(slots::InnerId(0));
            fx.arena.set(fx.shared_addrs[sh] + 0xE98, fx.inner_addrs[0]);
        }
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let (aux_ans, comb_ans) = (rng.any(), rng.any());
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, map) = (log32.clone(), fx.shared_of.clone());
            let shared_addr = fx.shared_addrs[sh];
            set_callee(1, Sig::Cdecl1, Box::new(move |a| {
                log.lock().unwrap().push(format!("lookup:{}", a[0]));
                let _ = &map;
                shared_addr
            }));
            let log = log32.clone();
            set_callee(2, Sig::Cdecl1, Box::new(move |a| {
                log.lock().unwrap().push(format!("aux:{}", a[0]));
                aux_ans
            }));
            let (log, map) = (log32.clone(), fx.slot_of.clone());
            set_callee(3, Sig::Cdecl2, Box::new(move |a| {
                log.lock().unwrap().push(format!("combine:{}:{}", map[&(a[0] - 0x18)], a[1]));
                comb_ans
            }));
        }
        let r32 = unsafe { rw_s16f00(fx.slot_addrs[s] as *mut u8, rng.any()) };
        let sh_id = slots::SharedId(sh as u32);
        let mut lookup = {
            let log = logl.clone();
            move |_: &mut slots::Slots, z: u32| {
                log.lock().unwrap().push(format!("lookup:{z}"));
                Some(sh_id)
            }
        };
        let mut aux_step = {
            let log = logl.clone();
            move |_: &mut slots::Slots, v: u32| {
                log.lock().unwrap().push(format!("aux:{v}"));
                aux_ans
            }
        };
        let mut combine = {
            let log = logl.clone();
            move |_: &mut slots::Slots, t: slots::SlotId, v: u32| {
                log.lock().unwrap().push(format!("combine:{}:{v}", t.0));
                comb_ans
            }
        };
        let rl =
            slots::bind_aux_and_combine(&mut fx.lifted, slots::SlotId(s as u32), &mut lookup, &mut aux_step, &mut combine);
        assert_eq!(r32, rl);
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_reset_slot_state() {
    let _g = test_guard();
    let mut rng = Rng(0xD01);
    for _ in 0..200 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        // Non-zero words so the reset is observable (raw form).
        for k in 0..4 {
            let v = rng.any() | 0x0101_0101;
            fx.arena.set(fx.slot_addrs[s] + 0x30 + (k * 4) as u32, v);
            fx.lifted.slots[s].words30[k as usize] = v;
        }
        unsafe { rw_s16f01(fx.slot_addrs[s] as *mut u8, rng.any()) };
        slots::reset_slot_state(&mut fx.lifted, slots::SlotId(s as u32));
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_forward_flags_if_present() {
    let _g = test_guard();
    let mut rng = Rng(0xD02);
    for _ in 0..150 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let answer = rng.any();
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, map) = (log32.clone(), fx.inner_of.clone());
            set_callee(1, Sig::This4, Box::new(move |a| {
                log.lock().unwrap().push(format!("step:{}:{}:{}:{}:{}", map[&a[0]], a[1], a[2], a[3], a[4]));
                answer
            }));
        }
        let r32 = unsafe { rw_s16f02(fx.slot_addrs[s] as *const u8) };
        let mut step = {
            let log = logl.clone();
            move |_: &mut slots::Slots, t: slots::InnerId, a: u32, b: u32, c: u32, d: u32| {
                log.lock().unwrap().push(format!("step:{}:{a}:{b}:{c}:{d}", t.0));
                answer
            }
        };
        let rl = slots::forward_flags_if_present(&mut fx.lifted, slots::SlotId(s as u32), &mut step);
        assert_eq!(r32, rl);
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_slots_eq(&fx);
    }
}

// Clamp indices into the comparable range for the indexed-fetch tests:
// negative and zero stay (null paths); upper-OOB is pulled in-range; null
// holders are filled in (both sides fault/panic on those: uncomparable).
fn clamp_fetch_indices(fx: &mut SlotsFx) {
    for i in 0..fx.inner_addrs.len() {
        if fx.lifted.inners[i].holder.is_none() {
            fx.lifted.inners[i].holder = Some(slots::HolderId(0));
            fx.arena.set(fx.inner_addrs[i] + 0x9C, fx.holder_addrs[0]);
        }
        let n = &fx.lifted.inners[i];
        if let Some(h) = n.holder {
            let len = fx.lifted.tables[fx.lifted.holders[h.0 as usize].0 as usize].len() as u32;
            if (n.index as i32) >= 0 && n.index >= len {
                let v = if len == 0 { 0 } else { len - 1 };
                fx.lifted.inners[i].index = v;
                fx.arena.set(fx.inner_addrs[i] + 0xA0, v);
            }
        }
    }
    for i in 0..fx.slot_addrs.len() {
        if fx.lifted.slots[i].holder.is_none() {
            fx.lifted.slots[i].holder = Some(slots::HolderId(0));
            fx.arena.set(fx.slot_addrs[i] + 0x9C, fx.holder_addrs[0]);
        }
        let s = &fx.lifted.slots[i];
        if let Some(h) = s.holder {
            let len = fx.lifted.tables[fx.lifted.holders[h.0 as usize].0 as usize].len() as u32;
            // prev_entry reads index-1; keep index <= len.
            if (s.index as i32) >= 0 && s.index > len {
                fx.lifted.slots[i].index = len;
                fx.arena.set(fx.slot_addrs[i] + 0xA0, len);
            }
        }
    }
}

#[test]
fn d_indexed_entry_or_null() {
    let _g = test_guard();
    let mut rng = Rng(0xD05);
    for _ in 0..250 {
        let mut fx = build_slots(&mut rng);
        clamp_fetch_indices(&mut fx);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let r32 = unsafe { rw_s16f05(fx.slot_addrs[s] as *const u8) };
        let rl = slots::indexed_entry_or_null(&fx.lifted, slots::SlotId(s as u32));
        // Null (0) vs payload: payloads are arbitrary u32, including
        // possibly 0; compare via the is-some channel plus value.
        assert_eq!((r32 == 0) as bool, rl.is_none() || rl == Some(0));
        if let Some(v) = rl {
            assert_eq!(r32, v);
        } else {
            assert_eq!(r32, 0);
        }
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_prev_indexed_entry_or_null() {
    let _g = test_guard();
    let mut rng = Rng(0xD06);
    for _ in 0..250 {
        let mut fx = build_slots(&mut rng);
        clamp_fetch_indices(&mut fx);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let r32 = unsafe { rw_s16f06(fx.slot_addrs[s] as *const u8) };
        let rl = slots::prev_indexed_entry_or_null(&fx.lifted, slots::SlotId(s as u32));
        if let Some(v) = rl {
            assert_eq!(r32, v);
        } else {
            assert_eq!(r32, 0);
        }
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_prev_entry_or_null() {
    let _g = test_guard();
    let mut rng = Rng(0xD07);
    for _ in 0..250 {
        let mut fx = build_slots(&mut rng);
        clamp_fetch_indices(&mut fx);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let r32 = unsafe { rw_s16f07(fx.slot_addrs[s] as *const u8) };
        let rl = slots::prev_entry_or_null(&fx.lifted, slots::SlotId(s as u32));
        if let Some(v) = rl {
            assert_eq!(r32, v);
        } else {
            assert_eq!(r32, 0);
        }
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_status_byte_or_zero() {
    let _g = test_guard();
    let mut rng = Rng(0xD08);
    for _ in 0..250 {
        let fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let r32 = unsafe { rw_s16f08(fx.slot_addrs[s] as *const u8) };
        let rl = slots::status_byte_or_zero(&fx.lifted, slots::SlotId(s as u32));
        // KNOWN DIVERGENCE: only the low byte is comparable; the upper
        // three bytes leak the link address on the 32-bit side.
        assert_eq!(r32 & 0xFF, rl as u32, "s={s}");
        if fx.lifted.slots[s].linked.is_none() {
            assert_eq!(r32, 0);
        } else {
            let linked = fx.arena.w(fx.slot_addrs[s] + 0x0C);
            assert_eq!(r32 & 0xFFFF_FF00, linked & 0xFFFF_FF00, "quirk shape");
        }
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_probe_value_differs() {
    let _g = test_guard();
    let mut rng = Rng(0xD09);
    let limits = [0.0f32, 42.0, 65.0, 255.0, f32::NAN, f32::INFINITY, -0.0, 1e10];
    for _ in 0..160 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let limit = limits[rng.below(limits.len() as u32) as usize];
        let lim_addr = fx.arena.alloc(4);
        fx.arena.set(lim_addr, limit.to_bits());
        map_va(0xFE8628, lim_addr);
        let sample = rng.any();
        let log32 = new_log();
        let logl = new_log();
        {
            let log = log32.clone();
            set_callee(1, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("probe:{}", a[0]));
                sample
            }));
        }
        let r32 = unsafe { rw_s16f09(fx.slot_addrs[s] as *const u8) };
        let mut probe = {
            let log = logl.clone();
            move |_: &mut slots::Slots, c: u32| {
                log.lock().unwrap().push(format!("probe:{c}"));
                sample
            }
        };
        let rl = slots::probe_value_differs(&mut fx.lifted, slots::SlotId(s as u32), limit, &mut probe);
        assert_eq!(r32, rl as u32, "limit={limit} sample={sample:#x}");
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_activate_if_present() {
    let _g = test_guard();
    let mut rng = Rng(0xD0B);
    for _ in 0..150 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let (setup_ans, step_ans) = (rng.any(), rng.any());
        let log32 = new_log();
        let logl = new_log();
        {
            let log = log32.clone();
            set_callee(1, Sig::Cdecl1, Box::new(move |a| {
                log.lock().unwrap().push(format!("setup:{}", a[0]));
                setup_ans
            }));
            let (log, map) = (log32.clone(), fx.slot_of.clone());
            set_callee(2, Sig::This1, Box::new(move |a| {
                // Head cookie k names slot k-1 on the lifted side.
                log.lock().unwrap().push(format!("step:{}:{}", map[&a[0]] + 1, a[1]));
                step_ans
            }));
        }
        let r32 = unsafe { rw_s16f11(fx.slot_addrs[s] as *const u8) };
        let mut setup = {
            let log = logl.clone();
            move |_: &mut slots::Slots, z: u32| {
                log.lock().unwrap().push(format!("setup:{z}"));
                setup_ans
            }
        };
        let mut step = {
            let log = logl.clone();
            move |_: &mut slots::Slots, h: u32, m: u32| {
                log.lock().unwrap().push(format!("step:{h}:{m}"));
                step_ans
            }
        };
        let rl = slots::activate_if_present(&mut fx.lifted, slots::SlotId(s as u32), &mut setup, &mut step);
        assert_eq!(r32, rl);
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_invalidate_inner() {
    let _g = test_guard();
    let mut rng = Rng(0xD0C);
    for _ in 0..200 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        unsafe { rw_s16f12(fx.slot_addrs[s] as *const u8) };
        slots::invalidate_inner(&mut fx.lifted, slots::SlotId(s as u32));
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_forward_bytes() {
    let _g = test_guard();
    let mut rng = Rng(0xD0D);
    for _ in 0..120 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let flag = rng.any();
        // head (cookie rule: nonzero head names a real slot by mapping).
        unsafe { rw_s16f13(fx.slot_addrs[s] as *const u8, flag) };
        slots::forward_byte_head(&mut fx.lifted, slots::SlotId(s as u32), flag);
        assert_slots_eq(&fx);
        // linked.
        unsafe { rw_s16f14(fx.slot_addrs[s] as *const u8, flag) };
        slots::forward_byte_linked(&mut fx.lifted, slots::SlotId(s as u32), flag);
        assert_slots_eq(&fx);
        // slot.
        unsafe { rw_s16f15(fx.slot_addrs[s] as *const u8, flag) };
        slots::forward_byte_slot(&mut fx.lifted, slots::SlotId(s as u32), flag);
        assert_slots_eq(&fx);
        // aux.
        unsafe { rw_s16f16(fx.slot_addrs[s] as *const u8, flag) };
        slots::forward_byte_aux(&mut fx.lifted, slots::SlotId(s as u32), flag);
        assert_slots_eq(&fx);
        // child.
        unsafe { rw_s16f18(fx.slot_addrs[s] as *const u8, flag) };
        slots::forward_byte_child(&mut fx.lifted, slots::SlotId(s as u32), flag);
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_maybe_refresh_and_forward() {
    let _g = test_guard();
    let mut rng = Rng(0xD11);
    for _ in 0..150 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let flag = [0u32, 1, 0xFF, 0x100, rng.any()][rng.below(5) as usize];
        let (probe_ans, fetch_ans) = (rng.any(), rng.any());
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, map) = (log32.clone(), fx.slot_of.clone());
            set_callee(1, Sig::This0, Box::new(move |a| {
                log.lock().unwrap().push(format!("probe:{}", map[&a[0]]));
                probe_ans
            }));
            let (log, map) = (log32.clone(), fx.slot_of.clone());
            set_callee(2, Sig::This1, Box::new(move |a| {
                log.lock().unwrap().push(format!("refresh:{}:{}", map[&a[0]], a[1]));
                0
            }));
            let (log, map) = (log32.clone(), fx.inner_of.clone());
            set_callee(3, Sig::This0, Box::new(move |a| {
                let h = if a[0] == 0 { "null".to_string() } else { map[&a[0]].to_string() };
                log.lock().unwrap().push(format!("fetch:{h}"));
                fetch_ans
            }));
        }
        let r32 = unsafe { rw_s16f17(fx.slot_addrs[s] as *mut u8, flag) };
        let mut probe = {
            let log = logl.clone();
            move |_: &mut slots::Slots, t: slots::SlotId| {
                log.lock().unwrap().push(format!("probe:{}", t.0));
                probe_ans
            }
        };
        let mut refresh = {
            let log = logl.clone();
            move |_: &mut slots::Slots, t: slots::SlotId, z: u32| {
                log.lock().unwrap().push(format!("refresh:{}:{z}", t.0));
                0
            }
        };
        let mut head_fetch = {
            let log = logl.clone();
            move |_: &mut slots::Slots, h: Option<slots::InnerId>| {
                log.lock().unwrap().push(format!("fetch:{}", h.map_or("null".to_string(), |id| id.0.to_string())));
                fetch_ans
            }
        };
        let rl = slots::maybe_refresh_and_forward(
            &mut fx.lifted,
            slots::SlotId(s as u32),
            flag,
            &mut probe,
            &mut refresh,
            &mut head_fetch,
        );
        assert_eq!(r32, rl, "flag={flag:#x} probe={probe_ans:#x}");
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_step_word_store_flag() {
    let _g = test_guard();
    let mut rng = Rng(0xD13);
    for _ in 0..150 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let (word, flag) = (rng.any(), rng.any());
        let answer = rng.any();
        // Half the trials: the step replaces the aux pointer.
        let replace = rng.below(2) == 0 && fx.lifted.slots[s].aux.is_some();
        let new_aux = rng.below(fx.slot_addrs.len() as u32) as usize;
        let log32 = new_log();
        let logl = new_log();
        {
            let (log, smap) = (log32.clone(), fx.slot_of.clone());
            let this_addr = fx.slot_addrs[s];
            let new_aux_addr = fx.slot_addrs[new_aux];
            set_callee(1, Sig::This1, Box::new(move |a| {
                log.lock().unwrap().push(format!("step:{}:{}", smap[&a[0]], a[1]));
                if replace {
                    unsafe { *((this_addr + 0x10) as *mut u32) = new_aux_addr; }
                }
                answer
            }));
        }
        let r32 = unsafe { rw_s16f19(fx.slot_addrs[s] as *mut u8, word, flag) };
        let mut step = {
            let log = logl.clone();
            move |st: &mut slots::Slots, t: slots::SlotId, w: u32| {
                log.lock().unwrap().push(format!("step:{}:{w}", t.0));
                if replace {
                    st.slots[s].aux = Some(slots::SlotId(new_aux as u32));
                }
                answer
            }
        };
        let rl = slots::step_word_store_flag(&mut fx.lifted, slots::SlotId(s as u32), word, flag, &mut step);
        assert_eq!(r32, rl, "word={word:#x} flag={flag:#x} replace={replace}");
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_slots_eq(&fx);
    }
}

#[test]
fn d_forward_byte_head() {
    let _g = test_guard();
    run_forward(0xD0D0, 0);
}
#[test]
fn d_forward_byte_linked() {
    let _g = test_guard();
    run_forward(0xD0D1, 1);
}
#[test]
fn d_forward_byte_slot() {
    let _g = test_guard();
    run_forward(0xD0D2, 2);
}
#[test]
fn d_forward_byte_aux() {
    let _g = test_guard();
    run_forward(0xD0D3, 3);
}
#[test]
fn d_forward_byte_child() {
    let _g = test_guard();
    run_forward(0xD0D4, 4);
}

fn run_forward(seed: u64, which: u32) {
    let mut rng = Rng(seed);
    for _ in 0..120 {
        let mut fx = build_slots(&mut rng);
        let s = rng.below(fx.slot_addrs.len() as u32) as usize;
        let flag = rng.any();
        match which {
            0 => unsafe { rw_s16f13(fx.slot_addrs[s] as *const u8, flag) },
            1 => unsafe { rw_s16f14(fx.slot_addrs[s] as *const u8, flag) },
            2 => unsafe { rw_s16f15(fx.slot_addrs[s] as *const u8, flag) },
            3 => unsafe { rw_s16f16(fx.slot_addrs[s] as *const u8, flag) },
            _ => unsafe { rw_s16f18(fx.slot_addrs[s] as *const u8, flag) },
        }
        match which {
            0 => slots::forward_byte_head(&mut fx.lifted, slots::SlotId(s as u32), flag),
            1 => slots::forward_byte_linked(&mut fx.lifted, slots::SlotId(s as u32), flag),
            2 => slots::forward_byte_slot(&mut fx.lifted, slots::SlotId(s as u32), flag),
            3 => slots::forward_byte_aux(&mut fx.lifted, slots::SlotId(s as u32), flag),
            _ => slots::forward_byte_child(&mut fx.lifted, slots::SlotId(s as u32), flag),
        }
        assert_slots_eq(&fx);
    }
}

// ---------------- pool family ----------------

type Pool32 = extern "thiscall" fn(*mut u8) -> u32;

fn run_pool_simple(seed: u64, kind: pool::PoolKind, vtable: u32, call: Pool32, wide: bool) {
    let mut rng = Rng(seed);
    for trial in 0..60 {
        let mut arena = Arena32::new();
        // Counts: small normally; huge with forced alloc failure.
        let (count, ok) = match trial % 6 {
            4 => (0x100_0000u32, false),
            5 => (u32::MAX, false),
            _ => (rng.below(9), trial % 2 == 0),
        };
        let header = arena.alloc(12);
        arena.set(header, count);
        arena.set(header + 4, 0xDEAD_BEEF);
        arena.set(header + 8, 0xDEAD_BEEF);
        let block = arena.alloc(65536);
        let log32 = new_log();
        set_callee(1, Sig::Cdecl1, {
            let log = log32.clone();
            Box::new(move |a| {
                log.lock().unwrap().push(format!("alloc:{}", a[0]));
                if ok { block } else { 0 }
            })
        });
        let r32 = unsafe { call(header as *mut u8) };
        let logl = new_log();
        let mut alloc = {
            let log = logl.clone();
            move |n: u32| {
                log.lock().unwrap().push(format!("alloc:{n}"));
                ok
            }
        };
        let rl = pool::init_simple(count, kind, &mut alloc);
        // Same byte count requested on both sides (the clamp included).
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        assert_eq!(log32.lock().unwrap()[0], format!("alloc:{}", pool::checked_total(count, kind.stride())));
        if !ok {
            assert_eq!(r32, 0);
            assert_eq!(arena.w(header + 8), 0);
            assert!(rl.is_none());
            continue;
        }
        let base = block.wrapping_add(0x10);
        assert_eq!(arena.w(block), count, "block head");
        assert_eq!(arena.w(header + 4), 0, "header zero");
        assert_eq!(arena.w(header + 8), base, "header base");
        let p = rl.expect("lifted alloc ok");
        assert_eq!(p.count, count);
        assert_eq!(p.elements.len(), count as usize);
        for (i, el) in p.elements.iter().enumerate() {
            let ea = base.wrapping_add((i as u32).wrapping_mul(kind.stride()));
            assert_eq!(arena.w(ea), vtable, "element {i} vtable");
            assert_eq!(arena.b(ea + 8), 0, "element {i} status");
            assert_eq!(arena.w(ea + 8) & 0xFF, el.status & 0xFF);
            assert_eq!(el.kind, kind);
            if wide {
                assert_eq!(arena.w(ea + 0x60), 0);
                assert_eq!(arena.w(ea + 0x64), 0xFFFF_FFFF);
                assert_eq!(el.wide_extra, Some((0, 0xFFFF_FFFF)));
            } else {
                assert_eq!(el.wide_extra, None);
            }
        }
        // Pointer-valued returns are representation: assert the 32-bit shape
        // as a harness self-check, not as lifted behaviour.
        if wide {
            if count == 0 {
                assert_eq!(r32, block);
            } else {
                assert_eq!(r32, base.wrapping_add(8).wrapping_add(count.wrapping_mul(kind.stride())));
            }
        } else {
            assert_eq!(r32, base.wrapping_add(count.wrapping_mul(kind.stride())));
        }
    }
}

#[test]
fn d_pool_init_stride_60() {
    let _g = test_guard();
    run_pool_simple(0x60, pool::PoolKind::S60, 0xE97B9C, rw_009dcf80, false);
}
#[test]
fn d_pool_init_stride_70() {
    let _g = test_guard();
    run_pool_simple(0x70, pool::PoolKind::S70, 0xE97C5C, rw_009dd000, false);
}
#[test]
fn d_pool_init_stride_80_a() {
    let _g = test_guard();
    run_pool_simple(0x80, pool::PoolKind::S80a, 0xE97C9C, rw_009dd080, false);
}
#[test]
fn d_pool_init_stride_160() {
    let _g = test_guard();
    run_pool_simple(0x160, pool::PoolKind::S160, 0xE97BDC, rw_009dd100, false);
}
#[test]
fn d_pool_init_stride_70_wide() {
    let _g = test_guard();
    run_pool_simple(0x170, pool::PoolKind::S70wide, 0xE97CDC, rw_009dd180, true);
}
#[test]
fn d_pool_init_stride_80_b() {
    let _g = test_guard();
    run_pool_simple(0x180, pool::PoolKind::S80b, 0xE97D1C, rw_009dd280, false);
}

#[test]
fn d_pool_init_stride_3d0_constructed() {
    let _g = test_guard();
    let mut rng = Rng(0x3D0);
    for trial in 0..60 {
        let mut arena = Arena32::new();
        let (count, ok) = match trial % 6 {
            4 => (0x100_0000u32, false),
            5 => (u32::MAX, false),
            _ => (rng.below(9), trial % 2 == 0),
        };
        let header = arena.alloc(12);
        arena.set(header, count);
        let block = arena.alloc(65536);
        let base = block.wrapping_add(0x10);
        let log32 = new_log();
        set_callee(1, Sig::Cdecl1, {
            let log = log32.clone();
            Box::new(move |a| {
                log.lock().unwrap().push(format!("alloc:{}", a[0]));
                if ok { block } else { 0 }
            })
        });
        set_callee(2, Sig::This0, {
            let log = log32.clone();
            Box::new(move |a| {
                let i = a[0].wrapping_sub(base) / 0x3D0;
                log.lock().unwrap().push(format!("construct:{i}"));
                0x5000 + i
            })
        });
        let r32 = unsafe { rw_009dd210(header as *mut u8) };
        let logl = new_log();
        let mut alloc = {
            let log = logl.clone();
            move |n: u32| {
                log.lock().unwrap().push(format!("alloc:{n}"));
                ok
            }
        };
        let mut construct = {
            let log = logl.clone();
            move |i: usize| {
                log.lock().unwrap().push(format!("construct:{i}"));
                0x5000 + i as u32
            }
        };
        let (rl, last) = pool::init_constructed(count, &mut alloc, &mut construct);
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
        if !ok {
            assert_eq!(r32, 0);
            assert!(rl.is_none() && last.is_none());
            continue;
        }
        let p = rl.expect("lifted alloc ok");
        assert_eq!(p.elements.len(), count as usize);
        if count == 0 {
            assert_eq!(r32, block, "empty pool returns the block (32-bit shape)");
            assert_eq!(last, None);
        } else {
            assert_eq!(r32, 0x5000 + count - 1, "last construction result");
            assert_eq!(last, Some(0x5000 + count - 1));
        }
    }
}

// ---------------- value list + one-shot ----------------

#[test]
fn d_list_contains() {
    let _g = test_guard();
    let mut rng = Rng(0x115);
    for _ in 0..250 {
        let mut arena = Arena32::new();
        let n = rng.below(7) as usize;
        let addrs: Vec<u32> = (0..n).map(|_| arena.alloc(8)).collect();
        let mut nodes = Vec::new();
        for i in 0..n {
            let v = rng.below(5);
            let next = if i + 1 < n { Some(i + 1) } else { None };
            arena.set(addrs[i], v);
            arena.set(addrs[i] + 4, next.map_or(0, |j| addrs[j]));
            nodes.push(misc::ListNode { value: v, next: next.map(|j| misc::ListId(j as u32)) });
        }
        let lifted = misc::ListArena { nodes };
        let head_raw = if n == 0 { 0 } else { addrs[0] };
        let head = if n == 0 { None } else { Some(misc::ListId(0)) };
        let val = rng.below(6);
        let r32 = unsafe { rw_009de2c0(&head_raw as *const u32, val) };
        let rl = misc::list_contains(&lifted, head, val);
        assert_eq!(r32, rl as u32, "val={val}");
        // Unchanged by the walk.
        for i in 0..n {
            assert_eq!(arena.w(addrs[i]), lifted.nodes[i].value);
        }
    }
}

#[test]
fn d_oneshot_table_init() {
    let _g = test_guard();
    let mut rng = Rng(0x1CE);
    for _ in 0..120 {
        let mut arena = Arena32::new();
        // Globals block mirrors the 0x103AE84/88/8C layout.
        let g = arena.alloc(12);
        let done0 = rng.below(3) == 0;
        let table_cookie = rng.any();
        let count = [0u16, 1, 7, rng.below(300) as u16][rng.below(4) as usize];
        arena.setb(g, done0 as u8);
        arena.set(g + 4, table_cookie);
        arena.set(g + 8, (arena.w(g + 8) & 0xFFFF_0000) | count as u32);
        map_va(0x103AE84, g);
        map_va(0x103AE88, g + 4);
        map_va(0x103AE8C, g + 8);
        let log32 = new_log();
        set_callee(1, Sig::Cdecl4, {
            let log = log32.clone();
            Box::new(move |a| {
                log.lock().unwrap().push(format!("setup:{}:{}:{}:{:#x}", a[0], a[1], a[2], a[3]));
                0
            })
        });
        let r32 = unsafe { rw_009ddf00() };
        let mut st = misc::OneShotState { done: done0, table: table_cookie, count };
        let logl = new_log();
        let mut setup = {
            let log = logl.clone();
            move |t: u32, c: u32, w: u32, p: misc::SetupProgram| {
                assert_eq!(p, misc::SetupProgram::Default);
                log.lock().unwrap().push(format!("setup:{t}:{c}:{w}:{:#x}", 0x9DDEE0u32));
            }
        };
        misc::oneshot_table_init(&mut st, &mut setup);
        assert_eq!(r32, 0);
        assert_eq!(arena.b(g), st.done as u8, "done latch");
        assert_eq!(arena.w(g + 4), st.table, "table untouched");
        assert_eq!(*log32.lock().unwrap(), *logl.lock().unwrap());
    }
}

// ---------------- lift-only edge behaviour ----------------
// These paths are unrepresentable on the 32-bit side (faults or garbage
// reads there); the lift turns them into loud panics.

#[test]
#[should_panic]
fn lifted_indexed_fetch_oob_panics() {
    let _g = test_guard();
    let mut st = slots::Slots::default();
    st.tables.push(vec![1, 2, 3]);
    st.holders.push(slots::TableId(0));
    let mut inner = slots::Inner::blank();
    inner.holder = Some(slots::HolderId(0));
    inner.index = 99;
    st.inners.push(inner);
    let mut s = slots::Slot::blank();
    s.child = Some(slots::InnerId(0));
    st.slots.push(s);
    let _ = slots::indexed_entry_or_null(&st, slots::SlotId(0));
}

#[test]
#[should_panic(expected = "sweep anchor without")]
fn lifted_sweep_null_sibling_panics() {
    let _g = test_guard();
    let mut arena = chain::ChainArena {
        nodes: vec![{
            let mut n = chain::ChainNode::blank();
            n.flags = 0x0C;
            n
        }],
    };
    let _ = chain::sweep_touch_and_link(
        &mut arena,
        chain::NodeId(0),
        &mut |_, _| Some(chain::NodeId(0)),
        &mut |_, _| 0,
        &mut |_, _, _| 0,
    );
}


