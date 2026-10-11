// original: 0x00C78190 phbound_register_all_entries (proposed)

/// Visit the registry's fixed 330 slots. Flagged slots and null entries are
/// skipped. For each live entry, pass its `+8` token and slot index in a
/// two-word scratch record, together with the fixed registration mode, to
/// the registration helper.
lf_checker_rt::export!(cdecl, rw_phbound_register_all_entries() -> () {
    unsafe {
        const TABLE_GLOBAL: u32 = 0x016e_c7b8;
        const TABLE_ENTRIES: u32 = 0;
        const TABLE_FLAGS: u32 = 4;
        const TABLE_STRIDE: u32 = 0x0c;
        const ENTRY_TOKEN: u32 = 8;
        const REGISTRY_ENTRY_COUNT: u32 = 0x14a;
        const REGISTER: u32 = 1;
        const REGISTER_MODE: u32 = 8;

        #[repr(C)]
        struct RegisterCallData {
            token: u32,
            index: u32,
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read_volatile() }
        }

        let table = unsafe { lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_volatile() };
        let entries = unsafe { read_u32(table + TABLE_ENTRIES) };
        let flags = unsafe { read_u32(table + TABLE_FLAGS) };
        let stride = unsafe { read_u32(table + TABLE_STRIDE) };

        for index in 0..REGISTRY_ENTRY_COUNT {
            if unsafe { read_u8(flags.wrapping_add(index)) } & 0x80 != 0 {
                continue;
            }
            let entry = entries.wrapping_add(stride.wrapping_mul(index));
            if entry == 0 {
                continue;
            }
            let mut call_data = RegisterCallData {
                token: unsafe { read_u32(entry + ENTRY_TOKEN) },
                index,
            };
            let call_data_pointer = core::ptr::addr_of_mut!(call_data) as u32;
            let _ = lf_checker_rt::callee_cdecl!(REGISTER, u32, call_data_pointer, REGISTER_MODE);
        }
    }
});
