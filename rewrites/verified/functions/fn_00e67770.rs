// original: 0x00e67770 register_model_jetmax
/// Register the `jetmax` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_JETMAX_NODE: u32 = 0x012FA0F4;
pub const REGISTER_MODEL_JETMAX_NAME: u32 = 0x00E9E2A8;
lf_checker_rt::export!(cdecl, rw_00e67770() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_JETMAX_NODE), lf_checker_rt::relocated(REGISTER_MODEL_JETMAX_NAME))
});
