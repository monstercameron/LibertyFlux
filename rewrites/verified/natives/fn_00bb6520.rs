// original: 0x00bb6520 PLAYSTATS_MISSION_CANCELLED
/// Native handler `PLAYSTATS_MISSION_CANCELLED`: report a cancelled mission: forward the mission word to the stats sender.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bb6520(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0),);
        ans
    }
});
