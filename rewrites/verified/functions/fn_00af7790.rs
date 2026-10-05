// original: 0x00AF7790 veh_state_zero_tail (proposed)

/// Clear the fourteen-byte state tail at the end of the object.
///
/// Zeroes bytes `this + 0x54` through `this + 0x61`: an eight-byte word, one
/// dword and one word. The original does this with one SSE zero register;
/// only the cleared bytes are observable. No value is returned.
///
/// Original: 0x00AF7790 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF7790(this: u32) -> u32 {
    unsafe {
        const TAIL: u32 = 0x54;
        const TAIL_LEN: usize = 14;
        core::ptr::write_bytes((this + TAIL) as *mut u8, 0, TAIL_LEN);
        0
    }
});
