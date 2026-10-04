// original: 0x00bc6490 GET_VEHICLE_COMPONENT_INFO
/// Script native `GET_VEHICLE_COMPONENT_INFO` (hash 0x3B5D0F27).
///
/// Passes a fixed engine helper pointer and the call context itself to the engine dispatcher, which reads the script arguments from the context. No return slot is written.
export!(cdecl, rw_00bc6490(ctx: *const u8) -> u32 {
    // The engine dispatcher reads the script arguments from the
    // context itself; the first word is a helper code pointer the
    // original pushes as a relocated immediate.
    let helper = lf_k2_rt::relocated(0x00BCBAD0);
    callee_cdecl!(1, u32, helper, ctx as u32)
});
