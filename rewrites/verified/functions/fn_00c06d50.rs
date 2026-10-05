// original: 0x00c06d50 stream_alloc_clear_pairs
/// Allocate `count` 40-byte records and clear two words in each.
///
/// Allocates `count * 40` bytes (cdecl/1) and, for a positive `count`, zeroes
/// the words at +0x20 and +0x24 of every element (skipping a null base).
/// Returns the allocated pointer. With a null base and `count` above 1 the
/// original faults writing through a small computed address; the rewrite
/// computes the same addresses so the fault parity holds. Stdcall: one stack
/// word, callee cleans 4.
lf_checker_rt::export!(stdcall, rw_00c06d50(count: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const ALLOC: u32 = 1;
        const STRIDE: u32 = 40;
        let bytes = (count as i32).wrapping_mul(STRIDE as i32) as u32;
        let base: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, bytes);
        if (count as i32) > 0 {
            let mut elem = base;
            let mut left = count as i32;
            while left != 0 {
                if elem != 0 {
                    wr32(elem + 0x20, 0);
                    wr32(elem + 0x24, 0);
                }
                elem = elem.wrapping_add(STRIDE);
                left -= 1;
            }
        }
        base
    }
});
