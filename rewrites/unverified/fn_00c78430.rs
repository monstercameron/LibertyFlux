// original: 0x00C78430 phbound_registry_find_by_pointer (proposed)

/// Convert the input key with a helper, then scan the global registry table.
/// The signed table count controls the loop; flagged slots and null entries
/// are skipped. A matching entry returns its zero-based index, while an empty
/// or exhausted search returns `u32::MAX`.
lf_checker_rt::export!(cdecl, rw_phbound_registry_find_by_pointer(key_input: u32) -> u32 {
    unsafe {
        const HASH_KEY: u32 = 1;
        const TABLE_GLOBAL: u32 = 0x016e_c7b8;
        const TABLE_ENTRIES: u32 = 0;
        const TABLE_FLAGS: u32 = 4;
        const TABLE_COUNT: u32 = 8;
        const TABLE_STRIDE: u32 = 0x0c;
        const ENTRY_KEY: u32 = 8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_volatile() }
        }

        let key = lf_checker_rt::callee_cdecl!(HASH_KEY, u32, key_input);
        let table = unsafe { lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_volatile() };
        let entries = unsafe { read_u32(table + TABLE_ENTRIES) };
        let flags = unsafe { read_u32(table + TABLE_FLAGS) };
        let count = unsafe { read_u32(table + TABLE_COUNT) } as i32;
        let stride = unsafe { read_u32(table + TABLE_STRIDE) };

        let mut index = 0u32;
        while (index as i32) < count {
            if unsafe { read_u8(flags.wrapping_add(index)) } & 0x80 == 0 {
                let entry = entries.wrapping_add(stride.wrapping_mul(index));
                if entry != 0 && unsafe { read_u32(entry + ENTRY_KEY) } == key {
                    return index;
                }
            }
            index = index.wrapping_add(1);
        }
        u32::MAX
    }
});
