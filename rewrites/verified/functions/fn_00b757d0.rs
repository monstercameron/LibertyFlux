// original: 0x00b757d0 slot_array_reset (proposed)

/// Reset a fifteen-slot array of 16-byte entries plus a trailing word.
///
/// Writes -1 to the first dword and 0 to the second byte-word of each entry
/// (`this+16*i` and `this+16*i+4` for i in 0..15), and 0 to the dword at
/// `this+0xf0` just past the last entry. Always returns 0.
///
/// Original: 0x00b757d0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b757d0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 15;
        const STRIDE: u32 = 16;
        const TRAIL_OFF: u32 = 0xf0;
        const EMPTY: u32 = 0xffff_ffff;
        ((this + TRAIL_OFF) as *mut u32).write_unaligned(0);
        let mut i = 0u32;
        while i < COUNT {
            let e = this + i * STRIDE;
            (e as *mut u32).write_unaligned(EMPTY);
            ((e + 4) as *mut u8).write(0);
            i += 1;
        }
        0
    }
});
