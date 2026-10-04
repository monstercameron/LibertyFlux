// original: 0x00bb8e30 PED_QUEUE_CONSIDER_PEDS_WITH_FLAG_FALSE
// rw_ped_queue_consider_peds_with_flag_false: native PED_QUEUE_CONSIDER_PEDS_WITH_FLAG_FALSE (handler 0x00BB8E30).
//
// Forwards one flag id to the ped-queue consider-list appender. No return slot.
export!(cdecl, rw_ped_queue_consider_peds_with_flag_false(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
