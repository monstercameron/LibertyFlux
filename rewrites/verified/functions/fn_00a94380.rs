// original: 0x00a94380 stream_entry_block_count (proposed)

/// Number of whole 2K-unit blocks in a stream entry's stored size, rounded up.
///
/// The entry holds a size in units of 4 bytes at `+0x08`. The result is
/// `ceil((size >> 2) / 2048)`, computed as `((size >> 2) + 0x7ff) >> 11`.
/// Pure leaf: no calls, no globals, no writes.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a94380(this: u32) -> u32 {
    unsafe {
        const ENT_SIZE: u32 = 0x08;
        const ROUND_UP: u32 = 0x7ff;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let units = rd32(this.wrapping_add(ENT_SIZE)) >> 2;
        units.wrapping_add(ROUND_UP) >> 11
    }
});
