// original: 0x00963190 mode_transition_set
/// Set the dispatcher mode, running transition checks first.
///
/// Takes the new mode word. Modes 3-6 and 8-10 run the gate checks (skip
/// the transition helper when the override flag at `0x11F7060` is 1, when
/// the generation at `0x12088B4` differs from `0xF1C040`, or when the flag
/// word at `0x11F66A0` is non-zero; otherwise call it and latch `0x11F7077`
/// unless the mode-guard helper vetoes); modes 11-17 call the mode-guard
/// helper (no arguments) and skip the transition helper when its low byte
/// is non-zero; every other mode, including 7, skips both. The old mode at
/// `0x1037720` is then replaced and kept at `0x1037724`, and the old mode is
/// returned. The mode-minus-3 dispatch is unsigned (`ja`); the guard answer
/// is tested by low byte only.
lf_checker_rt::export!(cdecl, rw_00963190(mode_arg: u32) -> u32 {
    unsafe {
        const OVERRIDE: u32 = 0x11f7060;
        const GEN: u32 = 0x12088b4;
        const GEN_REF: u32 = 0xf1c040;
        const LATCH_EN: u32 = 0x11f66a0;
        const LATCH: u32 = 0x11f7077;
        const MODE: u32 = 0x1037720;
        const PREV: u32 = 0x1037724;
        const MODE_GUARD_OK: u32 = 0x12;
        let g = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();
        let run_helper = match mode_arg {
            3 | 4 | 5 | 6 | 8 | 9 | 10 => {
                if g(OVERRIDE) == 1 || g(GEN) != g(GEN_REF) {
                    g(LATCH_EN) == 0
                } else if g(MODE) != MODE_GUARD_OK {
                    true
                } else {
                    g(LATCH_EN) == 0
                }
            }
            11 | 12 | 13 | 14 | 15 | 16 | 17 => {
                let r: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
                (r & 0xff) == 0
            }
            _ => false,
        };
        if run_helper {
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            (lf_checker_rt::global::<u8>(LATCH) as *mut u8).write(1);
        }
        let old = g(MODE);
        (lf_checker_rt::global::<u32>(MODE) as *mut u32).write_unaligned(mode_arg);
        (lf_checker_rt::global::<u32>(PREV) as *mut u32).write_unaligned(old);
        old
    }
});
