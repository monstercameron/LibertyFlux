// original: 0x008FB170 stream_pool_alloc
/// Allocate and reset a pool of streaming entries.
///
/// Stores the count, allocates `count * 0x54` bytes (saturating the
/// size to 0xFFFFFFFF on overflow) through the allocator call, then
/// writes both -1 links of every entry and resets each through the
/// entry routine. A non-positive count skips the loop. Thiscall, one
/// stack argument; returns the last per-entry result, or the array
/// base when the loop is skipped.
export!(thiscall, rw_008fb170(this: u32, count: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 0x54;
        ((this + 4) as *mut u32).write_unaligned(count);
        let full = (count as u64).wrapping_mul(ENTRY as u64);
        let size = if full > 0xFFFFFFFF { 0xFFFFFFFF } else { full as u32 };
        let base: u32 = callee_cdecl!(1, u32, size);
        ((this) as *mut u32).write_unaligned(base);
        let mut ans = base;
        let n = count as i32;
        if n > 0 {
            let mut i: i32 = 0;
            let mut off: u32 = 0;
            while i < n {
                let e = base.wrapping_add(off);
                ((e + 0x48) as *mut u32).write_unaligned(0xFFFFFFFF);
                ((e + 0x4c) as *mut u32).write_unaligned(0xFFFFFFFF);
                ans = callee_thiscall!(2, u32, e);
                i += 1;
                off = off.wrapping_add(ENTRY);
            }
        }
        ans
    }
});
