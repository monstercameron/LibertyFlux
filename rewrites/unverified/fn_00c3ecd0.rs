// original: 0x00c3ecd0 train_any_slot_active (proposed)
/// Report whether any of thirteen status bytes is non-zero.
///
/// Scans the bytes at `this+0x2c + i*0x30` for `i` in 0..13 and returns 1
/// in AL on the first non-zero byte, else 0 in AL. The upper 24 bits of
/// EAX keep whatever the address computation left there (the scan pointer
/// with its low byte cleared), which is identical on both sides for the
/// same heap input and is compared as part of EAX. No calls.
///
/// Original: 0x00c3ecd0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c3ecd0(this: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x2c;
        const STRIDE: u32 = 0x30;
        const COUNT: u32 = 13;
        let mut p = this.wrapping_add(FIRST);
        let mut i = 0u32;
        loop {
            if ((p as *const u8).read()) != 0 {
                // Original returns with AL=1 over the stale high bits of
                // the scan pointer: reproduce the value, not just the byte.
                return (p & 0xffff_ff00) | 1;
            }
            i += 1;
            p = p.wrapping_add(STRIDE);
            if i >= COUNT {
                // EAX has advanced past all thirteen slots here.
                return p & 0xffff_ff00;
            }
        }
    }
});
