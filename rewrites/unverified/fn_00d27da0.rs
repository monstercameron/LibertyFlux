// original: 0x00d27da0 target_slot_clear (proposed)

/// Clear a target-slot record in place.
///
/// `this` points to the record. Zeroes the words at `+0x00`, `+0x04`, `+0x08`,
/// `+0x10`, `+0x14`, `+0x18`, `+0x1c`, `+0x20`, `+0x24` and `+0x30` and the byte
/// at `+0x2d`, clears the low three bits of the word at `+0x28` and bit 0 of
/// the byte at `+0x2c`. Everything else in the record is left untouched.
///
/// Original: 0x00D27DA0 (thiscall, no stack arguments). Returns `this`.
lf_checker_rt::export!(thiscall, rw_00d27da0(this: u32) -> u32 {
    unsafe {
        const KEEP_MASK: u32 = 0xffff_fff8;
        const FLAG_MASK: u8 = 0xfe;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this + 0x28, rd32(this + 0x28) & KEEP_MASK);
        ((this + 0x2c) as *mut u8).write(((this + 0x2c) as *const u8).read() & FLAG_MASK);
        for off in [0x14u32, 0x18, 0x1c, 0x20] {
            wr32(this + off, 0);
        }
        ((this + 0x2d) as *mut u8).write(0);
        for off in [0x24u32, 0x08, 0x04, 0x00, 0x10, 0x30] {
            wr32(this + off, 0);
        }
        this
    }
});
