// original: 0x009cc890 STOP_PED_MOBILE_RINGING
// rw_stop_ped_mobile_ringing: native STOP_PED_MOBILE_RINGING (handler 0x009CC890).
//
// Forwards one ped handle to the phone-silencer. No return slot.
export!(cdecl, rw_stop_ped_mobile_ringing(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
