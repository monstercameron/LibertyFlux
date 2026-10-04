// original: 0x009cc3e0 RETUNE_RADIO_TO_STATION_INDEX
/// RETUNE_RADIO_TO_STATION_INDEX: Retunes the radio to the station with the given index; forward 1 script argument to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_009cc3e0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0,)
});
