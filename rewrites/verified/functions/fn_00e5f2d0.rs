// original: 0x00e5f2d0 forward_codeptr_e6f3c0
/// Forward this unit's fixed code address to the shared worker routine.
///
/// Pushes the constant and calls the routine (cdecl, one argument), cleaning
/// the argument off the stack afterwards. Returns the routine's answer,
/// matching EAX.
export!(cdecl, rw_00e5f2d0() -> u32 {
    callee_cdecl!(1, u32, relocated(0xE6F3C0))
});
