// original: 0x00e67720 register_model_e2_new_boat_top
/// Register the `e2_new_boat_top` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_E2_NEW_BOAT_TOP_NODE: u32 = 0x012FA3C4;
pub const REGISTER_MODEL_E2_NEW_BOAT_TOP_NAME: u32 = 0x00E9E5D8;
lf_checker_rt::export!(cdecl, rw_00e67720() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_TOP_NODE), lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_TOP_NAME))
});
