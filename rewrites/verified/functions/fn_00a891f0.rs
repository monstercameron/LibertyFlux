// original: 0x00a891f0 vtable_float_field_load
/// Load the float 8 bytes past a vtable call's answer.
///
/// Calls vtable slot 0x18 of the argument object (intercepted,
/// thiscall/0) and returns the single-precision value 8 bytes past the
/// returned pointer, bit-exact, through the x87 return channel.
export!(stdcall, rw_00a891f0(obj: u32) -> f32 {
    unsafe {
        let vtable = *(obj as *const u32);
        let target = *((vtable.wrapping_add(0x60)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        let answer = f(obj);
        *((answer.wrapping_add(8)) as *const f32)
    }
});
