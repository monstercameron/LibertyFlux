//! Host tests of the seek-target record: the edge cases a reader would ask
//! about. The differential proof against the verified 32-bit rewrite lives
//! in the `lf-pedmoverdiff` test crate.

use lf_peds_tasks::ped_task::{SEEK_MODE, SeekGoal, SeekMover, SeekRecord, TUNING};

fn mover(x: f32, y: f32, z: f32) -> SeekMover {
    SeekMover {
        position: [x, y, z],
        spare: 0x1234_5678,
    }
}

#[test]
fn step_points_from_the_goal_through_the_mover_at_eight_units() {
    // The mover is three units east of the goal: the unit direction is
    // east, the step is eight units east, and the stepped point is eight
    // units further from the goal than the mover.
    let record = SeekRecord::toward(&mover(3.0, 0.0, 5.0), &SeekGoal { x: 0.0, y: 0.0 });
    assert_eq!(record.at, [3.0, 0.0, 5.0]);
    // The unit direction is one third of a unit in single precision, so the
    // step is eight within rounding.
    assert!(
        (record.stepped[0] - (3.0 - 8.0)).abs() < 1e-4,
        "x {}",
        record.stepped[0]
    );
    assert_eq!(record.stepped[1], 0.0);
    assert_eq!(record.stepped[2], 5.0);
}

#[test]
fn a_diagonal_step_has_length_eight() {
    let record = SeekRecord::toward(&mover(3.0, 4.0, 0.0), &SeekGoal { x: 0.0, y: 0.0 });
    let dx = record.at[0] - record.stepped[0];
    let dy = record.at[1] - record.stepped[1];
    assert!(
        (dx.hypot(dy) - 8.0).abs() < 1e-4,
        "step length {}",
        dx.hypot(dy)
    );
}

#[test]
fn a_mover_on_the_goal_takes_no_step() {
    // A zero sum takes a zero inverse, so the step is zero and the stepped
    // point is the mover itself.
    let record = SeekRecord::toward(&mover(2.0, -2.0, 1.0), &SeekGoal { x: 2.0, y: -2.0 });
    assert_eq!(record.stepped, [2.0, -2.0, 1.0]);
}

#[test]
fn the_height_is_never_stepped() {
    let record = SeekRecord::toward(&mover(3.0, 4.0, 7.5), &SeekGoal { x: 0.0, y: 0.0 });
    assert_eq!(record.stepped[2], 7.5);
}

#[test]
fn a_nan_position_gives_a_nan_step_and_keeps_the_copied_words() {
    let record = SeekRecord::toward(&mover(f32::NAN, 0.0, 0.0), &SeekGoal { x: 0.0, y: 0.0 });
    assert!(record.stepped[0].is_nan());
    assert_eq!(record.at_spare, 0x1234_5678);
}

#[test]
fn the_record_carries_the_tuning_twice_and_a_zero_frame_word() {
    let record = SeekRecord::toward(&mover(0.0, 0.0, 0.0), &SeekGoal { x: 1.0, y: 1.0 });
    assert_eq!(record.tuning_a, TUNING);
    assert_eq!(record.tuning_b, TUNING);
    assert_eq!(record.frame_word, 0);
    assert_eq!(TUNING[0], 8.0);
    assert_eq!(TUNING[1], 64.0);
    // The bits read as 9999.5; see the constant's comment.
    assert_eq!(TUNING[2].to_bits(), 0x461C_3C00);
}

#[test]
fn the_mode_word_is_two() {
    assert_eq!(SEEK_MODE, 2);
}
