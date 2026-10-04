// original: 0x00e67710 register_model_e2_new_boat_bow
/// Register the `e2_new_boat_bow` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_E2_NEW_BOAT_BOW_NODE: u32 = 0x012FA328;
pub const REGISTER_MODEL_E2_NEW_BOAT_BOW_NAME: u32 = 0x00E9E5C8;
lf_checker_rt::export!(cdecl, rw_00e67710() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_BOW_NODE), lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_BOW_NAME))
});
