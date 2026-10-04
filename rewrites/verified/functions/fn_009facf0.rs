// original: 0x009facf0 CPlayStatIntInt::vf3
/// Int-int play-statistic serialise step: forwards `(msg, this)` to the
/// shared writer and returns its result unchanged.
export!(thiscall, rw_rs227_009facf0(this_ptr: u32, msg: u32) -> u32 {
    unsafe { callee_cdecl!(0, u32, msg, this_ptr) }
});
