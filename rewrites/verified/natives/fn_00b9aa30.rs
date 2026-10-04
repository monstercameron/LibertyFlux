// original: 0x00b9aa30 GET_RANDOM_CAR_NODE
/// Script native `GET_RANDOM_CAR_NODE` (hash 0x588E1506).
///
/// Calls the engine with a relocated engine pointer (a HIGHLOW fixup sits on
/// the pushed immediate in the executable, so it tracks the image base) and
/// the call context pointer itself. No return slot is written.
export!(cdecl, rw_00b9aa30(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0x00B9_C730), ctx as u32)
});
