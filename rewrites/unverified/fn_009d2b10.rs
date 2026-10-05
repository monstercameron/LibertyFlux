// original: 0x009D2B10 pool_array_create (proposed)
//
/// Allocates an array of fixed-size records and initialises each header.
///
/// Allocates `count * 0x88` bytes through the allocator (callee) and, for a
/// positive (signed) count, writes each record's header: `0x34` at
/// `record + 0x80` and `-1` at `record + 0x84`. A non-positive count skips
/// the loop. Note the original's null check only guards the first record:
/// with a null block and `count >= 2` the later iterations compute nonzero
/// record addresses from the null base and fault writing to low memory; the
/// rewrite reproduces that exactly (verified as fault parity). Returns the
/// block (possibly null). Stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_009D2B10(count: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x88;
        const MAGIC_OFF: u32 = 0x80;
        const MAGIC: u32 = 0x34;
        const TAG_OFF: u32 = 0x84;
        const TAG: u32 = 0xffffffff;
        const NEW: u32 = 1;
        let block: u32 = lf_checker_rt::callee_cdecl!(NEW, u32, count.wrapping_mul(STRIDE));
        if (count as i32) <= 0 {
            return block;
        }
        let mut left = count;
        let mut rec = block;
        while left != 0 {
            if rec != 0 {
                (rec.wrapping_add(MAGIC_OFF) as *mut u32).write_unaligned(MAGIC);
                (rec.wrapping_add(TAG_OFF) as *mut u32).write_unaligned(TAG);
            }
            rec = rec.wrapping_add(STRIDE);
            left = left.wrapping_sub(1);
        }
        block
    }
});
