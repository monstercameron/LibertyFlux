// original: 0x00888040 stream_slot_index (proposed)

/// Index of a stream-slot record from its address, or -1 for null.
///
/// A null pointer yields 0xffff_ffff. Otherwise the byte distance from the
/// table base (the table-base global) is divided by the 0xa0-byte stride
/// with the original's multiply-and-shift sequence: the high word of the
/// unsigned product with 0xcccc_cccd, shifted right by 7.
///
/// Original: 0x00888040 (cdecl, one stack word; callee pops nothing).
lf_checker_rt::export!(cdecl, rw_00888040(ptr: u32) -> u32 {
    unsafe {
        const TABLE_GLOBAL: u32 = 0x0115_a46c;
        const MAGIC: u64 = 0xcccc_cccd;
        if ptr == 0 {
            0xffff_ffff
        } else {
            let base = (lf_checker_rt::relocated(TABLE_GLOBAL) as *const u32)
                .read_unaligned();
            let d = ptr.wrapping_sub(base);
            (((MAGIC * d as u64) >> 32) as u32) >> 7
        }
    }
});
