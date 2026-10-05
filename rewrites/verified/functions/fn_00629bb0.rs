// original: 0x00629BB0 streaming_set_uniform_3arg (proposed)

/// Forward one uniform value to the short render-state callee.
///
/// Loads the inner object at `INNER` (`+0x0c`) of `this`, then calls the
/// callee (thiscall on the state object at `+0x18` of the inner object) with
/// the address `inner + 0x14`, the field at `+0x38` and the incoming
/// argument. Returns the callee's answer (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00629bb0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x0c;
        const STATE: u32 = 0x18;
        const ADDR_OFF: u32 = 0x14;
        const FIELD: u32 = 0x38;
        const CALLEE_STATE: u32 = 1;
        let inner = ((this + INNER) as *const u32).read_unaligned();
        let state = ((inner + STATE) as *const u32).read_unaligned();
        let field = ((this + FIELD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            CALLEE_STATE, u32, state, inner.wrapping_add(ADDR_OFF), field, arg0)
    }
});
