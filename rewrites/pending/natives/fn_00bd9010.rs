// original: 0x00BD9010 RESERVE_NETWORK_MISSION_OBJECTS
// RESERVE_NETWORK_MISSION_OBJECTS: forward the count to the engine call.
export!(cdecl, rw_00BD9010(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a)
    }
});
