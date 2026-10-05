// original: 0x00988600 audCutsceneAudioEntity::vf1

/// Cutscene audio entity virtual slot 1: creates two handles, then tail-calls
/// the base slot.
///
/// Clears the handle word at `+0x94` and the flag byte at `+0xa2` of `this`,
/// then asks the audio manager at `MANAGER` (callee ids 1 and 2, one vtable-ish
/// argument each) for two handles, storing the answers in the globals
/// `HANDLE_A` and `HANDLE_B`. Clears the word at `+0x98` and tail-calls the
/// base implementation (callee id 3) with `this`, returning its answer.
/// Original: thiscall, no stack words, ends in a jump.
lf_checker_rt::export!(thiscall, rw_00988600(this: u32) -> u32 {
    const MANAGER: u32 = 0x115d9a0;
    const HANDLE_A: u32 = 0x1282fa0;
    const HANDLE_B: u32 = 0x1282fa4;
    const CREATE_A: u32 = 1;
    const CREATE_B: u32 = 2;
    const TAIL_BASE: u32 = 3;
    unsafe {
        let mgr = lf_checker_rt::relocated(MANAGER);
        ((this + 0x94) as *mut u32).write_unaligned(0);
        ((this + 0xa2) as *mut u8).write(0);
        let a: u32 = lf_checker_rt::callee_thiscall!(
            CREATE_A,
            u32,
            mgr,
            lf_checker_rt::relocated(0xe8e454)
        );
        *lf_checker_rt::global::<u32>(HANDLE_A) = a;
        let b: u32 = lf_checker_rt::callee_thiscall!(
            CREATE_B,
            u32,
            mgr,
            lf_checker_rt::relocated(0xe8e468)
        );
        *lf_checker_rt::global::<u32>(HANDLE_B) = b;
        ((this + 0x98) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(TAIL_BASE, u32, this)
    }
});
