// original: 0x00bc0350 alloc_task_78_bounded (proposed)

/// Allocate a kind-0x78 task selected by a bounded index.
///
/// Reads an index through the parameter getter (cdecl, `(param, 4)`). An
/// index above `0x7F` is returned unchanged and nothing else happens.
/// Otherwise it reads the allocator pointer from `G_ALLOC` and allocates an
/// object (thiscall, no stack arguments). On success the object is specialised
/// with the index (thiscall, one word), its flag byte at `+0x24` is cleared,
/// and `(handle, task, 0x78)` is reported to the result sink (cdecl, three
/// words); the sink's answer is returned. When the allocation fails the
/// original stores through the null pointer (`[0+0x24]`) and faults; the
/// rewrite reproduces that fault identically instead of calling the sink.
///
/// Original: 0x00BC0350 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00bc0350(handle: u32, param: u32) -> u32 {
    const G_ALLOC: u32 = 0x0167E2A0;
    const MAX_INDEX: u32 = 0x7F;
    const KIND: u32 = 0x78;
    const OFF_FLAG: u32 = 0x24;
    let index: u32 = lf_checker_rt::callee_cdecl!(1, u32, param, 4);
    if index > MAX_INDEX {
        return index;
    }
    let alloc: u32 =
        unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
    let obj: u32 = lf_checker_rt::callee_thiscall!(2, u32, alloc);
    if obj == 0 {
        unsafe { (OFF_FLAG as *mut u8).write(0) };
        return lf_checker_rt::callee_cdecl!(4, u32, handle, 0, KIND);
    }
    let task: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj, index);
    unsafe { ((task + OFF_FLAG) as *mut u8).write(0) };
    lf_checker_rt::callee_cdecl!(4, u32, handle, task, KIND)
});
