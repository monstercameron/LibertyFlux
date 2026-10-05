// original: 0x0062C410 rage::VerletWaterPerturbation::vf1

/// Forward one perturbation parameter block to the render-state callee.
///
/// Calls the callee (thiscall on the state object at `+0x18` of `arg0`) with
/// the address `arg0 + 0x14`, the word at `+4` of `arg1` and the field at
/// `+0x10` of `this`. Returns the callee's answer (thiscall, two arguments).
lf_checker_rt::export!(thiscall, rw_0062c410(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x18;
        const ADDR_OFF: u32 = 0x14;
        const FIELD: u32 = 0x10;
        const CALLEE_STATE: u32 = 1;
        let state = ((arg0 + STATE) as *const u32).read_unaligned();
        let word = ((arg1 + 4) as *const u32).read_unaligned();
        let field = ((this + FIELD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            CALLEE_STATE, u32, state, arg0.wrapping_add(ADDR_OFF), word, field)
    }
});
