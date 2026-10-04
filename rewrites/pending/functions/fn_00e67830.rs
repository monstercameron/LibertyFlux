// original: 0x00e67830 register_model_biff
/// Register the `biff` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_BIFF_NODE: u32 = 0x012FA394;
pub const REGISTER_MODEL_BIFF_NAME: u32 = 0x00E9E034;
lf_checker_rt::export!(cdecl, rw_00e67830() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_BIFF_NODE), lf_checker_rt::relocated(REGISTER_MODEL_BIFF_NAME))
});
