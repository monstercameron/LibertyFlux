// original: 0x009a8740 audio_ptr_tailjump
/// Resolve the effect pointer from the entity tags and tail-run it.
///
/// Reads the tag byte at `this+4`: 0xff means a null effect.
/// Otherwise the tag byte at `this+0x40` selects a cell out of the
/// global row table (rows of 0x6f40 bytes starting at table `+0x6f14`),
/// added to the global multiplier times the first tag. The resolved
/// pointer goes in both ECX and EDX and control tail-jumps to the
/// effect runner (stubbed): the rewrite ends in the same stub call,
/// which returns to the caller for it. Thiscall, no stack words,
/// dword result (the runner's answer).
export!(thiscall, rw_009A8740(this: u32) -> u32 {
    unsafe {
        const TAG_A: u32 = 4;
        const TAG_B: u32 = 0x40;
        const MIX_GLOBAL: u32 = 0x115d968;
        const TABLE_GLOBAL: u32 = 0x115d988;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BASE: u32 = 0x6f14;
        let ta = ((this + TAG_A) as *const u8).read() as u32;
        if ta == 0xff {
            return callee_fastcall!(1, u32, 0, 0);
        }
        let tb = ((this + TAG_B) as *const u8).read() as u32;
        let mix = global::<u32>(MIX_GLOBAL).read_unaligned();
        let tab = global::<u32>(TABLE_GLOBAL).read_unaligned();
        let cell = ((tb * ROW_STRIDE + tab + ROW_BASE) as *const u32).read_unaligned();
        let ptr = mix.wrapping_mul(ta).wrapping_add(cell);
        callee_fastcall!(1, u32, ptr, ptr)
    }
});
