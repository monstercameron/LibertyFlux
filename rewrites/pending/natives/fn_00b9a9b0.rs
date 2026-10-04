// original: 0x00b9a9b0 GET_NTH_CLOSEST_WATER_NODE_WITH_HEADING
/// Script native `GET_NTH_CLOSEST_WATER_NODE_WITH_HEADING` (hash 0x36F453FF).
///
/// Calls the engine with a fixed image pointer (the handler pushes the word
/// 0xB9C540, whose position carries a HIGHLOW relocation, so the runtime
/// value tracks the loaded image base) followed by the call context
/// pointer itself. No return slot is written.
export!(cdecl, rw_00b9a9b0(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, lf_rn90_rt::relocated(0x00B9C540), ctx as u32)
});
