// original: 0x00AF73B0 veh_pool_release (proposed)

/// Release one pooled entry, dropping it when its reference count empties.
///
/// `index` addresses a row of the pool whose metadata is reached through the
/// global pool pointer: flag bytes at `meta + 0x4`, rows at `meta + 0x0`
/// with stride `meta + 0xC`. An entry whose flag byte has bit 7 set is
/// already gone. Otherwise the dword at row + 4 counts down; while it stays
/// positive nothing more happens. At zero, callee 1 is asked whether the
/// entry survives (nonzero keeps it); when it answers zero, callee 2 frees
/// the entry. Nothing is returned.
///
/// Original: 0x00AF73B0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00AF73B0(index: u32) -> u32 {
    unsafe {
        const POOL_META: u32 = 0x16EC7B8;
        const NOTIFY_TAG: u32 = 0x104B888;
        const ASK_KEEP: u32 = 1;
        const FREE_ENTRY: u32 = 2;
        const GONE_BIT: u8 = 0x80;
        let meta = (lf_checker_rt::global::<u32>(POOL_META) as *const u32).read_unaligned();
        let flags = ((meta + 4) as *const u32).read_unaligned();
        if ((flags.wrapping_add(index)) as *const u8).read() & GONE_BIT != 0 {
            return 0;
        }
        let stride = ((meta + 0xC) as *const u32).read_unaligned();
        let row = ((meta as *const u32).read_unaligned()).wrapping_add(stride.wrapping_mul(index));
        if row == 0 {
            return 0;
        }
        let refs = ((row + 4) as *mut u32).read_unaligned().wrapping_sub(1);
        ((row + 4) as *mut u32).write_unaligned(refs);
        if (refs as i32) > 0 {
            return 0;
        }
        let tag = (lf_checker_rt::global::<u32>(NOTIFY_TAG) as *const u32).read_unaligned();
        let keep = lf_checker_rt::callee_cdecl!(ASK_KEEP, u32, index, tag);
        if (keep as u8) != 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(FREE_ENTRY, u32, index);
        0
    }
});
