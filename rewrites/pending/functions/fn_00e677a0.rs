// original: 0x00e677a0 register_model_reefer
/// Register the `reefer` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_REEFER_NODE: u32 = 0x012FA5B0;
pub const REGISTER_MODEL_REEFER_NAME: u32 = 0x00E9E2BC;
lf_checker_rt::export!(cdecl, rw_00e677a0() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_REEFER_NODE), lf_checker_rt::relocated(REGISTER_MODEL_REEFER_NAME))
});
