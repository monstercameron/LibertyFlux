// original: 0x00AC5E70 shader_fx_construct (proposed)

/// Construct a bone-damage effect object in place at `this`.
///
/// The original calls the allocator callee with the object size, installs
/// the vtable pointer, zeroes the body words and the three parameter rows,
/// sets the four colour words to 1.0, clears the low two flag bits and the
/// mode byte (thiscall, no stack arguments). It returns `this`.
lf_checker_rt::export!(thiscall, rw_00AC5E70(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const SIZE: u32 = 0x370;
        const VTABLE: u32 = 0x00EA5DE0;
        const BODY: u32 = 0x20;
        const BODY_WORDS: u32 = 0x80;
        const ROWS: u32 = 0x2EC;
        const ROW_COUNT: u32 = 11;
        const ROW_STRIDE: u32 = 0x2C;
        const COLOR: u32 = 0x350;
        const ONE: u32 = 0x3f80_0000;
        lf_checker_rt::callee_thiscall!(ALLOC, u32, this, SIZE);
        (this as *mut u32).write_unaligned(VTABLE);
        for i in 0..BODY_WORDS {
            (this.wrapping_add(BODY + i * 4) as *mut u32).write_unaligned(0);
        }
        let mut p = this.wrapping_add(ROWS);
        for _ in 0..ROW_COUNT {
            (p.wrapping_sub(ROW_STRIDE) as *mut u32).write_unaligned(0);
            (p as *mut u32).write_unaligned(0);
            (p.wrapping_add(ROW_STRIDE) as *mut u32).write_unaligned(0);
            p = p.wrapping_add(4);
        }
        for i in 0..4u32 {
            (this.wrapping_add(COLOR + i * 4) as *mut u32).write_unaligned(ONE);
        }
        let f = (this.wrapping_add(0x360) as *mut u8);
        *f = *f & 0xfc;
        (this.wrapping_add(0x361) as *mut u8).write(0);
        this
    }
});
