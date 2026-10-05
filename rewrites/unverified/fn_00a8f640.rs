// original: 0x00A8F640 pool_slot_flag_bit10 (proposed)

/// Test bit 10 of the flag word behind a pool slot handle.
///
/// Same lookup as the sibling datum readers: the signed 16-bit handle at
/// `this+0x2E` indexes the global pool table, entry `+0x70` is the datum
/// page, and the flag word at page `+0x6C` is tested. Returns bit 10 (value
/// 1 or 0). Pure loads, no calls.
///
/// Original: thiscall, no stack arguments, returns u32 in EAX.
lf_checker_rt::export!(thiscall, rw_00A8F640(this: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const FLAGS_OFF: u32 = 0x6c;
        const BIT: u32 = 10;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        (((page + FLAGS_OFF) as *const u32).read_unaligned() >> BIT) & 1
    }
});
