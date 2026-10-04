// original: 0x00bb1e00 GET_LOCAL_PLAYER_WEAPON_STAT
/// Script native `GET_LOCAL_PLAYER_WEAPON_STAT` (hash 0x3CCC5AFD).
///
/// Forwards two script arguments (a weapon and a stat selector) to the
/// engine and stores its full 32-bit answer into the return slot. Unlike
/// the boolean natives, this handler keeps the whole answer (`mov`, not
/// `movzx`).
export!(cdecl, rw_00bb1e00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
