// original: 0x00BD9060 RESERVE_NETWORK_MISSION_VEHICLES_FOR_HOST
// RESERVE_NETWORK_MISSION_VEHICLES_FOR_HOST: forwards one script word.
export!(cdecl, rw_fn_bd9060(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let _ans: u32 = callee_cdecl!(1, u32, *args);
    }
});
