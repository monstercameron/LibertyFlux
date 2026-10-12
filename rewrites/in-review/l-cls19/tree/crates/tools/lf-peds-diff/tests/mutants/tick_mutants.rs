//! Wrong versions of the lifted scenario tick, for the differential test.
//!
//! Generated from the lifted tick body by a script: the same code, with one
//! mutation point per wrong version. `K` selects the version; `K == 0` is the
//! lifted code unchanged. The wrapper stores the scenario so the body reads its
//! fields through `Deref`, as the lifted body reads them through `self`.

use core::ops::{Deref, DerefMut};

/// The scenario, wrapped so the generated body can name its own methods.
pub struct Mut<const K: u32>(pub Scen);

impl<const K: u32> Deref for Mut<K> {
    type Target = Scen;

    fn deref(&self) -> &Scen {
        &self.0
    }
}

impl<const K: u32> DerefMut for Mut<K> {
    fn deref_mut(&mut self) -> &mut Scen {
        &mut self.0
    }
}

use core::hint::black_box;

use lf_peds_tasks::move_between_points::{MoveBetweenPointsScenario as Scen, STATUS_DONE, STATUS_STICKY, TickHost, WorldQuery};

/// Auxiliary identifier that selects the chat variant of the scenario.
pub const AUX_CHAT: u32 = 0x1D;
/// Auxiliary identifier that selects the narrow weight table.
pub const AUX_NARROW: u32 = 0x1E;
/// Lower bound of the randomised wait, as a fraction of the configured span (0.8).
const WAIT_LO: f32 = f32::from_bits(0x3F4C_CCCD);
/// Upper bound of the randomised wait, as a fraction of the configured span (1.2).
const WAIT_HI: f32 = f32::from_bits(0x3F99_999A);
/// Scale that maps a random draw into [0, 1) (about 2^-15).
const RAND_SCALE: f32 = f32::from_bits(0x3800_0100);
/// Squared distance beyond which a sampled destination is committed (4.0).
const FAR_SQUARED: f32 = f32::from_bits(0x4080_0000);
/// Full weight passed to the second route call (1.0, as its bits).
const FULL_WEIGHT_BITS: u32 = 0x3F80_0000;
/// The idle sentinel (-1.0, as its bits), passed to the dock calls.
const NEG_ONE_BITS: u32 = 0xBF80_0000;
/// Flag words of the first route call: the one after the weight, the one after the output words, and the one after the pedestrian.
const ROUTE_FIRST_FLAGS: [u32; 3] = [1, 1, 1];
/// Flag words of the second route call: the one after the weight, the one after the output words, and the zero after the pedestrian.
const ROUTE_SECOND_FLAGS: [u32; 3] = [1, 1, 0];
/// The boarding call's constant words in the order they are pushed, with the gate word and the fare pair left out.
const BOARD_FLAGS: [u32; 7] = [2, 0xFFFF_FFFF, 1, 0, 0, 0, 1];
/// Trailing argument words of the seat-information call.
const SEAT_TAIL: [u32; 2] = [0, 0];
/// The argument words of the dock call of the move leg and of the heading seat:
/// zero, minus one (as bits), three zeros and 0xc.
const DOCK_ARGS: [u32; 6] = [0, NEG_ONE_BITS, 0, 0, 0, 0xC];
/// The chat ping weight (0.05, as its bits).
const CHAT_PING_WEIGHT_BITS: u32 = 0x3D4C_CCCD;

/// Multiplies with the operands pinned in the order the original gives them.
#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    black_box(a) * black_box(b)
}

/// Adds with the operands pinned in the order the original gives them.
#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    black_box(a) + black_box(b)
}

/// Subtracts with the operands pinned in the order the original gives them.
#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    black_box(a) - black_box(b)
}

impl<const K: u32> Mut<K> {
    /// One tick of the scenario. Returns the word the pedestrian runs this frame: the
    /// current sub-task, or, on the state-move paths, the anchor of the routed leg.
    ///
    /// The tick first marks the pedestrian as scenario-driven. When the scenario is
    /// active and not waiting on its guard, it re-polls its sub-operation for kinds
    /// one and two and returns zero when one accepts (recording it in the status).
    /// It then resolves the configuration and dispatches on the state: zero and one
    /// have their own paths, any other state falls through to the tail, which pings
    /// the pedestrian for the chat variant and returns the sub-task.
    pub fn tick<H: TickHost>(&mut self, host: &mut H, ped: u32) -> u32 {
        host.set_scenario_flag(ped);
        if self.active != 0 && (K == 1 || self.pending_guard == 0) {
            if self.status & STATUS_STICKY != 0 {
                return 0;
            }
            if host.sub_operation(self, ped, 1) & 0xff != 0 {
                self.status |= STATUS_DONE;
                return 0;
            }
            if self.status & STATUS_STICKY != 0 {
                return 0;
            }
            if host.sub_operation(self, ped, if K == 2 { 1 } else { 2 }) & 0xff != 0 {
                self.status |= STATUS_DONE;
                return 0;
            }
        }
        let config = host.config(self.aux_id);
        let outcome = match self.state {
            0 => if K == 4 { self.move_state(host, ped) } else { self.wait_state(host, ped, config) },
            1 => self.move_state(host, ped),
            _ => None,
        };
        match outcome {
            Some(answer) => answer,
            None => self.tail_self(host, ped),
        }
    }

    /// The tail on this scenario: the chat ping when the auxiliary word selects it,
    /// then the sub-task.
    fn tail_self<H: TickHost>(&self, host: &mut H, ped: u32) -> u32 {
        if self.aux_id == if K == 5 { AUX_NARROW } else { AUX_CHAT } {
            host.chat_ping(ped, CHAT_PING_WEIGHT_BITS);
        }
        self.sub_task
    }

    /// State zero: counts the wait timer down. On expiry it draws a new wait between
    /// 0.8 and 1.2 of the configured span, samples a destination, and commits it
    /// (moving to state one) when it lies beyond the threshold from the pedestrian.
    /// Returns the function result when the state decides it, else `None`.
    fn wait_state<H: TickHost>(&mut self, host: &mut H, ped: u32, config: u32) -> Option<u32> {
        let step = host.frame_step();
        let left = sub(self.wait_timer, step);
        self.wait_timer = left;
        if !(0.0 > left) {
            return None;
        }
        let span = host.config_span(config);
        let s0 = mul(span, WAIT_LO);
        let s1 = mul(span, WAIT_HI);
        let draw = host.random();
        let width = sub(s1, s0);
        let frac = mul(draw as i32 as f32, RAND_SCALE);
        self.wait_timer = add(mul(frac, if K == 8 { s0 } else { width }), if K == 8 { width } else { s0 });
        let narrow = self.aux_id == AUX_NARROW;
        let weight = host.weight_bits(narrow);
        let base = host.placement_base(ped);
        let route = host.route_first(base, weight, ROUTE_FIRST_FLAGS, ped);
        if route.ok & 0xff == 0 {
            return None;
        }
        let leg_anchor = route.anchor;
        let dest = host.refine_first();
        let [px, py, pz] = host.position(base);
        let dx = sub(f32::from_bits(dest[0]), px);
        let dy = sub(f32::from_bits(dest[1]), py);
        let dz = sub(f32::from_bits(dest[2]), pz);
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        if if K == 3 { dist2 < FAR_SQUARED } else { dist2 <= FAR_SQUARED } {
            return None;
        }
        self.point = dest;
        let group = host.ped_group(ped);
        let seated = host.group_seat(group);
        let anchor_word = host.intel_anchor(ped);
        let leg = host.commit_route(base, seated | anchor_word);
        host.set_anchor_leg(leg_anchor, leg);
        if leg != 0 {
            host.set_anchor_state(leg_anchor, 1);
        }
        Some(tail_anchor(host, leg_anchor, ped))
    }

    /// State one: validates the leg, schedules and routes it, resolves the world
    /// anchor and seats the pedestrian. Returns the function result when the state
    /// decides it, else `None`.
    fn move_state<H: TickHost>(&mut self, host: &mut H, ped: u32) -> Option<u32> {
        let leg_ok = host.leg_ok(self.payload);
        if leg_ok & 0xff == if K == 10 { 1 } else { 0 } {
            return None;
        }
        let leg = self.payload;
        let planned = host.schedule(leg);
        host.schedule_use(leg);
        self.payload = 0;
        if planned != 0 {
            self.state = 0;
            return None;
        }
        let routed = host.route_second(ped, FULL_WEIGHT_BITS, ROUTE_SECOND_FLAGS);
        if routed.ok & 0xff == 0 {
            self.state = 0;
            return None;
        }
        let leg_anchor = routed.anchor;
        let leg_aux = routed.aux;
        host.refine_leg(leg_anchor, leg_aux);
        host.commit_a(self, leg_aux);
        self.state = if K == 9 { 1 } else { 2 };
        host.commit_b(self, leg_anchor);
        let world = host.world();
        let world_a = host.world_query(WorldQuery::A, world);
        let anchor = if world_a == 0 {
            0
        } else {
            host.anchor_of(world_a)
        };
        let dockw = host.world_query(WorldQuery::B, world);
        if dockw == 0 {
            return Some(heading::<K, _>(host, anchor, 0, world, leg_anchor));
        }
        let world_c = host.world_query(WorldQuery::C, world);
        let mut dock = 0;
        if world_c != 0 {
            dock = host.dock(world_c, DOCK_ARGS);
        }
        let world_d = host.world_query(WorldQuery::D, world);
        let mut boarded = 0;
        if world_d != 0 {
            let (fare_lo, fare_hi) = host.move_fare();
            boarded = host.board(world_d, fare_lo, fare_hi, BOARD_FLAGS);
        }
        host.seat_info(dockw, if K == 7 { dock } else { boarded }, if K == 7 { boarded } else { dock }, SEAT_TAIL);
        Some(heading::<K, _>(host, anchor, boarded, world, leg_anchor))
    }
}

/// The tail on an anchor: the chat ping when the anchor's auxiliary word selects it,
/// then the anchor's sub-task.
fn tail_anchor<H: TickHost>(host: &mut H, anchor: u32, ped: u32) -> u32 {
    if host.anchor_aux(anchor) == AUX_CHAT {
        host.chat_ping(ped, CHAT_PING_WEIGHT_BITS);
    }
    host.anchor_sub(anchor)
}

/// The heading block: attaches the boarding, measures the leg, derives the approach
/// distance from the heading, and seats the pedestrian. Always returns the anchor.
fn heading<const K: u32, H: TickHost>(host: &mut H, anchor: u32, boarded: u32, world: u32, leg_anchor: u32) -> u32 {
    host.attach(anchor, boarded);
    let (leg_len, leg_vec) = host.leg_service(leg_anchor);
    host.leg_length(leg_len);
    let mask = host.sign_mask();
    let leg_vector = [
        f32::from_bits(leg_vec[1] ^ mask),
        f32::from_bits(leg_vec[2]),
        f32::from_bits(leg_vec[3]),
    ];
    // The heading callee takes the leg vector in vector registers and answers a
    // double. The float answer is narrowed exactly as the original narrows it.
    let approach = host.heading(leg_vector) as f32;
    let world_e = host.world_query(WorldQuery::E, world);
    if world_e == 0 {
        host.attach(anchor, 0);
        return anchor;
    }
    let world_f = host.world_query(WorldQuery::F, world);
    let mut seat = 0;
    if world_f != 0 {
        seat = host.dock(world_f, DOCK_ARGS);
    }
    let world_g = host.world_query(if K == 6 { WorldQuery::F } else { WorldQuery::G }, world);
    let mut gate2 = 0;
    if world_g != 0 {
        let (fare_lo, fare_hi) = host.approach_fare();
        gate2 = host.approach(world_g, approach.to_bits(), fare_lo, fare_hi);
    }
    let boarded2 = host.seat_info(world_e, gate2, seat, SEAT_TAIL);
    host.attach(anchor, boarded2);
    anchor
}
