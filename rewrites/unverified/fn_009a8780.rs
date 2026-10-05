// original: 0x009a8780 audio_param_push
/// Push a float parameter through the audio entity's effect chain.
///
/// Does nothing when the entity pointer at `this+0x3ac4` is null.
/// Otherwise the parameter `x` is first transformed by the shaper
/// (stubbed, cdecl/1, float in and float ST0 out); the shaped bits
/// are parked in the second stack slot. Then the effect pointer is
/// resolved from the entity's tag bytes exactly like the tail-jump
/// helper does (null when the byte at `+4` is 0xff, else the global
/// mix of the bytes at `+4` and `+0x40`), the effect is run on `x`
/// (stubbed, thiscall/1), and finally the shaped bits re-read as a
/// signed integer, converted back to float, are pushed to the entity
/// (stubbed, thiscall/1). Thiscall, two stack words, no result.
export!(thiscall, rw_009A8780(this: u32, x: u32, _dummy: u32) -> u32 {
    unsafe {
        const ENTITY: u32 = 0x3ac4;
        const TAG_A: u32 = 4;
        const TAG_B: u32 = 0x40;
        const MIX_GLOBAL: u32 = 0x115d968;
        const TABLE_GLOBAL: u32 = 0x115d988;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BASE: u32 = 0x6f14;
        let ent = ((this + ENTITY) as *const u32).read_unaligned();
        if ent == 0 {
            return 0;
        }
        let shaped: f32 = callee_cdecl!(1, f32, x);
        let parked = shaped.to_bits();
        let ta = ((ent + TAG_A) as *const u8).read() as u32;
        let eff = if ta == 0xff {
            0
        } else {
            let tb = ((ent + TAG_B) as *const u8).read() as u32;
            let mix = global::<u32>(MIX_GLOBAL).read_unaligned();
            let tab = global::<u32>(TABLE_GLOBAL).read_unaligned();
            let cell = ((tb * ROW_STRIDE + tab + ROW_BASE) as *const u32).read_unaligned();
            mix.wrapping_mul(ta).wrapping_add(cell)
        };
        let _: u32 = callee_thiscall!(2, u32, eff, x);
        let back = (parked as i32) as f32;
        let ent2 = ((this + ENTITY) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(3, u32, ent2, back.to_bits());
        0
    }
});
