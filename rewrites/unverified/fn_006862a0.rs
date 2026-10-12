// original: 0x006862A0 dof_array_read_entry (proposed)

/// Find a DOF record by the two key words and copy its four payload words to
/// the caller's output structure. The stdcall entry receives the key words
/// first and second, then a writable 16-byte output buffer. A scripted
/// stdcall helper returns the record pointer; a null result or a set blocked
/// bit (`+4 & 0x10`) returns false without changing the output. For an
/// accepted record, the four words at `+0x10`, `+0x14`, `+0x18`, and `+0x1c`
/// are copied bit for bit, including float payloads, and AL is set to one.
/// The original's helper receives the key words in their source order.
lf_checker_rt::export!(stdcall, rw_006862A0(key_first: u32, key_second: u32, output: u32) -> u32 {
    unsafe {
        const HELPER_ID: u32 = 1;
        const BLOCKED_FLAGS: u8 = 0x10;
        const RECORD_FLAGS: u32 = 0x04;
        const RECORD_DATA: u32 = 0x10;
        const OUTPUT_DATA: u32 = 0x00;
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
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let record = unsafe {
            lf_checker_rt::callee_stdcall!(HELPER_ID, u32, key_first, key_second)
        };
        if record == 0 || unsafe { read_u8(record + RECORD_FLAGS) } & BLOCKED_FLAGS != 0 {
            return 0;
        }

        for word_index in 0..WORDS {
            let value = unsafe { read_u32(record + RECORD_DATA + word_index * WORD_BYTES) };
            unsafe { write_u32(output + OUTPUT_DATA + word_index * WORD_BYTES, value) };
        }
        1
    }
});
