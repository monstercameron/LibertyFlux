// original: 0x00a64880 guarded_code_store
/// Stores a guarded two-byte code at +0xF0.
///
/// Code 0x20 is ignored. Code 0 always stores; any other code stores only
/// when its accompanying value exceeds the byte already at +0xF1, compared
/// as signed 32-bit integers. Only the low byte of each argument is stored.
/// Returns nothing.
export!(thiscall, rw_00a64880(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        if a == 0x20 {
            return 0;
        }
        if a == 0 || (b as i32) > *((this + 0xF1) as *const u8) as i32 {
            *((this + 0xF0) as *mut u8) = a as u8;
            *((this + 0xF1) as *mut u8) = b as u8;
        }
    }
    0
});
