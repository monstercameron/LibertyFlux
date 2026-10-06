// original: 0x005ef150 T_CB_Generic_4Args<void(*)(rage::Color32&, float, float, float), rage::Color32, float, float, float>::vf1

/// Invoke the stored four-argument colour callback.
/// `this` holds the function pointer at `+0x08`, a colour word at `+0x0C`,
/// and three floats at `+0x10`/`+0x14`/`+0x18`.
/// Calls `target(this+0x0C, f0, f1, f2)` cdecl; the return is discarded.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the holder object (id 1, cdecl, 4 arguments).
lf_checker_rt::export!(thiscall, rw_005ef150(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const COLOUR: u32 = 0x0C;
        const F0: u32 = 0x10;
        const F1: u32 = 0x14;
        const F2: u32 = 0x18;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32, u32, u32) =
            core::mem::transmute(target as usize);
        let a0 = ((this + F0) as *const u32).read_unaligned();
        let a1 = ((this + F1) as *const u32).read_unaligned();
        let a2 = ((this + F2) as *const u32).read_unaligned();
        f(this + COLOUR, a0, a1, a2);
    }
    0
});
