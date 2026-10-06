// original: 0x00D78A10 clear_status_bits (proposed)

/// Clear bit 0 of the byte at `obj + 0xf1c` and bit 7 of the byte at
/// `obj + 0xf1b`, and return `obj`.
///
/// Cdecl, one stack word.
use lf_checker_rt::export;

export!(cdecl, rw_00d78a10(obj: u32) -> u32 {
    unsafe {
        const FLAG_A_OFF: u32 = 0xf1c;
        const FLAG_B_OFF: u32 = 0xf1b;
        let a = ((obj + FLAG_A_OFF) as *const u8).read();
        ((obj + FLAG_A_OFF) as *mut u8).write(a & 0xfe);
        let b = ((obj + FLAG_B_OFF) as *const u8).read();
        ((obj + FLAG_B_OFF) as *mut u8).write(b & 0x7f);
    }
    obj
});
