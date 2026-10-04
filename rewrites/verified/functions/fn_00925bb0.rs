// original: 0x00925BB0 T_SetShaderGroupVar_1Arg<rage::Matrix44>::vf1
/// Apply a matrix shader-group variable through its target object.
///
/// Virtual slot 1: calls the group applier as a method on the object at +8,
/// passing the word at +12 and a pointer to the inline matrix at +0x10.
export!(thiscall, rw_00925BB0(obj: u32) -> u32 {
    unsafe {
        let o = obj as *const u32;
        callee_thiscall!(1, u32, *o.add(2), *o.add(3), obj.wrapping_add(0x10));
        0
    }
});
