// original: 0x00A638B0 has_task_for_native_ids_0x3ae_0x3a9_0x3b5

/// Ask the native bridge about IDs `0x3ae`, `0x3a9`, then `0x3b5`,
/// stopping at the first nonzero result. Return a one-byte boolean in AL.
/// The bridge context is `this + 0x44` in ECX and each identifier is one
/// stack word. Calls are scripted independently by the checker.
lf_checker_rt::export!(thiscall, rw_00a638b0(this: u32) -> u32 {
    let context = this.wrapping_add(0x44);
    if lf_checker_rt::callee_thiscall!(1, u32, context, 0x3ae) != 0 { return 1; }
    if lf_checker_rt::callee_thiscall!(2, u32, context, 0x3a9) != 0 { return 1; }
    if lf_checker_rt::callee_thiscall!(3, u32, context, 0x3b5) != 0 { return 1; }
    0
});
