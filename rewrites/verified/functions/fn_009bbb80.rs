// original: 0x009bbb80 write_input_slot (proposed)

/// Write a value into the first free input slot, if there is one.
///
/// Asks helper id 1 (cdecl, no arguments) for the first free slot index of
/// the table at 0x0128E960 (26 entries of 0x40 bytes). When it answers -1
/// writes nothing and returns -1. Otherwise, with `base` the slot start,
/// stores the stack words (each named for its destination offset) `w04` at
/// `+0x04`, `w10` at `+0x10`, `w14` at `+0x14`, `w18` at `+0x18`, `w20` at
/// `+0x20`, `w24` at `+0x24`, `w28` at `+0x28`, `w2c` at `+0x2c`, `w30` at
/// `+0x30`, the low byte of `bytev` at `+0x34`, sets the flag byte at
/// `+0x00` to 1, and returns the index. All float moves are bitwise; no
/// floating-point arithmetic happens.
///
/// Edge cases: a -1 answer returns -1 with no writes; the index is used
/// unconditionally otherwise, so an out-of-range answer faults (or writes
/// outside the table) on both sides alike.
///
/// Original: cdecl, ten stack words in the order w10, w14, w18, w30, w04,
/// w20, w24, w28, w2c, bytev; one callee.
lf_checker_rt::export!(cdecl, rw_009bbb80(w10: u32, w14: u32, w18: u32, w30: u32, w04: u32, w20: u32, w24: u32, w28: u32, w2c: u32, bytev: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0128e960;
        const STRIDE: u32 = 0x40;
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let idx: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if idx == 0xFFFFFFFF {
            return 0xFFFFFFFF;
        }
        let base = lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(STRIDE));
        wr(base + 0x10, w10);
        wr(base + 0x14, w14);
        wr(base + 0x18, w18);
        wr(base + 0x04, w04);
        wr(base + 0x20, w20);
        wr(base + 0x24, w24);
        wr(base + 0x28, w28);
        wr(base + 0x2c, w2c);
        wr(base + 0x30, w30);
        ((base + 0x34) as *mut u8).write(bytev as u8);
        (base as *mut u8).write(1);
        idx
    }
});
