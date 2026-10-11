// original: 0x00DB1480 mainloop_timing_query_slot_by_index
/// Sends one stack argument to the shared indexed-query helper. The wrapper
/// uses a relocated static object as the helper's `this` pointer, forwards the
/// complete 32-bit argument unchanged, and returns the helper's complete EAX
/// result without additional state changes.
lf_checker_rt::export!(cdecl, rw_00DB1480(index: u32) -> u32 {
    const HELPER_OBJECT_VA: u32 = 0x0198_1A4C;
    let helper_object = lf_checker_rt::relocated(HELPER_OBJECT_VA);
    lf_checker_rt::callee_thiscall!(1, u32, helper_object, index)
});
