// original: 0x00A8F870 vehicle_model_flag_bit9

/// Read one vehicle-model flag through the pool handle's page entry.
///
/// The signed 16-bit handle at byte offset `0x2e` selects a pointer in the
/// pool table. The entry's page pointer is at `+0x70`; the selected flag word
/// is at page offset `0x6c`. The function returns bit 9 as a full 32-bit
/// value. Thiscall takes only the object pointer in ECX.
lf_checker_rt::export!(thiscall, rw_00a8f870(this: u32) -> u32 {
    unsafe {
        const HANDLE_OFFSET: u32 = 0x2e;
        const POOL_POINTER_TABLE: u32 = 0x0129_5cd8;
        const PAGE_POINTER_OFFSET: u32 = 0x70;
        const PAGE_FLAGS_OFFSET: u32 = 0x6c;
        const FLAG_BIT: u32 = 9;

        let handle = ((this + HANDLE_OFFSET) as *const i16).read_unaligned();
        let table_slot = lf_checker_rt::relocated(POOL_POINTER_TABLE)
            .wrapping_add((handle as i32 as u32).wrapping_mul(4));
        let pool_entry = (table_slot as *const u32).read_unaligned();
        let page = ((pool_entry + PAGE_POINTER_OFFSET) as *const u32).read_unaligned();
        let flags = ((page + PAGE_FLAGS_OFFSET) as *const u32).read_unaligned();
        u32::from(((flags >> FLAG_BIT) & 1) != 0)
    }
});
