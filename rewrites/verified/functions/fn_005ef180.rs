// original: 0x005ef180 T_CB_Generic_4Args<void(*)(rage::grcTexture*, CRect&, CRect&, rage::Color32&), rage::grcTexture*, CRect, CRect, rage::Color32>::vf1

/// Invoke the stored four-argument texture-blit callback.
/// `this` holds the function pointer at `+0x08`, a texture pointer at `+0x0C`,
/// two 4-word rects at `+0x10`/`+0x20`, and a colour word at `+0x30`.
/// Calls `target(tex, this+0x10, this+0x20, this+0x30)` cdecl; the return is discarded.
///
/// Original: thiscall, no stack arguments, one indirect callee through
/// the holder object (id 1, cdecl, 4 arguments).
lf_checker_rt::export!(thiscall, rw_005ef180(this: u32) -> u32 {
    unsafe {
        const TARGET: u32 = 0x08;
        const TEX: u32 = 0x0C;
        const SRC: u32 = 0x10;
        const DST: u32 = 0x20;
        const COLOUR: u32 = 0x30;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32, u32, u32) =
            core::mem::transmute(target as usize);
        let tex = ((this + TEX) as *const u32).read_unaligned();
        f(tex, this + SRC, this + DST, this + COLOUR);
    }
    0
});
