// original: 0x00D3DDE0 reset_request_holder

/// Reset a request holder passed in ECX. The original stores zero to the
/// leading byte and the four-byte word at offset 4, leaves all other bytes
/// alone, returns the original object pointer in EAX, and consumes no stack
/// arguments (thiscall).
lf_checker_rt::export!(thiscall, rw_D3DDE0(this: u32) -> u32 {
    const ACTIVE_BYTE: u32 = 0;
    const REQUEST_WORD: u32 = 4;

    unsafe {
        core::ptr::write((this + ACTIVE_BYTE) as *mut u8, 0);
        core::ptr::write_unaligned((this + REQUEST_WORD) as *mut u32, 0);
    }
    this
});
