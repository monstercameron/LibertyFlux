// original: 0x00dfdd2f heap_free
/// Release a heap block, translating a failure into an `errno` value.
///
/// Frees `ptr` from the process heap. When the release fails, reads the
/// system error, maps it to its `errno` equivalent and stores it in the
/// thread's `errno` slot. A null pointer is a no-op returning 0 here; the
/// original passes its incoming register through on that path, which no
/// checker input can observe, so that path is untested (see lane report).
export!(cdecl, rw_00dfdd2f(ptr: u32) -> u32 {
    unsafe {
        if ptr == 0 {
            return 0;
        }
        let heap = lf_checker_rt::global::<u32>(0x17ac2b4).read();
        let ok = callee_stdcall!(1, u32, heap, 0, ptr);
        if ok != 0 {
            return ok;
        }
        let slot = callee_cdecl!(2, u32,);
        let code = callee_stdcall!(3, u32,);
        let mapped = callee_cdecl!(4, u32, code);
        (slot as *mut u32).write(mapped);
        mapped
    }
});
