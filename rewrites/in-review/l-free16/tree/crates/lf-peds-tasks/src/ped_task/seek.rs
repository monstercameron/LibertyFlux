//! The seek-target record: the mover's position, a goal, and the step the
//! mover takes toward the goal in the plane.
//!
//! [`SeekRecord::toward`] builds the 0x40-byte record a seek task reads.
//! The record holds the mover's position (three floats and one word), two
//! constant quadruples of tuning floats, the position minus a step, and a
//! zero word. The step is the unit direction from the goal to the mover in
//! x and y, scaled by eight; its z component is zero, so the z step is the
//! unit direction's z (zero) times eight.
//!
//! The routine also writes a mode word of two, carried here as
//! [`SEEK_MODE`]: the caller stores it in the task's mode slot.

#![forbid(unsafe_code)]

/// The mode word the seek routine stores beside the record.
pub const SEEK_MODE: u32 = 2;
/// The step length the record scales the unit direction by.
pub const STEP_SCALE: f32 = 8.0;
/// The tuning quadruple the record carries twice, once before the step
/// and once after it. The four values are the routine's constants as the
/// original stores them.
pub const TUNING: [f32; 4] = [8.0, 64.0, 10_000.0, 10_000.0];

// The float operations are pinned in the original's operand order: each
// operand passes through `black_box`, so the optimiser cannot commute the
// operations, and a NaN's payload comes out as the original's does.
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// The mover's position block: three position floats and one word the
/// record copies as bits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeekMover {
    /// Position x, y and z.
    pub position: [f32; 3],
    /// The fourth word of the position block, copied as bits.
    pub spare: u32,
}

/// The goal: the destination's x and y.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeekGoal {
    /// Destination x.
    pub x: f32,
    /// Destination y.
    pub y: f32,
}

/// The seek record: sixteen words, in the order the routine writes them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeekRecord {
    /// The mover's position x, y and z.
    pub at: [f32; 3],
    /// The mover's fourth position word, as bits.
    pub at_spare: u32,
    /// The first tuning quadruple.
    pub tuning_a: [f32; 4],
    /// The position minus the step: x, y and z.
    pub stepped: [f32; 3],
    /// A word the original copies from an uninitialised frame slot; the
    /// rewrite pins it to zero, and so does the lift.
    pub frame_word: u32,
    /// The second tuning quadruple.
    pub tuning_b: [f32; 4],
}

impl SeekRecord {
    /// Builds the seek record for a mover heading toward a goal.
    ///
    /// The planar difference `d` is the mover minus the goal in x and y.
    /// The inverse length is `1 / sqrt(dx*dx + dy*dy)`, or zero when that
    /// sum is not above zero (a NaN sum takes the division). The step is
    /// the unit direction times [`STEP_SCALE`]: x and y from `d`, and z
    /// from the zero the routine multiplies in.
    #[must_use]
    pub fn toward(mover: &SeekMover, goal: &SeekGoal) -> Self {
        let [px, py, pz] = mover.position;
        let dx = sub(px, goal.x);
        let dy = sub(py, goal.y);
        let len2 = add(mul(dx, dx), mul(dy, dy));
        let inv = if len2 <= 0.0 {
            0.0
        } else {
            core::hint::black_box(1.0_f32) / core::hint::black_box(core::hint::black_box(len2).sqrt())
        };
        let dir_x = mul(dx, inv);
        let dir_y = mul(dy, inv);
        let dir_z = mul(inv, 0.0);
        let step_x = mul(dir_x, STEP_SCALE);
        let step_y = mul(dir_y, STEP_SCALE);
        let step_z = mul(dir_z, STEP_SCALE);
        SeekRecord {
            at: [px, py, pz],
            at_spare: mover.spare,
            tuning_a: TUNING,
            stepped: [sub(px, step_x), sub(py, step_y), sub(pz, step_z)],
            frame_word: 0,
            tuning_b: TUNING,
        }
    }
}
