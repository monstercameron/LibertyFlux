// original: 0x00a87a20 current_object_guarded_call
/// Call vtable slot 3 while registered as the current object.
///
/// Stores `this` into the current-object global, calls slot 3 of the
/// object's vtable (intercepted, thiscall/0) with `this`, clears the
/// global, and returns the callee's answer.
export!(thiscall, rw_00a87a20(this_obj: u32) -> u32 {
    unsafe {
        *global::<u32>(0x12fb1b8) = this_obj;
        let vtable = *(this_obj as *const u32);
        let target = *((vtable.wrapping_add(0xc)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        let answer = f(this_obj);
        *global::<u32>(0x12fb1b8) = 0;
        answer
    }
});
