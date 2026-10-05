// original: 0x00883d80 stream_endpoint_init (proposed)
/// Initialise a streaming endpoint: store the bound value, clear the state.
///
/// Writes `value` to `+0x20`, zeroes `+0x08`, `+0x0c`, `+0x10`, `+0x14`,
/// `+0x18`, `+0x1c` and `+0x24` plus the half-word at `+0x60` and the flag
/// byte at `+0x62`, and sets the state word at `+0x04` to all-ones.
///
/// Return value: the original finishes with `(an instruction of the original)`, so the low byte is
/// 1 while the upper three bytes are whatever the argument's were; the
/// rewrite reproduces that exactly.
///
/// Original: thiscall, one stack argument, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00883d80(this: u32, value: u32) -> u32 {
    unsafe {
        const EMPTY: u32 = 0xFFFF_FFFF;
        let base = this as *mut u8;
        let w = |off: u32, v: u32| (base.add(off as usize) as *mut u32).write_unaligned(v);
        w(0x20, value);
        w(0x10, 0);
        w(0x08, 0);
        w(0x14, 0);
        w(0x04, EMPTY);
        w(0x0c, 0);
        w(0x1c, 0);
        w(0x24, 0);
        (base.add(0x60) as *mut u16).write_unaligned(0);
        base.add(0x62).write(0);
        w(0x18, 0);
        (value & 0xFFFF_FF00) | 1
    }
});
