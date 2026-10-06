// original: 0x005ef070 T_CB_Generic_6Args<void(*)(HtmlRenderState&, float, float, float, float, unsigned int), HtmlRenderState, float, float, float, float, int>::vf1

/// Invoke the stored six-argument render callback.
/// `this` holds the function pointer at `+0x08`, a 49-word render state at `+0x0C`,
/// four floats at `+0xD0`..`+0xDC`, and an unsigned int at `+0xE0`.
/// Calls `target(this+0x0C, f0, f1, f2, f3, u)` cdecl; the return is discarded.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the holder object (id 1, cdecl, 6 arguments).
lf_checker_rt::export!(thiscall, rw_005ef070(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const STATE: u32 = 0x0C;
        const F0: u32 = 0xD0;
        const F1: u32 = 0xD4;
        const F2: u32 = 0xD8;
        const F3: u32 = 0xDC;
        const UVAL: u32 = 0xE0;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32, u32, u32, u32, u32) =
            core::mem::transmute(target as usize);
        let a0 = ((this + F0) as *const u32).read_unaligned();
        let a1 = ((this + F1) as *const u32).read_unaligned();
        let a2 = ((this + F2) as *const u32).read_unaligned();
        let a3 = ((this + F3) as *const u32).read_unaligned();
        let u = ((this + UVAL) as *const u32).read_unaligned();
        f(this + STATE, a0, a1, a2, a3, u);
    }
    0
});
