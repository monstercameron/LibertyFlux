// original: 0x00A32610 CEntity::vf18

/// Forward one entity update to its optional range-remap component and then
/// to virtual slot 19. The entity is the 32-bit `this` pointer; its optional
/// component pointer is at byte offset `0x34`, and its vtable pointer is at
/// offset `0`. Both callees use thiscall with one callee-cleaned word. The
/// first result is ignored and the virtual call's EAX is returned.
lf_checker_rt::export!(thiscall, rw_00a32610(this: u32, argument: u32) -> u32 {
    const RANGE_REMAP: u32 = 0x34;
    const VTABLE: u32 = 0x00;
    const UPDATE_DRAWABLE_SLOT: u32 = 0x4c;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let range_remapper = read_u32(this.wrapping_add(RANGE_REMAP));
        if range_remapper != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, range_remapper, argument);
        }

        let vtable = read_u32(this.wrapping_add(VTABLE));
        let update_drawable = read_u32(vtable.wrapping_add(UPDATE_DRAWABLE_SLOT));
        let update: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(update_drawable as usize);
        update(this, argument)
    }
});
