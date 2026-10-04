// original: 0x0057d6f0 leaderboard_174_vf2_keycheck
/// Board-174 key check publishing the board id (virtual slot 2).
///
/// Same behaviour as the board-172 slot with board 174's id word.
export!(thiscall, rw_0057d6f0(this_: u32, out: u32, key: u32) -> u32 {
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
        (out as *mut u32).write_unaligned(relocated(0x00FD0644));
        out
    }
});
