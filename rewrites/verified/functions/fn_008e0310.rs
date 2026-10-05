// original: 0x008e0310 pool_slot_occupied (proposed)

/// Whether a pool slot currently holds a live entry.
///
/// The pool context is the global at `CTX`: word `+0x00` is the entry base,
/// `+0x04` the per-slot flag bytes, `+0x0c` the entry stride. Slot `index`
/// is dead when its flag byte has bit `0x80` set (returns 0); otherwise
/// returns whether the computed entry address (`base + stride * index`) is
/// non-zero. Cdecl, one stack argument, no calls.
lf_checker_rt::export!(cdecl, rw_008e0310(index: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const ENTRY_BASE_OFF: u32 = 0x00;
        const FLAG_BASE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const DEAD_FLAG: u8 = 0x80;
        let ctx = lf_checker_rt::global::<u32>(CTX).read_unaligned();
        let flag_base = ((ctx + FLAG_BASE_OFF) as *const u32).read_unaligned();
        let flag = (flag_base.wrapping_add(index) as *const u8).read();
        if flag & DEAD_FLAG != 0 {
            0
        } else {
            let stride = ((ctx + STRIDE_OFF) as *const u32).read_unaligned();
            let base = ((ctx + ENTRY_BASE_OFF) as *const u32).read_unaligned();
            u32::from(base.wrapping_add(stride.wrapping_mul(index)) != 0)
        }
    }
});
