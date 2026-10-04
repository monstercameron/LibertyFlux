// original: 0x0051d470 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race25NoHolds, player_schema::LeaderboardInfo, 10>::vf2
/// Interface query on a leaderboard object: succeeding ids stamp an address.
///
/// Calls slot 1 of the object's own function table (an indirect call the
/// checker intercepts by planting its stub address in a fabricated table)
/// to get the object's interface id. When it equals `expect` and `out_ptr`
/// is non-null, stores this instantiation's fixed read-only-data address
/// through `out_ptr` and returns `out_ptr`; otherwise returns 0 (a null
/// `out_ptr` also returns 0 even on an id match). The stored address is a
/// relocated constant: the worker maps the image at a different base than
/// the file, so the rewrite derives it with `relocated`, never literally.
///
/// Arguments: `out_ptr`, the tag receiver (may be null); `expect`, the
/// wanted interface id. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_0051d470(this: u32, out_ptr: u32, expect: u32) -> u32 {
    unsafe {
        const TAG_FILE_VA: u32 = 0x00fdc63c;
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
        let tag = lf_checker_rt::relocated(TAG_FILE_VA);
        (out_ptr as *mut u32).write_unaligned(tag);
        out_ptr
    }
});
