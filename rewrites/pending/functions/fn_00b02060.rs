// original: 0x00b02060 swap_and_sift_down
/// Swap the 8-byte record at `lo` with the one ending at `hi`, then
/// sift-down from `lo` over (hi-lo-8)/8 records. Returns that result.
export!(cdecl, rw_00b02060(lo: u32, hi: u32, c: u32) -> u32 {
    unsafe {
        let a0 = (lo as *const u32).read_unaligned();
        let dx = (hi.wrapping_sub(8) as *const u32).read_unaligned();
        let cx = (hi.wrapping_sub(4) as *const u32).read_unaligned();
        (hi.wrapping_sub(8) as *mut u32).write_unaligned(a0);
        let a1 = (lo.wrapping_add(4) as *const u32).read_unaligned();
        (hi.wrapping_sub(4) as *mut u32).write_unaligned(a1);
        let count = ((hi.wrapping_sub(lo).wrapping_sub(8) as i32) >> 3) as u32;
        callee_cdecl!(1, u32, lo, 0, count, dx, cx, c)
    }
});
