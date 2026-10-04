// original: 0x00925B60 T_SetGlobalVar_1Arg<rage::grcRenderTarget*>::vf1
/// Apply a render-target shader variable: forward two object words.
///
/// Virtual slot 1 of the setter object: calls the shared two-argument
/// applier with the words at +8 and +12. Returns nothing.
export!(thiscall, rw_00925B60(obj: u32) -> u32 {
    unsafe {
        let o = obj as *const u32;
        callee_cdecl!(1, u32, *o.add(2), *o.add(3));
        0
    }
});
