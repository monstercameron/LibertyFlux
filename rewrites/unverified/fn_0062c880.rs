// original: 0x0062C880 rage::SkyhatPerlinNoise::vf0 (symbols)

/// Deleting destructor: run the destructor, then free on request.
///
/// It first stamps the class vtable pointer at `+0x00`, then calls the destructor (patched callee) with `this`, then tests the flag's
/// low bit: when set, the object is released through the thread-local
/// allocator's free slot (`+0x0c`). Returns `this` (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0062c880(this: u32, flag: u32) -> u32 {
    unsafe {
        const CALLEE_DTOR: u32 = 1;
        ((this + 0x00) as *mut u32).write_unaligned(lf_checker_rt::relocated(0xFE2404));
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_DTOR, u32, this);
        if flag & 1 == 0 {
            return this;
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
