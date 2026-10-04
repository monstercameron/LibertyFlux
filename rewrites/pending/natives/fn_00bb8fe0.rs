// original: 0x00bb8fe0 SET_PED_QUEUE_MEMBERSHIP_LIST
/// Script native `SET_PED_QUEUE_MEMBERSHIP_LIST` (hash 0x02BD46BB).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb8fe0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
