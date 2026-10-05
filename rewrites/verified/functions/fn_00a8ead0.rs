// original: 0x00A8EAD0 pool_slot_datum_field (proposed)

/// Read a field from the datum behind a pool slot handle.
///
/// `this` points to a slot whose signed 16-bit handle sits at `+0x2E`. The
/// handle indexes the global pool table (one pointer per slot); entry `+0x70`
/// is the datum page and `+8` within it the returned field. Pure loads, no
/// calls. Out-of-range handles read whatever the table memory holds and may
/// fault, exactly like the original.
///
/// Original: thiscall, no stack arguments, returns u32 in EAX.
lf_checker_rt::export!(thiscall, rw_00A8EAD0(this: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 0x2e;
        const POOL_TABLE: u32 = 0x1295cd8;
        const PAGE_OFF: u32 = 0x70;
        const FIELD_OFF: u32 = 8;
        let idx = ((this + HANDLE_OFF) as *const i16).read_unaligned() as i32 as u32;
        let entry = (lf_checker_rt::relocated(POOL_TABLE)
            .wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let page = ((entry + PAGE_OFF) as *const u32).read_unaligned();
        ((page + FIELD_OFF) as *const u32).read_unaligned()
    }
});
