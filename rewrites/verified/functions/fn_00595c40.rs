// original: 0x00595c40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_263, player_schema::LeaderboardInfo, 10>::vf2
/// Ask the object for its leaderboard id through vtable slot 1 and, when the
/// answer equals the expected key and the out-pointer is non-null, install this
/// class's vtable there. Returns the out-pointer on success, null otherwise.
///
/// The stored address is an image pointer with a loader fixup, so the rewrite
/// derives it from the file address like any other global.
export!(thiscall, rw_00595c40(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const CLASS_VTABLE: u32 = 0x00FD8454;
        let vtable = *(this as *const u32);
        let slot = *((vtable.wrapping_add(4)) as *const u32);
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if get_id(this) != key {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        *(out as *mut u32) = relocated(CLASS_VTABLE);
        out
    }
});
