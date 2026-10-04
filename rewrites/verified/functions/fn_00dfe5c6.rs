// original: 0x00dfe5c6 forward_3arg_zero1
// rs03f18: three-argument forward with a constant first argument (cdecl/2).
//
// Calls the worker with a constant zero first argument followed by its own
// two arguments, returning the worker's answer.
export!(cdecl, rw_rs03f18(a: u32, b: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, 0, a, b) }
});
