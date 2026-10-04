// original: 0x00e67810 register_model_apc
/// Register the `apc` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_APC_NODE: u32 = 0x012FA004;
pub const REGISTER_MODEL_APC_NAME: u32 = 0x00E9E594;
lf_checker_rt::export!(cdecl, rw_00e67810() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_APC_NODE), lf_checker_rt::relocated(REGISTER_MODEL_APC_NAME))
});
