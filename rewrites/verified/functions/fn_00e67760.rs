// original: 0x00e67760 register_model_floater
/// Register the `floater` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_FLOATER_NODE: u32 = 0x012FA4C0;
pub const REGISTER_MODEL_FLOATER_NAME: u32 = 0x00E9E2F0;
lf_checker_rt::export!(cdecl, rw_00e67760() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_FLOATER_NODE), lf_checker_rt::relocated(REGISTER_MODEL_FLOATER_NAME))
});
