// original: 0x00b1b5a0 scale_level_by_rate (proposed)

/// Scales the object's level byte by a measured rate, storing both.
///
/// Thiscall with no stack arguments. Fetches a measurement block (cdecl
/// of a scratch buffer and the constant 0x30; only the word at +4 of the
/// returned block is used), converts the level byte at +0x40 to float,
/// stores it to +0x04 and stores its negation-times-rate to +0x34 (the
/// negation flips the sign bit, matching the original's sign-mask xor).
/// Returns the level byte.
lf_checker_rt::export!(thiscall, rw_00b1b5a0(this: u32) -> u32 {
    unsafe {
        const SIGN_MASK: u32 = 0x8000_0000;
        const RATE_TAG: u32 = 0x30;
        let mut scratch = [0u32; 2];
        let block: u32 = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            scratch.as_mut_ptr() as u32,
            RATE_TAG
        );
        let rate = ((block + 4) as *const f32).read_unaligned();
        let level = ((this + 0x40) as *const u8).read();
        let f = level as f32;
        ((this + 4) as *mut f32).write_unaligned(f);
        let prod = core::hint::black_box(f) * core::hint::black_box(rate);
        ((this + 0x34) as *mut u32).write_unaligned(prod.to_bits() ^ SIGN_MASK);
        level as u32
    }
});
