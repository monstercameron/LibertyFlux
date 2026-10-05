// original: 0x006298D0 streaming_set_uniform_6arg (proposed)

/// Forward one uniform value to the render-state callee with fixed tags.
///
/// Loads the inner object at `INNER` (`+0x0c`) of `this`, then calls the
/// callee (thiscall on the state object at `+0x18` of the inner object) with
/// the address `inner + 0x14`, the field at `+0x3c`, the incoming argument
/// and the constant tags `0x40`, `1`, `9`. Returns the callee's answer
/// (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_006298d0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x0c;
        const STATE: u32 = 0x18;
        const ADDR_OFF: u32 = 0x14;
        const FIELD: u32 = 0x3c;
        const CALLEE_STATE: u32 = 1;
        let inner = ((this + INNER) as *const u32).read_unaligned();
        let state = ((inner + STATE) as *const u32).read_unaligned();
        let field = ((this + FIELD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            CALLEE_STATE, u32, state, inner.wrapping_add(ADDR_OFF), field, arg0, 0x40, 1, 9)
    }
});
