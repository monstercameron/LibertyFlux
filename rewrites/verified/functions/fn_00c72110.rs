// original: 0x00C72110 CAnimAssociations::vf2

/// Release every non-null association stored in the global animation table,
/// release the table itself, then clear its three global state words. The
/// signed table count is at the first global, the table pointer is at the
/// second, and each entry's owned pointer is at `+0x40 + index*0x58`. Calls to
/// the allocator's release routine are observed through one cdecl stand-in.
lf_checker_rt::export!(cdecl, rw_00c72110() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        const GLOBAL_COUNT: u32 = 0x016d_d638;
        const GLOBAL_TABLE: u32 = 0x016d_d640;
        const GLOBAL_AUX: u32 = 0x016d_d644;
        const ENTRY_POINTERS: u32 = 0x40;
        const ENTRY_STRIDE: u32 = 0x58;
        const RELEASE: u32 = 1;

        let count = read_u32(lf_checker_rt::relocated(GLOBAL_COUNT)) as i32;
        let table = read_u32(lf_checker_rt::relocated(GLOBAL_TABLE));
        if count > 0 {
            for index in 0..count as u32 {
                let pointer_slot = table
                    .wrapping_add(ENTRY_POINTERS)
                    .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
                let owned_pointer = read_u32(pointer_slot);
                if owned_pointer != 0 {
                    let _ = lf_checker_rt::callee_cdecl!(RELEASE, u32, owned_pointer);
                    write_u32(pointer_slot, 0);
                }
            }
        }

        let table = read_u32(lf_checker_rt::relocated(GLOBAL_TABLE));
        let _ = lf_checker_rt::callee_cdecl!(RELEASE, u32, table);
        write_u32(lf_checker_rt::relocated(GLOBAL_TABLE), 0);
        write_u32(lf_checker_rt::relocated(GLOBAL_AUX), 0);
        write_u32(lf_checker_rt::relocated(GLOBAL_COUNT), 0);
    }
    0
});
