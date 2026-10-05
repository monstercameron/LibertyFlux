// original: 0x0062C1E0 rage::VerletWaterSimulation::vf1

/// Push five simulation parameter words to the render-state callees.
///
/// The global mode byte selects the encoding of the first two words: when it
/// equals `MODE_FLOAT` the signed integers at `+0x10`/`+0x14` of `this` are
/// converted to float (`cvtdq2ps`, round to nearest even) and tagged 2,
/// otherwise they pass through as integers tagged 1. Each word travels by
/// address in a scratch slot (compared by snapshot, not by address) to the
/// wide callee (thiscall on `arg0 + 0x18`: `arg0 + 0x14`, `arg1 + 4` or
/// `arg1 + 8`, the word address, `4`, `1`, the tag). The remaining three
/// words at `+0x18`/`+0x1c`/`+0x20` go by value with `arg1 + 0x0c`/`+0x10`/
/// `+0x14` to the short callee. Returns the last callee's answer (thiscall,
/// two arguments). Note: the original reuses its incoming second-argument
/// slot as the scratch slot, so the stack comparison is off for this proof.
lf_checker_rt::export!(thiscall, rw_0062c1e0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const MODE_FLOAT: u8 = 0x63;
        const MODE_BYTE: u32 = 0x105C77F;
        const STATE: u32 = 0x18;
        const ADDR_OFF: u32 = 0x14;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        let float_mode = *lf_checker_rt::global::<u8>(MODE_BYTE) == MODE_FLOAT;
        let esi = arg0.wrapping_add(ADDR_OFF);
        let state = rd(arg0, STATE);
        let raw_a = rd(this, 0x10);
        let (tag_a, word_a) = if float_mode {
            (2u32, (raw_a as i32 as f32).to_bits())
        } else {
            (1u32, raw_a)
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            1, u32, state, esi, rd(arg1, 4), &word_a as *const u32 as u32, 4, 1, tag_a);
        let raw_b = rd(this, 0x14);
        let (tag_b, word_b) = if float_mode {
            (2u32, (raw_b as i32 as f32).to_bits())
        } else {
            (1u32, raw_b)
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, state, esi, rd(arg1, 8), &word_b as *const u32 as u32, 4, 1, tag_b);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(3, u32, state, esi, rd(arg1, 0x0c), rd(this, 0x18));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, state, esi, rd(arg1, 0x10), rd(this, 0x1c));
        lf_checker_rt::callee_thiscall!(5, u32, state, esi, rd(arg1, 0x14), rd(this, 0x20))
    }
});
