//! The aim blend of a ped task record: two gates, a hook on the target that
//! answers a direction, and a six-cell accumulator of the scaled direction.
//!
//! The record keeps the accumulator as two rows of three floats: the first
//! row at [`CELL_ROW_0`] and the second at [`CELL_ROW_1`]. Each row adds the
//! same scaled direction, with the operand order given on each cell (the
//! first column of the first and second rows adds the scaled value first;
//! the other columns add the cell first). A flag byte at [`FLAG_OFFSET`]
//! gains bit [`FLAG_BLENDED`] on every path that returns.
//!
//! The blend runs in this order. The gate is asked about the primary target;
//! when its low byte is zero the gate is asked about the fallback target, and
//! when that low byte is zero too the flag is set and the fallback's answer
//! is returned. Otherwise the target's hook answers a direction, the
//! direction's length gives a factor (zero for a zero length, else length
//! divided by length, which is one or not-a-number), the record's scale is
//! read, the hook answers the direction again, and the scaled direction is
//! accumulated into the two rows.
//!
//! The hook and the gate are callees, and they travel as [`AimBlendCallees`].
//! The original hands the hook a scratch slot that it never reads back; the
//! lift does not model that slot, and the proof masks the argument (see
//! [`REGISTRY_ROWS`]).

use super::registry::State;
use super::shape_registry::Row;

/// Record offset of the first accumulator row (three floats).
pub const CELL_ROW_0: usize = 0x230;

/// Record offset of the second accumulator row (three floats).
pub const CELL_ROW_1: usize = 0x240;

/// Record offset of the flag byte this routine sets bits in.
pub const FLAG_OFFSET: usize = 0x38c;

/// The bit this routine sets in the flag byte.
pub const FLAG_BLENDED: u8 = 0x02;

/// What the blend returns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlendResult {
    /// Both gates failed: the fallback gate's answer, returned unchanged.
    Gated(u32),
    /// The blend ran. The original returns the second hook answer, a
    /// pointer to three floats; the lift returns the floats it read there.
    Blended {
        /// The direction the second hook answer holds.
        vector: [f32; 3],
    },
}

/// The record's accumulator and flag byte, owned.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AimBlend {
    /// Two rows of three accumulated floats each.
    pub rows: [[f32; 3]; 2],
    /// The flag byte. Only [`FLAG_BLENDED`] is set here.
    pub flags: u8,
}

/// The callees and the global the blend needs, as methods on the record.
///
/// The gate is called on the record with a candidate and answers one word.
/// The hook is called on the target and answers the direction it holds. The
/// scale is a global the record reads at the point the original reads it.
pub trait AimBlendCallees {
    /// A target object, as the caller passed it.
    type Target: Copy;

    /// The gate: thiscall on the record with the candidate as argument.
    fn gate(&mut self, owner: &mut AimBlend, candidate: Self::Target) -> u32;

    /// The target's hook: the direction it answers, as three floats.
    fn hook(&mut self, owner: &mut AimBlend, target: Self::Target) -> [f32; 3];

    /// The scale global, read after the first hook answer.
    fn scale(&mut self, owner: &mut AimBlend) -> f32;
}

/// Multiplication with both operands pinned, as the other lifts pin them.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Addition with both operands pinned, in the order given.
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Division with both operands pinned, in the order given.
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

impl AimBlend {
    /// Blends the target's direction into the accumulator, through the gates
    /// described in the module documentation.
    pub fn blend<C: AimBlendCallees>(
        &mut self,
        callees: &mut C,
        primary: C::Target,
        fallback: C::Target,
    ) -> BlendResult {
        let first = callees.gate(self, primary);
        if (first as u8) == 0 {
            let second = callees.gate(self, fallback);
            if (second as u8) == 0 {
                self.flags |= FLAG_BLENDED;
                return BlendResult::Gated(second);
            }
        }

        // Length of the first direction: x*x + y*y + z*z, folded left.
        let [x, y, z] = callees.hook(self, primary);
        let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
        let len = len2.sqrt();
        // A zero length of either sign gives a zero factor; anything else
        // gives length / length.
        let factor = if len == 0.0 { 0.0 } else { div(len, len) };

        let scale = callees.scale(self);
        let [vx, vy, vz] = callees.hook(self, primary);
        let bx = mul(vx, factor);
        let by = mul(vy, factor);
        let bz = mul(vz, factor);
        let cx = mul(scale, bx);
        let cy = mul(by, scale);
        let cz = mul(bz, scale);

        self.rows[0][0] = add(cx, self.rows[0][0]);
        self.rows[0][1] = add(self.rows[0][1], cy);
        self.rows[0][2] = add(self.rows[0][2], cz);
        self.rows[1][0] = add(cx, self.rows[1][0]);
        self.rows[1][1] = add(self.rows[1][1], cy);
        self.rows[1][2] = add(self.rows[1][2], cz);
        self.flags |= FLAG_BLENDED;
        BlendResult::Blended {
            vector: [vx, vy, vz],
        }
    }
}

/// Registry rows for this routine: what the proof compares and what it
/// narrows.
pub const REGISTRY_ROWS: &[Row] = &[Row {
    class: "AimBlend",
    method: "blend",
    state: State::Proven,
    narrows: &[
        "the hook is given a scratch slot the original never reads back; the lift does not own a scratch slot, so the proof masks that one argument of each hook call (both sides compared with the argument set to zero)",
        "the hook's answer is a pointer to three floats in the original; the lift takes the three floats the pointer holds, read at the point the original reads them; the proof compares the floats bit for bit and the answer as the second pointer",
        "the gate and the hook may write the record in the original; in the proof they write nothing, so the lift's order of reads of the accumulator and flag is compared but not a callee's own writes",
        "the record's other fields (its other words and flag bits) are not modelled; the proof checks the whole planted record bytes after the call, so an unexpected write would be caught",
    ],
}];
