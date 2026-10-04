// original: 0x00e677d0 register_model_tropic
/// Register the `tropic` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_TROPIC_NODE: u32 = 0x012FA6A0;
pub const REGISTER_MODEL_TROPIC_NAME: u32 = 0x00E9E2D4;
lf_checker_rt::export!(cdecl, rw_00e677d0() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_TROPIC_NODE), lf_checker_rt::relocated(REGISTER_MODEL_TROPIC_NAME))
});
