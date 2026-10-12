// original: 0x008A5840 audifsound_method_forward_thunk

/// Forward one audio object to the shared if-sound method.
///
/// Moves the stack argument into ECX and tail-calls the shared routine,
/// returning its result. Only the first nine bytes run from this entry; the
/// bytes that follow in the inventory range are sibling thunks with their
/// own entries. Cdecl-shaped entry, one stack word.
lf_checker_rt::export!(cdecl, rw_008A5840(arg: u32) -> u32 {
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, arg) }
});
