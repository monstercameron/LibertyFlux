// original: 0x005C0A80 CViewportPrimaryOrtho::CViewportPrimaryOrtho

/// Construct a primary orthographic viewport in place. The object receives
/// its primary-ortho virtual table, then its nested scene at offset 0x10 is
/// initialized with six fixed 32-bit words. The viewport's initialized byte
/// at offset 0x560 is set last. Both constructor helpers are intercepted by
/// the checker; the function returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_005c0a80(this: u32) -> u32 {
    const PRIMARY_ORTHO_VTABLE_VA: u32 = 0x00FE0A08;
    const SCENE_OFFSET: u32 = 0x10;
    const INITIALIZED_FLAG_OFFSET: u32 = 0x560;
    const BASE_VIEWPORT_CONSTRUCTOR: u32 = 1;
    const SCENE_CONSTRUCTOR: u32 = 2;

    unsafe {
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_VIEWPORT_CONSTRUCTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_ORTHO_VTABLE_VA));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            SCENE_CONSTRUCTOR,
            u32,
            this.wrapping_add(SCENE_OFFSET),
            0,
            0x3F800000,
            0x3F800000,
            0,
            0,
            0x3F800000
        );
        (this.wrapping_add(INITIALIZED_FLAG_OFFSET) as *mut u8).write_unaligned(1);
    }

    this
});
