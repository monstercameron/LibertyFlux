// original: 0x0065B660 rage::IntervalShadows::vf0 (symbols)

/// Deleting destructor: run the destructor, then free on request.
///
/// Calls the destructor (patched callee) with `this`, then tests the flag's
/// low bit: when set and `this` is non-null, the object is released through
/// the thread-local allocator's free slot (`+0x0c`). Returns `this`
/// (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0065b660(this: u32, flag: u32) -> u32 {
    unsafe {
        const CALLEE_DTOR: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_DTOR, u32, this);
        if flag & 1 == 0 {
            return this;
        }
        if this == 0 {
            return 0;
        }
        let holder = lf_checker_rt::tls_slot(0);
        let frobj = ((holder + 8) as *const u32).read_unaligned();
        let fvt = (frobj as *const u32).read_unaligned();
        let ftgt = ((fvt + 0x0c) as *const u32).read_unaligned();
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ftgt as usize);
        free(frobj, this);        this
    }
});
