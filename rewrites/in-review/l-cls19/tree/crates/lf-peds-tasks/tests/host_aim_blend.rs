//! Host tests of the aim blend: the gate paths, the flag bit and the
//! arithmetic on values whose results can be read by hand. They run on every
//! host; the differential cases against the verified rewrite are in the
//! `lf-taskshapediff2` test crate.

use lf_peds_tasks::ped_task::aim_blend::{AimBlend, AimBlendCallees, BlendResult, FLAG_BLENDED};

/// Callees that log the gate candidates and hook calls, and answer from
/// fixed values.
struct Fixed {
    gate_answers: Vec<u32>,
    direction: [f32; 3],
    scale: f32,
    gates: Vec<u32>,
    hooks: u32,
}

impl Fixed {
    fn new(gate_answers: Vec<u32>, direction: [f32; 3], scale: f32) -> Self {
        Self {
            gate_answers,
            direction,
            scale,
            gates: Vec::new(),
            hooks: 0,
        }
    }
}

impl AimBlendCallees for Fixed {
    type Target = u32;

    fn gate(&mut self, _owner: &mut AimBlend, candidate: u32) -> u32 {
        self.gates.push(candidate);
        self.gate_answers[self.gates.len() - 1]
    }

    fn hook(&mut self, _owner: &mut AimBlend, _target: u32) -> [f32; 3] {
        self.hooks += 1;
        self.direction
    }

    fn scale(&mut self, _owner: &mut AimBlend) -> f32 {
        self.scale
    }
}

fn empty() -> AimBlend {
    AimBlend {
        rows: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
        flags: 0x10,
    }
}

#[test]
fn both_gates_failing_returns_the_fallback_answer_and_sets_the_flag() {
    let mut owner = empty();
    let mut callees = Fixed::new(vec![0x100, 0x200], [9.0, 9.0, 9.0], 1.0);
    let result = owner.blend(&mut callees, 7, 8);
    assert_eq!(result, BlendResult::Gated(0x200));
    assert_eq!(callees.gates, vec![7, 8]);
    assert_eq!(callees.hooks, 0);
    assert_eq!(owner.flags, 0x10 | FLAG_BLENDED);
    assert_eq!(owner.rows, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
}

#[test]
fn a_set_low_byte_on_the_primary_skips_the_fallback_gate() {
    let mut owner = empty();
    let mut callees = Fixed::new(vec![0x101, 0], [0.0, 0.0, 0.0], 1.0);
    let result = owner.blend(&mut callees, 7, 8);
    assert!(matches!(result, BlendResult::Blended { .. }));
    assert_eq!(callees.gates, vec![7]);
    assert_eq!(callees.hooks, 2);
}

#[test]
fn a_zero_length_direction_adds_nothing() {
    let mut owner = empty();
    let mut callees = Fixed::new(vec![1, 1], [0.0, -0.0, 0.0], 3.0);
    owner.blend(&mut callees, 1, 2);
    assert_eq!(owner.rows, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
}

#[test]
fn a_unit_length_direction_is_scaled_into_the_rows() {
    // Length of (3, 0, 4) is 5, so the factor is 1; the scale 2 then doubles it.
    let mut owner = empty();
    let mut callees = Fixed::new(vec![1, 1], [3.0, 0.0, 4.0], 2.0);
    let result = owner.blend(&mut callees, 1, 2);
    assert_eq!(
        result,
        BlendResult::Blended {
            vector: [3.0, 0.0, 4.0]
        }
    );
    // Row 0: first column adds the scaled x first; the rest add the cell first.
    assert_eq!(owner.rows[0], [1.0 + 6.0, 2.0 + 0.0, 3.0 + 8.0]);
    assert_eq!(owner.rows[1], [4.0 + 6.0, 5.0 + 0.0, 6.0 + 8.0]);
    assert_eq!(owner.flags, 0x10 | FLAG_BLENDED);
}

#[test]
fn a_not_a_number_direction_reaches_the_rows() {
    let mut owner = empty();
    let mut callees = Fixed::new(vec![1, 1], [f32::NAN, 0.0, 0.0], 1.0);
    owner.blend(&mut callees, 1, 2);
    assert!(owner.rows[0][0].is_nan());
    assert!(owner.rows[1][0].is_nan());
}
