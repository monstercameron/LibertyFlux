// original: 0x00d29090 CPedTargetting::vf10 (symbols)

/// Test whether a ped may target another, tail-calling the decider.
///
/// Bails out with 0 unless the candidate record `arg` has exactly the wanted
/// flag bits (`[arg + 0x28] & 0x3c0 == 0xc0`), differs from the current target
/// at `this + 0x24c`, either has its byte at `+0x210` clear or a state word
/// at `+0xa74` outside {1, 2}, and holds a nonzero link at `+0x224`.
/// Otherwise tail-calls the target decider (intercepted) with the current
/// target's link block and `arg`. Only the low byte of the early-out result
/// is defined.
///
/// Original: 0x00D29090 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d29090(this: u32, arg: u32) -> u32 {
    unsafe {
        const FLAGS_OFF: u32 = 0x28;
        const FLAGS_MASK: u32 = 0x3c0;
        const FLAGS_WANT: u32 = 0xc0;
        const CURRENT_OFF: u32 = 0x24c;
        const FLAG_OFF: u32 = 0x210;
        const STATE_OFF: u32 = 0xa74;
        const LINK_OFF: u32 = 0x224;
        if unsafe { ((arg + FLAGS_OFF) as *const u32).read_unaligned() } & FLAGS_MASK != FLAGS_WANT {
            return 0;
        }
        let current = unsafe { ((this + CURRENT_OFF) as *const u32).read_unaligned() };
        if arg == current {
            return 0;
        }
        if unsafe { ((arg + FLAG_OFF) as *const u8).read() } != 0 {
            let state = unsafe { ((arg + STATE_OFF) as *const u32).read_unaligned() };
            if state == 1 || state == 2 {
                return 0;
            }
        }
        if unsafe { ((arg + LINK_OFF) as *const u32).read_unaligned() } == 0 {
            return 0;
        }
        let block = unsafe { ((current + LINK_OFF) as *const u32).read_unaligned() };
        lf_checker_rt::callee_thiscall!(1, u32, block, arg)
    }
});
