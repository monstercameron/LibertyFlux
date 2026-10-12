//! The follow-point route task: the complex move task that walks a ped along
//! a route of points, one leg at a time.
//!
//! The fields are the state the verified methods read or write. The
//! constructors, the clone, the destructor and the route-advance poll are
//! all here.

use super::host::{PedView, RoutePollHost, RouteTaskHost};
use super::route::{RoutePoint, RouteTable};

/// Sub-task type id of the route itself: the sub-task that keeps running.
pub const SUBTASK_ROUTE: u32 = 0x516;
/// Sub-task type id of a finished leg; also the order id that ends a leg.
pub const SUBTASK_LEG_DONE: u32 = 0x11c;
/// Sub-task type id of a finished go-to.
pub const SUBTASK_GO_TO_DONE: u32 = 0x11a;
/// Sub-task type id of a finished turn.
pub const SUBTASK_TURN_DONE: u32 = 0x384;
/// Order id that ends a leg, given when the waypoint index has run past the
/// route's points. Same value as [`SUBTASK_LEG_DONE`].
pub const ORDER_LEG_DONE: u32 = 0x11c;

/// Flag bit 1: cleared whenever a waypoint is picked.
pub const FLAG_ROUTE_SEARCH: u32 = 0x2;
/// Flag bit 2: when set, the route repeats instead of ending at the last
/// waypoint.
pub const FLAG_REPEAT: u32 = 0x4;
/// Ped state bit 2: when set, the ped may not follow a route.
pub const PED_BLOCKED: u8 = 0x4;

/// Type-query answer selecting the sub-task's first slot.
pub const SUB_QUERY_A: u32 = 0x1;
/// Type-query answer selecting the sub-task's second slot.
pub const SUB_QUERY_B: u32 = 0x38;
/// Owner flag bit 0: the owner is finished and the poll returns at once.
pub const OWNER_FLAG_DONE: u32 = 0x1;
/// Owner flag bit 1: set when the poll advanced the route.
pub const OWNER_FLAG_ADVANCED: u32 = 0x2;
/// Squared length under which a slot's point counts as reached.
pub const REACHED_LEN_SQ: f32 = 0.015625;

/// Stands in for "no segment chosen" in the waypoint search.
const NO_SEGMENT: i32 = -1;

/// The state of a follow-point route task that its verified methods read
/// or write. Every field is owned Rust data; the opaque handles are the
/// words the host uses to name the task, its sub-task and the ped.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FollowPointRouteTask {
    /// Opaque handle the host knows this task by.
    pub handle: u32,
    /// Opaque handle of the current sub-task (never zero while the task runs).
    pub subtask: u32,
    /// The current route's points. `None` is the null word: the ped is not
    /// moving along a route.
    pub current_route: Option<RouteTable>,
    /// The waypoint route the task walks.
    pub waypoints: RouteTable,
    /// The route cursor word: the index of the current point, and the
    /// task handle word that the handle copy reads.
    pub route_cursor: u32,
    /// The index of the waypoint being walked; -1 as a word when none was
    /// chosen.
    pub waypoint_index: u32,
    /// The task's flag word.
    pub flags: u32,
    /// The move speed word the constructor was given (copied by the clone).
    pub speed_bits: u32,
    /// The route target word the constructor was given, as the clone copies
    /// it from the task's table word.
    pub route_target: u32,
    /// The first route argument the constructor was given.
    pub route_start: u32,
    /// The second route argument the constructor was given.
    pub route_end: u32,
    /// The final value the constructor was given.
    pub final_value: u32,
}

// Each arithmetic step is a separate call with its operands fixed in
// order, so the compiler cannot swap them. The float results (and NaN
// payloads in particular) then match the verified rewrite's order exactly.
#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

impl FollowPointRouteTask {
    /// Writes the route cursor word into the handle slot of an output
    /// record. The record must hold at least the slot's four bytes; the
    /// verified method writes the same four bytes and no others.
    ///
    /// # Panics
    ///
    /// When the record is shorter than the handle slot.
    pub fn copy_handle_into(&self, record: &mut [u8]) {
        record[HANDLE_RECORD_OFFSET..HANDLE_RECORD_OFFSET + 4]
            .copy_from_slice(&self.route_cursor.to_le_bytes());
    }

    /// The current route's point at the cursor.
    ///
    /// # Panics
    ///
    /// When there is no current route, or the cursor is not below its
    /// length. The original faults on a null route; the lift refuses to.
    #[must_use]
    pub fn current_point(&self) -> RoutePoint {
        self.current_route
            .as_ref()
            .expect("follow-point task has no current route")
            .point(self.route_cursor as usize)
    }

    /// The waypoint at `index`.
    ///
    /// # Panics
    ///
    /// When `index` is not below the waypoint count.
    #[must_use]
    pub fn waypoint(&self, index: u32) -> RoutePoint {
        self.waypoints.point(index as usize)
    }

    /// Picks the waypoint the ped should head for, then asks the task system
    /// for the matching order. Returns the task system's answer.
    ///
    /// Clears [`FLAG_ROUTE_SEARCH`]. When the stored index already equals
    /// the waypoint count, it orders the leg-done order and stops. When the
    /// ped's state byte blocks route following, it returns 0. When there is
    /// no current route, it orders the next order without a search. Else it
    /// scans every segment from point `i` to point `(i + 1) % count`, keeping
    /// the closest projection that lies strictly inside a segment; when none
    /// qualifies, it keeps the nearest route point. The winner (or -1 when
    /// the count is not positive) is stored as the waypoint index.
    pub fn pick_waypoint<H: RouteTaskHost>(&mut self, ped: &PedView, host: &mut H) -> u32 {
        self.flags &= !FLAG_ROUTE_SEARCH;
        let count = self.waypoints.len() as i32;
        if count == self.waypoint_index as i32 {
            return host.order_ped(self.handle, ORDER_LEG_DONE, ped.handle);
        }
        if ped.state_byte & PED_BLOCKED != 0 {
            return 0;
        }
        if self.current_route.is_none() {
            let next = host.next_order_id(self.handle);
            return host.order_ped(self.handle, next, ped.handle);
        }

        let pos = ped.position;
        let mut best_next: i32 = NO_SEGMENT;
        let mut best = f32::MAX;
        if count > 0 {
            // Closest projection strictly inside a segment. `idx` is always
            // below `count`, so the current point is `idx` itself.
            for idx in 0..count {
                let next = (idx + 1) % count;
                let cur = idx;
                let a = self.waypoints.point(cur as usize);
                let b = self.waypoints.point(next as usize);
                let (dx, dy, dz) = (sub(b.x, a.x), sub(b.y, a.y), sub(b.z, a.z));
                let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                let seglen = core::hint::black_box(len2).sqrt();
                // A zero-length segment takes a zero direction, not a
                // reciprocal of zero. A not-a-number squared length still
                // takes the reciprocal, as the verified method does.
                let inv = if len2 > 0.0 || len2.is_nan() {
                    div(1.0, seglen)
                } else {
                    0.0
                };
                let (nx, ny, nz) = (mul(dx, inv), mul(dy, inv), mul(dz, inv));
                let rx = sub(pos.x, a.x);
                let ry = sub(pos.y, a.y);
                let rz = sub(pos.z, a.z);
                let t = add(add(mul(rx, nx), mul(ry, ny)), mul(rz, nz));
                if t > 0.0 && seglen > t {
                    let px = add(a.x, mul(nx, t));
                    let py = add(a.y, mul(ny, t));
                    let pz = add(a.z, mul(nz, t));
                    let ex = sub(pos.x, px);
                    let ey = sub(pos.y, py);
                    let ez = sub(pos.z, pz);
                    let d2 = add(add(mul(ey, ey), mul(ex, ex)), mul(ez, ez));
                    if mul(best, best) > d2 {
                        best_next = next;
                        best = d2;
                    }
                }
            }
            best = f32::MAX;
        }
        if best_next == NO_SEGMENT && count > 0 {
            // No projection qualified: the nearest route point wins.
            for e in 0..count {
                let p = self.waypoints.point(e as usize);
                let dx = sub(pos.x, p.x);
                let dy = sub(pos.y, p.y);
                let dz = sub(pos.z, p.z);
                let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if mul(best, best) > d2 {
                    best_next = e;
                    best = d2;
                }
            }
        }
        self.waypoint_index = best_next as u32;
        let next = host.next_order_id(self.handle);
        host.order_ped(self.handle, next, ped.handle)
    }

    /// Steps the route on when the current sub-task has finished.
    ///
    /// The sub-task's type decides. The route itself keeps running (0). A
    /// finished leg or a finished go-to starts the next route leg. A finished
    /// turn starts the next leg only past the last waypoint without the
    /// repeat flag; otherwise it advances the waypoint index. Any other
    /// type advances the index. Creation goes through the host's factory,
    /// with the type id and the ped handle.
    pub fn advance_after_subtask<H: RouteTaskHost>(&mut self, ped: u32, host: &mut H) -> u32 {
        // The verified method may query the type up to four times; the
        // query changes nothing, so the lift asks once.
        let kind = host.subtask_type(self.subtask);
        if kind == SUBTASK_ROUTE {
            return 0;
        }
        if kind == SUBTASK_LEG_DONE || kind == SUBTASK_GO_TO_DONE {
            return host.create_subtask(self.handle, SUBTASK_ROUTE, ped);
        }
        if kind == SUBTASK_TURN_DONE
            && self.waypoint_index.wrapping_add(1) == self.waypoints.len() as u32
            && self.flags & FLAG_REPEAT == 0
        {
            return host.create_subtask(self.handle, SUBTASK_ROUTE, ped);
        }
        self.waypoint_index = self.waypoint_index.wrapping_add(1);
        let next = host.plan_next_subtask(self.handle);
        host.create_subtask(self.handle, next, ped)
    }
}

/// Byte offset of the handle slot in an output record.
pub const HANDLE_RECORD_OFFSET: usize = 0x38;

/// The arguments of the route-task constructor, as the verified constructors
/// receive them. Only bit 0 of the three flag arguments is used.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RouteTaskInit {
    /// The move speed, as the bits of a single-precision float.
    pub speed_bits: u32,
    /// The route target: an opaque word the route constructor receives.
    pub target: u32,
    /// The route the task follows. `None` is the null word stored as the
    /// current route.
    pub route_tag: Option<RouteTable>,
    /// The first route argument, passed to the route constructor.
    pub route_start: u32,
    /// The second route argument, passed to the route constructor.
    pub route_end: u32,
    /// Its bit 0 sets flag bit 0.
    pub include_start: u32,
    /// Its bit 0 sets flag bit 3.
    pub include_end: u32,
    /// Its bit 0 sets flag bit 2.
    pub route_flags: u32,
    /// The route id, passed to the route constructor.
    pub route_id: u32,
    /// The final value, passed to the route constructor.
    pub final_value: u32,
}

impl FollowPointRouteTask {
    /// The route-task constructor. The move base is built first, then the
    /// flag word is read back from the base (its bits 0, 2 and 3 are replaced
    /// by the arguments' bits; all other bits are kept), then the route
    /// table is installed, the waypoint index is cleared, and the route
    /// constructor runs with the target and the route arguments.
    #[must_use]
    pub fn construct<H: RouteTaskHost>(handle: u32, init: RouteTaskInit, host: &mut H) -> Self {
        let mut task = Self {
            handle,
            ..Self::default()
        };
        host.move_base_ctor(&mut task, init.speed_bits);
        let base_flags = task.flags;
        task.flags = ((init.include_end & 1) << 3)
            | ((init.route_flags & 1) << 2)
            | (init.include_start & 1)
            | (base_flags & FLAG_KEEP_MASK);
        task.current_route = init.route_tag;
        task.waypoint_index = 0;
        task.speed_bits = init.speed_bits;
        task.route_target = init.target;
        task.route_start = init.route_start;
        task.route_end = init.route_end;
        task.final_value = init.final_value;
        host.route_ctor(
            handle,
            init.target,
            init.route_start,
            init.route_end,
            init.final_value,
            init.route_id,
        );
        task
    }

    /// The destructor: releases the route table word the task owns when it
    /// is not zero, then runs the move base's destructor and returns its
    /// answer. The vtable words the original writes are the class identity
    /// and are not represented here.
    pub fn destroy<H: RouteTaskHost>(&mut self, host: &mut H) -> u32 {
        if self.route_target != 0 {
            host.release_route(self.route_target);
        }
        host.base_destroy(self.handle)
    }

    /// The route-advance poll. `a0` and `a1` are the words the owner calls
    /// receive; `a2` is the current sub-task, or zero.
    ///
    /// With a sub-task and a step (the waypoint index plus one) below the
    /// route count, the sub-task's type picks one of its slots. When the
    /// owner gate is open, the owner's next-slot object is present and the
    /// slot's point lies within the reach length, the helper decides, and
    /// on success the owner's finish call runs. Then a zero `a1` clears the
    /// route count. A finished owner answers 1; otherwise the owner's poll
    /// decides, and on success the owner's advanced flag is set and the
    /// answer is 1, else 0. The owner is the task's `subtask` word.
    pub fn poll<H: RoutePollHost>(&mut self, a0: u32, a1: u32, a2: u32, host: &mut H) -> u32 {
        let owner = self.subtask;
        if a2 != 0 {
            let step = self.waypoint_index.wrapping_add(1);
            if (step as i32) < (self.waypoints.len() as i32) {
                let query = host.subtask_query(a2);
                let slot = match query {
                    SUB_QUERY_A => host.subtask_slot_a(a2),
                    SUB_QUERY_B => host.subtask_slot_b(a2),
                    _ => 0,
                };
                if (host.owner_gate(owner) & 0xFF) != 0 {
                    let slotb = host.owner_next(owner);
                    if slotb != 0 && slot != 0 {
                        let p = host.slot_point(slot);
                        let lensq = add(add(mul(p.x, p.x), mul(p.y, p.y)), mul(p.z, p.z));
                        if REACHED_LEN_SQ > lensq {
                            let fv = host.slotb_float(slotb);
                            let iv = host.slotb_int(slotb);
                            let ok = host.helper_ok(slot, iv, fv.to_bits()) & 0xFF;
                            if ok != 0 {
                                host.owner_finish(owner, a0, 0, a2);
                            }
                        }
                    }
                }
            }
        }
        if a1 == 0 {
            self.waypoints.clear();
        }
        if (host.owner_flags(owner) & OWNER_FLAG_DONE) == 0 {
            if (host.owner_poll(owner, a0, a1, a2) & 0xFF) != 0 {
                let flags = host.owner_flags(owner);
                host.set_owner_flags(owner, flags | OWNER_FLAG_ADVANCED);
                return 1;
            }
            return 0;
        }
        1
    }

    /// The clone: allocates a new task through the host and constructs it
    /// from this task's words. The flag bits the constructor takes are read
    /// back from this task's flag word (bit 0 as the start include, bit 3 as
    /// the end include, bit 2 as the route flag). Returns the new task, or
    /// `None` when the allocator gives nothing; the new task's handle is the
    /// allocation's answer.
    #[must_use]
    pub fn clone_task<H: RouteTaskHost>(&self, host: &mut H) -> Option<Self> {
        let handle = host.allocate_task();
        if handle == 0 {
            return None;
        }
        let init = RouteTaskInit {
            speed_bits: self.speed_bits,
            target: self.route_target,
            route_tag: self.current_route.clone(),
            route_start: self.route_start,
            route_end: self.route_end,
            include_start: self.flags & 1,
            include_end: (self.flags >> 3) & 1,
            route_flags: (self.flags >> 2) & 1,
            route_id: self.waypoint_index,
            final_value: self.final_value,
        };
        Some(Self::construct(handle, init, host))
    }

    /// The constructor of the selector form: the move speed comes from the
    /// speed helper, called before the move base, and the rest is as in
    /// [`FollowPointRouteTask::construct`]. The `speed_bits` of `init` is
    /// ignored.
    #[must_use]
    pub fn construct_from_selector<H: RouteTaskHost>(
        handle: u32,
        selector: u32,
        mut init: RouteTaskInit,
        host: &mut H,
    ) -> Self {
        init.speed_bits = host.select_speed(selector).to_bits();
        Self::construct(handle, init, host)
    }
}

/// The flag bits the constructor keeps from the move base: every bit except
/// bits 0, 2 and 3.
pub const FLAG_KEEP_MASK: u32 = 0xffff_fff2;
