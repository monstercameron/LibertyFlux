// original: 0x0050B190 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race49NoHolds,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>
/// Constructor for the `Ranked_Race49NoHolds` leaderboard-info object (`thiscall`, object in ECX).
///
/// Calls the shared base constructor, then installs this class's primary
/// vtable and the vtable of the embedded member at `+0x4a0`, sets the
/// initialised flag bit, and initialises the member's index/count fields.
/// Returns the object pointer.
export!(thiscall, rw_0050b190(this_ptr: u32) -> u32 {
    // File VAs of the two vtables (both relocated at load).
    const MAIN_VFPTR: u32 = 0x00fd9f80;
    const MEMBER_VFPTR: u32 = 0x00fcf7a4;
    // Byte offset of the embedded member whose vtable and fields follow.
    const MEMBER_OFF: u32 = 0x4a0;
    // Byte offset of the flag byte whose low bit marks initialisation.
    const FLAG_OFF: u32 = 0x5a4;
    // Byte offset of the member's index field, initialised to "none".
    const INDEX_OFF: u32 = 0x4a4;
    // Byte offsets of the member's five trailing fields, zeroed.
    const ZERO_OFFS: [u32; 5] = [0x4a8, 0x4ac, 0x4b0, 0x4b4, 0x4b8];
    // Base constructor takes and returns the object; its answer is discarded.
    let _base_answer = callee_thiscall!(2, u32, this_ptr);
    unsafe {
        let obj = this_ptr as *mut u8;
        (obj as *mut u32).write(relocated(MAIN_VFPTR));
        (obj.add(MEMBER_OFF as usize) as *mut u32).write(relocated(MEMBER_VFPTR));
        let flags = obj.add(FLAG_OFF as usize);
        flags.write(flags.read() | 1);
        (obj.add(INDEX_OFF as usize) as *mut u32).write(0xffff_ffff);
        for off in ZERO_OFFS {
            (obj.add(off as usize) as *mut u32).write(0);
        }
        this_ptr
    }
});
