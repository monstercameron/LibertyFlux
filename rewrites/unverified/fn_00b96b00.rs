// original: 0x00b96b00 NativeImpl_IS_POINT_OBSCURED_BY_A_MISSION_ENTITY

/// Tests whether a point is obscured, from a normalized extent box.
///
/// Expands (`x`, `y`, `z`) by (`r0`, `r1`, `r2`) with single-precision
/// subtract/add in the original's operand order, then normalizes each pair
/// with `comiss`/`jbe` semantics (no swap when unordered). Packs the six
/// normalized bounds into two overlapping frame words views and tests them
/// with (`TAG`, 0x1C, 0x0D) through `TEST`: the first view holds
/// (min_z, max_x, max_y, max_z), the second (max_x, max_y, max_z, scratch),
/// where the last word is never stored by the original (frame scratch, zero
/// under the contract's `stack_fill`). Returns whether the minimum-z bits
/// are positive as a signed integer.
///
/// `TAG` is pushed as a literal: its immediate has no relocation entry, so
/// both sides push the identical raw value. Both buffer pointers are skipped
/// call arguments with snapshot-verified contents (see `narrowed`). The
/// normalized x minimum is stored to the frame but belongs to neither view,
/// so it is unobservable and this rewrite drops it.
///
/// Original: 0x00B96B00 (cdecl, six stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b96b00(x: u32, y: u32, z: u32, r0: u32, r1: u32, r2: u32) -> u32 {
    const TEST: u32 = 1;
    const TAG: u32 = 0x009498C0;

    #[inline(always)]
    fn fsub(a: u32, b: u32) -> u32 {
        (f32::from_bits(core::hint::black_box(a)) - f32::from_bits(core::hint::black_box(b)))
            .to_bits()
    }
    #[inline(always)]
    fn fadd(a: u32, b: u32) -> u32 {
        (f32::from_bits(core::hint::black_box(a)) + f32::from_bits(core::hint::black_box(b)))
            .to_bits()
    }
    #[inline(always)]
    fn norm(mut lo: f32, mut hi: f32) -> (u32, u32) {
        if lo > hi {
            core::mem::swap(&mut lo, &mut hi);
        }
        (lo.to_bits(), hi.to_bits())
    }

    let (min_x, max_x) = norm(
        f32::from_bits(fsub(x, r0)),
        f32::from_bits(fadd(x, r0)),
    );
    let (min_y, max_y) = norm(
        f32::from_bits(fsub(y, r1)),
        f32::from_bits(fadd(y, r1)),
    );
    let (min_z, max_z) = norm(
        f32::from_bits(fsub(z, r2)),
        f32::from_bits(fadd(z, r2)),
    );
    let mut view_a = [min_z, max_x, max_y, max_z];
    let mut view_b = [max_x, max_y, max_z, 0];
    let _: u32 = lf_checker_rt::callee_cdecl!(
        TEST, u32, view_b.as_mut_ptr() as u32, TAG, view_a.as_mut_ptr() as u32, 0x1C, 0x0D
    );
    u32::from((min_z as i32) > 0)
});
