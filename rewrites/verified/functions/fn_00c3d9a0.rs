// original: 0x00c3d9a0 train_store_speed_signed (proposed)
/// Store a speed value, negating it unless the reverse flag is set.
///
/// `obj` points to the car, `fbits` is a float bit pattern. Always stores
/// it at `obj+0x14a0` first; when flag bit 6 (mask 0x40) of the byte at
/// `obj+0x14e4` is clear, stores the sign-flipped bits instead (XOR with
/// the sign mask, so -0.0 stays distinct from +0.0 and NaN payloads are
/// preserved). Returns obj.
///
/// Original: 0x00c3d9a0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00c3d9a0(obj: u32, fbits: u32) -> u32 {
    unsafe {
        const SPEED: u32 = 0x14a0;
        const FLAGS: u32 = 0x14e4;
        const REVERSE_BIT: u8 = 0x40;
        const SIGN_MASK: u32 = 0x8000_0000;
        ((obj + SPEED) as *mut u32).write_unaligned(fbits);
        if (((obj + FLAGS) as *const u8).read() & REVERSE_BIT) == 0 {
            ((obj + SPEED) as *mut u32).write_unaligned(fbits ^ SIGN_MASK);
        }
        obj
    }
});
