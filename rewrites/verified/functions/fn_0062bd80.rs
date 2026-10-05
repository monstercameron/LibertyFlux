// original: 0x0062BD80 refcount_release_and_free (proposed)

/// Drop one counted reference held in a slot, destroying at zero.
///
/// Loads the object from the slot at `arg`: a null slot returns at once.
/// Otherwise the 32-bit count at `+0x20` is decremented; when it reaches zero
/// the destroy callee runs (patched, thiscall on the object) and the object
/// is freed through the thread-local allocator's free slot (`+0x0c`). The
/// slot is cleared whenever it held an object. The accumulator at return
/// holds a callee answer or an incoming leftover, so it is not compared
/// (cdecl, one argument).
lf_checker_rt::export!(cdecl, rw_0062bd80(arg: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x20;
        const CALLEE_DESTROY: u32 = 1;
        let obj = (arg as *const u32).read_unaligned();
        if obj == 0 {
            return 0;
        }
        let left = ((obj + COUNT) as *const u32).read_unaligned().wrapping_sub(1);
        ((obj + COUNT) as *mut u32).write_unaligned(left);
        if left == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_DESTROY, u32, obj);
            let holder = lf_checker_rt::tls_slot(0);
            let frobj = ((holder + 8) as *const u32).read_unaligned();
            let fvt = (frobj as *const u32).read_unaligned();
            let ftgt = ((fvt + 0x0c) as *const u32).read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(ftgt as usize);
            free(frobj, obj);
        }
        (arg as *mut u32).write_unaligned(0);
        0
    }
});
