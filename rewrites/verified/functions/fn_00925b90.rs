// original: 0x00925B90 T_SetGlobalVar_1Arg<rage::Vector4>::vf1
/// Apply a vector shader variable: forward id, data pointer and kind tags.
///
/// Virtual slot 1: calls the shared four-argument applier with the word at
/// +8, a pointer to the inline vector at +0x10, and the constant tags 1, 5.
export!(thiscall, rw_00925B90(obj: u32) -> u32 {
    unsafe {
        let o = obj as *const u32;
        callee_cdecl!(1, u32, *o.add(2), obj.wrapping_add(0x10), 1, 5);
        0
    }
});
