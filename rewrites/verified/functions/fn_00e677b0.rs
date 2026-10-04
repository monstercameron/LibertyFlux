// original: 0x00e677b0 register_model_smuggler
/// Register the `smuggler` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_SMUGGLER_NODE: u32 = 0x012F9EE4;
pub const REGISTER_MODEL_SMUGGLER_NAME: u32 = 0x00E9E2E4;
lf_checker_rt::export!(cdecl, rw_00e677b0() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_SMUGGLER_NODE), lf_checker_rt::relocated(REGISTER_MODEL_SMUGGLER_NAME))
});
