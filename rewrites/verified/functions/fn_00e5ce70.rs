// original: 0x00e5ce70 registrar_forward_e6e6c0
/// Forward a fixed code pointer to the registrar helper and return its answer.
///
/// Pushes `0xE6E6C0` and calls the registrar (`cdecl/1`, stubbed); the
/// caller-side `(an instruction of the original)` balances the stack. Takes no arguments.
export!(cdecl, rw_00e5ce70() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE6E6C0)) }
});
