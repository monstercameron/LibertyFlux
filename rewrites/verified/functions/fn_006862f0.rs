// original: 0x006862F0 dof_array_write_entry (proposed)

/// Copy four raw words from the caller's 16-byte structure into a DOF record
/// selected by two key words. The stdcall entry receives the keys first and
/// second and the source pointer third. A scripted stdcall helper returns the
/// destination record pointer; a null pointer returns false without writes.
/// Otherwise the source words at `+0..+0x0c` are copied to destination
/// offsets `+0x10..+0x1c`, and flag bit `0x10` in destination byte `+4` is
/// cleared while its other bits are preserved. AL is set to one. The four
/// payload words are copied bit for bit, including float encodings.
lf_checker_rt::export!(stdcall, rw_006862F0(key_first: u32, key_second: u32, source: u32) -> u32 {
    unsafe {
        const HELPER_ID: u32 = 1;
        const RECORD_FLAGS: u32 = 0x04;
        const RECORD_DATA: u32 = 0x10;
        const SOURCE_DATA: u32 = 0x00;
        const CLEAR_FLAG: u8 = 0x10;
        const WORDS: u32 = 4;
        const WORD_BYTES: u32 = 4;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let record = unsafe {
            lf_checker_rt::callee_stdcall!(HELPER_ID, u32, key_first, key_second)
        };
        if record == 0 {
            return 0;
        }

        for word_index in 0..WORDS {
            let value = unsafe { read_u32(source + SOURCE_DATA + word_index * WORD_BYTES) };
            unsafe { write_u32(record + RECORD_DATA + word_index * WORD_BYTES, value) };
        }
        let flags = unsafe { read_u8(record + RECORD_FLAGS) };
        unsafe { write_u8(record + RECORD_FLAGS, flags & !CLEAR_FLAG) };
        1
    }
});
