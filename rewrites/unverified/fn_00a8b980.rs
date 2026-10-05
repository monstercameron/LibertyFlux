// original: 0x00a8b980 pool_grow_array (proposed)

/// Append one 0x30-byte record to this array, allocating on first use.
///
/// `this` holds the buffer at +0x150, the live count at +0x154 (16-bit)
/// and the capacity at +0x156 (16-bit); `src` points to the record and
/// `count` is the capacity to allocate. When the live count is zero the
/// buffer is allocated (empty when `count` is zero) and the capacity and
/// buffer stored; otherwise the existing buffer is reused. The record is
/// copied to buffer+index*0x30 and the live count incremented. Returns
/// `src`, as the original leaves it in eax. The proof pins the index to
/// 0..2 and the count to 0..4.
///
/// Original: 0x00A8B980 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a8b980(this: u32, src: u32, count: u32) -> u32 {
    unsafe {
        const CALLEE_ALLOC: u32 = 1;
        const BUF: u32 = 0x150;
        const LIVE: u32 = 0x154;
        const CAP: u32 = 0x156;
        const RECORD: u32 = 0x30;
        let live = ((this + LIVE) as *const u16).read_unaligned();
        if live == 0 {
            let buf = if count == 0 {
                0
            } else {
                let bytes = count.wrapping_mul(RECORD);
                lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, bytes)
            };
            ((this + CAP) as *mut u16).write_unaligned(count as u16);
            ((this + BUF) as *mut u32).write_unaligned(buf);
        }
        let index = ((this + LIVE) as *const u16).read_unaligned();
        ((this + LIVE) as *mut u16)
            .write_unaligned(index.wrapping_add(1));
        let base = ((this + BUF) as *const u32).read_unaligned();
        let dst = base.wrapping_add((index as u32).wrapping_mul(RECORD));
        let mut off = 0u32;
        while off < RECORD {
            let w = ((src + off) as *const u32).read_unaligned();
            ((dst + off) as *mut u32).write_unaligned(w);
            off += 4;
        }
        src
    }
});
