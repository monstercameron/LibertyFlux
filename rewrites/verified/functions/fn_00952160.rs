// original: 0x00952160 global_mode_dispatch
/// Issues the mode call selected by the global mode word.
///
/// Does nothing when the argument is null or the counters show no advance.
/// Otherwise one two-word call goes out with arguments fixed by the mode,
/// echoing the argument only in the register modes. Returns 1 when the mode
/// was examined, 0 when skipped.
export!(cdecl, rw_00952160(arg: u32) -> u32 {
    unsafe {
        const CURRENT: u32 = 0x011F_707C;
        const PREVIOUS: u32 = 0x011F_70C4;
        const MODE: u32 = 0x0103_7720;
        if arg == 0 {
            return 0;
        }
        if *global::<u32>(PREVIOUS) == (*global::<u32>(CURRENT)).wrapping_sub(1) {
            return 0;
        }
        match *global::<u32>(MODE) {
            3 => {
                callee_cdecl!(1, u32, 4, 0);
            }
            4 => {
                callee_cdecl!(1, u32, 3, 0);
            }
            9 => {
                callee_cdecl!(1, u32, 5, 0);
            }
            0xA => {
                callee_cdecl!(1, u32, 6, 0);
            }
            5 => {
                callee_cdecl!(1, u32, 1, 1);
            }
            8 | 6 | 7 => {
                callee_cdecl!(1, u32, 2, arg);
            }
            _ => {}
        }
        1
    }
});
