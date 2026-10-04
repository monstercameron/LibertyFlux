// original: 0x00cad190 route_records_equal (proposed)
/// Compare two counted route records: count, positions and names.
///
/// Both objects hold a signed count at `+0x00`. Each entry contributes a
/// 3-float position (compared bit-exactly, NaN never equal) at `+0x1D0` with
/// stride 16, and a name record at `+0x04` with stride `0x38` compared through
/// the string-pair helper (intercepted callee 1, thiscall/1). Returns 1 when
/// the counts match and every entry agrees, else 0; a non-positive count
/// compares nothing and returns 1. Original is thiscall(`this`, `other`).
lf_checker_rt::export!(thiscall, rw_00cad190(this: u32, other: u32) -> u32 {
    unsafe {
        const CMP: u32 = 1;
        const COUNT: u32 = 0x00;
        const POS0: u32 = 0x1D0;
        const POS_STRIDE: u32 = 0x10;
        const NAME0: u32 = 0x04;
        const NAME_STRIDE: u32 = 0x38;
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
            let prow = (i as u32).wrapping_mul(POS_STRIDE);
            for k in 0..3u32 {
                let a = f32::from_bits(dw(this, POS0.wrapping_add(prow).wrapping_add(k * 4)));
                let b = f32::from_bits(dw(other, POS0.wrapping_add(prow).wrapping_add(k * 4)));
                if a != b {
                    return 0;
                }
            }
            let nrow = (i as u32).wrapping_mul(NAME_STRIDE);
            let r: u32 = lf_checker_rt::callee_thiscall!(CMP, u32,
                this.wrapping_add(NAME0).wrapping_add(nrow),
                other.wrapping_add(NAME0).wrapping_add(nrow));
            if r & 0xFF == 0 {
                return 0;
            }
            i += 1;
        }
        1
    }
});
