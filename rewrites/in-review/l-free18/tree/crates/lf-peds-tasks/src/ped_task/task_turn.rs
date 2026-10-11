//! The turn tracker of a ped task: a blend rate that rises toward one while
//! the gate is open, and a target point pulled back along the task's
//! direction, scaled by that rate.
//!
//! The owning type is [`TurnTask`], the two words of the task the tracker
//! reads and writes: the single-precision rate and the flag byte whose
//! reset bit sets the rate back to one. The direction comes from the
//! parameter block: either the block's own direction, read through its
//! inner pointer, or the sine and cosine of its angle, which travel as
//! [`TurnTrig`]. The target point is an owned [`Vec3`] the caller passes in
//! and gets back updated.
//!
//! All arithmetic is single precision, in the original's order: each
//! direction component is scaled by [`DIR_SCALE`] and then by the rate.

/// The gate: the tracker advances only while the gate value is strictly
/// greater than this (single precision, 0.02).
pub const RATE_GATE: f32 = f32::from_bits(0x3CA3_D70A);
/// The amount the rate rises by each advance (single precision, 0.01).
pub const RATE_STEP: f32 = f32::from_bits(0x3C23_D70A);
/// The scale each direction component is multiplied by first (single
/// precision, 0.1).
pub const DIR_SCALE: f32 = f32::from_bits(0x3DCC_CCCD);
/// The rate's upper bound and its reset value.
pub const RATE_ONE: f32 = 1.0;
/// The flag bit that resets the rate to one before the gate is tested.
pub const FLAG_RESET: u8 = 0x80;

/// A point or a direction in three single-precision components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The first component.
    pub x: f32,
    /// The second component.
    pub y: f32,
    /// The third component.
    pub z: f32,
}

/// The task words the tracker reads and writes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurnTask {
    /// Single-precision word at offset 0x2c4: the blend rate.
    pub rate: f32,
    /// Byte at offset 0x38c: the flags; [`FLAG_RESET`] resets the rate.
    pub flag: u8,
}

/// The parameter block the tracker reads its direction from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TurnParams {
    /// Single-precision word at offset 0x1c: the angle the sine and cosine
    /// are taken of when there is no inner direction.
    pub angle: f32,
    /// The inner direction, or none when the original's inner pointer is
    /// null. The direction's components sit at offsets 0x10, 0x14 and 0x18
    /// of the inner block.
    pub inner: Option<Vec3>,
}

/// The two trigonometric collaborators. Each takes the angle and answers a
/// single-precision value; the original calls the sine first, then the
/// cosine, and only when the inner direction is absent.
pub trait TurnTrig {
    /// The sine of the angle.
    fn sine(&mut self, angle: f32) -> f32;
    /// The cosine of the angle.
    fn cosine(&mut self, angle: f32) -> f32;
}

/// Negation by flipping the sign bit, as the original does it.
#[must_use]
fn negate(value: f32) -> f32 {
    f32::from_bits(value.to_bits() ^ 0x8000_0000)
}

impl TurnTask {
    /// Advances the tracker: resets the rate if the flag asks, raises it by
    /// [`RATE_STEP`] while the gate is open (capped at one; a not-a-number
    /// sum also becomes one), and pulls the target back along the direction
    /// scaled by the rate.
    ///
    /// The direction is the inner block's when there is one; otherwise it is
    /// the negated sine, the cosine and zero of the angle. The rate read for
    /// the scaling is the one after the update.
    pub fn advance(
        &mut self,
        target: &mut Vec3,
        params: &TurnParams,
        gate: f32,
        trig: &mut impl TurnTrig,
    ) {
        if self.flag & FLAG_RESET != 0 {
            self.rate = RATE_ONE;
        }
        if gate > RATE_GATE {
            let mut raised = self.rate + RATE_STEP;
            if !(RATE_ONE > raised) {
                raised = RATE_ONE;
            }
            self.rate = raised;
        }
        let (dx, dy, dz) = match params.inner {
            Some(direction) => (direction.x, direction.y, direction.z),
            None => {
                let sine = trig.sine(params.angle);
                let cosine = trig.cosine(params.angle);
                (negate(sine), cosine, 0.0)
            }
        };
        let rate = self.rate;
        let dx = (dx * DIR_SCALE) * rate;
        let dy = (dy * DIR_SCALE) * rate;
        let dz = (dz * DIR_SCALE) * rate;
        target.x -= dx;
        target.y -= dy;
        target.z -= dz;
    }
}
