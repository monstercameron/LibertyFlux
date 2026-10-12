// original: 0x00686340 dof_array_read_entry_raw (proposed)

/// Read the raw 16-byte payload of a DOF record selected by two key words.
/// The stdcall entry receives both key words followed by the output buffer.
/// A scripted stdcall helper returns the record pointer. A null record or a
/// set blocked bit (`+4 & 0x10`) returns false without changing the buffer.
/// Otherwise the words at record offsets `+0x10..+0x1c` are copied exactly
/// to output offsets `+0..+0x0c`, and AL is set to one. The original moves
/// these bytes in two 64-bit chunks; the rewrite expresses the same 16-byte
/// copy as four words and performs no floating-point arithmetic.
lf_checker_rt::export!(stdcall, rw_00686340(key_first: u32, key_second: u32, output: u32) -> u32 {
    unsafe {
        const HELPER_ID: u32 = 1;
        const BLOCKED_FLAGS: u8 = 0x10;
        const RECORD_FLAGS: u32 = 0x04;
        const RECORD_DATA: u32 = 0x10;
        const WORDS: u32 = 4;
        const WORD_BYTES: u32 = 4;
        #[inline(always)] unsafe fn read_u8(a: u32) -> u8 { unsafe { (a as *const u8).read() } }
        #[inline(always)] unsafe fn read_u32(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_u32(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        let record = unsafe { lf_checker_rt::callee_stdcall!(HELPER_ID, u32, key_first, key_second) };
        if record == 0 || unsafe { read_u8(record + RECORD_FLAGS) } & BLOCKED_FLAGS != 0 { return 0; }
        for word_index in 0..WORDS {
            let value = unsafe { read_u32(record + RECORD_DATA + word_index * WORD_BYTES) };
            unsafe { write_u32(output + word_index * WORD_BYTES, value) };
        }
        1
    }
});
