// original: 0x009684C0 init_timer_block
/// Initialise one 0x54-byte timer block at `this`.
///
/// Zeroes every dword from `+0x0` to `+0x48` except `+0xC`, `+0x1C`,
/// `+0x2C` and `+0x3C`, which keep their old values; writes the default
/// 2000 (`0x7D0`) at `+0x4C`; and clears the low three bits of the dword
/// at `+0x50`. No return value (`ret: none`).
///
/// Original: 0x009684C0 (thiscall, no stack words).

export!(thiscall, rw_009684C0(this: u32) -> u32 {
    unsafe {
        const DEFAULT: u32 = 0x7D0;
        let base = this as *mut u32;
        for w in [0x00, 0x01, 0x02, 0x04, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0C, 0x0D, 0x0E] {
            base.add(w).write_unaligned(0);
        }
        base.add(0x10).write_unaligned(0);
        base.add(0x11).write_unaligned(0);
        base.add(0x12).write_unaligned(0);
        base.add(0x13).write_unaligned(DEFAULT);
        let flags = base.add(0x14).read_unaligned();
        base.add(0x14).write_unaligned(flags & !7);
        0
    }
});
