// original: 0x006595A0 stream_clear_child_flags

/// Write each child's zero-based index as one byte at the offset selected by
/// that child record, then return the collection's 16-bit entry count. `this + 8`
/// holds that count and `this + 4` points to the entry-pointer array. Each
/// entry has a child-pointer array at `+0x28` and a 16-bit child count at
/// `+0x2c`; every child record supplies a byte offset at `+0x28`, and the
/// corresponding byte in its parent entry receives the child index modulo 256. The counts are
/// unsigned 16-bit values, so the machine's signed loop branches do not
/// change their range.
lf_checker_rt::export!(thiscall, rw_006595A0(this: u32) -> u32 {
    unsafe {
        const ENTRY_ARRAY: u32 = 0x04;
        const ENTRY_COUNT: u32 = 0x08;
        const CHILD_ARRAY: u32 = 0x28;
        const CHILD_COUNT: u32 = 0x2c;
        const CHILD_BYTE_OFFSET: u32 = 0x28;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u16) as u32 }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u32) }
        }

        let entry_count = read_u16(this.wrapping_add(ENTRY_COUNT));
        let entry_array = read_u32(this.wrapping_add(ENTRY_ARRAY));
        let mut entry_index = 0u32;

        while entry_index < entry_count {
            let entry_pointer = read_u32(entry_array.wrapping_add(entry_index.wrapping_mul(4)));
            let child_count = read_u16(entry_pointer.wrapping_add(CHILD_COUNT));
            let child_array = read_u32(entry_pointer.wrapping_add(CHILD_ARRAY));
            let mut child_index = 0u32;

            while child_index < child_count {
                let child_pointer = read_u32(child_array.wrapping_add(child_index.wrapping_mul(4)));
                let byte_offset = read_u32(child_pointer.wrapping_add(CHILD_BYTE_OFFSET));
                core::ptr::write(
                    entry_pointer.wrapping_add(byte_offset) as *mut u8,
                    child_index as u8,
                );
                child_index += 1;
            }
            entry_index += 1;
        }
        entry_count
    }
});
