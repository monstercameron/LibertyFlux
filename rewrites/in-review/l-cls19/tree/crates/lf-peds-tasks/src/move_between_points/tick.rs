//! The scenario's main update tick, lifted from its verified rewrite.
//!
//! Each tick flags the pedestrian's intelligence as scenario-driven, re-polls the
//! scenario's own sub-operation unless it is idle, then dispatches on the state:
//! state zero counts the wait timer down and, on expiry, samples and commits a
//! destination; state one validates the leg, schedules it, routes it and seats the
//! pedestrian with a heading. The tick returns the sub-task the pedestrian runs.
//!
//! Every callee and collaborator is a method of [`TickHost`], so the lifted code
//! reads as the logic of the tick. The float operations keep the order of the
//! rewrite, with each operand pinned so that a not-a-number keeps its payload.

use core::hint::black_box;

use super::scenario::{MoveBetweenPointsScenario as Scen, STATUS_DONE, STATUS_STICKY};

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

/// The route answer of the first route call: an ok word, the leg anchor and a spare word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteFirst {
    /// Non-zero (in its low byte) when a destination was sampled.
    pub ok: u32,
    /// The anchor the leg is written to.
    pub anchor: u32,
    /// The second out-word; the tick never reads it.
    pub spare: u32,
}

/// The route answer of the second route call: an ok word, the leg anchor and its auxiliary word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteSecond {
    /// Non-zero (in its low byte) when the leg was routed.
    pub ok: u32,
    /// The anchor of the routed leg.
    pub anchor: u32,
    /// The auxiliary word of the routed leg.
    pub aux: u32,
}

/// Which world query a call makes. The seven queries share one shape: the world
/// is the only argument, and the answer is a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldQuery {
    /// First query of the world: an anchor source.
    A,
    /// Dock source of the move leg.
    B,
    /// Dock source of the boarding gate.
    C,
    /// Boarding source.
    D,
    /// Seating source of the heading.
    E,
    /// Dock source of the heading seat.
    F,
    /// Approach source of the heading.
    G,
}

/// The callees, collaborators and shared words the tick uses.
///
/// Handles are opaque words, zero meaning none. A method named for what it
/// answers stands in for one callee or one read of shared game state; the proof
/// records each call with its arguments in order, and each shared read is answered
/// by the host from the same data on both sides.
pub trait TickHost {
    /// Flags the pedestrian's intelligence as scenario-driven.
    fn set_scenario_flag(&mut self, ped: u32);

    /// The scenario's own sub-operation (its virtual operation for `kind`, 1 or 2).
    /// The low byte of the answer says whether the sub-operation accepted.
    fn sub_operation(&mut self, task: &mut Scen, ped: u32, kind: u32) -> u32;

    /// The scenario configuration for an auxiliary identifier.
    fn config(&mut self, aux_id: u32) -> u32;

    /// The configured wait span of a configuration (read from its record).
    fn config_span(&mut self, config: u32) -> f32;

    /// A random draw.
    fn random(&mut self) -> u32;

    /// The frame step (read from shared game state).
    fn frame_step(&mut self) -> f32;

    /// A weight word from shared game state: the narrow or the wide table.
    fn weight_bits(&mut self, narrow: bool) -> u32;

    /// The base of the pedestrian's placement (its position block).
    fn placement_base(&mut self, ped: u32) -> u32;

    /// The three position components at a base.
    fn position(&mut self, base: u32) -> [f32; 3];

    /// The first route call: samples a destination from the base, the weight and the pedestrian.
    fn route_first(&mut self, base: u32, weight: u32, flags: [u32; 3], ped: u32) -> RouteFirst;

    /// The destination words the first sample refines to.
    fn refine_first(&mut self) -> [u32; 4];

    /// The pedestrian's group record.
    fn ped_group(&mut self, ped: u32) -> u32;

    /// The group's seat call; its answer is combined with the intelligence anchor.
    fn group_seat(&mut self, group: u32) -> u32;

    /// The anchor word of the pedestrian's intelligence record.
    fn intel_anchor(&mut self, ped: u32) -> u32;

    /// The commit of the destination: a leg word from the base and the combined word.
    fn commit_route(&mut self, base: u32, value: u32) -> u32;

    /// Writes the leg word of an anchor.
    fn set_anchor_leg(&mut self, anchor: u32, leg: u32);

    /// Writes the state word of an anchor.
    fn set_anchor_state(&mut self, anchor: u32, state: u32);

    /// Whether a leg is valid.
    fn leg_ok(&mut self, leg: u32) -> u32;

    /// Schedules a leg; a non-zero answer means it is already planned.
    fn schedule(&mut self, leg: u32) -> u32;

    /// Releases a scheduled leg.
    fn schedule_use(&mut self, leg: u32);

    /// The second route call: routes the scenario's point for the pedestrian.
    fn route_second(&mut self, ped: u32, weight: u32, flags: [u32; 3]) -> RouteSecond;

    /// Refines a routed leg into a destination; the answer is not read.
    fn refine_leg(&mut self, anchor: u32, aux: u32);

    /// The first commit call on the scenario.
    fn commit_a(&mut self, task: &mut Scen, aux: u32);

    /// The second commit call on the scenario.
    fn commit_b(&mut self, task: &mut Scen, anchor: u32);

    /// The world (read from shared game state).
    fn world(&mut self) -> u32;

    /// One of the seven world queries.
    fn world_query(&mut self, query: WorldQuery, world: u32) -> u32;

    /// The anchor of a world answer.
    fn anchor_of(&mut self, world_a: u32) -> u32;

    /// The dock call on a world answer, with its six argument words.
    fn dock(&mut self, world: u32, args: [u32; 6]) -> u32;

    /// The fare words for the boarding call (read from shared game state).
    fn move_fare(&mut self) -> (u32, u32);

    /// The boarding call with the fare words.
    fn board(&mut self, world: u32, fare_lo: u32, fare_hi: u32, flags: [u32; 7]) -> u32;

    /// The seat-information call. The first word is the object, then two words and two zeros.
    fn seat_info(&mut self, object: u32, a: u32, b: u32, tail: [u32; 2]) -> u32;

    /// Attaches a boarding to an anchor.
    fn attach(&mut self, anchor: u32, boarded: u32);

    /// The leg service of an anchor: its length and the four words of the leg vector.
    fn leg_service(&mut self, anchor: u32) -> (u32, [u32; 4]);

    /// Records the length of a leg.
    fn leg_length(&mut self, len: u32);

    /// The sign mask of the leg vector (read from shared game state).
    fn sign_mask(&mut self) -> u32;

    /// The heading: the approach distance for a leg vector.
    fn heading(&mut self, leg_vector: [f32; 3]) -> f64;

    /// The fare words for the approach call (read from shared game state).
    fn approach_fare(&mut self) -> (u32, u32);

    /// The approach call on a world answer.
    fn approach(&mut self, world: u32, approach_bits: u32, fare_lo: u32, fare_hi: u32) -> u32;

    /// The auxiliary word of an anchor.
    fn anchor_aux(&mut self, anchor: u32) -> u32;

    /// The sub-task word of an anchor.
    fn anchor_sub(&mut self, anchor: u32) -> u32;

    /// The chat ping of a pedestrian.
    fn chat_ping(&mut self, ped: u32, weight_bits: u32);
}

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

impl Scen {
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
        if self.active != 0 && self.pending_guard == 0 {
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
            if host.sub_operation(self, ped, 2) & 0xff != 0 {
                self.status |= STATUS_DONE;
                return 0;
            }
        }
        let config = host.config(self.aux_id);
        let outcome = match self.state {
            0 => self.wait_state(host, ped, config),
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
        if self.aux_id == AUX_CHAT {
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
        self.wait_timer = add(mul(frac, width), s0);
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
        if dist2 <= FAR_SQUARED {
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
        if leg_ok & 0xff == 0 {
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
        self.state = 2;
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
            return Some(heading(host, anchor, 0, world, leg_anchor));
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
        host.seat_info(dockw, boarded, dock, SEAT_TAIL);
        Some(heading(host, anchor, boarded, world, leg_anchor))
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
fn heading<H: TickHost>(host: &mut H, anchor: u32, boarded: u32, world: u32, leg_anchor: u32) -> u32 {
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
    let world_g = host.world_query(WorldQuery::G, world);
    let mut gate2 = 0;
    if world_g != 0 {
        let (fare_lo, fare_hi) = host.approach_fare();
        gate2 = host.approach(world_g, approach.to_bits(), fare_lo, fare_hi);
    }
    let boarded2 = host.seat_info(world_e, gate2, seat, SEAT_TAIL);
    host.attach(anchor, boarded2);
    anchor
}
