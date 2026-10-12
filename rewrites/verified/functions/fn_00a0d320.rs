// original: 0x00A0D320 CEntity::get_rigid_body
/// Returns the rigid-body pointer only when the entity's 32-bit flag word
/// contains the rigid-body bit; a clear bit produces a null pointer.
lf_checker_rt::export!(thiscall, rw_00A0D320(entity: u32) -> u32 {
    unsafe {
        const ENTITY_FLAGS: u32 = 0x24;
        const RIGID_BODY: u32 = 0x38;
        const HAS_RIGID_BODY: u32 = 0x0400_0000;
        let flags = (entity.wrapping_add(ENTITY_FLAGS) as *const u32).read_unaligned();
        if flags & HAS_RIGID_BODY != 0 {
            (entity.wrapping_add(RIGID_BODY) as *const u32).read_unaligned()
        } else {
            0
        }
    }
});
