// original: 0x00be7530 CTaskSimpleShovePed::vf17

/// Advance a shove-ped task, resolving the target lazily, and report done.
///
/// `this` points to the task, `ped` to the ped. While the shove handle at
/// `+0x14` is null and the armed flag at `+0x20` is clear, resolves the
/// target at `+0x1c` through callee 1 (thiscall with `this`, one word: the
/// ped) when it is -1, returning 1 early when the resolution itself answers
/// -1, then starts the shove (callee 2, thiscall with `this`, two words: ped
/// and the target). Returns 0 while the armed flag is clear; with no handle
/// returns 1; otherwise tears the handle down (callee 3, one word: the task,
/// modelled as stdcall because its entry `ecx` is path-dependent; then
/// callee 4, thiscall on the handle, one word holding the float -1000.0) and
/// clears the handle. Returns 1 once finished.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7530(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_SHOVE: u32 = 0x14;
        const OFF_TARGET: u32 = 0x1c;
        const OFF_ARMED: u32 = 0x20;
        const RELEASE_ARG: u32 = 0xc47a0000; // -1000.0f
        const RESOLVE: u32 = 1;
        const START_SHOVE: u32 = 2;
        const TEARDOWN: u32 = 3;
        const RELEASE: u32 = 4;

        if ((this + OFF_SHOVE) as *const u32).read_unaligned() == 0
            && ((this + OFF_ARMED) as *const u8).read() == 0
        {
            if ((this + OFF_TARGET) as *const u32).read_unaligned() == 0xFFFFFFFF {
                let h: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, ped);
                ((this + OFF_TARGET) as *mut u32).write_unaligned(h);
                if h == 0xFFFFFFFF {
                    return 1;
                }
            }
            let target = ((this + OFF_TARGET) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(START_SHOVE, u32, this, ped, target);
        }
        if ((this + OFF_ARMED) as *const u8).read() == 0 {
            return 0;
        }
        let shove = ((this + OFF_SHOVE) as *const u32).read_unaligned();
        if shove == 0 {
            return 1;
        }
        lf_checker_rt::callee_stdcall!(TEARDOWN, u32, this);
        lf_checker_rt::callee_thiscall!(RELEASE, u32, shove, RELEASE_ARG);
        ((this + OFF_SHOVE) as *mut u32).write_unaligned(0);
        1
    }
});
