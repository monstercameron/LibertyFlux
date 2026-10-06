//! The weighted candidate picker: sixteen weights drawn by a random threshold.
//!
//! The 32-bit picker object holds sixteen 32-byte entry records, sixteen
//! cached result slots (`-1` means empty), sixteen float weights and a
//! signed entry count. The lift owns the weights, the slots and the count
//! as ordinary Rust; the entry records are never read, only addressed for
//! the fill call, so they travel as the picked index.
//!
//! Behaviour (one method, [`WeightedPicker::pick`]): build the prefix sums
//! of the first `count.min(16)` weights (whole groups of four first, then
//! the tail one by one, each in the original's operand order), draw the
//! random source, form `threshold = float(rand) * SCALE * total`, and take
//! the first index whose prefix strictly exceeds the threshold, or the last
//! index when none does. A non-positive count answers [`EMPTY`] without
//! scanning (after drawing, as the original does). The picked slot is
//! returned as-is unless it holds [`EMPTY`], in which case the fill
//! collaborator computes it and the filled value is returned.

/// Entries the picker holds.
pub const MAX_ENTRIES: usize = 16;

/// Slot value meaning empty, and the answer for a non-positive count.
pub const EMPTY: u32 = 0xFFFF_FFFF;

/// The threshold scale from the 32-bit form (about 3.05e-5).
pub const SCALE: f32 = f32::from_bits(0x3800_0100);

/// The random source the pick draws from (one numbered callee).
pub trait PickerRand {
    /// Draws the next unsigned word.
    fn draw(&mut self) -> u32;
}

impl<F: FnMut() -> u32> PickerRand for F {
    fn draw(&mut self) -> u32 {
        self()
    }
}

/// The fill collaborator that computes an empty slot (one numbered callee).
pub trait PickerFill {
    /// Computes the value for `index` (entry and slot share the index).
    fn fill(&mut self, index: usize) -> u32;
}

impl<F: FnMut(usize) -> u32> PickerFill for F {
    fn fill(&mut self, index: usize) -> u32 {
        self(index)
    }
}

/// The picker: weights, cached slots and the signed entry count.
#[derive(Clone, Debug)]
pub struct WeightedPicker {
    /// One weight per entry; only the first `count.min(16)` are read.
    pub weights: [f32; MAX_ENTRIES],
    /// Cached results; [`EMPTY`] means the fill collaborator must run.
    pub slots: [u32; MAX_ENTRIES],
    /// Entry count, used as a signed value; honest counts are 0..=16.
    pub count: i32,
}

#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

impl WeightedPicker {
    /// Picks one entry by a random threshold over prefix sums.
    ///
    /// The random source is always drawn exactly once, even for a
    /// non-positive count. The fill collaborator runs at most once, only
    /// when the picked slot holds [`EMPTY`].
    pub fn pick<R: PickerRand, F: PickerFill>(&mut self, rand: &mut R, fill: &mut F) -> u32 {
        let mut prefix = [0.0f32; MAX_ENTRIES];
        let mut total = 0.0f32;
        if self.count > 0 {
            let n = (self.count as u32).min(MAX_ENTRIES as u32);
            // Whole groups of four first (only when the count reaches four),
            // then the tail one by one; the two orders differ, both pinned.
            let nvec = if self.count >= 4 { n & !3 } else { 0 };
            let mut i = 0u32;
            while i < nvec {
                let p = add(self.weights[i as usize], total);
                prefix[i as usize] = p;
                total = p;
                i += 1;
            }
            while i < n {
                total = add(total, self.weights[i as usize]);
                prefix[i as usize] = total;
                i += 1;
            }
        }
        let drawn = rand.draw();
        // The original converts with a signed int-to-float instruction,
        // which rounds: the precision loss is the behaviour.
        #[allow(clippy::cast_precision_loss)]
        let as_float = drawn as i32 as f32;
        let threshold = mul(mul(as_float, SCALE), total);
        if self.count <= 0 {
            return EMPTY;
        }
        let n = (self.count as u32).min(MAX_ENTRIES as u32);
        let mut idx = 0u32;
        while idx < n {
            if prefix[idx as usize] > threshold {
                break;
            }
            if idx + 1 >= n {
                break;
            }
            idx += 1;
        }
        let slot = &mut self.slots[idx as usize];
        if *slot == EMPTY {
            *slot = fill.fill(idx as usize);
        }
        *slot
    }
}
