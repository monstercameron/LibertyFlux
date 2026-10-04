// original: 0x00a1f950 cam_pair_snap (proposed)

/// Snaps two stored/current float pairs together when close enough.
///
/// `this` points to a record with two pairs: stored floats at `+A_STORED`
/// and `+B_STORED`, current floats at `+A_CUR` and `+B_CUR`. For each
/// pair, when `EPS` (1e-4) is strictly above the absolute difference
/// `|cur - stored|` (absolute value by clearing the sign bit), the stored
/// slot takes the current value; otherwise the current slot takes the
/// stored value. A NaN difference takes the second path. Returns nothing.
///
/// Original: 0x00a1f950 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1f950(this: u32) -> u32 {
    unsafe {
        const A_STORED: u32 = 0x304;
        const B_STORED: u32 = 0x308;
        const A_CUR: u32 = 0x314;
        const B_CUR: u32 = 0x318;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const EPS: f32 = f32::from_bits(0x38d1_b717); // 1e-4
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        unsafe fn wr32(x: u32, v: u32) {
            unsafe { (x as *mut u32).write_unaligned(v) }
        }
        unsafe fn snap(this: u32, stored: u32, cur: u32) {
            unsafe {
                let s = f32::from_bits(rd32(this + stored));
                let c = f32::from_bits(rd32(this + cur));
                let d = f32::from_bits(sub(c, s).to_bits() & ABS_MASK);
                if EPS > d {
                    wr32(this + stored, c.to_bits());
                } else {
                    wr32(this + cur, s.to_bits());
                }
            }
        }
        snap(this, A_STORED, A_CUR);
        snap(this, B_STORED, B_CUR);
        0
    }
});
