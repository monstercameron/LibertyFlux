// original: 0x005eef70 T_CB_Generic_1Arg<void(*)(float), float>::vf1

/// Invoke the stored one-argument float callback.
/// `this` holds the function pointer at `+0x08` and one float argument at `+0x0C`.
/// Calls `target(f0)` with the cdecl convention; the callee return is discarded.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the holder object (id 1, cdecl, 1 argument).
lf_checker_rt::export!(thiscall, rw_005eef70(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const F0: u32 = 0x0C;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32) =
            core::mem::transmute(target as usize);
        let a0 = ((this + F0) as *const u32).read_unaligned();
        f(a0);
    }
    0
});
