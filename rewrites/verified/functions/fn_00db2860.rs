// original: 0x00DB2860 UITexture::release_texture_resource

/// Release the receiver's cached texture resource only when its readiness
/// query is true. If the cached resource pointer at +0x1D4 is nonnull, call
/// the two release helpers in order, then clear the pointer field. The method
/// returns no value and preserves the receiver's stack cleanup.
lf_checker_rt::export!(thiscall, rw_00db2860(this: u32) -> u32 {
    const VTABLE_READY_SLOT: u32 = 0x140;
    const RESOURCE_SLOT: u32 = 0x1D4;

    unsafe {
        let vtable = (this as *const u32).read_unaligned();
        let ready_address = (vtable.wrapping_add(VTABLE_READY_SLOT) as *const u32).read_unaligned();
        let is_ready: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(ready_address as usize);
        if is_ready(this) == 0 { return 0; }

        let resource = (this.wrapping_add(RESOURCE_SLOT) as *const u32).read_unaligned();
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, resource);
            let _ = lf_checker_rt::callee_cdecl!(3, u32, resource);
        }
        (this.wrapping_add(RESOURCE_SLOT) as *mut u32).write_unaligned(0);
    }
    0
});
