// original: 0x00e67700 register_model_e2_new_boat_main
/// Register the `e2_new_boat_main` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_E2_NEW_BOAT_MAIN_NODE: u32 = 0x012FA688;
pub const REGISTER_MODEL_E2_NEW_BOAT_MAIN_NAME: u32 = 0x00E9E5B4;
lf_checker_rt::export!(cdecl, rw_00e67700() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_MAIN_NODE), lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_MAIN_NAME))
});
