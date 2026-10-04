// original: 0x009fb030 CPlayStatIntStr::vf4
/// Int-string play-statistic commit step.
///
/// Runs the shared check on `(msg, this)`; if it passes, commits the field
/// at `this + 0x38` through the message with limit `0x20`. Returns 1 only
/// if both agree.
export!(thiscall, rw_rs227_009fb030(this_ptr: u32, msg: u32) -> u8 {
    unsafe {
        let ok = callee_cdecl!(0, u32, msg, this_ptr);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let ok = callee_thiscall!(1, u32, msg, this_ptr.wrapping_add(0x38), 0x20);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        1
    }
});
