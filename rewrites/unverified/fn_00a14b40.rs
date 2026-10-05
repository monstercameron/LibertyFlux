// original: 0x00a14b40 axis_pair_snap (proposed)
/// Snap two pairs of axis values together when they are nearly equal.
///
/// For the pairs at `this + 0x1a8`/`0x190` and `this + 0x1ac`/`0x194`: when
/// the absolute difference is strictly below the epsilon constant at
/// 0x00fe868c, the second slot takes the first slot's value, otherwise the
/// first takes the second's. The absolute value clears the sign bit and the
/// comparison is SSE `comiss` (unordered keeps the else branch). No return
/// value is set. Thiscall, no stack arguments.
export!(thiscall, rw_00a14b40(this: u32) -> u32 {
    unsafe {
        const EPS_ADDR: u32 = 0x00fe868c;
        #[inline(always)]
        unsafe fn rd(slot: u32) -> f32 {
            unsafe { f32::from_bits((slot as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wr(slot: u32, v: f32) {
            unsafe { (slot as *mut u32).write_unaligned(v.to_bits()) }
        }
        let eps = f32::from_bits(*global::<u32>(EPS_ADDR));
        for (first, second) in [(0x1a8u32, 0x190u32), (0x1acu32, 0x194u32)] {
            let a = rd(this + first);
            let b = rd(this + second);
            let d = sub_abs(a, b);
            if eps > d {
                wr(this + second, a);
            } else {
                wr(this + first, b);
            }
        }
        0
    }
});

/// Absolute difference with the original's operand order and sign clearing.
#[inline(always)]
fn sub_abs(a: f32, b: f32) -> f32 {
    let d = core::hint::black_box(a) - core::hint::black_box(b);
    f32::from_bits(d.to_bits() & 0x7fffffff)
}
