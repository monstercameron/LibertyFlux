// original: 0x00d29000 targeting_allocator_init (proposed)

/// Create the targeting allocator on first use and publish it globally.
///
/// Allocates a 28-byte block with the heap helper (intercepted); when that
/// succeeds, constructs the allocator in it with the setup helper
/// (intercepted, fixed arguments `0x50`, table `0xEE1B4C`, flags `0x270`) and
/// publishes the pointer to the global slot. A failed allocation leaves the
/// global untouched. Returns the published pointer, or null.
///
/// Original: 0x00D29000 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00d29000() -> u32 {
    unsafe {
        const BLOCK_SIZE: u32 = 0x1c;
        const GLOBAL_SLOT: u32 = 0x0171_f9c0;
        const SETUP_A0: u32 = 0x50;
        const SETUP_TABLE_VA: u32 = 0x00ee_1b4c;
        const SETUP_FLAGS: u32 = 0x270;
        let block: u32 = lf_checker_rt::callee_cdecl!(1, u32, BLOCK_SIZE);
        if block == 0 {
            return 0;
        }
        // The table address is an image address: relocate it like the loader
        // does for the original's immediate operand.
        let table = lf_checker_rt::relocated(SETUP_TABLE_VA);
        let alloc: u32 = lf_checker_rt::callee_thiscall!(2, u32, block, SETUP_A0, table, SETUP_FLAGS);
        unsafe { (lf_checker_rt::relocated(GLOBAL_SLOT) as *mut u32).write_unaligned(alloc) };
        alloc
    }
});
