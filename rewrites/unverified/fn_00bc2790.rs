// original: 0x00bc2790 alloc_task_35_gated (proposed)

/// Allocate a kind-0x35 task for a handle unless the shutdown gate is closed.
///
/// Calls the gate (cdecl, no arguments): when its low byte is non-zero the
/// function returns that answer unchanged and does nothing else. Otherwise it
/// reads the allocator pointer from `G_ALLOC`, allocates an object through it
/// (thiscall, no stack arguments) and, when the allocation fails, reports
/// `(handle, null, 0x35)` to the result sink (cdecl, three words). On success
/// it initialises the object (ecx-only call), stores the vtable pointer
/// `0xEB747C` at `+0x00` and `param` at `+0x14`, then reports
/// `(handle, object, 0x35)` to the sink. Returns the sink's answer, or the
/// gate's answer when the gate is closed.
///
/// Original: 0x00BC2790 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00bc2790(handle: u32, param: u32) -> u32 {
    const G_ALLOC: u32 = 0x0167E2A0;
    const VTABLE: u32 = 0x00EB747C;
    const OFF_PARAM: u32 = 0x14;
    const KIND: u32 = 0x35;
    let gate: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    if gate & 0xFF != 0 {
        return gate;
    }
    let alloc: u32 =
        unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
    let obj: u32 = lf_checker_rt::callee_thiscall!(2, u32, alloc);
    if obj == 0 {
        return lf_checker_rt::callee_cdecl!(4, u32, handle, 0, KIND);
    }
    let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj);
    unsafe {
        (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((obj + OFF_PARAM) as *mut u32).write_unaligned(param);
    }
    lf_checker_rt::callee_cdecl!(4, u32, handle, obj, KIND)
});
