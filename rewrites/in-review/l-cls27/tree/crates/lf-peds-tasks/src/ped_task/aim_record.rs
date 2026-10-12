//! The aim record of an aim task: the normalised direction from the owner's
//! position toward a target point, handed to the owner's driver, which then
//! answers the record that replaces it.
//!
//! The owner is an object that holds a pointer to its position block and a
//! pointer to its driver. Its position block carries the owner's x and y at
//! fixed offsets; the owner's driver is an opaque handle, passed to the two
//! collaborators. The task holds the three floats added to the target point
//! after the first collaborator succeeds; they travel as [`AimTask`].
//!
//! The record is four words: the direction's x and y, a zero z, and a fourth
//! word. The fourth word is zero until the driver answers; the original
//! copies it from an uninitialised stack word, and the lift uses the zero the
//! verified rewrite writes there.

use super::Vec3;

/// The single-precision one, the numerator of the normalisation.
const ONE: f32 = 1.0;

/// The four words of the aim record, as bits: direction x, direction y, z,
/// and the trailing word.
pub type RecordWords = [u32; 4];

/// The owner of an aim task, as far as the record reads it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AimOwner {
    /// The owner's x and y, read from its position block.
    pub position: [f32; 2],
    /// The driver the owner forwards the record to, or none when the
    /// original's driver pointer is null.
    pub driver: Option<u32>,
}

/// The task's accumulated offset, added to the target point after the first
/// collaborator succeeds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AimTask {
    /// The three accumulated components.
    pub accumulated: Vec3,
}

/// The two collaborators of the aim update. Each receives the values the
/// original passes by pointer, and the implementation supplies the constant
/// arguments the original fixes (a zero fifth argument and a zero frame word).
pub trait AimCallees {
    /// The first collaborator, called with the driver, the owner's handle,
    /// the record and the target point. Its answer's low byte decides whether
    /// the update continues.
    fn advance(&mut self, driver: u32, owner: u32, record: RecordWords, target: Vec3) -> u32;
    /// The second collaborator, called on the driver with the record. It
    /// answers a sixteen-byte result, which the update reads back as the new
    /// record.
    fn driver_call(&mut self, driver: u32, record: RecordWords) -> RecordWords;
}

/// Continuation mask of the first collaborator's answer: the low byte.
const CONTINUE_MASK: u32 = 0xFF;
/// The low byte set on the update's returned value.
const RETURN_FLAG: u32 = 1;

/// Normalises a two-component vector by its length, as the original does it:
/// zero when the squared length is not positive, otherwise one over the
/// length. The product order is the original's.
#[must_use]
pub fn normalised_direction(dx: f32, dy: f32) -> (f32, f32, f32) {
    let len2 = add(mul(dy, dy), mul(dx, dx));
    let inv = if len2 <= 0.0 {
        0.0
    } else {
        core::hint::black_box(ONE) / core::hint::black_box(core::hint::black_box(len2).sqrt())
    };
    (mul(inv, dx), mul(dy, inv), mul(inv, 0.0))
}

/// Single-precision product with the operands pinned in the original's
/// order, so a not-a-number keeps the sign and payload the original gives it.
#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Single-precision sum with the operands pinned in the original's order.
#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

impl AimTask {
    /// Updates the aim record toward `target` and returns the update's answer.
    ///
    /// The record first holds the normalised direction with a zero fourth
    /// word. With no driver the answer is zero and the record stays. With a
    /// driver, the first collaborator is called with the record and the
    /// target; if its low byte is zero the answer is zero and the record
    /// stays. Otherwise the task's accumulated offset is added to the target,
    /// the second collaborator answers the new record, and the answer is the
    /// record's last word with its low byte set.
    pub fn update(
        &self,
        owner_handle: u32,
        owner: &AimOwner,
        target: &mut Vec3,
        point: [f32; 2],
        record: &mut RecordWords,
        calls: &mut impl AimCallees,
    ) -> u32 {
        let (x, y, z) =
            normalised_direction(point[0] - owner.position[0], point[1] - owner.position[1]);
        *record = [x.to_bits(), y.to_bits(), z.to_bits(), 0];
        let Some(driver) = owner.driver else {
            return 0;
        };
        let proceed = calls.advance(driver, owner_handle, *record, *target);
        if proceed & CONTINUE_MASK == 0 {
            return 0;
        }
        target.x = add(target.x, self.accumulated.x);
        target.y = add(target.y, self.accumulated.y);
        target.z = add(target.z, self.accumulated.z);
        let answer = calls.driver_call(driver, *record);
        *record = answer;
        (answer[3] & 0xFFFF_FF00) | RETURN_FLAG
    }
}
