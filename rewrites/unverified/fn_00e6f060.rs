// original: 0x00E6F060 register_image_table_range

/// Stores a relocated constructor object pointer, calls the intercepted
/// thiscall initializer with the pointer global in ECX, then writes the
/// relocated entry address 32 times into a sparse table. Entries descend
/// by stride 0x3A4; EAX returns the lowest entry address.
lf_checker_rt::export!(cdecl, rw_00e6f060() -> u32 {
    const POINTER_GLOBAL_VA: u32 = 0x019E8678;
    const INITIAL_OBJECT_VA: u32 = 0x00FCCAC0;
    const TABLE_BASE_VA: u32 = 0x019EFB2C;
    const TABLE_ENTRY_VA: u32 = 0x00FCB7C0;
    const TABLE_STRIDE: u32 = 0x3A4;
    const TABLE_COUNT: u32 = 32;

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let pointer_global = lf_checker_rt::relocated(POINTER_GLOBAL_VA);
        write_u32(pointer_global, lf_checker_rt::relocated(INITIAL_OBJECT_VA));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, pointer_global);
        for index in 1..=TABLE_COUNT {
            let entry_va = TABLE_BASE_VA - TABLE_STRIDE * index;
            let entry = lf_checker_rt::relocated(entry_va);
            write_u32(entry, lf_checker_rt::relocated(TABLE_ENTRY_VA));
        }
        lf_checker_rt::relocated(TABLE_BASE_VA) - TABLE_STRIDE * TABLE_COUNT
    }
});
