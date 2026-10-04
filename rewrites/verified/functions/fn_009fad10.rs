// original: 0x009fad10 CPlayStatSessionInfo::vf3
/// Session-info play-statistic serialise step: forwards `(msg, this)` to the
/// shared writer and returns its result unchanged.
export!(thiscall, rw_rs227_009fad10(this_ptr: u32, msg: u32) -> u32 {
    unsafe { callee_cdecl!(0, u32, msg, this_ptr) }
});
