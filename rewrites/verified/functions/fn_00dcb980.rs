// original: 0x00DCB980 vehicle_collision_response_is_allowed

/// Determines whether a vehicle collision response is enabled for an entity.
///
/// This cdecl function takes an entity pointer and returns a one-byte boolean
/// in AL. A successful global helper vetoes the response. Otherwise it sign
/// extends the entity's 16-bit table index, reads the matching table pointer,
/// and allows the response only when the pointed-to byte at `+0xee` is at
/// least three. The index arithmetic remains signed before it is applied to
/// the table base.
lf_checker_rt::export!(cdecl, rw_00dcb980(entity: u32) -> u8 {
    unsafe {
        const ENTITY_INDEX: u32 = 0x2e;
        const RESPONSE_LEVEL: u32 = 0xee;
        const RESPONSE_TABLE_FILE_VA: u32 = 0x01295CD8;
        const TEST_RESPONSE_GATE: u32 = 1;

        #[inline(always)]
        unsafe fn read_i16(address: u32) -> i16 {
            unsafe { (address as *const i16).read_unaligned() }
        }

        let helper_succeeded = lf_checker_rt::callee_cdecl!(TEST_RESPONSE_GATE, u32,);
        if helper_succeeded as u8 != 0 {
            return 0;
        }

        let table_index = read_i16(entity.wrapping_add(ENTITY_INDEX)) as i32;
        let entry_file_va = RESPONSE_TABLE_FILE_VA
            .wrapping_add(table_index.wrapping_mul(4) as u32);
        let response_info = lf_checker_rt::global::<u32>(entry_file_va).read_unaligned();
        let response_level = (response_info.wrapping_add(RESPONSE_LEVEL) as *const u8).read();
        u8::from(response_level >= 3)
    }
});
