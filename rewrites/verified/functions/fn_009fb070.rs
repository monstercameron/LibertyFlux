// original: 0x009fb070 CPlayStatSessionInfo::vf4
/// Session-info play-statistic commit step: forwards `(msg, this)` to the
/// shared check and returns its result unchanged.
export!(thiscall, rw_rs227_009fb070(this_ptr: u32, msg: u32) -> u32 {
    unsafe { callee_cdecl!(0, u32, msg, this_ptr) }
});
