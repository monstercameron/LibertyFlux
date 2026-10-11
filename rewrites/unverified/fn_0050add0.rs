// original: 0X0050ADD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race39NoHolds,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Initialize this specialization of `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race39NoHolds,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>`.
///
/// The 32-bit object pointer is passed in ECX. The function calls the shared
/// leaderboard-info base constructor, installs this specialization's vtable,
/// initializes the embedded info object at `+0x4a0` with its vtable and an
/// empty tag, sets bit 0 of the byte at `+0x5a4` while preserving its other
/// bits, and clears the embedded object's remaining five words. It returns
/// the original object pointer in EAX.
lf_checker_rt::export!(thiscall, rw_0050add0(this_ptr: u32) -> u32 {
    // The base constructor is intercepted and answered by the proof contract.
    lf_checker_rt::callee_thiscall!(2, u32, this_ptr);
    unsafe {
    const OBJECT_VTABLE: u32 = 0x00FD9CB4;
    const MEMBER_INFO_VTABLE: u32 = 0x00FDF6DC;
    const MEMBER_INFO_OFFSET: usize = 0x4A0;
    const MEMBER_TAG_OFFSET: usize = 0x4A4;
    const MEMBER_ZERO_WORDS: [usize; 4] = [0x4A8, 0x4AC, 0x4B0, 0x4B4];
    const MEMBER_LAST_WORD_OFFSET: usize = 0x4B8;
    const INITIALIZED_FLAG_OFFSET: usize = 0x5A4;
    const INITIALIZED_FLAG: u8 = 1;
    const EMPTY_TAG: u32 = u32::MAX;
    const LAST_MEMBER_WORD: u32 = 0;

        let object = this_ptr as *mut u8;
        (object as *mut u32).write(lf_checker_rt::relocated(OBJECT_VTABLE));
        (object.add(MEMBER_INFO_OFFSET) as *mut u32)
            .write(lf_checker_rt::relocated(MEMBER_INFO_VTABLE));
        *object.add(INITIALIZED_FLAG_OFFSET) |= INITIALIZED_FLAG;
        (object.add(MEMBER_TAG_OFFSET) as *mut u32).write(EMPTY_TAG);
        for offset in MEMBER_ZERO_WORDS {
            (object.add(offset) as *mut u32).write(0);
        }
        (object.add(MEMBER_LAST_WORD_OFFSET) as *mut u32).write(LAST_MEMBER_WORD);
    }
    this_ptr
});
