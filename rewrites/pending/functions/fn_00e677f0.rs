// original: 0x00e677f0 register_model_caddy
/// Register the `caddy` model: prepend its static node to the
/// global model list via the shared prepend helper, and return the node.
pub const REGISTER_MODEL_CADDY_NODE: u32 = 0x012F9F50;
pub const REGISTER_MODEL_CADDY_NAME: u32 = 0x00E9E5A0;
lf_checker_rt::export!(cdecl, rw_00e677f0() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(REGISTER_MODEL_CADDY_NODE), lf_checker_rt::relocated(REGISTER_MODEL_CADDY_NAME))
});
