// original: 0x00bb8e60 PED_QUEUE_REJECT_PEDS_WITH_FLAG_TRUE
/// Script native `PED_QUEUE_REJECT_PEDS_WITH_FLAG_TRUE` (hash 0x79E5237B).
///
/// Forwards one flag value to the engine's ped-queue worker. No return
/// slot is written.
export!(cdecl, rw_00bb8e60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
