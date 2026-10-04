// original: 0x0051ba30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race19NoHolds, player_schema::LeaderboardInfo, 10>::vf2
/// Interface query on a leaderboard object: succeeding ids stamp a tag.
///
/// Calls slot 1 of the object's own function table (an indirect call the
/// checker intercepts by planting its stub address in a fabricated table)
/// to get the object's interface id. When it equals `expect` and `out_ptr`
/// is non-null, stores this instantiation's tag word through `out_ptr` and
/// returns `out_ptr`; otherwise returns 0 (a null `out_ptr` also returns 0
/// even on an id match).
///
/// Arguments: `out_ptr`, the tag receiver (may be null); `expect`, the
/// wanted interface id. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_0051ba30(this: u32, out_ptr: u32, expect: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00fdf7a4;
        const VTABLE_SLOT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let target = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if query(this) != expect {
            return 0;
        }
        if out_ptr == 0 {
            return 0;
        }
        (out_ptr as *mut u32).write_unaligned(TAG);
        out_ptr
    }
});
