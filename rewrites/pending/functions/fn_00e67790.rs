// original: 0x00e67790 register_model_predator
/// Register the `predator` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_PREDATOR_NODE: u32 = 0x012F9E30;
pub const REGISTER_MODEL_PREDATOR_NAME: u32 = 0x00E9E29C;
lf_checker_rt::export!(cdecl, rw_00e67790() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_PREDATOR_NODE), lf_checker_rt::relocated(REGISTER_MODEL_PREDATOR_NAME))
});
