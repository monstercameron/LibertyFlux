// original: 0x00e67820 register_model_n_apc
/// Register the `n_apc` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_N_APC_NODE: u32 = 0x012FA244;
pub const REGISTER_MODEL_N_APC_NAME: u32 = 0x00E9E634;
lf_checker_rt::export!(cdecl, rw_00e67820() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_N_APC_NODE), lf_checker_rt::relocated(REGISTER_MODEL_N_APC_NAME))
});
