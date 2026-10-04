// original: 0x00a7c5b0 bu_task_release_field_128
/// When the owned field exists and reports state 5, resets it through the
/// shared helper, clears the in-use flag bit and releases the field.
export!(thiscall, rw_00a7c5b0(obj: *mut u8) -> u32 {
    unsafe {
        let inner = *((obj as *const u8).add(0x128) as *const u32);
        if inner == 0 {
            return 0;
        }
        let vt = *(inner as *const u32);
        let state: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 0x28) as *const u32) as usize);
        let k = state(inner);
        if k != 5 {
            return k;
        }
        let q = callee_thiscall!(2, u32, inner, 0);
        *((obj as *mut u8).add(0x13c) as *mut u8) &= !0x20;
        *((obj as *mut u8).add(0x128) as *mut u32) = 0;
        q
    }
});
