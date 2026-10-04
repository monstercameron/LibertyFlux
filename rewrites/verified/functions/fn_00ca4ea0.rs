// original: 0x00ca4ea0 propose: release_member_at_4
/// Releases the reference-counted member at +4 (virtual release with count
/// 1 through the member's vtable slot 0) and nulls the slot. When the slot
/// is already null the original returns whatever happened to be in EAX at
/// entry; the rewrite returns 0 there, which the primary contract does not
/// check (`ret: none`); a second contract with a non-null slot checks EAX.
lf_rs75_rt::export!(thiscall, rw_00ca4ea0(this: u32) -> u32 {
    unsafe {
        let obj = *((this + 4) as *const u32);
        if obj != 0 {
            let vtbl = *(obj as *const u32);
            let target = *(vtbl as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let ans = release(obj, 1);
            *((this + 4) as *mut u32) = 0;
            ans
        } else {
            *((this + 4) as *mut u32) = 0;
            0
        }
    }
});
