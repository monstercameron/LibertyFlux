// original: 0x00698C20 packed_cost_total (proposed)

/// Cost-weighted sum over a small integer array.
///
/// `obj` points at a two-field header: the array pointer at `+0` and a
/// 16-bit element count at `+4`. `scale` sets the cost shift: one less than
/// its bit length (`0` when `scale` is 0 or 1, up to 31). Each element `x`
/// contributes `1 + ((abs(x) >> shift) + shift) + (x != 0) as u32`, all
/// wrapping. A zero count yields 0.
///
/// Original: fastcall `(obj: ecx, scale: edx) -> eax`, no calls.
lf_checker_rt::export!(fastcall, rw_00698C20(obj: u32, scale: u32) -> u32 {
    unsafe {
        const ARR_PTR: u32 = 0;
        const COUNT: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        let shift: u32 = if scale == 0 { 0 } else { 31 - scale.leading_zeros() };
        let count = rd16(obj.wrapping_add(COUNT)) as u32;
        if (count as i32) <= 0 {
            return 0;
        }
        let arr = rd32(obj.wrapping_add(ARR_PTR));
        let mut total: u32 = 0;
        let mut i: u32 = 0;
        while i < count {
            let x = rd32(arr.wrapping_add(i.wrapping_mul(4)));
            let mag = (x as i32).wrapping_abs() as u32;
            total = total.wrapping_add(1);
            total = total.wrapping_add(mag.wrapping_shr(shift).wrapping_add(shift));
            if x != 0 {
                total = total.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        total
    }
});
