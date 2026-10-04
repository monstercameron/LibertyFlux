// original: 0x00e677e0 register_model_tuga
/// Register the `tuga` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_TUGA_NODE: u32 = 0x012FA5BC;
pub const REGISTER_MODEL_TUGA_NAME: u32 = 0x00E9E2B4;
lf_checker_rt::export!(cdecl, rw_00e677e0() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_TUGA_NODE), lf_checker_rt::relocated(REGISTER_MODEL_TUGA_NAME))
});
