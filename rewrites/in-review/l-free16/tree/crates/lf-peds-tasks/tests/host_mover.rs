//! Host tests of the mover pose: the edge cases a reader would ask about.
//!
//! These run on every host. The differential proof against the verified
//! 32-bit rewrites lives in the `lf-pedmoverdiff` test crate.

use lf_peds_tasks::ped_task::{BlendState, MoverCallees, MoverPose, NOT_READY, SNAP_FLAG};

/// A stand-in for the family's callees that records each call by name and
/// answers from fixed values.
struct Stand {
    calls: Vec<&'static str>,
    ready: u32,
    height: f32,
}

impl Stand {
    fn new(ready: u32) -> Self {
        Stand {
            calls: Vec::new(),
            ready,
            height: 0.0,
        }
    }

    fn count(&self, name: &str) -> usize {
        self.calls.iter().filter(|c| **c == name).count()
    }
}

impl MoverCallees for Stand {
    type Handle = u32;

    fn setup(&mut self, _task: u32, _state: u32, _target: Option<u32>, _t: f32) {
        self.calls.push("setup");
    }

    fn ready(&mut self, _task: u32) -> u32 {
        self.calls.push("ready");
        self.ready
    }

    fn blend_into(&mut self, _task: u32, _state: u32, _target: u32, _t: f32) {
        self.calls.push("blend");
    }

    fn encode(&mut self, _task: u32, code: u32) -> f32 {
        self.calls.push("encode");
        // Each two-bit code maps to a distinct float, so a wrong field shows.
        code as f32 + 0.5
    }

    fn notify_flag(&mut self, _task: u32, _arg: u32) {
        self.calls.push("flag");
    }

    fn notify_state(&mut self, _task: u32, _state: u32) {
        self.calls.push("state");
    }

    fn decode(&mut self, _task: u32, bits: u32) -> u32 {
        self.calls.push("decode");
        // The code is the factor's low two bits of its integer part.
        (f32::from_bits(bits) as u32) & 3
    }

    fn set_height(&mut self, _state: u32, z: f32) {
        self.calls.push("set_height");
        self.height = z;
    }

    fn height(&mut self, _state: u32) -> f32 {
        self.calls.push("height");
        self.height
    }
}

fn blank_state() -> BlendState {
    BlendState {
        x: 0.0,
        y: 0.0,
        heading: 0.0,
        heading_mirror: 0.0,
        factors: [0.0; 3],
        top: 0,
        flag: 0,
        level: 0.0,
        flags: 0,
    }
}

fn pose(x: f32, y: f32, z: f32, heading: f32) -> MoverPose {
    MoverPose {
        pos: [x, y, z],
        heading,
        mode: 0,
        flag: 0,
        level: 0,
    }
}

#[test]
fn lerp_moves_the_position_by_the_factor() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    let from = pose(0.0, 10.0, -4.0, 0.0);
    let to = pose(8.0, 20.0, 4.0, 0.0);
    from.lerp_into(&mut stand, 7, &mut state, &to, 0.25);
    assert_eq!(state.x, 2.0);
    assert_eq!(state.y, 12.5);
    assert_eq!(stand.count("set_height"), 1);
    assert_eq!(stand.height, -2.0);
}

#[test]
fn lerp_takes_the_short_way_round_the_circle() {
    // Three radians to minus three is a gap of six, more than a half turn;
    // the blend goes the short way, through pi, and lands on minus three.
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    let from = pose(0.0, 0.0, 0.0, 3.0);
    let to = pose(0.0, 0.0, 0.0, -3.0);
    from.lerp_into(&mut stand, 0, &mut state, &to, 1.0);
    assert!((state.heading + 3.0).abs() < 1e-5, "heading {}", state.heading);
    assert_eq!(state.heading_mirror, state.heading);
}

#[test]
fn lerp_keeps_the_long_way_inside_half_a_turn() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    let from = pose(0.0, 0.0, 0.0, 0.0);
    let to = pose(0.0, 0.0, 0.0, 1.0);
    from.lerp_into(&mut stand, 0, &mut state, &to, 0.5);
    assert_eq!(state.heading, 0.5);
}

#[test]
fn lerp_with_a_nan_heading_gap_takes_the_short_path() {
    // A NaN gap fails the comparison, so the blend is the plain one and the
    // result is NaN, as the short path gives.
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    let from = pose(0.0, 0.0, 0.0, f32::NAN);
    let to = pose(0.0, 0.0, 0.0, 1.0);
    from.lerp_into(&mut stand, 0, &mut state, &to, 0.5);
    assert!(state.heading.is_nan());
}

#[test]
fn drive_snaps_when_the_factor_is_zero_without_probing_readiness_first() {
    let mut stand = Stand::new(1);
    let mut state = blank_state();
    let task = pose(1.0, 2.0, 3.0, 4.0);
    task.drive_blend(&mut stand, 0, 0, &mut state, Some(9), 0.0);
    assert_eq!((state.x, state.y), (1.0, 2.0));
    assert_eq!((state.heading, state.heading_mirror), (4.0, 4.0));
    assert_eq!(stand.height, 3.0);
    // A zero factor snaps before the probe is asked.
    assert_eq!(stand.count("ready"), 0);
    assert_eq!(stand.count("blend"), 0);
}

#[test]
fn drive_snaps_on_the_snap_flag_without_probing_readiness() {
    let mut stand = Stand::new(1);
    let mut state = blank_state();
    state.flags = SNAP_FLAG;
    let task = pose(1.0, 2.0, 3.0, 4.0);
    task.drive_blend(&mut stand, 0, 0, &mut state, Some(9), 0.5);
    assert_eq!(stand.count("ready"), 0);
    assert_eq!(state.x, 1.0);
}

#[test]
fn drive_snaps_when_the_probe_says_not_ready() {
    let mut stand = Stand::new(NOT_READY);
    let mut state = blank_state();
    let task = pose(1.0, 2.0, 3.0, 4.0);
    task.drive_blend(&mut stand, 0, 0, &mut state, Some(9), 0.5);
    assert_eq!(stand.count("ready"), 1);
    assert_eq!(stand.count("blend"), 0);
    assert_eq!(state.x, 1.0);
}

#[test]
fn drive_blends_when_ready_and_a_target_exist() {
    let mut stand = Stand::new(1);
    let mut state = blank_state();
    let task = pose(1.0, 2.0, 3.0, 4.0);
    task.drive_blend(&mut stand, 0, 0, &mut state, Some(9), 0.5);
    assert_eq!(stand.count("ready"), 2);
    assert_eq!(stand.count("blend"), 1);
    assert_eq!(stand.count("set_height"), 0);
}

#[test]
fn drive_does_not_blend_without_a_target() {
    let mut stand = Stand::new(1);
    let mut state = blank_state();
    let task = pose(1.0, 2.0, 3.0, 4.0);
    task.drive_blend(&mut stand, 0, 0, &mut state, None, 0.5);
    assert_eq!(stand.count("ready"), 2);
    assert_eq!(stand.count("blend"), 0);
}

#[test]
fn drive_treats_a_nan_factor_as_a_blend_not_a_snap() {
    // A NaN factor is not zero, so the step probes readiness and, with a
    // target, blends.
    let mut stand = Stand::new(1);
    let mut state = blank_state();
    let task = pose(1.0, 2.0, 3.0, 4.0);
    task.drive_blend(&mut stand, 0, 0, &mut state, Some(9), f32::NAN);
    assert_eq!(stand.count("blend"), 1);
}

#[test]
fn drive_refreshes_the_packed_mode_fields() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    let mut task = pose(0.0, 0.0, 0.0, 0.0);
    // Codes 1, 2, 3 in the three fields and 2 in the top field.
    task.mode = 0b10_11_10_01;
    task.flag = 0xff;
    task.level = 51;
    task.drive_blend(&mut stand, 0, 0, &mut state, None, 0.5);
    assert_eq!(state.top, 2);
    assert_eq!(state.flag, 1);
    assert_eq!(state.factors, [1.5, 2.5, 3.5]);
    assert!((state.level - 0.2).abs() < 1e-6);
}

#[test]
fn commit_writes_the_position_heading_and_height_back() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    state.x = 5.0;
    state.y = -6.0;
    state.heading = 0.75;
    let mut task = pose(0.0, 0.0, 0.0, 0.0);
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.pos[0], 5.0);
    assert_eq!(task.pos[1], -6.0);
    assert_eq!(task.heading, 0.75);
    assert_eq!(stand.count("flag"), 1);
    assert_eq!(stand.count("state"), 1);
    assert_eq!(stand.count("height"), 1);
}

#[test]
fn commit_keeps_the_flag_byte_bits_other_than_bit_zero() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    state.flag = 1;
    let mut task = pose(0.0, 0.0, 0.0, 0.0);
    task.flag = 0b1010_1110;
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.flag, 0b1010_1111);
    state.flag = 0;
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.flag, 0b1010_1110);
}

#[test]
fn commit_clamps_the_level_to_the_unit_interval() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    let mut task = pose(0.0, 0.0, 0.0, 0.0);

    state.level = 2.0;
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.level, 255);

    state.level = -0.5;
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.level, 0);

    state.level = 0.5;
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.level, 127);
}

#[test]
fn commit_turns_a_nan_level_into_the_indefinite_zero_byte() {
    // A NaN passes the clamp; the convert then gives the indefinite value,
    // whose low byte is zero.
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    state.level = f32::NAN;
    let mut task = pose(0.0, 0.0, 0.0, 0.0);
    task.level = 9;
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.level, 0);
}

#[test]
fn commit_rebuilds_the_packed_mode_from_the_decoded_codes() {
    let mut stand = Stand::new(0);
    let mut state = blank_state();
    state.factors = [1.0, 2.0, 3.0];
    state.top = 3;
    let mut task = pose(0.0, 0.0, 0.0, 0.0);
    task.commit_blend(&mut stand, 0, 0, &state);
    assert_eq!(task.mode, 0b11_11_10_01);
}
