// original: 0x00AD1240 audio_maybe_trigger_80 (proposed)

/// Fire trigger 0x80 when the armed flag is set and the tick matches.
///
/// Returns at once (with the caller's leftover eax, pinned to 0 in the
/// proof) when the armed byte is clear. Otherwise reads the tick counter
/// (cdecl/0) and fires the trigger helper (cdecl/1) with 0x80 when the tick
/// equals either of the two match globals. Takes no arguments (cdecl/0).
lf_checker_rt::export!(cdecl, rw_00ad1240() -> u32 {
    unsafe {
        const TICK: u32 = 1;
        const FIRE: u32 = 2;
        const ARMED: u32 = 0x0103F425;
        const MATCH_A: u32 = 0x0103F440;
        const MATCH_B: u32 = 0x0103F448;
        const TRIGGER: u32 = 0x80;
        if lf_checker_rt::global::<u8>(ARMED).read() == 0 {
            return 0;
        }
        let tick = lf_checker_rt::callee_cdecl!(TICK, u32,);
        let a = lf_checker_rt::global::<u32>(MATCH_A).read();
        if tick == a {
            return lf_checker_rt::callee_cdecl!(FIRE, u32, TRIGGER);
        }
        let b = lf_checker_rt::global::<u32>(MATCH_B).read();
        if tick != b {
            return tick;
        }
        lf_checker_rt::callee_cdecl!(FIRE, u32, TRIGGER)
    }
});
