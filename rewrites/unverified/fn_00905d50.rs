// original: 0x00905D50 input_slots_rebuild

/// When the rebuild gate is clear, preserve the no-work path: invoke the
/// cookie helper and leave the derived array and counters untouched.
///
/// The gated rebuild and deduplication scan are outside this proof. The
/// cookie helper's ECX is a stack-derived guard value and is not compared;
/// the call itself, return stack adjustment, globals and undeclared writes
/// remain checked.
lf_checker_rt::export!(cdecl, rw_00905D50() -> u32 {
    const ID_COOKIE_CHECK: u32 = 10;
    let _ = unsafe { lf_checker_rt::callee_thiscall!(ID_COOKIE_CHECK, u32, 0) };
    0
});

lf_checker_rt::export!(cdecl, mut_00905D50() -> u32 {
    0
});
