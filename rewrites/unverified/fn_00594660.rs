// original: 0x00594660 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_258, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this row's id through OUT when the live id matches EXPECTED.
///
/// `this` is the leaderboard-info object; its virtual slot 1 (the vtable
/// word at +4) reports the live row id. When it differs from EXPECTED the
/// live id is returned unchanged and nothing is stored. When it matches and
/// OUT is non-null, ROW_ID is stored through OUT and OUT is returned; a null
/// OUT returns 0.
///
/// Original: thiscall, object in ecx, (out, expected) on the stack. The
/// virtual call keeps (this) in ecx and is intercepted by a planted stub.
lf_checker_rt::export!(thiscall, rw_00594660(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
        const ROW_ID: u32 = 0xfd319c;
        const VTABLE_SLOT1_OFF: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let target = (vtable.wrapping_add(VTABLE_SLOT1_OFF) as *const u32).read_unaligned();
        let current: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        let got = current(this);
        if got != expected {
            return got;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(ROW_ID);
        out
    }
});
