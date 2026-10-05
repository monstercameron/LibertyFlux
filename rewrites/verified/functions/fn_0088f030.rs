// original: 0x0088F030 rage::audVoicePcAdpcm::vf10

/// Current playback position of this ADPCM voice, or -1 when the voice is
/// not playing.
///
/// `this` points to the voice. Its own state gate (slot `+0x18` of its
/// table) runs first; when it reports false the answer is -1. When synth
/// bit 4 of the flags at `+0x8c` is set, the child at `+0x140` is measured
/// (callee 2) and the mixer singleton (reached through its global) is
/// asked for its cursor (callee 3); the answer is the position helper
/// (callee 4) applied to the voice rate words, plus the helper applied to
/// the non-negative part of (measure minus cursor) and the mixer's word at
/// `+0x80`. Otherwise the child is polled (callee 5) and the answer is the
/// helper applied to that answer plus the scaled frequency word at
/// `+0x124` biased by `+0x128`, with the base at `+0xc`.
///
/// Original: 0x0088F030 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088F030(this: u32) -> u32 {
    unsafe {
        const OWN_GATE: u32 = 0x18;
        const FLAGS: u32 = 0x8c;
        const SYNTH: u8 = 0x10;
        const CHILD: u32 = 0x140;
        const MIXER_GLOBAL: u32 = 0x0115_a448;
        const MIXER_WORD: u32 = 0x80;
        const RATE_A: u32 = 0x128;
        const RATE_BASE: u32 = 0x0c;
        const FREQ: u32 = 0x124;
        const BIAS: u32 = 0x128;
        const MEASURE_CHILD: u32 = 2;
        const MIXER_CURSOR: u32 = 3;
        const POS_HELPER: u32 = 4;
        const POLL_CHILD: u32 = 5;

        let table = (this as *const u32).read_unaligned();
        let gate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((table + OWN_GATE) as *const u32).read_unaligned() as usize
        );
        if gate(this) as u8 == 0 {
            return 0xffff_ffff;
        }
        let child = ((this + CHILD) as *const u32).read_unaligned();
        if ((this + FLAGS) as *const u8).read() & SYNTH != 0 {
            let m = lf_checker_rt::callee_thiscall!(MEASURE_CHILD, u32, child);
            let mixer = (lf_checker_rt::relocated(MIXER_GLOBAL) as *const u32)
                .read_unaligned();
            let c = lf_checker_rt::callee_thiscall!(MIXER_CURSOR, u32, mixer);
            let lead = if m >= c { m.wrapping_sub(c) } else { 0 };
            let ra = ((this + RATE_A) as *const u32).read_unaligned();
            let rb = ((this + RATE_BASE) as *const u32).read_unaligned();
            let p = lf_checker_rt::callee_cdecl!(POS_HELPER, u32, ra, rb);
            let mw = ((mixer.wrapping_add(MIXER_WORD)) as *const u32)
                .read_unaligned();
            let q = lf_checker_rt::callee_cdecl!(POS_HELPER, u32, lead, mw);
            p.wrapping_add(q)
        } else {
            let ans = lf_checker_rt::callee_thiscall!(POLL_CHILD, u32, child);
            let freq = ((this + FREQ) as *const u32).read_unaligned();
            let base = ((this + RATE_BASE) as *const u32).read_unaligned();
            let bias = ((this + BIAS) as *const u32).read_unaligned();
            let scaled = (((freq >> 1) << 17).wrapping_add(ans) >> 1)
                .wrapping_add(bias);
            lf_checker_rt::callee_cdecl!(POS_HELPER, u32, scaled, base)
        }
    }
});
