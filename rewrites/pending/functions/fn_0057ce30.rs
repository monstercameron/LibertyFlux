// original: 0x0057ce30 leaderboard_172_vf2_keycheck
/// Board-172 key check publishing the board id (virtual slot 2).
///
/// Queries the object through its first virtual slot and compares the result
/// with `key`: on a match it stores this board's id word into `*out` (when
/// `out` is not null) and returns `out`, otherwise it returns null.
export!(thiscall, rw_0057ce30(this_: u32, out: u32, key: u32) -> u32 {
    unsafe {
        let vtable = (this_ as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(4) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if query(this_) != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(relocated(0x00FDF464));
        out
    }
});
