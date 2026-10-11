// original: 0x00c72180 animation_manager_cleanup

/// Dispose of the current animation manager through its first virtual slot,
/// release each non-null item in the global item table, release that table,
/// and clear the manager, table, and 16-bit item-count globals. The table
/// count is zero-extended before iteration; null items skip the item cleanup
/// and free calls but their table slots are still cleared. The virtual call
/// takes the manager in ECX and one stack argument with value one.
lf_checker_rt::export!(cdecl, rw_00c72180() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        const GLOBAL_MANAGER: u32 = 0x016d_d634;
        const GLOBAL_ITEMS: u32 = 0x016d_d654;
        const GLOBAL_ITEM_COUNT: u32 = 0x016d_d658;
        const ITEM_CLEANUP: u32 = 2;
        const RELEASE: u32 = 3;

        let manager = read_u32(lf_checker_rt::relocated(GLOBAL_MANAGER));
        if manager != 0 {
            let vtable = read_u32(manager);
            let cleanup_address = read_u32(vtable);
            let cleanup: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(cleanup_address as usize);
            let _ = cleanup(manager, 1);
            write_u32(lf_checker_rt::relocated(GLOBAL_MANAGER), 0);
        }

        let item_count = read_u32(lf_checker_rt::relocated(GLOBAL_ITEM_COUNT)) as u16;
        for index in 0..u32::from(item_count) {
            let table = read_u32(lf_checker_rt::relocated(GLOBAL_ITEMS));
            let item_slot = table.wrapping_add(index.wrapping_mul(4));
            let item = read_u32(item_slot);
            if item != 0 {
                let _ = lf_checker_rt::callee_thiscall!(ITEM_CLEANUP, u32, item);
                let _ = lf_checker_rt::callee_cdecl!(RELEASE, u32, item);
            }

            let table_after_calls = read_u32(lf_checker_rt::relocated(GLOBAL_ITEMS));
            write_u32(table_after_calls.wrapping_add(index.wrapping_mul(4)), 0);
        }

        let table = read_u32(lf_checker_rt::relocated(GLOBAL_ITEMS));
        let _ = lf_checker_rt::callee_cdecl!(RELEASE, u32, table);
        write_u32(lf_checker_rt::relocated(GLOBAL_ITEMS), 0);
        write_u32(lf_checker_rt::relocated(GLOBAL_ITEM_COUNT), 0);
    }
    0
});
