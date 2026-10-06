// original: 0x00c8bd00 audio_bank_init (proposed)
///
/// Initialises the twelve 0x30-byte voice slots at `this+0` (block `k`
/// zeroes double-words +0x00, +0x04, +0x08, the byte +0x10 and
/// double-words +0x14, +0x18, +0x1c; the words +0x0c and +0x11..+0x13 and
/// the tail +0x20..+0x2f of each block are left alone) and the header at
/// `this+0x240` (double-words +0x240, +0x244, byte +0x248, double-words
/// +0x250, +0x254, +0x258, +0x260, +0x264, +0x268; +0x24c is left alone).
/// Returns `this`. Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c8bd00(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const BLOCKS: u32 = 12;
        const PITCH: u32 = 0x30;
        let mut k: u32 = 0;
        while k < BLOCKS {
            let b = this.wrapping_add(k.wrapping_mul(PITCH));
            wr32(b.wrapping_add(0x00), 0);
            wr32(b.wrapping_add(0x04), 0);
            wr32(b.wrapping_add(0x08), 0);
            ((b.wrapping_add(0x10)) as *mut u8).write(0);
            wr32(b.wrapping_add(0x14), 0);
            wr32(b.wrapping_add(0x18), 0);
            wr32(b.wrapping_add(0x1c), 0);
            k += 1;
        }
        wr32(this.wrapping_add(0x240), 0);
        wr32(this.wrapping_add(0x244), 0);
        ((this.wrapping_add(0x248)) as *mut u8).write(0);
        wr32(this.wrapping_add(0x250), 0);
        wr32(this.wrapping_add(0x254), 0);
        wr32(this.wrapping_add(0x258), 0);
        wr32(this.wrapping_add(0x260), 0);
        wr32(this.wrapping_add(0x264), 0);
        wr32(this.wrapping_add(0x268), 0);
        this
    }
});
