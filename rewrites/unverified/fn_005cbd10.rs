// original: 0x005CBD10 HUD_WEAPON_SCOPE_BIT_3

/// Applies the scope HUD state when the weapon mode and prior request allow
/// it. A seven-argument cdecl query returns an object-table index; the chosen
/// object's thiscall helper receives the fixed scope resource pointer, then
/// the routine sets the object's enabled word and stores the index globally.
/// The function's EAX value is incidental on the early-return path.
lf_checker_rt::export!(cdecl, rw_005cbd10() -> u32 {
    unsafe {
        const WEAPON_MODE_GLOBAL: u32 = 0x011d6fd4;
        const REQUEST_INDEX_GLOBAL: u32 = 0x0118f028;
        const FIRST_QUERY_VALUE: u32 = 0x0118f03c;
        const SECOND_QUERY_VALUE: u32 = 0x0118f040;
        const OBJECT_TABLE: u32 = 0x0118e7f8;
        const OBJECT_ENABLED_OFFSET: u32 = 0x14;
        const EXPECTED_WEAPON_MODE: u32 = 2;
        const RESOURCE_POINTER: u32 = 0x019d3b20;
        const QUERY_CALLEE: u32 = 1;
        const APPLY_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_global(address: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(address).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_global(address: u32, value: u32) {
            unsafe { lf_checker_rt::global::<u32>(address).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        if read_global(WEAPON_MODE_GLOBAL) != EXPECTED_WEAPON_MODE || read_global(REQUEST_INDEX_GLOBAL) != 0 {
            return 0;
        }

        let object_index: u32 = lf_checker_rt::callee_cdecl!(
            QUERY_CALLEE,
            u32,
            lf_checker_rt::relocated(0x00f8f3cc),
            2,
            lf_checker_rt::relocated(0x0118f02c),
            lf_checker_rt::relocated(0x0118f034),
            2,
            read_global(SECOND_QUERY_VALUE),
            read_global(FIRST_QUERY_VALUE)
        );
        let object_slot = OBJECT_TABLE.wrapping_add(object_index.wrapping_mul(4));
        let object = read_global(object_slot);
        write_global(REQUEST_INDEX_GLOBAL, object_index);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            APPLY_CALLEE,
            u32,
            object,
            lf_checker_rt::relocated(RESOURCE_POINTER)
        );
        write_word(object.wrapping_add(OBJECT_ENABLED_OFFSET), 1);
        object
    }
});
