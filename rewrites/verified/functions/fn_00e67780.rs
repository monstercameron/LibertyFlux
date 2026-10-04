// original: 0x00e67780 register_model_marquis
/// Register the `marquis` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_MARQUIS_NODE: u32 = 0x012FA3AC;
pub const REGISTER_MODEL_MARQUIS_NAME: u32 = 0x00E9E2C4;
lf_checker_rt::export!(cdecl, rw_00e67780() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_MARQUIS_NODE), lf_checker_rt::relocated(REGISTER_MODEL_MARQUIS_NAME))
});
