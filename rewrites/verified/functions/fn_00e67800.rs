// original: 0x00e67800 register_model_ambulance
/// Register the `ambulance` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_AMBULANCE_NODE: u32 = 0x012FA304;
pub const REGISTER_MODEL_AMBULANCE_NAME: u32 = 0x00E9DFE8;
lf_checker_rt::export!(cdecl, rw_00e67800() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_AMBULANCE_NODE), lf_checker_rt::relocated(REGISTER_MODEL_AMBULANCE_NAME))
});
