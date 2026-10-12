// original: 0x00D3DDE0 reset_request_holder

/// Clear the request-holder's active byte and handle word. The object arrives
/// in ECX; byte 0 and the dword at offset 4 become zero, other bytes are
/// untouched, EAX returns the original object pointer, and no stack words are
/// consumed.
lf_checker_rt::export!(thiscall, rw_D3DDE0(this: u32) -> u32 {
    const ACTIVE_BYTE: u32 = 0;
    const REQUEST_WORD: u32 = 4;
    unsafe {
        core::ptr::write((this + ACTIVE_BYTE) as *mut u8, 0);
        core::ptr::write_unaligned((this + REQUEST_WORD) as *mut u32, 0);
    }
    this
});
