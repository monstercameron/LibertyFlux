// original: 0x00c9d9a0 wrap_bool_not

/// Test whether the classification helper finds anything.
///
/// Forwards `arg` to the helper (callee 1) and returns its answer with the
/// low byte replaced by 1 when the answer is non-zero, else 0. The upper
/// bytes are the helper's own answer bytes.
///
/// Original: 0x00c9d9a0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00c9d9a0(arg: u32) -> u32 {
    let r: u32 = lf_checker_rt::callee_stdcall!(1, u32, arg);
    (r & 0xffff_ff00) | ((r != 0) as u32)
});
