//! The follow-point route task: the complex move task that walks a ped along
//! a route of points, one leg at a time.
//!
//! The fields are the state the verified methods read or write. The
//! constructor and the clone and destructor paths are not lifted yet; the
//! task is built from its state record, which is how the proofs set it up.

use super::host::{PedView, RouteTaskHost};
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
