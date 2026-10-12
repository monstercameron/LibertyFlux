// original: 0x00516B40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race1NoHolds,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2
///
/// Identify the object through its virtual slot and stamp the caller's slot.
///
/// Calls the virtual function at slot +4 of the object `this` points to (the
/// call keeps `this` in ECX and takes no stack arguments), then requires its
/// result to equal `expected` and `slot` to be non-null. On success the
/// leaderboard vtable address is stored through `slot` and `slot` is
/// returned; otherwise the result is null.
///
/// thiscall: the object pointer arrives in ECX, `slot` and `expected` on the
/// stack in that order.
lf_checker_rt::export!(thiscall, rw_00516b40(this: u32, slot: u32, expected: u32) -> u32 {
    const VIRTUAL_SLOT_AT: u32 = 4;
    const LEADERBOARD_VTABLE: u32 = 0x00FD5FAC;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let vtable = read_u32(this);
        let identify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(read_u32(vtable.wrapping_add(VIRTUAL_SLOT_AT)) as usize);
        if identify(this) != expected {
            return 0;
        }
        if slot == 0 {
            return 0;
        }
        (slot as *mut u32).write_unaligned(lf_checker_rt::relocated(LEADERBOARD_VTABLE));
        slot
    }
});
