// original: 0x00963760 mode_select_store
/// Run the mode helper for a requested mode and cache the outcome.
///
/// Reads the current mode from `0x1037720`: when it is 3, 4, 9 or 10 the
/// call does nothing and returns it. Otherwise the helper (cdecl/1) runs on
/// the argument, its answer is kept at `0x11F70C8`, and `0x11F70CC` becomes
/// the argument when it is 0..8, else keeps its value. Direct paths return
/// the helper's answer; the default path returns the stored value.
lf_checker_rt::export!(cdecl, rw_00963760(mode_arg: u32) -> u32 {
    unsafe {
        const CUR: u32 = 0x1037720;
        const CACHED_ANSWER: u32 = 0x11f70c8;
        const CACHED_MODE: u32 = 0x11f70cc;
        let cur = (lf_checker_rt::global::<u32>(CUR) as *const u32).read_unaligned();
        if cur == 3 || cur == 4 || cur == 9 || cur == 0x0a {
            return cur;
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(1, u32, mode_arg);
        (lf_checker_rt::global::<u32>(CACHED_ANSWER) as *mut u32).write_unaligned(r);
        let direct = mode_arg <= 8 && mode_arg != 4;
        let nxt = if mode_arg <= 8 {
            mode_arg
        } else {
            (lf_checker_rt::global::<u32>(CACHED_MODE) as *const u32).read_unaligned()
        };
        (lf_checker_rt::global::<u32>(CACHED_MODE) as *mut u32).write_unaligned(nxt);
        if direct { r } else { nxt }
    }
});
