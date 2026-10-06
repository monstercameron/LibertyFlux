//! The input-ui bounds accumulator: component-wise bounds over a stride.
//!
//! [`accumulate_bounds`] restates `input_ui_bounds_accumulate`: for each
//! item a measure call fills a high and a low float triple, three running
//! maxima (from a high seed) fold the highs and three running minima
//! (from a low seed) fold the lows, then a flag word is set to 1 and a
//! sink call reports both triples. The 32-bit form walks `arg1..arg2` in
//! 32-byte steps until the cursor equals the end; the lift iterates a
//! stated item count, so the loop always ends.
//!
//! Evidence: `fn_00abaf50.rs` + `r-b257/contracts/fn_00abaf50.json`
//! (contract file name per the lane's folder layout).

/// Item stride in the 32-bit walk (32 bytes).
pub const ITEM_STRIDE: u32 = 0x20;

/// Fills one item's low and high triples.
pub trait Measure {
    /// Measures item `item` (0-based) into `lo` and `hi`, in order.
    fn measure(&mut self, item: usize, lo: &mut [f32; 3], hi: &mut [f32; 3]);
}

/// Reports the accumulated bounds and answers the routine's result.
///
/// The 32-bit sink takes nine words (both triple pointers, the tag
/// twice, the range ends, an extra word, the flag address, a mode
/// word); the lift takes the triples by value, each remaining word
/// once, and the flag by mutable reference. The differential test
/// pins the doubled tag and reconstructs both range ends per case.
pub trait BoundsSink {
    /// Reports the bounds; `flag` already holds 1 on entry.
    fn report(
        &mut self,
        lo: &[f32; 3],
        hi: &[f32; 3],
        tag: u32,
        extra: u32,
        flag: &mut u32,
        mode: u32,
    ) -> u32;
}

/// Maximum in the original's compare order: keeps `acc` only when it
/// is ordered-above `new`, else takes `new`.
///
/// A NaN candidate replaces the running value; a NaN running value is
/// replaced by any ordered candidate. Operand order is pinned: a plain
/// comparison lets the compiler swap the operands, which NaN payloads
/// can observe.
#[must_use]
#[inline]
pub fn fold_max(acc: f32, new: f32) -> f32 {
    if core::hint::black_box(acc) > core::hint::black_box(new) {
        acc
    } else {
        new
    }
}

/// Minimum in the original's compare order: keeps `acc` only when
/// `new` is ordered-above it, else takes `new` (same NaN rules and
/// pinned order as [`fold_max`]).
#[must_use]
#[inline]
pub fn fold_min(acc: f32, new: f32) -> f32 {
    if core::hint::black_box(new) > core::hint::black_box(acc) {
        acc
    } else {
        new
    }
}

/// Accumulates bounds over `items` items and reports through the sink.
///
/// Returns the sink's answer. `seed_hi` and `seed_lo` are the two seed
/// floats (read-only image words in the 32-bit form, parameters here).
pub fn accumulate_bounds<M: Measure + ?Sized, S: BoundsSink + ?Sized>(
    items: usize,
    seed_hi: f32,
    seed_lo: f32,
    measure: &mut M,
    sink: &mut S,
    tag: u32,
    extra: u32,
    flag: &mut u32,
    mode: u32,
) -> u32 {
    let mut hi = [seed_hi; 3];
    let mut lo = [seed_lo; 3];
    for item in 0..items {
        let mut out_hi = [0.0f32; 3];
        let mut out_lo = [0.0f32; 3];
        measure.measure(item, &mut out_lo, &mut out_hi);
        for k in 0..3 {
            hi[k] = fold_max(hi[k], out_hi[k]);
            lo[k] = fold_min(lo[k], out_lo[k]);
        }
    }
    *flag = 1;
    sink.report(&lo, &hi, tag, extra, flag, mode)
}
