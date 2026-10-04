// original: 0x00967A90 init_dual_fade
/// Initialise the two fade/counter parameter blocks at `this + 0x10`.
///
/// Writes eight dwords: `0, 0, 0, 1.0, 0, 1.0, 0, 0` at offsets
/// `0x10..=0x2C`. Each block of four is `(0, 0-or-1.0 pattern)`: the
/// original writes the two `1.0f` words (`0x3F800000`) at `+0x1C` and
/// `+0x24` and zeroes the rest. Returns `this`.
///
/// Original: 0x00967A90 (thiscall, no stack words).

export!(thiscall, rw_00967A90(this: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3F80_0000;
        let base = this as *mut u32;
        base.add(0x04).write_unaligned(0);
        base.add(0x05).write_unaligned(0);
        base.add(0x06).write_unaligned(0);
        base.add(0x07).write_unaligned(ONE);
        base.add(0x08).write_unaligned(0);
        base.add(0x09).write_unaligned(ONE);
        base.add(0x0A).write_unaligned(0);
        base.add(0x0B).write_unaligned(0);
        this
    }
});
