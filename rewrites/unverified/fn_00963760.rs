// original: 0x00963760 mode_value_dispatch
/// Resolve a mode value through the mode lookup and record it.
///
/// When the global mode is 3, 4, 9 or 10, returns it at once. Otherwise
/// passes the argument through the mode lookup (stubbed by the checker),
/// records the answer, and records the argument itself as the sub-mode when
/// it is between 0 and 8, returning the lookup answer. Argument 4 takes the
/// shared tail instead and returns 4; larger arguments leave the sub-mode
/// untouched and return its old value.
export!(cdecl, rw_00963760(arg: u32) -> u32 {
    unsafe {
        let mode = *global::<u32>(0x1037720);
        if mode == 3 || mode == 4 || mode == 9 || mode == 10 {
            return mode;
        }
        let ans = callee_cdecl!(1, u32, arg);
        *global::<u32>(0x11F70C8) = ans;
        if arg == 4 {
            *global::<u32>(0x11F70CC) = 4;
            4
        } else if arg <= 8 {
            *global::<u32>(0x11F70CC) = arg;
            ans
        } else {
            let cur = *global::<u32>(0x11F70CC);
            *global::<u32>(0x11F70CC) = cur;
            cur
        }
    }
});
