// original: 0x009fac80 CPlayStatInt::vf3
/// Integer play-statistic serialise step (`CPlayStatInt::vf3`).
///
/// Validates the message against this stat, asks the message whether field
/// `0x34` may be written, appends the stat's value plus the message's two
/// offsets, then advances the message cursor. Returns 1 on the full path,
/// 0 if either gate refuses. Only the low byte of each gate matters.
export!(thiscall, rw_rs227_009fac80(this_ptr: u32, msg: u32) -> u8 {
    unsafe {
        let ok = callee_cdecl!(0, u32, msg, this_ptr);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let value = *((this_ptr + 0x34) as *const u32);
        let ok = callee_thiscall!(1, u32, msg, 0x20);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let base = *(msg as *const u32);
        let b = *((msg + 4) as *const u32);
        let c = *((msg + 0xC) as *const u32);
        callee_cdecl!(2, u32, base, value, 0x20, b.wrapping_add(c));
        callee_thiscall!(3, u32, msg, 0x20);
        1
    }
});
