// original: 0x00e67730 register_model_e2_new_boat_top2
/// Register the `e2_new_boat_top2` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_E2_NEW_BOAT_TOP2_NODE: u32 = 0x012FA4CC;
pub const REGISTER_MODEL_E2_NEW_BOAT_TOP2_NAME: u32 = 0x00E9E5E8;
lf_checker_rt::export!(cdecl, rw_00e67730() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_TOP2_NODE), lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_TOP2_NAME))
});
