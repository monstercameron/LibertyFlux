// original: 0x00ca4ef0 propose: release_member_at_8
/// Releases the reference-counted member at +8 (virtual release with count
/// 1 through the member's vtable slot 0) and nulls the slot. Null-slot
/// return is entry garbage in the original; handled like rw_00ca4ea0.
lf_rs75_rt::export!(thiscall, rw_00ca4ef0(this: u32) -> u32 {
    unsafe {
        let obj = *((this + 8) as *const u32);
        if obj != 0 {
            let vtbl = *(obj as *const u32);
            let target = *(vtbl as *const u32);
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            let ans = release(obj, 1);
            *((this + 8) as *mut u32) = 0;
            ans
        } else {
            *((this + 8) as *mut u32) = 0;
            0
        }
    }
});
