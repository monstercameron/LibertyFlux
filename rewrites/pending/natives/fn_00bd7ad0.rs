// original: 0x00bd7ad0 GET_NETWORK_ID_FROM_VEHICLE
/// Script native handler `GET_NETWORK_ID_FROM_VEHICLE` (hash 0x1BC70617).
///
/// Forwards script arguments 0..1 to the engine worker and returns its answer.
export!(cdecl, rw_00bd7ad0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
