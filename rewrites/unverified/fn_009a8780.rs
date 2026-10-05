// original: 0x009a8780 audio_param_push
/// Push a float parameter through the audio entity's effect chain.
///
/// Does nothing when the entity pointer at `this+0x3ac4` is null.
/// Otherwise the parameter `x` is first transformed by the shaper
/// (stubbed, cdecl/1, float in and float ST0 out); the shaped bits
/// overwrite the first stack slot. Then the effect pointer is
/// resolved from the entity's tag bytes (null when the byte at `+4`
/// is 0xff, else the global mix of the bytes at `+4` and `+0x40`),
/// the effect is run on the shaped value (stubbed, thiscall/1),
/// and finally the second stack word re-read as a signed integer,
/// converted back to float, is pushed to the entity (stubbed,
/// thiscall/1). Thiscall, two stack words, no result.
export!(thiscall, rw_009A8780(this: u32, x: u32, arg1: u32) -> u32 {
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
        let _: u32 = callee_thiscall!(2, u32, eff, shaped.to_bits());
        let back = (arg1 as i32) as f32;
        let ent2 = ((this + ENTITY) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(3, u32, ent2, back.to_bits());
        0
    }
});
