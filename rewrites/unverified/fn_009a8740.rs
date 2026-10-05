// original: 0x009a8740 audio_ptr_tailjump
/// Resolve the effect pointer for the entity and tail-jump to run it.
///
/// Loads the entity pointer at `this+0x3ac4`; a null entity means a
/// null effect. Otherwise the tag byte at entity `+4` decides: 0xff
/// means a null effect, anything else mixes the global multiplier
/// with the cell selected by the tag byte at entity `+0x40` out of
/// the global row table (rows of 0x6f40 bytes starting at table
/// `+0x6f14`). The resolved pointer goes in EDX, the argument in ECX,
/// and control tail-jumps to the effect runner (stubbed): the rewrite
/// ends in the same stub call, which returns to the caller for it.
/// Thiscall, one stack word, dword result (the runner's answer).
export!(thiscall, rw_009A8740(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ENTITY: u32 = 0x3ac4;
        const TAG_A: u32 = 4;
        const TAG_B: u32 = 0x40;
        const MIX_GLOBAL: u32 = 0x115d968;
        const TABLE_GLOBAL: u32 = 0x115d988;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BASE: u32 = 0x6f14;
        let ent = ((this + ENTITY) as *const u32).read_unaligned();
        let eff = if ent == 0 {
            0
        } else {
            let ta = ((ent + TAG_A) as *const u8).read() as u32;
            if ta == 0xff {
                0
            } else {
                let tb = ((ent + TAG_B) as *const u8).read() as u32;
                let mix = global::<u32>(MIX_GLOBAL).read_unaligned();
                let tab = global::<u32>(TABLE_GLOBAL).read_unaligned();
                let cell = ((tb * ROW_STRIDE + tab + ROW_BASE) as *const u32).read_unaligned();
                mix.wrapping_mul(ta).wrapping_add(cell)
            }
        };
        callee_fastcall!(1, u32, arg0, eff)
    }
});
