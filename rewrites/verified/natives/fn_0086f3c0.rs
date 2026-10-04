// original: 0x0086f3c0 SHIFT_LEFT
/// Script native `SHIFT_LEFT` (hash 0x102A0A6C).
///
/// A leaf: shifts the first script argument left by the second, modulo 32
/// (the hardware shift count mask), and stores the result into the return
/// slot. The exit value is the slot pointer.
export!(cdecl, rw_0086f3c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let value = *args;
        let count = *args.add(1);
        *slot = value.wrapping_shl(count & 31);
        slot as u32
    }
});
