// original: 0x00DB3320 UITexture::set_cached_source_bounds

/// Copy the two 64-bit halves of a source-bounds record into the texture's
/// cached bounds fields at byte offsets 0x214 and 0x21C. The receiver is in
/// ECX and the source pointer is the single stack argument; the callee pops
/// that argument. The copy is byte preserving, including arbitrary float or
/// padding bit patterns in the record.
lf_checker_rt::export!(thiscall, rw_00db3320(this: u32, source: u32) -> u32 {
    unsafe {
        const SOURCE_LOW: u32 = 0;
        const SOURCE_HIGH: u32 = 8;
        const CACHED_BOUNDS_LOW: u32 = 0x214;
        const CACHED_BOUNDS_HIGH: u32 = 0x21c;

        let low = ((source + SOURCE_LOW) as *const u64).read_unaligned();
        let high = ((source + SOURCE_HIGH) as *const u64).read_unaligned();
        ((this + CACHED_BOUNDS_LOW) as *mut u64).write_unaligned(low);
        ((this + CACHED_BOUNDS_HIGH) as *mut u64).write_unaligned(high);
    }
    source
});
