// original: 0x00ade0c0 T_CB_Generic_4Args<void(*)(rage::Vector4&, float, float, float), rage::Vector4, float, float, float>::vf1

/// Invoke the stored four-argument callback of a generic callback holder.
///
/// `this` points to the holder: the function pointer at `+0x08`, a
/// four-float vector at `+0x10`, and three float arguments at `+0x20`,
/// `+0x24`, `+0x28`. Calls the stored pointer as
/// `target(this+0x10, f0, f1, f2)` with the cdecl convention and returns
/// nothing (the callee's return value is discarded).
///
/// Edge cases: none in this function; the target is always called exactly
/// once with the four words in object order.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the heap object (id 1, cdecl, four arguments).
lf_checker_rt::export!(thiscall, rw_00ade0c0(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const VECTOR: u32 = 0x10;
        const ARG0: u32 = 0x20;
        const ARG1: u32 = 0x24;
        const ARG2: u32 = 0x28;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32, u32, u32) =
            core::mem::transmute(target as usize);
        let f0 = ((this + ARG0) as *const u32).read_unaligned();
        let f1 = ((this + ARG1) as *const u32).read_unaligned();
        let f2 = ((this + ARG2) as *const u32).read_unaligned();
        f(this + VECTOR, f0, f1, f2);
    }
    0
});
