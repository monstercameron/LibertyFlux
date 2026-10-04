// original: 0x00cad240 counted_vec3s_equal (proposed)
/// Compare two counted arrays of 3-float vectors for exact equality.
///
/// Both objects hold a signed count at `+0x00` followed by records of
/// 16 bytes from `+0x10`, of which the first three dwords are compared as
/// floats. Returns 1 when the counts match and every compared float is
/// bit-equal (`ucomiss` equality: NaN never equals, signed zeros equal),
/// else 0. A non-positive count means nothing to compare and returns 1.
/// Original is thiscall(`this`, `other`).
lf_checker_rt::export!(thiscall, rw_00cad240(this: u32, other: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x00;
        const VEC0: u32 = 0x10;
        const STRIDE: u32 = 0x10;
        #[inline(always)]
        unsafe fn dw(base: u32, off: u32) -> u32 {
            unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        let n = dw(this, COUNT) as i32;
        if n != dw(other, COUNT) as i32 {
            return 0;
        }
        let mut i = 0i32;
        while i < n {
            let row = (i as u32).wrapping_mul(STRIDE);
            for k in 0..3u32 {
                let a = f32::from_bits(dw(this, VEC0.wrapping_add(row).wrapping_add(k * 4)));
                let b = f32::from_bits(dw(other, VEC0.wrapping_add(row).wrapping_add(k * 4)));
                if a != b {
                    return 0;
                }
            }
            i += 1;
        }
        1
    }
});
