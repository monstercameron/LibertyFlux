// original: 0x00ad1a50 call_then_tail_jump_stub
/// Helper call with two constant words, then hand off to shared logic.
///
/// Takes no object and reads no registers. Calls the helper with the two
/// constant words, drops them from the stack, then transfers control to
/// the shared continuation, returning whatever that call answers.
export!(cdecl, rw_00ad1a50() -> u32 {
    callee_cdecl!(2, u32, 0x13, 0);
    callee_cdecl!(1, u32,)
});
