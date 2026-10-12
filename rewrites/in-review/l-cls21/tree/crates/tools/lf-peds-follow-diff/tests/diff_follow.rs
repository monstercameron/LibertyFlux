//! Differential cases: the lifted follow-point route task against its verified
//! 32-bit rewrites, on the same generated inputs.
//!
//! Each case builds real 32-bit memory for the rewrite (the task object, the
//! route tables, the ped and the output records), runs the rewrite and the
//! lifted method on the same inputs, and compares the return value, the
//! order of every host call with its arguments, the written fields and the
//! written bytes. Floats are compared bit for bit. A deliberately wrong lift
//! of each method must be caught. 32-bit target only.

#![cfg(target_arch = "x86")]
// Raw-address memory for the rewrites, and the checker runtime's callee table.
#![allow(unsafe_code)]
#![allow(clippy::all, clippy::pedantic)]

use core::cell::{Cell, RefCell};
use std::sync::{Mutex, MutexGuard};

use lf_peds_follow_diff::rewrites::{
    fn_00cace90, fn_00caf2a0, fn_00cabc00, fn_00cabca0, fn_00cb2970, fn_00cb3b50, fn_00cb73a0, fn_00cb73d0,
    fn_00cbc9f0,
};
use lf_peds_tasks::follow_point_route::host::{PedView, RouteTaskHost};
use lf_peds_tasks::follow_point_route::route::{RoutePoint, RouteTable};
use lf_peds_tasks::follow_point_route::task::{
    FLAG_KEEP_MASK, FLAG_REPEAT, FLAG_ROUTE_SEARCH, FollowPointRouteTask, ORDER_LEG_DONE,
    PED_BLOCKED, RouteTaskInit,
    SUBTASK_GO_TO_DONE, SUBTASK_LEG_DONE, SUBTASK_ROUTE, SUBTASK_TURN_DONE,
};

/// Trials per differential test.
const TRIALS: usize = 3000;
/// Trials per wrong-lift test: enough that a real difference shows up.
const WRONG_TRIALS: usize = 3000;

/// Byte offsets of the task object's fields, as the rewrites read them.
const OFF_SUBTASK: usize = 0x08;
const OFF_CURRENT: usize = 0x20;
const OFF_CURSOR: usize = 0x24;
const OFF_WAYPOINTS: usize = 0x34;
const OFF_INDEX: usize = 0x38;
const OFF_FLAGS: usize = 0x3c;
/// Ped fields the waypoint pick reads: a pointer to a block that holds the
/// position at 0x30, and the state byte.
const PED_BLOCK_PTR: usize = 0x20;
const PED_STATE: usize = 0x26c;
const BLOCK_POS: usize = 0x30;

/// The callee ids the rewrites use, as the checker numbers them.
const CALLEE_1: usize = 1;
const CALLEE_2: usize = 2;
const CALLEE_3: usize = 3;

/// Splitmix64: a small deterministic generator.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    fn u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }

    fn chance(&mut self, num: u64, den: u64) -> bool {
        self.below(den) < num
    }
}

/// One coordinate, as its bits. Mostly small halves, so that segments,
/// projections and distances land exactly on ties; also zeros of both signs,
/// infinities, not-a-number values with payloads (quiet and signalling), and
/// arbitrary bit patterns.
fn coord(rng: &mut Rng) -> u32 {
    match rng.below(24) {
        0 => 0x7FC0_0001, // quiet not-a-number, payload 1
        1 => 0xFFC0_0002, // quiet not-a-number, negative, payload 2
        2 => 0x7F80_0001, // signalling not-a-number
        3 => 0x7F80_0000, // positive infinity
        4 => 0xFF80_0000, // negative infinity
        5 => 0x8000_0000, // negative zero
        6 => rng.u32(),   // any bit pattern
        _ => {
            // Halves from -2.0 to 3.0.
            let v = (rng.below(11) as i32 - 4) as f32 * 0.5;
            v.to_bits()
        }
    }
}

fn point(rng: &mut Rng) -> [u32; 3] {
    [coord(rng), coord(rng), coord(rng)]
}

fn to_points(bits: &[[u32; 3]]) -> Vec<RoutePoint> {
    bits.iter()
        .map(|p| RoutePoint {
            x: f32::from_bits(p[0]),
            y: f32::from_bits(p[1]),
            z: f32::from_bits(p[2]),
        })
        .collect()
}

fn pt(bits: [u32; 3]) -> RoutePoint {
    RoutePoint {
        x: f32::from_bits(bits[0]),
        y: f32::from_bits(bits[1]),
        z: f32::from_bits(bits[2]),
    }
}

/// A route table in the rewrites' memory layout: the count word, then one
/// four-word entry per point (three coordinates and a zero pad word).
fn route_words(points: &[[u32; 3]]) -> Vec<u32> {
    let mut w = vec![0u32; 4 * (points.len() + 1)];
    w[0] = points.len() as u32;
    for (i, p) in points.iter().enumerate() {
        let b = 4 * (i + 1);
        w[b] = p[0];
        w[b + 1] = p[1];
        w[b + 2] = p[2];
        w[b + 3] = 0;
    }
    w
}

/// The task object's memory before a call: a fixed pattern, so that a write
/// to any word the method should not touch shows up.
fn obj_words() -> Vec<u32> {
    (0..64u32).map(|i| 0xA5A5_0000 | i).collect()
}

fn set_word(words: &mut [u32], byte_offset: usize, value: u32) {
    words[byte_offset / 4] = value;
}

fn set_byte(words: &mut [u32], byte_offset: usize, value: u8) {
    let base = words.as_mut_ptr() as *mut u8;
    // SAFETY: the offset lies inside the word buffer the caller sized for it.
    unsafe { *base.add(byte_offset) = value };
}

fn addr<T>(v: &[T]) -> u32 {
    v.as_ptr() as usize as u32
}

// ---- Host calls: the same answers from the stubs and from the lifted host. ----

/// Mixes three words into an answer. Both sides call it with the same
/// arguments, so their answers agree and feed the same results.
fn mix(a: u32, b: u32, c: u32) -> u32 {
    a.wrapping_mul(0x9E37_79B1) ^ b.rotate_left(11).wrapping_add(0x7F4A_7C15) ^ c.wrapping_mul(0x85EB_CA77)
}

fn ans_order(task: u32, order: u32, ped: u32) -> u32 {
    mix(task, order, ped)
}
fn ans_next(task: u32) -> u32 {
    mix(task, 0x51, 0x52)
}
fn ans_create(task: u32, id: u32, ped: u32) -> u32 {
    mix(task ^ 0x77, id, ped)
}
fn ans_release(ctx: u32, table: u32) -> u32 {
    mix(ctx, table, 0x99)
}
fn ans_base_dtor(task: u32) -> u32 {
    mix(task, 0x77, 0x88)
}
fn ans_plan(task: u32) -> u32 {
    mix(task, 0x33, 0x44)
}

/// One host call, as the rewrite and the lifted method both log it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ev {
    Order { task: u32, order: u32, ped: u32 },
    Next { task: u32 },
    Create { task: u32, id: u32, ped: u32 },
    Plan { task: u32 },
    Kind { sub: u32 },
    Select { selector: u32 },
    Base { task: u32, speed: u32 },
    RouteCtor { task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32 },
    Alloc { mgr: u32 },
    Release { ctx: u32, table: u32 },
    BaseDtor { task: u32 },
}

thread_local! {
    static REC: RefCell<Vec<Ev>> = const { RefCell::new(Vec::new()) };
    static KIND: Cell<u32> = const { Cell::new(0) };
}

fn record(e: Ev) {
    REC.with(|r| r.borrow_mut().push(e));
}

fn take_calls() -> Vec<Ev> {
    // The rewrites query a sub-task's type a variable number of times; the
    // query changes nothing, so the comparison drops those queries from both
    // sides and keeps every other call in order.
    REC.with(|r| std::mem::take(&mut *r.borrow_mut()))
        .into_iter()
        .filter(|e| !matches!(e, Ev::Kind { .. }))
        .collect()
}

extern "thiscall" fn order_stub(task: u32, order: u32, ped: u32) -> u32 {
    record(Ev::Order { task, order, ped });
    ans_order(task, order, ped)
}
extern "thiscall" fn next_stub(task: u32) -> u32 {
    record(Ev::Next { task });
    ans_next(task)
}
extern "thiscall" fn create_stub(task: u32, id: u32, ped: u32) -> u32 {
    record(Ev::Create { task, id, ped });
    ans_create(task, id, ped)
}
extern "thiscall" fn plan_stub(task: u32) -> u32 {
    record(Ev::Plan { task });
    ans_plan(task)
}
extern "thiscall" fn kind_stub(sub: u32) -> u32 {
    record(Ev::Kind { sub });
    KIND.with(|k| k.get())
}

/// Lifted host: the same calls, logged the same way, with the same answers.
#[derive(Default)]
struct RecHost {
    log: Vec<Ev>,
    /// The allocator manager the case reads from its global.
    mgr: u32,
    /// The handle the allocation answers with (0 for none).
    new_handle: u32,
    /// The release context the case reads from its global.
    ctx: u32,
}

impl RouteTaskHost for RecHost {
    fn next_order_id(&mut self, task: u32) -> u32 {
        self.log.push(Ev::Next { task });
        ans_next(task)
    }
    fn order_ped(&mut self, task: u32, order: u32, ped: u32) -> u32 {
        self.log.push(Ev::Order { task, order, ped });
        ans_order(task, order, ped)
    }
    fn subtask_type(&mut self, subtask: u32) -> u32 {
        self.log.push(Ev::Kind { sub: subtask });
        KIND.with(|k| k.get())
    }
    fn create_subtask(&mut self, task: u32, id: u32, ped: u32) -> u32 {
        self.log.push(Ev::Create { task, id, ped });
        ans_create(task, id, ped)
    }
    fn plan_next_subtask(&mut self, task: u32) -> u32 {
        self.log.push(Ev::Plan { task });
        ans_plan(task)
    }
    fn select_speed(&mut self, selector: u32) -> f32 {
        self.log.push(Ev::Select { selector });
        ans_select(selector)
    }
    fn move_base_ctor(&mut self, task: &mut FollowPointRouteTask, speed_bits: u32) -> u32 {
        self.log.push(Ev::Base { task: task.handle, speed: speed_bits });
        let f = ans_base(task.handle, speed_bits);
        task.flags = f;
        f
    }
    fn route_ctor(&mut self, task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
        self.log.push(Ev::RouteCtor { task, target, start, end, final_value, id });
        ans_route(task, target, start, end, final_value, id)
    }
    fn allocate_task(&mut self) -> u32 {
        self.log.push(Ev::Alloc { mgr: self.mgr });
        self.new_handle
    }
    fn release_route(&mut self, table: u32) -> u32 {
        self.log.push(Ev::Release { ctx: self.ctx, table });
        ans_release(self.ctx, table)
    }
    fn base_destroy(&mut self, task: u32) -> u32 {
        self.log.push(Ev::BaseDtor { task });
        ans_base_dtor(task)
    }
}

impl RecHost {
    fn calls(self) -> Vec<Ev> {
        self.log.into_iter().filter(|e| !matches!(e, Ev::Kind { .. })).collect()
    }
}

// ---- The callee table the rewrites call through. ----

/// The shared checker runtime reads its callee stubs from a table of 256
/// words. This crate fills the entries a case uses before each rewrite call.
static mut CALLEE_TABLE: [u32; 256] = [0; 256];

fn install(entries: &[(usize, u32)]) {
    let table = core::ptr::addr_of_mut!(CALLEE_TABLE).cast::<u32>();
    for &(id, stub) in entries {
        // SAFETY: every id used here is below 256.
        unsafe { table.add(id).write(stub) };
    }
    // SAFETY: the table is static and outlives every rewrite call.
    unsafe { lf_checker_rt::CHECKER_CTABLE = table.cast_const() };
}

/// Serialises the cases: the callee table and the recorders are shared.
static LOCK: Mutex<()> = Mutex::new(());

fn guard() -> MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

// ---- Outcomes compared between the rewrite and the lifted method. ----

/// Everything a case can observe: the return value, the host calls in
/// order, the object's index and flag words, the bytes an output record
/// received, and whether the rewrite wrote anything it should not have.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Outcome {
    ret: u32,
    calls: Vec<Ev>,
    index: u32,
    flags: u32,
    out: Vec<u8>,
    clobbered: bool,
}

// ---- Waypoint pick (verified rewrite fn_00cb2970). ----

#[derive(Debug, Clone)]
struct PickCase {
    points: Vec<[u32; 3]>,
    current: Option<Vec<[u32; 3]>>,
    pos: [u32; 3],
    index: u32,
    state: u8,
    flags: u32,
}

fn gen_pick(rng: &mut Rng) -> PickCase {
    let n = match rng.below(10) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 1 + rng.below(6) as usize,
    };
    let mut points: Vec<[u32; 3]> = Vec::new();
    for i in 0..n {
        if i > 0 && rng.chance(1, 6) {
            // A repeated point: a zero-length segment.
            points.push(points[i - 1]);
        } else {
            points.push(point(rng));
        }
    }
    // Three ways to place the ped: on a route point, beside the middle of a
    // segment (so that a projection lands inside it), or anywhere.
    let pos = match rng.below(3) {
        0 if n > 0 => points[rng.below(n as u64) as usize],
        1 if n > 1 => {
            let i = rng.below((n - 1) as u64) as usize;
            let a = points[i].map(f32::from_bits);
            let b = points[i + 1].map(f32::from_bits);
            let mid = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5 + 0.5, (a[2] + b[2]) * 0.5];
            mid.map(f32::to_bits)
        }
        _ => point(rng),
    };
    let index = match rng.below(6) {
        0 => n as u32,
        1 => (n as u32).wrapping_sub(1),
        2 => u32::MAX,
        3 => 0,
        _ => rng.u32() % (n as u32 + 2),
    };
    let state = if rng.chance(1, 4) {
        (rng.u32() as u8) | PED_BLOCKED as u8
    } else {
        (rng.u32() as u8) & !(PED_BLOCKED as u8)
    };
    let current = if rng.chance(3, 4) {
        Some(points.clone())
    } else {
        None
    };
    PickCase {
        points,
        current,
        pos,
        index,
        state,
        flags: rng.u32(),
    }
}

/// Runs the rewrite on the case's memory. Returns the outcome and the
/// addresses the rewrite saw for the task and the ped, which the lifted run
/// reuses as its handles.
fn pick_rewrite(c: &PickCase) -> (Outcome, u32, u32) {
    let route = route_words(&c.points);
    let current = c.current.as_ref().map(|p| route_words(p));
    let mut block = vec![0u32; 16];
    set_word(&mut block, BLOCK_POS, c.pos[0]);
    set_word(&mut block, BLOCK_POS + 4, c.pos[1]);
    set_word(&mut block, BLOCK_POS + 8, c.pos[2]);
    let mut ped = vec![0xC3C3_C3C3u32; 160];
    set_word(&mut ped, PED_BLOCK_PTR, addr(&block));
    set_byte(&mut ped, PED_STATE, c.state);
    let mut obj = obj_words();
    set_word(&mut obj, OFF_CURRENT, current.as_ref().map_or(0, |v| addr(v)));
    set_word(&mut obj, OFF_WAYPOINTS, addr(&route));
    set_word(&mut obj, OFF_INDEX, c.index);
    set_word(&mut obj, OFF_FLAGS, c.flags);

    let obj_before = obj.clone();
    let route_before = route.clone();
    let ped_before = ped.clone();
    let block_before = block.clone();

    install(&[(CALLEE_1, order_stub as *const () as usize as u32), (CALLEE_2, next_stub as *const () as usize as u32)]);
    let _ = take_calls();
    let this = addr(&obj);
    let ped_addr = addr(&ped);
    let ret = fn_00cb2970::rw_00cb2970(this, ped_addr);
    let calls = take_calls();

    let clobbered = obj.iter().enumerate().any(|(i, w)| {
        i != OFF_INDEX / 4 && i != OFF_FLAGS / 4 && *w != obj_before[i]
    }) || route != route_before
        || ped != ped_before
        || block != block_before;
    let outcome = Outcome {
        ret,
        calls,
        index: obj[OFF_INDEX / 4],
        flags: obj[OFF_FLAGS / 4],
        out: Vec::new(),
        clobbered,
    };
    (outcome, this, ped_addr)
}

/// The lifted method on the same case. `this` and `ped_addr` are the handles
/// the rewrite was given, so the logged arguments are comparable.
fn pick_lifted(c: &PickCase, this: u32, ped_addr: u32) -> Outcome {
    let mut task = FollowPointRouteTask {
        handle: this,
        subtask: 0,
        current_route: c.current.as_ref().map(|p| RouteTable::new(to_points(p))),
        waypoints: RouteTable::new(to_points(&c.points)),
        route_cursor: 0,
        waypoint_index: c.index,
        flags: c.flags,
        ..Default::default()
    };
    let ped = PedView {
        handle: ped_addr,
        position: pt(c.pos),
        state_byte: c.state,
    };
    let mut host = RecHost::default();
    let ret = task.pick_waypoint(&ped, &mut host);
    Outcome {
        ret,
        calls: host.calls(),
        index: task.waypoint_index,
        flags: task.flags,
        out: Vec::new(),
        clobbered: false,
    }
}

/// Deliberately wrong lifts of the pick. Each differs from the lifted method
/// in one step; each must be caught.
/// The pick without the segment projections: the nearest route point always wins.
fn pick_nearest_only(task: &mut FollowPointRouteTask, ped: &PedView, host: &mut RecHost) -> u32 {
    task.flags &= !FLAG_ROUTE_SEARCH;
    let count = task.waypoints.len() as i32;
    if count == task.waypoint_index as i32 {
        return host.order_ped(task.handle, ORDER_LEG_DONE, ped.handle);
    }
    if ped.state_byte & PED_BLOCKED != 0 {
        return 0;
    }
    if task.current_route.is_none() {
        let next = host.next_order_id(task.handle);
        return host.order_ped(task.handle, next, ped.handle);
    }
    let mut best_next: i32 = -1;
    let mut best = f32::MAX;
    for e in 0..count {
        let p = task.waypoints.point(e as usize);
        let dx = ped.position.x - p.x;
        let dy = ped.position.y - p.y;
        let dz = ped.position.z - p.z;
        let d2 = (dy * dy + dx * dx) + dz * dz;
        if best * best > d2 {
            best_next = e;
            best = d2;
        }
    }
    task.waypoint_index = best_next as u32;
    let next = host.next_order_id(task.handle);
    host.order_ped(task.handle, next, ped.handle)
}

fn pick_wrong(c: &PickCase, this: u32, ped_addr: u32, which: u32) -> Outcome {
    let mut task = FollowPointRouteTask {
        handle: this,
        subtask: 0,
        current_route: c.current.as_ref().map(|p| RouteTable::new(to_points(p))),
        waypoints: RouteTable::new(to_points(&c.points)),
        route_cursor: 0,
        waypoint_index: c.index,
        flags: c.flags,
        ..Default::default()
    };
    let mut ped = PedView {
        handle: ped_addr,
        position: pt(c.pos),
        state_byte: c.state,
    };
    let mut host = RecHost::default();
    let ret = match which {
        // Ignores the ped's block bit: follows a route even when blocked.
        0 => {
            ped.state_byte &= !(PED_BLOCKED as u8);
            task.pick_waypoint(&ped, &mut host)
        }
        // Stores the picked index one too high.
        1 => {
            let r = task.pick_waypoint(&ped, &mut host);
            task.waypoint_index = task.waypoint_index.wrapping_add(1);
            r
        }
        // Searches for the nearest route point only: no segment projection.
        _ => pick_nearest_only(&mut task, &ped, &mut host),
    };
    Outcome {
        ret,
        calls: host.calls(),
        index: task.waypoint_index,
        flags: task.flags,
        out: Vec::new(),
        clobbered: false,
    }
}

// ---- Advance after a sub-task (verified rewrite fn_00cb3b50). ----

#[derive(Debug, Clone)]
struct AdvCase {
    points: Vec<[u32; 3]>,
    index: u32,
    flags: u32,
    kind: u32,
    ped: u32,
}

fn gen_adv(rng: &mut Rng) -> AdvCase {
    let n = match rng.below(8) {
        0 => 0,
        _ => 1 + rng.below(5) as usize,
    };
    let points: Vec<[u32; 3]> = (0..n).map(|_| point(rng)).collect();
    let index = match rng.below(6) {
        0 => (n as u32).wrapping_sub(1),
        1 => n as u32,
        2 => u32::MAX,
        3 => 0,
        _ => rng.u32() % (n as u32 + 2),
    };
    let kind = match rng.below(5) {
        0 => SUBTASK_ROUTE,
        1 => SUBTASK_LEG_DONE,
        2 => SUBTASK_GO_TO_DONE,
        3 => SUBTASK_TURN_DONE,
        _ => 0x123,
    };
    let flags = if rng.chance(1, 2) {
        rng.u32() | FLAG_REPEAT
    } else {
        rng.u32() & !FLAG_REPEAT
    };
    AdvCase {
        points,
        index,
        flags,
        kind,
        ped: 0x1000 | (rng.u32() & 0xFFFF),
    }
}

fn adv_rewrite(c: &AdvCase) -> (Outcome, u32, u32) {
    let route = route_words(&c.points);
    // The sub-task object: its first word is a vtable whose type-query slot
    // (the fourth word) answers the case's kind.
    let mut vtable = vec![0u32; 8];
    vtable[3] = kind_stub as *const () as usize as u32;
    let mut sub = vec![0u32; 8];
    sub[0] = addr(&vtable);
    let mut obj = obj_words();
    set_word(&mut obj, OFF_SUBTASK, addr(&sub));
    set_word(&mut obj, OFF_WAYPOINTS, addr(&route));
    set_word(&mut obj, OFF_INDEX, c.index);
    set_word(&mut obj, OFF_FLAGS, c.flags);
    let obj_before = obj.clone();
    let route_before = route.clone();
    let sub_before = sub.clone();

    install(&[
        (CALLEE_2, create_stub as *const () as usize as u32),
        (CALLEE_3, plan_stub as *const () as usize as u32),
    ]);
    KIND.with(|k| k.set(c.kind));
    let _ = take_calls();
    let this = addr(&obj);
    let ret = fn_00cb3b50::rw_00CB3B50(this, c.ped);
    let calls = take_calls();

    let clobbered = obj.iter().enumerate().any(|(i, w)| {
        i != OFF_INDEX / 4 && *w != obj_before[i]
    }) || route != route_before
        || sub != sub_before;
    (
        Outcome {
            ret,
            calls,
            index: obj[OFF_INDEX / 4],
            flags: obj[OFF_FLAGS / 4],
            out: Vec::new(),
            clobbered,
        },
        this,
        addr(&sub),
    )
}

fn adv_lifted(c: &AdvCase, this: u32, sub: u32) -> Outcome {
    let mut task = FollowPointRouteTask {
        handle: this,
        subtask: sub,
        current_route: None,
        waypoints: RouteTable::new(to_points(&c.points)),
        route_cursor: 0,
        waypoint_index: c.index,
        flags: c.flags,
        ..Default::default()
    };
    KIND.with(|k| k.set(c.kind));
    let mut host = RecHost::default();
    let ret = task.advance_after_subtask(c.ped, &mut host);
    Outcome {
        ret,
        calls: host.calls(),
        index: task.waypoint_index,
        flags: task.flags,
        out: Vec::new(),
        clobbered: false,
    }
}

fn adv_wrong(c: &AdvCase, this: u32, sub: u32, which: u32) -> Outcome {
    let mut task = FollowPointRouteTask {
        handle: this,
        subtask: sub,
        current_route: None,
        waypoints: RouteTable::new(to_points(&c.points)),
        route_cursor: 0,
        waypoint_index: c.index,
        flags: c.flags,
        ..Default::default()
    };
    KIND.with(|k| k.set(c.kind));
    let mut host = RecHost::default();
    let ret = match which {
        // Reads the repeat flag as set at a finished turn.
        0 => {
            let saved = task.flags;
            task.flags |= FLAG_REPEAT;
            let r = task.advance_after_subtask(c.ped, &mut host);
            task.flags = saved;
            r
        }
        // Reads a finished turn as a finished leg.
        _ => {
            if c.kind == SUBTASK_TURN_DONE {
                KIND.with(|k| k.set(SUBTASK_LEG_DONE));
            }
            task.advance_after_subtask(c.ped, &mut host)
        }
    };
    Outcome {
        ret,
        calls: host.calls(),
        index: task.waypoint_index,
        flags: task.flags,
        out: Vec::new(),
        clobbered: false,
    }
}

// ---- Handle copy (verified rewrite fn_00cbc9f0). ----

fn cursor_rewrite(cursor: u32) -> Outcome {
    let mut obj = obj_words();
    set_word(&mut obj, OFF_CURSOR, cursor);
    let before = obj.clone();
    let rec = vec![0x5A5A_5A5Au32; 16];
    let rec_addr = addr(&rec);
    let ret = fn_00cbc9f0::rw_00cbc9f0(addr(&obj) as *mut u8, rec_addr as *mut u8);
    let clobbered = obj != before;
    Outcome {
        ret,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out: rec.iter().flat_map(|w| w.to_le_bytes()).collect(),
        clobbered,
    }
}

fn cursor_lifted(cursor: u32, rec_addr: u32) -> Outcome {
    let task = FollowPointRouteTask {
        route_cursor: cursor,
        ..Default::default()
    };
    let rec = vec![0x5A5A_5A5Au32; 16];
    let mut bytes: Vec<u8> = rec.iter().flat_map(|w| w.to_le_bytes()).collect();
    task.copy_handle_into(&mut bytes);
    Outcome {
        ret: rec_addr,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out: bytes,
        clobbered: false,
    }
}

fn cursor_wrong(cursor: u32, rec_addr: u32) -> Outcome {
    // Writes the cursor one too high.
    let task = FollowPointRouteTask {
        route_cursor: cursor.wrapping_add(1),
        ..Default::default()
    };
    let rec = vec![0x5A5A_5A5Au32; 16];
    let mut bytes: Vec<u8> = rec.iter().flat_map(|w| w.to_le_bytes()).collect();
    task.copy_handle_into(&mut bytes);
    Outcome {
        ret: rec_addr,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out: bytes,
        clobbered: false,
    }
}

// ---- Current point copy (verified rewrite fn_00cb73d0) and waypoint copy
// (verified rewrite fn_00cb73a0). ----

/// A case for the two copy methods: the current route (read by the first),
/// the waypoint route (read by the second), a cursor or index in range.
#[derive(Debug, Clone)]
struct CopyCase {
    current: Vec<[u32; 3]>,
    waypoints: Vec<[u32; 3]>,
    cursor: u32,
}

fn gen_copy(rng: &mut Rng) -> CopyCase {
    let n = 1 + rng.below(6) as usize;
    let current: Vec<[u32; 3]> = (0..n).map(|_| point(rng)).collect();
    let m = 1 + rng.below(6) as usize;
    let waypoints: Vec<[u32; 3]> = (0..m).map(|_| point(rng)).collect();
    // The cursor is below both tables' lengths: each copy reads its own table in range.
    let shared = n.min(m);
    CopyCase {
        current,
        waypoints,
        cursor: rng.below(shared as u64) as u32,
    }
}

fn current_rewrite(c: &CopyCase) -> Outcome {
    let route = route_words(&c.current);
    let mut obj = obj_words();
    set_word(&mut obj, OFF_CURRENT, addr(&route));
    set_word(&mut obj, OFF_CURSOR, c.cursor);
    let before = obj.clone();
    let dst = vec![0xEEEE_EEEEu32; 4];
    let dst_addr = addr(&dst);
    let ret = fn_00cb73d0::rw_cb73d0(addr(&obj), dst_addr);
    let clobbered = obj != before;
    Outcome {
        ret,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out: dst.iter().flat_map(|w| w.to_le_bytes()).collect(),
        clobbered,
    }
}

fn current_lifted(c: &CopyCase, dst_addr: u32) -> Outcome {
    let task = FollowPointRouteTask {
        current_route: Some(RouteTable::new(to_points(&c.current))),
        route_cursor: c.cursor,
        ..Default::default()
    };
    let p = task.current_point();
    let mut out: Vec<u8> = vec![0xEE; 16];
    write_point(&mut out, p);
    Outcome {
        ret: dst_addr,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out,
        clobbered: false,
    }
}

fn current_wrong(c: &CopyCase, dst_addr: u32) -> Outcome {
    // Reads the point before the cursor.
    let task = FollowPointRouteTask {
        current_route: Some(RouteTable::new(to_points(&c.current))),
        route_cursor: c.cursor.saturating_sub(1),
        ..Default::default()
    };
    let p = task.current_point();
    let mut out: Vec<u8> = vec![0xEE; 16];
    write_point(&mut out, p);
    Outcome {
        ret: dst_addr,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out,
        clobbered: false,
    }
}

fn waypoint_rewrite(c: &CopyCase) -> Outcome {
    let route = route_words(&c.waypoints);
    let mut obj = obj_words();
    set_word(&mut obj, OFF_WAYPOINTS, addr(&route));
    let before = obj.clone();
    let dst = vec![0xEEEE_EEEEu32; 4];
    let dst_addr = addr(&dst);
    // The index is the cursor value of the case; the rewrite reads it as its
    // second argument.
    let ret = fn_00cb73a0::rw_cb73a0(addr(&obj), dst_addr, c.cursor);
    let clobbered = obj != before;
    Outcome {
        ret,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out: dst.iter().flat_map(|w| w.to_le_bytes()).collect(),
        clobbered,
    }
}

fn waypoint_lifted(c: &CopyCase, dst_addr: u32) -> Outcome {
    let task = FollowPointRouteTask {
        waypoints: RouteTable::new(to_points(&c.waypoints)),
        ..Default::default()
    };
    let p = task.waypoint(c.cursor);
    let mut out: Vec<u8> = vec![0xEE; 16];
    write_point(&mut out, p);
    Outcome {
        ret: dst_addr,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out,
        clobbered: false,
    }
}

fn waypoint_wrong(c: &CopyCase, dst_addr: u32) -> Outcome {
    // Reads the current route's table instead of the waypoint table.
    let task = FollowPointRouteTask {
        waypoints: RouteTable::new(to_points(&c.current)),
        ..Default::default()
    };
    let p = task.waypoint(c.cursor);
    let mut out: Vec<u8> = vec![0xEE; 16];
    write_point(&mut out, p);
    Outcome {
        ret: dst_addr,
        calls: Vec::new(),
        index: 0,
        flags: 0,
        out,
        clobbered: false,
    }
}

/// Writes a point's three coordinates into the first twelve bytes, as the
/// caller of the lifted copy methods does.
fn write_point(out: &mut [u8], p: RoutePoint) {
    out[0..4].copy_from_slice(&p.x.to_bits().to_le_bytes());
    out[4..8].copy_from_slice(&p.y.to_bits().to_le_bytes());
    out[8..12].copy_from_slice(&p.z.to_bits().to_le_bytes());
}

// ---- Tests. ----

#[test]
fn pick_waypoint_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0001);
    let (mut done, mut blocked, mut scanned) = (0usize, 0usize, 0usize);
    let mut nan_or_inf = 0usize;
    for _ in 0..TRIALS {
        let c = gen_pick(&mut rng);
        let (rw, this, ped) = pick_rewrite(&c);
        assert!(!rw.clobbered, "rewrite wrote outside its two fields: {c:?}");
        let lf = pick_lifted(&c, this, ped);
        assert_eq!(rw, lf, "pick differs for {c:?}");
        if c.index == c.points.len() as u32 {
            done += 1;
        } else if c.state & PED_BLOCKED as u8 != 0 {
            blocked += 1;
        } else if c.current.is_some() && !c.points.is_empty() {
            scanned += 1;
        }
        if c.points.iter().any(|p| p.iter().any(|w| !f32::from_bits(*w).is_finite())) {
            nan_or_inf += 1;
        }
    }
    eprintln!(
        "pick: {TRIALS} trials; order-done {done}, blocked {blocked}, scanned {scanned}, \
         trials with a non-finite route point {nan_or_inf}"
    );
    assert!(done > 0 && blocked > 0 && scanned > 0);
}

#[test]
fn pick_waypoint_wrong_lift_is_caught() {
    let _g = guard();
    for which in 0..3u32 {
        let mut rng = Rng(0x5EED_0101 + which as u64);
        let mut caught = 0usize;
        for _ in 0..WRONG_TRIALS {
            let c = gen_pick(&mut rng);
            let (rw, this, ped) = pick_rewrite(&c);
            if pick_wrong(&c, this, ped, which) != rw {
                caught += 1;
            }
        }
        eprintln!("pick wrong lift {which}: caught in {caught} of {WRONG_TRIALS} trials");
        assert!(caught > 0, "wrong pick lift {which} was not caught");
    }
}

#[test]
fn advance_after_subtask_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0002);
    let mut kinds = [0usize; 5];
    for _ in 0..TRIALS {
        let c = gen_adv(&mut rng);
        let (rw, this, sub) = adv_rewrite(&c);
        assert!(!rw.clobbered, "rewrite wrote outside the index: {c:?}");
        let lf = adv_lifted(&c, this, sub);
        assert_eq!(rw, lf, "advance differs for {c:?}");
        let slot = match c.kind {
            SUBTASK_ROUTE => 0,
            SUBTASK_LEG_DONE => 1,
            SUBTASK_GO_TO_DONE => 2,
            SUBTASK_TURN_DONE => 3,
            _ => 4,
        };
        kinds[slot] += 1;
    }
    eprintln!("advance: {TRIALS} trials; per sub-task type {kinds:?}");
}

#[test]
fn advance_after_subtask_wrong_lift_is_caught() {
    let _g = guard();
    for which in 0..2u32 {
        let mut rng = Rng(0x5EED_0102 + which as u64);
        let mut caught = 0usize;
        for _ in 0..WRONG_TRIALS {
            let c = gen_adv(&mut rng);
            let (rw, this, sub) = adv_rewrite(&c);
            if adv_wrong(&c, this, sub, which) != rw {
                caught += 1;
            }
        }
        eprintln!("advance wrong lift {which}: caught in {caught} of {WRONG_TRIALS} trials");
        assert!(caught > 0, "wrong advance lift {which} was not caught");
    }
}

#[test]
fn copy_handle_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0003);
    for _ in 0..TRIALS {
        let cursor = rng.u32();
        let rw = cursor_rewrite(cursor);
        assert!(!rw.clobbered);
        let rec_addr = rw.ret;
        let lf = cursor_lifted(cursor, rec_addr);
        assert_eq!(rw, lf, "handle copy differs for cursor {cursor:#x}");
    }
}

#[test]
fn copy_handle_wrong_lift_is_caught() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0103);
    let mut caught = 0usize;
    for _ in 0..WRONG_TRIALS {
        let cursor = rng.u32();
        let rw = cursor_rewrite(cursor);
        if cursor_wrong(cursor, rw.ret) != rw {
            caught += 1;
        }
    }
    eprintln!("handle copy wrong lift: caught in {caught} of {WRONG_TRIALS} trials");
    assert!(caught > 0);
}

#[test]
fn current_point_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0004);
    for _ in 0..TRIALS {
        let c = gen_copy(&mut rng);
        let rw = current_rewrite(&c);
        assert!(!rw.clobbered);
        let lf = current_lifted(&c, rw.ret);
        assert_eq!(rw, lf, "current point differs for {c:?}");
    }
}

#[test]
fn current_point_wrong_lift_is_caught() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0104);
    let mut caught = 0usize;
    for _ in 0..WRONG_TRIALS {
        let c = gen_copy(&mut rng);
        let rw = current_rewrite(&c);
        if current_wrong(&c, rw.ret) != rw {
            caught += 1;
        }
    }
    eprintln!("current point wrong lift: caught in {caught} of {WRONG_TRIALS} trials");
    assert!(caught > 0);
}

#[test]
fn waypoint_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0005);
    for _ in 0..TRIALS {
        let c = gen_copy(&mut rng);
        let rw = waypoint_rewrite(&c);
        assert!(!rw.clobbered);
        let lf = waypoint_lifted(&c, rw.ret);
        assert_eq!(rw, lf, "waypoint differs for {c:?}");
    }
}

#[test]
fn waypoint_wrong_lift_is_caught() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0105);
    let mut caught = 0usize;
    for _ in 0..WRONG_TRIALS {
        let c = gen_copy(&mut rng);
        let rw = waypoint_rewrite(&c);
        if waypoint_wrong(&c, rw.ret) != rw {
            caught += 1;
        }
    }
    eprintln!("waypoint wrong lift: caught in {caught} of {WRONG_TRIALS} trials");
    assert!(caught > 0);
}







// ---- Constructors (verified rewrites fn_00cabca0 and fn_00cabc00). ----

fn ans_select(selector: u32) -> f32 {
    // Finite results only: this value crosses a float return, which takes
    // an x87 path on this target, and a signalling payload would be quieted
    // there on the rewrite side alone.
    ((mix(selector, 1, 2) % 1000) as f32) * 0.25 - 100.0
}

fn ans_base(task: u32, speed: u32) -> u32 {
    mix(task, speed, 0x3C)
}

fn ans_route(task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
    mix(task ^ target, start ^ end, final_value ^ id)
}

extern "cdecl" fn select_stub(selector: u32) -> f32 {
    record(Ev::Select { selector });
    ans_select(selector)
}

extern "thiscall" fn base_stub(this: u32, speed: u32) -> u32 {
    record(Ev::Base { task: this, speed });
    let f = ans_base(this, speed);
    // The move base writes the flag word of the object it constructs.
    // SAFETY: `this` is the task object the case built; its flag word lies inside it.
    unsafe { ((this as usize + OFF_FLAGS) as *mut u32).write_unaligned(f) };
    f
}

extern "thiscall" fn route6_stub(this: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
    record(Ev::RouteCtor { task: this, target, start, end, final_value, id });
    ans_route(this, target, start, end, final_value, id)
}

#[derive(Debug, Clone)]
struct CtorCase {
    selector: u32,
    speed_bits: u32,
    target: u32,
    route_tag: Option<Vec<[u32; 3]>>,
    route_start: u32,
    route_end: u32,
    include_start: u32,
    include_end: u32,
    route_flags: u32,
    route_id: u32,
    final_value: u32,
}

fn gen_ctor(rng: &mut Rng) -> CtorCase {
    let n = rng.below(4) as usize;
    let route_tag = if rng.chance(3, 4) {
        Some((0..n).map(|_| point(rng)).collect())
    } else {
        None
    };
    CtorCase {
        selector: rng.u32(),
        speed_bits: rng.u32(),
        target: rng.u32(),
        route_tag,
        route_start: rng.u32(),
        route_end: rng.u32(),
        // Only bit 0 of each flag argument is used; the other bits are random too.
        include_start: rng.u32(),
        include_end: rng.u32(),
        route_flags: rng.u32(),
        route_id: rng.u32(),
        final_value: rng.u32(),
    }
}

/// Byte offsets of the two vtable words the constructor writes.
const OFF_VTABLE: usize = 0x00;
const OFF_BASE_VTABLE: usize = 0x14;
/// The vtable constants the rewrites install, as file addresses.
const TASK_VTABLE_VA: u32 = 0x00ED_85BC;
const BASE_VTABLE_VA: u32 = 0x00ED_8618;

fn ctor_rewrite(c: &CtorCase, selector_form: bool) -> (Outcome, u32) {
    let tag = c.route_tag.as_ref().map(|p| route_words(p));
    let tag_word = tag.as_ref().map_or(0, |v| addr(v));
    let mut obj = obj_words();
    let before = obj.clone();
    if selector_form {
        install(&[
            (CALLEE_1, select_stub as *const () as usize as u32),
            (CALLEE_2, base_stub as *const () as usize as u32),
            (CALLEE_3, route6_stub as *const () as usize as u32),
        ]);
    } else {
        install(&[
            (CALLEE_1, base_stub as *const () as usize as u32),
            (CALLEE_2, route6_stub as *const () as usize as u32),
        ]);
    }
    let _ = take_calls();
    let this = addr(&obj);
    let ret = if selector_form {
        fn_00cabc00::rw_00cabc00(
            this,
            c.selector,
            c.target,
            tag_word,
            c.route_start,
            c.route_end,
            c.include_start,
            c.include_end,
            c.route_flags,
            c.route_id,
            c.final_value,
        )
    } else {
        fn_00cabca0::rw_00cabca0(
            this,
            c.speed_bits,
            c.target,
            tag_word,
            c.route_start,
            c.route_end,
            c.include_start,
            c.include_end,
            c.route_flags,
            c.route_id,
            c.final_value,
        )
    };
    let calls = take_calls();
    let written = [
        OFF_VTABLE / 4,
        OFF_BASE_VTABLE / 4,
        OFF_CURRENT / 4,
        OFF_INDEX / 4,
        OFF_FLAGS / 4,
    ];
    let clobbered = obj
        .iter()
        .enumerate()
        .any(|(i, w)| !written.contains(&i) && *w != before[i]);
    // The vtable words are the class identity: checked against the constants
    // the rewrite installs, not compared with the lifted object.
    assert_eq!(obj[OFF_VTABLE / 4], lf_checker_rt::relocated(TASK_VTABLE_VA));
    assert_eq!(obj[OFF_BASE_VTABLE / 4], lf_checker_rt::relocated(BASE_VTABLE_VA));
    (
        Outcome {
            ret,
            calls,
            index: obj[OFF_INDEX / 4],
            flags: obj[OFF_FLAGS / 4],
            out: vec![(obj[OFF_CURRENT / 4] != 0) as u8],
            clobbered,
        },
        this,
    )
}

fn ctor_init(c: &CtorCase) -> RouteTaskInit {
    RouteTaskInit {
        speed_bits: c.speed_bits,
        target: c.target,
        route_tag: c.route_tag.as_ref().map(|p| RouteTable::new(to_points(p))),
        route_start: c.route_start,
        route_end: c.route_end,
        include_start: c.include_start,
        include_end: c.include_end,
        route_flags: c.route_flags,
        route_id: c.route_id,
        final_value: c.final_value,
    }
}

fn ctor_lifted(c: &CtorCase, selector_form: bool, this: u32) -> Outcome {
    let mut host = RecHost::default();
    let init = ctor_init(c);
    let task = if selector_form {
        FollowPointRouteTask::construct_from_selector(this, c.selector, init, &mut host)
    } else {
        FollowPointRouteTask::construct(this, init, &mut host)
    };
    Outcome {
        ret: this,
        calls: host.calls(),
        index: task.waypoint_index,
        flags: task.flags,
        out: vec![task.current_route.is_some() as u8],
        clobbered: false,
    }
}

/// The constructor's host with its route arguments swapped: a wrong lift.
struct SwapRouteArgs(RecHost);

impl RouteTaskHost for SwapRouteArgs {
    fn next_order_id(&mut self, task: u32) -> u32 {
        self.0.next_order_id(task)
    }
    fn order_ped(&mut self, task: u32, order: u32, ped: u32) -> u32 {
        self.0.order_ped(task, order, ped)
    }
    fn subtask_type(&mut self, subtask: u32) -> u32 {
        self.0.subtask_type(subtask)
    }
    fn create_subtask(&mut self, task: u32, id: u32, ped: u32) -> u32 {
        self.0.create_subtask(task, id, ped)
    }
    fn plan_next_subtask(&mut self, task: u32) -> u32 {
        self.0.plan_next_subtask(task)
    }
    fn select_speed(&mut self, selector: u32) -> f32 {
        self.0.select_speed(selector)
    }
    fn move_base_ctor(&mut self, task: &mut FollowPointRouteTask, speed_bits: u32) -> u32 {
        self.0.move_base_ctor(task, speed_bits)
    }
    fn route_ctor(&mut self, task: u32, target: u32, start: u32, end: u32, final_value: u32, id: u32) -> u32 {
        self.0.route_ctor(task, target, end, start, final_value, id)
    }
    fn allocate_task(&mut self) -> u32 {
        self.0.allocate_task()
    }
    fn release_route(&mut self, table: u32) -> u32 {
        self.0.release_route(table)
    }
    fn base_destroy(&mut self, task: u32) -> u32 {
        self.0.base_destroy(task)
    }
}

/// The constructor with the flag word read before the move base runs: a wrong lift.
fn construct_flags_first(handle: u32, init: &RouteTaskInit, host: &mut RecHost) -> FollowPointRouteTask {
    let mut task = FollowPointRouteTask {
        handle,
        ..Default::default()
    };
    let base_flags = task.flags;
    host.move_base_ctor(&mut task, init.speed_bits);
    task.flags = ((init.include_end & 1) << 3)
        | ((init.route_flags & 1) << 2)
        | (init.include_start & 1)
        | (base_flags & FLAG_KEEP_MASK);
    task.current_route = init.route_tag.clone();
    task.waypoint_index = 0;
    host.route_ctor(handle, init.target, init.route_start, init.route_end, init.final_value, init.route_id);
    task
}

fn ctor_wrong(c: &CtorCase, selector_form: bool, this: u32, which: u32) -> Outcome {
    let init = ctor_init(c);
    match which {
        // The include-end and route-flag arguments are swapped.
        0 => {
            let mut init = init;
            core::mem::swap(&mut init.include_end, &mut init.route_flags);
            let mut host = RecHost::default();
            let task = if selector_form {
                FollowPointRouteTask::construct_from_selector(this, c.selector, init, &mut host)
            } else {
                FollowPointRouteTask::construct(this, init, &mut host)
            };
            Outcome {
                ret: this,
                calls: host.calls(),
                index: task.waypoint_index,
                flags: task.flags,
                out: vec![task.current_route.is_some() as u8],
                clobbered: false,
            }
        }
        // The flag word is read before the move base constructor runs.
        1 => {
            let mut host = RecHost::default();
            let task = construct_flags_first(this, &init, &mut host);
            Outcome {
                ret: this,
                calls: host.calls(),
                index: task.waypoint_index,
                flags: task.flags,
                out: vec![task.current_route.is_some() as u8],
                clobbered: false,
            }
        }
        // The two route arguments reach the route constructor in swapped order.
        _ => {
            let mut host = SwapRouteArgs(RecHost::default());
            let task = if selector_form {
                FollowPointRouteTask::construct_from_selector(this, c.selector, init, &mut host)
            } else {
                FollowPointRouteTask::construct(this, init, &mut host)
            };
            Outcome {
                ret: this,
                calls: host.0.calls(),
                index: task.waypoint_index,
                flags: task.flags,
                out: vec![task.current_route.is_some() as u8],
                clobbered: false,
            }
        }
    }
}

#[test]
fn constructors_match_rewrites() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0006);
    let mut tagged = 0usize;
    for _ in 0..TRIALS {
        let c = gen_ctor(&mut rng);
        for selector_form in [false, true] {
            let (rw, this) = ctor_rewrite(&c, selector_form);
            assert!(!rw.clobbered, "constructor wrote outside its fields: {c:?}");
            let lf = ctor_lifted(&c, selector_form, this);
            assert_eq!(rw, lf, "constructor differs (selector form {selector_form}) for {c:?}");
            if c.route_tag.is_some() {
                tagged += 1;
            }
        }
    }
    eprintln!("constructors: {TRIALS} trials per form; {tagged} with a route tag");
}

#[test]
fn constructor_wrong_lifts_are_caught() {
    let _g = guard();
    for which in 0..3u32 {
        let mut rng = Rng(0x5EED_0106 + which as u64);
        let mut caught = 0usize;
        for trial in 0..WRONG_TRIALS {
            let c = gen_ctor(&mut rng);
            let selector_form = trial % 2 == 0 && which != 1;
            let (rw, this) = ctor_rewrite(&c, selector_form);
            if ctor_wrong(&c, selector_form, this, which) != rw {
                caught += 1;
            }
        }
        eprintln!("constructor wrong lift {which}: caught in {caught} of {WRONG_TRIALS} trials");
        assert!(caught > 0, "wrong constructor lift {which} was not caught");
    }
}





// ---- Clone (verified rewrite fn_00caf2a0). ----

/// The allocator manager word the clone reads from its global.
const MGR_VALUE: u32 = 0x0BAD_F00D;
/// The global cell the clone's manager read lands on: the relocation base is
/// set so that the clone's global address maps to this cell.
static mut MGR_CELL: u32 = MGR_VALUE;
/// The file address of the allocator global the clone reads.
const ALLOCATOR_GLOBAL_VA: u32 = 0x0167_E2A0;

thread_local! {
    static NEW_OBJ: RefCell<Option<Vec<u32>>> = const { RefCell::new(None) };
    static NULL_ALLOC: Cell<bool> = const { Cell::new(false) };
}

/// The allocation callee: answers a fresh object, or none when the case
/// says the allocator is empty.
extern "thiscall" fn alloc_stub(mgr: u32) -> u32 {
    record(Ev::Alloc { mgr });
    if NULL_ALLOC.with(|n| n.get()) {
        return 0;
    }
    let v = obj_words();
    let a = addr(&v);
    NEW_OBJ.with(|n| *n.borrow_mut() = Some(v));
    a
}

/// The constructor callee of the clone: runs the verified constructor on the
/// new object. Its own two callees (the move base and the route) are swapped
/// in for the duration of the call and the clone's entries are restored after.
extern "thiscall" fn ctor_proxy(
    obj: u32,
    speed: u32,
    target: u32,
    tag: u32,
    start: u32,
    end: u32,
    include_start: u32,
    include_end: u32,
    route_flags: u32,
    id: u32,
    final_value: u32,
) -> u32 {
    install(&[
        (CALLEE_1, base_stub as *const () as usize as u32),
        (CALLEE_2, route6_stub as *const () as usize as u32),
    ]);
    let r = fn_00cabca0::rw_00cabca0(
        obj,
        speed,
        target,
        tag,
        start,
        end,
        include_start,
        include_end,
        route_flags,
        id,
        final_value,
    );
    install(&[
        (CALLEE_1, alloc_stub as *const () as usize as u32),
        (CALLEE_2, ctor_proxy as *const () as usize as u32),
    ]);
    r
}

#[derive(Debug, Clone)]
struct CloneCase {
    speed: u32,
    target: u32,
    route_start: u32,
    route_end: u32,
    final_value: u32,
    tag: Option<Vec<[u32; 3]>>,
    flags: u32,
    index: u32,
    null_alloc: bool,
}

fn gen_clone(rng: &mut Rng) -> CloneCase {
    let n = rng.below(4) as usize;
    let tag = if rng.chance(3, 4) {
        Some((0..n).map(|_| point(rng)).collect())
    } else {
        None
    };
    CloneCase {
        speed: rng.u32(),
        target: rng.u32(),
        route_start: rng.u32(),
        route_end: rng.u32(),
        final_value: rng.u32(),
        tag,
        flags: rng.u32(),
        index: rng.u32(),
        null_alloc: rng.chance(1, 8),
    }
}

/// Points the clone's global read at the manager cell.
fn set_manager_global() {
    let cell = core::ptr::addr_of!(MGR_CELL) as usize as u32;
    let xbase = cell.wrapping_sub(ALLOCATOR_GLOBAL_VA.wrapping_sub(0x40_0000));
    // SAFETY: the relocation base is only read by the rewrites' global and
    // relocated-address helpers during a call; the lock serialises the cases.
    unsafe { lf_checker_rt::CHECKER_XBASE = xbase };
    // SAFETY: the cell is a plain static word; it is written before each call.
    unsafe { core::ptr::addr_of_mut!(MGR_CELL).write(MGR_VALUE) };
}

/// The source task's memory for the clone: the constructor words at their
/// offsets, the table word, the index and the flags.
fn clone_source(c: &CloneCase, tag_word: u32) -> Vec<u32> {
    let mut src = obj_words();
    set_word(&mut src, 0x18, c.speed);
    set_word(&mut src, OFF_WAYPOINTS, c.target);
    set_word(&mut src, OFF_CURRENT, tag_word);
    set_word(&mut src, 0x2C, c.route_start);
    set_word(&mut src, 0x30, c.route_end);
    set_word(&mut src, 0x28, c.final_value);
    set_word(&mut src, OFF_INDEX, c.index);
    set_word(&mut src, OFF_FLAGS, c.flags);
    src
}

fn clone_rewrite(c: &CloneCase) -> (Outcome, u32) {
    let tag = c.tag.as_ref().map(|p| route_words(p));
    let tag_word = tag.as_ref().map_or(0, |v| addr(v));
    let src = clone_source(c, tag_word);
    let src_before = src.clone();
    set_manager_global();
    NULL_ALLOC.with(|n| n.set(c.null_alloc));
    NEW_OBJ.with(|n| *n.borrow_mut() = None);
    install(&[
        (CALLEE_1, alloc_stub as *const () as usize as u32),
        (CALLEE_2, ctor_proxy as *const () as usize as u32),
    ]);
    let _ = take_calls();
    let this = addr(&src);
    let ret = fn_00caf2a0::rw_00caf2a0(this);
    let calls = take_calls();
    let new = NEW_OBJ.with(|n| n.borrow_mut().take());
    let fresh = obj_words();
    let written = [OFF_VTABLE / 4, OFF_BASE_VTABLE / 4, OFF_CURRENT / 4, OFF_INDEX / 4, OFF_FLAGS / 4];
    let (index, flags, tag_present, new_clobbered) = match &new {
        Some(nw) => (
            nw[OFF_INDEX / 4],
            nw[OFF_FLAGS / 4],
            (nw[OFF_CURRENT / 4] != 0) as u8,
            nw.iter().enumerate().any(|(i, w)| !written.contains(&i) && *w != fresh[i]),
        ),
        None => (0, 0, 0, false),
    };
    let clobbered = src != src_before || new_clobbered;
    (
        Outcome {
            ret,
            calls,
            index,
            flags,
            out: vec![tag_present],
            clobbered,
        },
        this,
    )
}

fn clone_outcome(c: &CloneCase, this: u32, new_handle: u32, which: Option<u32>) -> Outcome {
    let mut task = FollowPointRouteTask {
        handle: this,
        speed_bits: c.speed,
        route_target: c.target,
        route_start: c.route_start,
        route_end: c.route_end,
        final_value: c.final_value,
        current_route: c.tag.as_ref().map(|p| RouteTable::new(to_points(p))),
        waypoint_index: c.index,
        flags: c.flags,
        ..Default::default()
    };
    match which {
        // The include-end and route-flag bits are read from each other's place.
        Some(0) => {
            let f = task.flags;
            let b2 = (f >> 2) & 1;
            let b3 = (f >> 3) & 1;
            task.flags = (f & !0xC) | (b2 << 3) | (b3 << 2);
        }
        // The speed and the route target are copied the wrong way round.
        Some(_) => core::mem::swap(&mut task.speed_bits, &mut task.route_target),
        None => {}
    }
    let mut host = RecHost {
        mgr: MGR_VALUE,
        new_handle: if c.null_alloc { 0 } else { new_handle },
        ..Default::default()
    };
    let r = task.clone_task(&mut host);
    let (ret, index, flags, tag) = match &r {
        None => (0, 0, 0, false),
        Some(t) => (t.handle, t.waypoint_index, t.flags, t.current_route.is_some()),
    };
    Outcome {
        ret,
        calls: host.calls(),
        index,
        flags,
        out: vec![tag as u8],
        clobbered: false,
    }
}

#[test]
fn clone_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0007);
    let (mut nulls, mut built) = (0usize, 0usize);
    for _ in 0..TRIALS {
        let c = gen_clone(&mut rng);
        let (rw, this) = clone_rewrite(&c);
        assert!(!rw.clobbered, "clone wrote outside the words it should: {c:?}");
        let lf = clone_outcome(&c, this, rw.ret, None);
        assert_eq!(rw, lf, "clone differs for {c:?}");
        if rw.ret == 0 {
            nulls += 1;
        } else {
            built += 1;
        }
    }
    eprintln!("clone: {TRIALS} trials; {built} built a copy, {nulls} with a null allocation");
    assert!(nulls > 0 && built > 0);
}

#[test]
fn clone_wrong_lift_is_caught() {
    let _g = guard();
    for which in 0..2u32 {
        let mut rng = Rng(0x5EED_0107 + which as u64);
        let mut caught = 0usize;
        for _ in 0..WRONG_TRIALS {
            let c = gen_clone(&mut rng);
            let (rw, this) = clone_rewrite(&c);
            if clone_outcome(&c, this, rw.ret, Some(which)) != rw {
                caught += 1;
            }
        }
        eprintln!("clone wrong lift {which}: caught in {caught} of {WRONG_TRIALS} trials");
        assert!(caught > 0, "wrong clone lift {which} was not caught");
    }
}


// ---- Destructor (verified rewrite fn_00cace90). ----

/// The shared release context the destructor reads from its global.
const CTX_VALUE: u32 = 0x0C0F_FEE5;
static mut CTX_CELL: u32 = CTX_VALUE;
/// The file address of the release-context global the destructor reads.
const CONTEXT_GLOBAL_VA: u32 = 0x0179_D114;

/// The release callee: answers for the context and the table word.
extern "thiscall" fn release_stub(ctx: u32, table: u32) -> u32 {
    record(Ev::Release { ctx, table });
    ans_release(ctx, table)
}

/// The move base's destructor callee.
extern "thiscall" fn base_dtor_stub(this: u32) -> u32 {
    record(Ev::BaseDtor { task: this });
    ans_base_dtor(this)
}

/// Points the destructor's global read at the context cell.
fn set_context_global() {
    let cell = core::ptr::addr_of!(CTX_CELL) as usize as u32;
    let xbase = cell.wrapping_sub(CONTEXT_GLOBAL_VA.wrapping_sub(0x40_0000));
    // SAFETY: the relocation base is read only during a rewrite call; the lock serialises the cases.
    unsafe { lf_checker_rt::CHECKER_XBASE = xbase };
    // SAFETY: the cell is a plain static word, written before each call.
    unsafe { core::ptr::addr_of_mut!(CTX_CELL).write(CTX_VALUE) };
}

fn dtor_rewrite(table_word: u32) -> (Outcome, u32) {
    let mut obj = obj_words();
    set_word(&mut obj, OFF_WAYPOINTS, table_word);
    let before = obj.clone();
    set_context_global();
    install(&[
        (CALLEE_1, base_dtor_stub as *const () as usize as u32),
        (CALLEE_2, release_stub as *const () as usize as u32),
    ]);
    let _ = take_calls();
    let this = addr(&obj);
    let ret = fn_00cace90::rw_00cace90(this);
    let calls = take_calls();
    let written = [OFF_VTABLE / 4, OFF_BASE_VTABLE / 4];
    let clobbered = obj
        .iter()
        .enumerate()
        .any(|(i, w)| !written.contains(&i) && *w != before[i]);
    assert_eq!(obj[OFF_VTABLE / 4], lf_checker_rt::relocated(TASK_VTABLE_VA));
    assert_eq!(obj[OFF_BASE_VTABLE / 4], lf_checker_rt::relocated(BASE_VTABLE_VA));
    (
        Outcome {
            ret,
            calls,
            index: 0,
            flags: 0,
            out: Vec::new(),
            clobbered,
        },
        this,
    )
}

fn dtor_lifted(table_word: u32, this: u32) -> Outcome {
    let mut task = FollowPointRouteTask {
        handle: this,
        route_target: table_word,
        ..Default::default()
    };
    let mut host = RecHost {
        ctx: CTX_VALUE,
        ..Default::default()
    };
    let ret = task.destroy(&mut host);
    Outcome {
        ret,
        calls: host.calls(),
        index: 0,
        flags: 0,
        out: Vec::new(),
        clobbered: false,
    }
}

/// Deliberately wrong destructors: the release runs even for a zero table
/// word (0), or the base destructor runs before the release (1).
fn dtor_wrong(table_word: u32, this: u32, which: u32) -> Outcome {
    let task = FollowPointRouteTask {
        handle: this,
        route_target: table_word,
        ..Default::default()
    };
    let mut host = RecHost {
        ctx: CTX_VALUE,
        ..Default::default()
    };
    let ret = if which == 0 {
        host.release_route(task.route_target);
        host.base_destroy(task.handle)
    } else {
        let r = host.base_destroy(task.handle);
        if task.route_target != 0 {
            host.release_route(task.route_target);
        }
        r
    };
    Outcome {
        ret,
        calls: host.calls(),
        index: 0,
        flags: 0,
        out: Vec::new(),
        clobbered: false,
    }
}

#[test]
fn destructor_matches_rewrite() {
    let _g = guard();
    let mut rng = Rng(0x5EED_0008);
    let (mut released, mut kept) = (0usize, 0usize);
    for _ in 0..TRIALS {
        let table_word = if rng.chance(1, 4) { 0 } else { rng.u32() | 1 };
        let (rw, this) = dtor_rewrite(table_word);
        assert!(!rw.clobbered, "destructor wrote outside the vtable words");
        let lf = dtor_lifted(table_word, this);
        assert_eq!(rw, lf, "destructor differs for table word {table_word:#x}");
        if table_word != 0 {
            released += 1;
        } else {
            kept += 1;
        }
    }
    eprintln!("destructor: {TRIALS} trials; {released} released a table, {kept} with none");
}

#[test]
fn destructor_wrong_lift_is_caught() {
    let _g = guard();
    for which in 0..2u32 {
        let mut rng = Rng(0x5EED_0108 + which as u64);
        let mut caught = 0usize;
        for _ in 0..WRONG_TRIALS {
            let table_word = if rng.chance(1, 4) { 0 } else { rng.u32() | 1 };
            let (rw, this) = dtor_rewrite(table_word);
            if dtor_wrong(table_word, this, which) != rw {
                caught += 1;
            }
        }
        eprintln!("destructor wrong lift {which}: caught in {caught} of {WRONG_TRIALS} trials");
        assert!(caught > 0, "wrong destructor lift {which} was not caught");
    }
}
