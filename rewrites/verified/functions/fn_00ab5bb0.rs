// original: 0x00ab5bb0 idmap_init (proposed)

/// Create an empty id map with a scripted bucket count and return it.
///
/// Zeroes the header (`+0` half-word, words at `+4` and `+8`, byte at
/// `+0xF`), asks the sizing callee for a bucket count with subject `0x10`,
/// stores it as the half-word at `+8`, allocates `count * 4` bytes through
/// the allocator callee into `+4`, zeroes every bucket, marks the header
/// byte at `+0xF` ready and returns the map. A null allocation with a
/// nonzero count faults on the first bucket store, on both sides alike.
///
/// Callees: 1 = bucket-count sizing (cdecl, one word),
/// 2 = allocator (cdecl, one word).
///
/// Original: 0x00ab5bb0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5bb0(this: u32) -> u32 {
    unsafe {
        const SIZING: u32 = 1;
        const ALLOC: u32 = 2;
        const SUBJECT: u32 = 0x10;
        const TABLE_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        const READY_OFF: u32 = 0x0F;
        ((this + TABLE_OFF) as *mut u32).write_unaligned(0);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(0);
        ((this + READY_OFF) as *mut u8).write(0);
        (this as *mut u16).write_unaligned(0);
        let count = lf_checker_rt::callee_cdecl!(SIZING, u32, SUBJECT) & 0xFFFF;
        ((this + COUNT_OFF) as *mut u16).write_unaligned(count as u16);
        // count * 4 with the original's overflow rule (unreachable for a
        // 16-bit count, kept for shape).
        let (size, overflow) = (count as u32).overflowing_mul(4);
        let bytes = if overflow { 0xFFFF_FFFF } else { size };
        let table = lf_checker_rt::callee_cdecl!(ALLOC, u32, bytes);
        ((this + TABLE_OFF) as *mut u32).write_unaligned(table);
        let mut i = 0u32;
        while i < count as u32 {
            ((table + 4 * i) as *mut u32).write_unaligned(0);
            i += 1;
        }
        ((this + READY_OFF) as *mut u8).write(1);
        this
    }
});
