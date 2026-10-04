// original: 0x008a1700 forward_thunk_8a0b50
/// C-linkage forwarder to a thiscall sound routine (jump thunk).
///
/// Same shape as [`rw_008a0950`]: stack argument into ECX, tail-jump,
/// target's answer is the return value.
export!(cdecl, rw_008a1700(arg: u32) -> u32 {
    callee_thiscall!(1, u32, arg)
});
