// original: 0x00C1DAA0 streaming_copy_current_entry

/// Copy the active sixteen-byte streaming record to the caller's buffer. The
/// record index is read from the global slot at file VA `0x01174790`; records
/// begin at `0x015e88f0` and advance by sixteen bytes. Four unaligned words
/// are copied in order, and the destination pointer is returned in EAX. The
/// function uses one cdecl stack argument and no floating-point operations.
lf_checker_rt::export!(cdecl, rw_00c1daa0(destination: u32) -> u32 {
    unsafe {
        const ACTIVE_INDEX_VA: u32 = 0x01174790;
        const RECORD_TABLE_VA: u32 = 0x015e88f0;
        const RECORD_STRIDE: u32 = 16;
        const WORD_BYTES: u32 = 4;
        const WORD_COUNT: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let active_index = read_u32(lf_checker_rt::relocated(ACTIVE_INDEX_VA));
        let record_address = lf_checker_rt::relocated(RECORD_TABLE_VA)
            .wrapping_add(active_index.wrapping_mul(RECORD_STRIDE));
        for word_index in 0..WORD_COUNT {
            let byte_offset = word_index * WORD_BYTES;
            let value = read_u32(record_address.wrapping_add(byte_offset));
            write_u32(destination.wrapping_add(byte_offset), value);
        }
        destination
    }
});
