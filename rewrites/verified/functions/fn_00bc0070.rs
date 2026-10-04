// original: 0x00bc0070 alloc_task_7e_select (proposed)

/// Allocate a kind-0x7E task with a variant selected by a flag byte.
///
/// Reads the allocator pointer from `G_ALLOC` and allocates an object
/// (thiscall, no stack arguments). When the allocation fails, `(handle, null,
/// 0x7E)` is reported to the result sink (cdecl, three words) and its answer
/// returned. On success a variant is picked from the low byte of `flags`:
/// `0x5E` when it is zero, `0x5C` otherwise. The object is specialised with
/// the variant (thiscall, one word), `(handle, task, 0x7E)` is reported to
/// the sink, and the sink's answer is returned.
///
/// Original: 0x00BC0070 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00bc0070(handle: u32, flags: u32) -> u32 {
    const G_ALLOC: u32 = 0x0167E2A0;
    const KIND: u32 = 0x7E;
    const VARIANT_ZERO: u32 = 0x5E;
    const VARIANT_NONZERO: u32 = 0x5C;
    let alloc: u32 =
        unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
    let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, alloc);
    if obj == 0 {
        return lf_checker_rt::callee_cdecl!(3, u32, handle, 0, KIND);
    }
    let variant: u32 = if flags & 0xFF == 0 { VARIANT_ZERO } else { VARIANT_NONZERO };
    let task: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj, variant);
    lf_checker_rt::callee_cdecl!(3, u32, handle, task, KIND)
});
