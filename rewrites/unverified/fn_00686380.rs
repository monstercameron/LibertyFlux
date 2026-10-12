// original: 0x00686380 dof_array_write_entry_raw (proposed)

/// Write a raw 16-byte payload into the DOF record selected by two key
/// words. The stdcall entry receives the keys first and second and a source
/// structure third. A scripted stdcall helper returns the destination
/// pointer; a null pointer returns false with no writes. For a nonnull
/// pointer, the four source words are copied to record offsets
/// `+0x10..+0x1c`, then bit `0x10` is cleared in the flag byte at `+4` while
/// all other flag bits are preserved. AL is set to one. The payload copy is
/// bitwise, with no floating-point operation.
lf_checker_rt::export!(stdcall, rw_00686380(key_first: u32, key_second: u32, source: u32) -> u32 {
    unsafe {
        const HELPER_ID: u32 = 1;
        const RECORD_FLAGS: u32 = 0x04;
        const RECORD_DATA: u32 = 0x10;
        const CLEAR_FLAG: u8 = 0x10;
        const WORDS: u32 = 4;
        const WORD_BYTES: u32 = 4;
        #[inline(always)] unsafe fn read_u8(a: u32) -> u8 { unsafe { (a as *const u8).read() } }
        #[inline(always)] unsafe fn read_u32(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)] unsafe fn write_u8(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        #[inline(always)] unsafe fn write_u32(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        let record = unsafe { lf_checker_rt::callee_stdcall!(HELPER_ID, u32, key_first, key_second) };
        if record == 0 { return 0; }
        for word_index in 0..WORDS {
            let value = unsafe { read_u32(source + word_index * WORD_BYTES) };
            unsafe { write_u32(record + RECORD_DATA + word_index * WORD_BYTES, value) };
        }
        let flags = unsafe { read_u8(record + RECORD_FLAGS) };
        unsafe { write_u8(record + RECORD_FLAGS, flags & !CLEAR_FLAG) };
        1
    }
});
