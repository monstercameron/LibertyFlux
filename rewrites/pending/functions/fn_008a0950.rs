// original: 0x008a0950 forward_thunk_89ff90
/// C-linkage forwarder to a thiscall sound routine (jump thunk).
///
/// Moves the stack argument into ECX and tail-jumps; the rewrite forwards
/// the same argument through the checker's tail-jump interception and
/// returns the target's answer.
export!(cdecl, rw_008a0950(arg: u32) -> u32 {
    callee_thiscall!(1, u32, arg)
});
