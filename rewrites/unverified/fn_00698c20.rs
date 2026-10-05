// original: 0x00698C20 bit_cost_sum

/// Estimate the bit cost of a quantizer table under a shift.
///
/// `fastcall` with the table object in ECX and a range word in EDX, no stack
/// arguments. Derives a shift from the index of the top set bit of the range
/// (0 when the range is 0 or 1), then sums over the signed-16 count at `+4`
/// of the absolute values in the array at `+0`, shifted down, plus per-entry
/// overhead. Returns the total.
/// Original: 0x00698C20, 68 bytes.
lf_checker_rt::export!(fastcall, rw_00698C20(obj: u32, bits: u32) -> u32 {
    unsafe {
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        let shift: u32 = if bits < 2 { 0 } else { 31 - bits.leading_zeros() };
        let arr = rd32(obj);
        let count = rd16(obj + 4) as u32;
        let mut total: u32 = 0;
        let mut i: u32 = 0;
        while i < count {
            let v = rd32(arr + i * 4) as i32;
            let mag: u32 = if v < 0 { v.wrapping_neg() as u32 } else { v as u32 };
            total = total.wrapping_add(1).wrapping_add((mag >> shift).wrapping_add(shift));
            if v != 0 {
                total = total.wrapping_add(1);
            }
            i += 1;
        }
        total
    }
});
