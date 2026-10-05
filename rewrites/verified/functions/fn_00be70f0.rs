// original: 0x00be70f0 CTaskSimpleDuckToggle::vf17

/// Toggle a ped's duck state from the task's mode field.
///
/// `this` points to the task, `ped` to the ped. Reads the mode at `+0x14`.
/// When it is 0 or -1 and the sense poll (callee 1, thiscall on the ped, no
/// stack words) answers non-zero, actuates with (0, -1) through callee 2
/// (thiscall on the ped, two words) and returns 1. Otherwise re-reads the
/// mode: when it is 1 or -1 and the sense poll now answers zero, actuates
/// with (1, -1). Returns 1 on every path.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be70f0(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_MODE: u32 = 0x14;
        const SENSE: u32 = 1;
        const ACTUATE: u32 = 2;

        let mode = ((this + OFF_MODE) as *const u32).read_unaligned();
        if mode == 0 || mode == 0xFFFFFFFF {
            let sensed: u32 = lf_checker_rt::callee_thiscall!(SENSE, u32, ped);
            if sensed as u8 != 0 {
                lf_checker_rt::callee_thiscall!(ACTUATE, u32, ped, 0, 0xFFFFFFFF);
                return 1;
            }
        }
        let mode = ((this + OFF_MODE) as *const u32).read_unaligned();
        if mode == 1 || mode == 0xFFFFFFFF {
            let sensed: u32 = lf_checker_rt::callee_thiscall!(SENSE, u32, ped);
            if sensed as u8 == 0 {
                lf_checker_rt::callee_thiscall!(ACTUATE, u32, ped, 1, 0xFFFFFFFF);
            }
        }
        1
    }
});
