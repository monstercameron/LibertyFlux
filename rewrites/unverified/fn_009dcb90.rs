// original: 0x009DCB90 pool_vec_init_ctor_0x6c (proposed)

/// Allocate a pool-vector buffer like its siblings, then construct each element.
///
/// Same 12-byte header and saturated `count * 0x6C + 4` allocation as the
/// plain vector initialisers, except every element is built by a per-element
/// constructor call instead of receiving a vtable stamp. Returns 0 when the
/// allocation fails, the buffer itself when the count is zero, and the last
/// constructor's answer otherwise.
///
/// Original: 0x009DCB90 (thiscall, no stack arguments, two outgoing calls).
lf_checker_rt::export!(thiscall, rw_009DCB90(this: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x6C;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;

        let count = (this as *const u32).read_unaligned();
        ((this as *mut u32).wrapping_add(1)).write_unaligned(0);
        let prod = (count as u64).wrapping_mul(STRIDE as u64);
        let size = if prod > 0xFFFF_FFFFu64 {
            0xFFFF_FFFF
        } else {
            (prod as u32).checked_add(4).unwrap_or(0xFFFF_FFFF)
        };
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, size);
        if buf == 0 {
            ((this as *mut u32).wrapping_add(2)).write_unaligned(0);
            return 0;
        }
        (buf as *mut u32).write_unaligned(count);
        let body = buf.wrapping_add(4);
        let mut built = buf;
        let mut slot = body;
        let mut left = count;
        while left != 0 {
            built = lf_checker_rt::callee_thiscall!(CTOR, u32, slot);
            slot = slot.wrapping_add(STRIDE);
            left -= 1;
        }
        ((this as *mut u32).wrapping_add(2)).write_unaligned(body);
        built
    }
});
