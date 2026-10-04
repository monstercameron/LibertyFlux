// original: 0x00e67740 register_model_e2_new_boat_top3
/// Register the `e2_new_boat_top3` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_E2_NEW_BOAT_TOP3_NODE: u32 = 0x012F9DF4;
pub const REGISTER_MODEL_E2_NEW_BOAT_TOP3_NAME: u32 = 0x00E9E5FC;
lf_checker_rt::export!(cdecl, rw_00e67740() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_TOP3_NODE), lf_checker_rt::relocated(REGISTER_MODEL_E2_NEW_BOAT_TOP3_NAME))
});
