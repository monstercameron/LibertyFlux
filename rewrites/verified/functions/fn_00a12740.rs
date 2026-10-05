// original: 0x00a12740 range_snap_clamp (proposed)
/// Snap the stored range value to a bound and clamp it from above.
///
/// `lo` and `hi` bound the float at `this + 0x190`: when the stored value
/// is strictly above `lo` it becomes `lo`, otherwise when `hi` is strictly
/// above it it becomes `hi`. Either snap sets the low-byte result to 1 and
/// clears the word at `this + 0x19c`. When `flag`'s low byte is non-zero
/// and the value is then strictly above 0.4 (0x3ecccccd), it is set to 0.4
/// instead, again with result 1 and a cleared word. Comparisons are SSE
/// ordered-greater (NaN takes the else branch). Only the low result byte is
/// set, so the contract compares `al`. Thiscall, three stack arguments.
export!(thiscall, rw_00a12740(this: u32, lo: u32, hi: u32, flag: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0x190;
        const STATE_OFF: u32 = 0x19c;
        const LIMIT_ADDR: u32 = 0x00fe881c;
        const LIMIT_BITS: u32 = 0x3ecccccd;
        let mut out: u8 = 0;
        let cur = f32::from_bits(((this + SLOT_OFF) as *const u32).read_unaligned());
        if cur > f32::from_bits(lo) {
            ((this + SLOT_OFF) as *mut u32).write_unaligned(lo);
            out = 1;
            ((this + STATE_OFF) as *mut u32).write_unaligned(0);
        } else if f32::from_bits(hi) > cur {
            ((this + SLOT_OFF) as *mut u32).write_unaligned(hi);
            out = 1;
            ((this + STATE_OFF) as *mut u32).write_unaligned(0);
        }
        if (flag & 0xff) != 0 {
            let now =
                f32::from_bits(((this + SLOT_OFF) as *const u32).read_unaligned());
            if now > f32::from_bits(*global::<u32>(LIMIT_ADDR)) {
                ((this + SLOT_OFF) as *mut u32).write_unaligned(LIMIT_BITS);
                ((this + STATE_OFF) as *mut u32).write_unaligned(0);
                out = 1;
            }
        }
        out as u32
    }
});
