// original: 0x005C34B0 ped_factory_run_slot_cleanup

/// Runs the category predicate for an indexed collection entry. A nonzero
/// predicate result dispatches the category action immediately. Otherwise,
/// the manager bitmap must mark the index available before the entry table is
/// inspected; a nonzero entry value triggers the cleanup helper. The manager
/// and category pointer are relocated globals. The original faults for a set
/// exclusion bit or an invalid table address, which the bounded contract
/// omits.
lf_checker_rt::export!(cdecl, rw_005c34b0(index: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x015f8ba4;
        const CATEGORY_GLOBAL: u32 = 0x0103fc40;
        const ITEMS_POINTER: u32 = 0x00;
        const FLAGS_POINTER: u32 = 0x04;
        const ENTRY_STRIDE: u32 = 0x0c;
        const EXCLUDED_MASK: u8 = 0x80;
        const PREDICATE_CALLEE: u32 = 1;
        const CATEGORY_ACTION_CALLEE: u32 = 2;
        const CLEANUP_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let category = lf_checker_rt::global::<u32>(CATEGORY_GLOBAL).read_unaligned();
        let should_dispatch: u32 =
            lf_checker_rt::callee_cdecl!(PREDICATE_CALLEE, u32, category, index);
        if should_dispatch as u8 != 0 {
            let _: u32 =
                lf_checker_rt::callee_cdecl!(CATEGORY_ACTION_CALLEE, u32, category, index);
            return 0;
        }

        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
        let flags = read_word(manager + FLAGS_POINTER);
        if read_byte(flags.wrapping_add(index)) & EXCLUDED_MASK != 0 {
            return 0;
        }
        let items = read_word(manager + ITEMS_POINTER);
        let stride = read_word(manager + ENTRY_STRIDE);
        let entry = items.wrapping_add(stride.wrapping_mul(index));
        if read_word(entry) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(CLEANUP_CALLEE, u32, index);
        }
        0
    }
});
