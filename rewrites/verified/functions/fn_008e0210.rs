// original: 0x008e0210 pool_slot_data_word (proposed)

/// Data word of a pool slot: the word at `entry + 4` for slot `index`, with
/// the entry address computed as in `pool_slot_occupied` from the context
/// global at `CTX` (`+0x00` entry base, `+0x04` flag bytes, `+0x0c` stride).
/// When the slot's flag byte has bit `0x80` set the original reads through
/// a null pointer instead, faulting; the rewrite performs the same read so
/// both sides fault identically. Cdecl, one stack argument, no calls.
lf_checker_rt::export!(cdecl, rw_008e0210(index: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const ENTRY_BASE_OFF: u32 = 0x00;
        const FLAG_BASE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const DATA_OFF: u32 = 0x04;
        const DEAD_FLAG: u8 = 0x80;
        let ctx = lf_checker_rt::global::<u32>(CTX).read_unaligned();
        let flag_base = ((ctx + FLAG_BASE_OFF) as *const u32).read_unaligned();
        let flag = (flag_base.wrapping_add(index) as *const u8).read();
        if flag & DEAD_FLAG != 0 {
            (DATA_OFF as *const u32).read_unaligned()
        } else {
            let stride = ((ctx + STRIDE_OFF) as *const u32).read_unaligned();
            let base = ((ctx + ENTRY_BASE_OFF) as *const u32).read_unaligned();
            let entry = base.wrapping_add(stride.wrapping_mul(index));
            ((entry + DATA_OFF) as *const u32).read_unaligned()
        }
    }
});
