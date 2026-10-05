// original: 0x008A9A20 audio_counter_bump (proposed)

/// Run a lock/unlock pair around the shared audio counter and return it.
///
/// Pushes global `0x115f858` to each of the lock (`0x403cb0`) and unlock
/// (`0x403cf0`) callees (both cdecl, one argument), reading global
/// `0x115f854` between the two calls and returning it. Neither call result
/// is used. Original is cdecl with no stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_008A9A20() -> u32 {
    const LOCK: u32 = 1;
    const UNLOCK: u32 = 2;
    const HANDLE: u32 = 0x0115_f858;
    const COUNTER: u32 = 0x0115_f854;
    unsafe {
        let h = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, h);
        let h2 = (lf_checker_rt::global::<u32>(HANDLE) as *const u32).read_unaligned();
        let c = (lf_checker_rt::global::<u32>(COUNTER) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, h2);
        c
    }
});
