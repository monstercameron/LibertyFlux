// original: 0x009faf90 CPlayStatFloat::vf4
/// Float play-statistic commit step (`CPlayStatFloat::vf4`).
///
/// Runs the shared check on `(msg, this)`; if it passes, commits the field
/// at `this + 0x34` through the message. Returns 1 only if both agree.
export!(thiscall, rw_rs227_009faf90(this_ptr: u32, msg: u32) -> u8 {
    unsafe {
        let ok = callee_cdecl!(0, u32, msg, this_ptr);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let ok = callee_thiscall!(1, u32, msg, this_ptr.wrapping_add(0x34));
        if (ok & 0xFF) == 0 {
            return 0;
        }
        1
    }
});
