// original: 0x00c47010 CCamScript::vf5 (symbols)
/// Refresh the script camera only while mode 2 is active.
///
/// When the mode global holds `ACTIVE_MODE` (2), passes `this + PAYLOAD`
/// to the refresh routine (callee 1, cdecl); otherwise does nothing.
/// Always returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c47010(this: u32) -> u32 {
    const MODE_GLOBAL: u32 = 0x011d6fd4;
    const ACTIVE_MODE: u32 = 2;
    const PAYLOAD: u32 = 0x10;
    const REFRESH: u32 = 1;
    unsafe {
        if lf_checker_rt::global::<u32>(MODE_GLOBAL).read_unaligned() == ACTIVE_MODE {
            lf_checker_rt::callee_cdecl!(REFRESH, u32, this + PAYLOAD);
        }
    }
    1
});
