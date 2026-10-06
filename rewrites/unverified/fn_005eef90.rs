// original: 0x005eef90 T_CB_Generic_5Args<void(*)(float, float, CRect&, HtmlRenderState&, bool), float, float, CRect, HtmlRenderState, bool>::vf1

/// Invoke the stored five-argument render callback.
/// `this` holds the function pointer at `+0x08`, two floats at `+0x0C`/`+0x10`,
/// a 4-word rect at `+0x14`, a 49-word render state at `+0x24`, and a flag byte at `+0xE8`.
/// Calls `target(f0, f1, this+0x14, this+0x24, flag)` cdecl; the return is discarded.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the holder object (id 1, cdecl, 5 arguments).
lf_checker_rt::export!(thiscall, rw_005eef90(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const F0: u32 = 0x0C;
        const F1: u32 = 0x10;
        const RECT: u32 = 0x14;
        const STATE: u32 = 0x24;
        const FLAG: u32 = 0xE8;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32, u32, u32, u32) =
            core::mem::transmute(target as usize);
        let a0 = ((this + F0) as *const u32).read_unaligned();
        let a1 = ((this + F1) as *const u32).read_unaligned();
        let flag = ((this + FLAG) as *const u8).read_unaligned() as u32;
        f(a0, a1, this + RECT, this + STATE, flag);
    }
    0
});
