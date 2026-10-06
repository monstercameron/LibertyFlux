// original: 0x0099DFA0 audio_voice_event_clamped (proposed)

/// Fire the voice event as `audio_fire_voice_event` but clamp negatives.
///
/// The source prefers `this`+0x94 over `this`+0x60; with neither the result
/// is 0. Otherwise the event fires exactly as in `audio_fire_voice_event`
/// (bank byte at +0x48 with its 0xFF fast path, program byte at +0x40 with
/// the 0x6F40 stride over the global table added to the scaled global
/// rate), and the answer is clamped at zero with a SIGNED comparison before
/// it is returned. Thiscall with no stack words.
lf_checker_rt::export!(thiscall, rw_0099DFA0(this: u32) -> u32 {
    unsafe {
        const SRC_FIRST: u32 = 0x94;
        const SRC_FALLBACK: u32 = 0x60;
        const PROGRAM: u32 = 0x40;
        const BANK: u32 = 0x48;
        const NO_BANK: u8 = 0xFF;
        const RATE: u32 = 0x0115D964;
        const TABLE_BASE: u32 = 0x0115D988;
        const STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F10;
        const EVENT_CALLEE: u32 = 1;
        let mut src = ((this.wrapping_add(SRC_FIRST)) as *const u32).read_unaligned();
        if src == 0 {
            src = ((this.wrapping_add(SRC_FALLBACK)) as *const u32).read_unaligned();
            if src == 0 {
                return 0;
            }
        }
        let bank = ((src.wrapping_add(BANK)) as *const u8).read();
        let id = if bank == NO_BANK {
            0
        } else {
            let prog = ((src.wrapping_add(PROGRAM)) as *const u8).read() as u32;
            let rate = (lf_checker_rt::global::<u32>(RATE) as *const u32).read_unaligned();
            let base = (lf_checker_rt::global::<u32>(TABLE_BASE) as *const u32).read_unaligned();
            let scaled = (rate as i32).wrapping_mul(bank as i32) as u32;
            let cell = (base.wrapping_add(prog.wrapping_mul(STRIDE)).wrapping_add(TABLE_BIAS)
                        as *const u32).read_unaligned();
            scaled.wrapping_add(cell)
        };
        let ans = lf_checker_rt::callee_thiscall!(EVENT_CALLEE, u32, id, 0);
        // Signed clamp at zero, as the original's `cmovns`.
        if (ans as i32) >= 0 {
            ans
        } else {
            0
        }
    }
});
