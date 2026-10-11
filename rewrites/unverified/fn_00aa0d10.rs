// original: 0x00AA0D10 find_loader_key_and_clear_state
/// Searches the 128 sixteen-byte records for the first record whose first
/// dword equals `key`. On a hit it clears the following state byte and returns
/// twice the record index. If no record matches, it returns 128 without a
/// write.
lf_checker_rt::export!(thiscall, rw_00AA0D10(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_OFFSET: u32 = 0x38B4;
        const ENTRY_STRIDE: u32 = 0x10;
        const ENTRY_COUNT: u32 = 0x80;
        const STATE_OFFSET: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let mut index = 0u32;
        while index < ENTRY_COUNT
            && read_u32(
                this.wrapping_add(TABLE_OFFSET)
                    .wrapping_add(index.wrapping_mul(ENTRY_STRIDE)),
            ) != key
        {
            index += 1;
        }
        if index == ENTRY_COUNT {
            return ENTRY_COUNT;
        }

        let state = this
            .wrapping_add(TABLE_OFFSET)
            .wrapping_add(index.wrapping_mul(ENTRY_STRIDE))
            .wrapping_add(STATE_OFFSET);
        write_u8(state, 0);
        index.wrapping_mul(2)
    }
});
