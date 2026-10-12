//! Host tests of the lifted shaft steps: the edge cases a reader would ask
//! about. The bit-for-bit comparison with the verified rewrites lives in the
//! `lf-shaftdiff` test crate, which runs on the 32-bit target.

use super::{Calibration, Shaft, ShaftHost};

/// A square-root host that records each radicand it is asked for.
#[derive(Default)]
struct Recording {
    radicands: Vec<f32>,
}

impl ShaftHost for Recording {
    fn root(&mut self, radicand: f32) -> f32 {
        self.radicands.push(radicand);
        radicand.sqrt()
    }
}

/// A shaft with unit calibration and the given gain.
fn unit_shaft(gain: f32) -> Shaft {
    Shaft {
        gain,
        calibration: [
            Calibration {
                scale: 1.0,
                divisor: 1.0,
            },
            Calibration {
                scale: 1.0,
                divisor: 1.0,
            },
        ],
        table: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0; 3]],
        ..Shaft::default()
    }
}

#[test]
fn zero_gain_takes_no_root() {
    // A zero gain makes the offset zero, so the squared length is zero and the
    // factor is zero: the original does not take the root then.
    for index in 0..4 {
        let mut shaft = unit_shaft(0.0);
        let mut host = Recording::default();
        let answer = match index {
            0 => shaft.advance_0(&mut host, 0.5),
            1 => shaft.advance_1(&mut host, 0.5),
            2 => shaft.advance_2(&mut host, 0.5),
            _ => shaft.advance_3(&mut host, 0.5),
        };
        assert!(host.radicands.is_empty(), "channel {index} took a root");
        assert_eq!(answer, 0.0, "channel {index} answer");
    }
}

#[test]
fn nonzero_length_takes_one_root_and_gives_unit_seeds() {
    let mut shaft = unit_shaft(1.0);
    let mut host = Recording::default();
    shaft.advance_1(&mut host, 1.0);
    assert_eq!(host.radicands.len(), 1, "one root per step");
    let s = shaft.channels[1].seeds;
    let length_squared = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    assert!(
        (length_squared - 1.0).abs() < 1e-5,
        "seeds not unit length: {length_squared}"
    );
}

#[test]
fn fixed_seeds_skip_the_root_and_use_their_axis() {
    // Fixed seeds: channel 0 on the first axis, channel 1 minus the first,
    // channel 2 minus the second, channel 3 on the second.
    let expected: [[f32; 3]; 4] = [
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];
    for (index, want) in expected.iter().enumerate() {
        let mut shaft = unit_shaft(2.0);
        shaft.seeds_fixed = true;
        let mut host = Recording::default();
        match index {
            0 => shaft.advance_0(&mut host, 0.25),
            1 => shaft.advance_1(&mut host, 0.25),
            2 => shaft.advance_2(&mut host, 0.25),
            _ => shaft.advance_3(&mut host, 0.25),
        };
        assert!(host.radicands.is_empty(), "fixed seeds took a root");
        for (got, w) in shaft.channels[index].seeds.iter().zip(want) {
            assert_eq!(got.to_bits(), w.to_bits(), "channel {index}");
        }
    }
}

#[test]
fn step_stores_its_input_and_clears_pending_for_that_channel_only() {
    let mut shaft = unit_shaft(0.5);
    shaft.pending = true;
    let mut host = Recording::default();
    shaft.advance_2(&mut host, 0.125);
    assert!(!shaft.pending, "step left the pending flag set");
    assert_eq!(shaft.channels[2].input, 0.125);
    for index in [0, 1, 3] {
        assert_eq!(
            shaft.channels[index].input, 0.0,
            "channel {index} was written"
        );
    }
}

#[test]
fn not_a_number_length_propagates_and_takes_the_root_path() {
    // A not-a-number step length is not zero, so the root is taken on it and
    // the answer is not a number.
    let mut shaft = unit_shaft(1.0);
    let mut host = Recording::default();
    let answer = shaft.advance_0(&mut host, f32::NAN);
    assert_eq!(host.radicands.len(), 1);
    assert!(host.radicands[0].is_nan());
    assert!(answer.is_nan());
}

#[test]
fn infinite_step_does_not_panic() {
    let mut shaft = unit_shaft(1.0);
    let mut host = Recording::default();
    let _ = shaft.advance_3(&mut host, f32::INFINITY);
    let _ = shaft.advance_0(&mut host, f32::NEG_INFINITY);
}
