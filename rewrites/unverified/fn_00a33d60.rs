// original: 0x00A33D60 CEntity::vf44

/// Unregister the entity's physics row unless its physics pointer at `+0x38`
/// is null or the row's 16-bit handle at `+8` is `0xffff`. The handle is
/// zero-extended, then the manager pointer loaded from the global slot is
/// passed in ECX with the handle and a zero flag as stack arguments. The
/// manager helper's result is ignored. Thiscall receives the entity in ECX.
lf_checker_rt::export!(thiscall, rw_00a33d60(this: u32) -> () {
    const PHYSICS_OBJECT: u32 = 0x38;
    const PHYSICS_HANDLE: u32 = 0x08;
    const INVALID_HANDLE: u16 = u16::MAX;
    const PHYSICS_MANAGER_VA: u32 = 0x012b_9c7c;
    const UNREGISTER: u32 = 1;
    const CLEAR_FLAG: u32 = 0;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let physics = read_u32(this.wrapping_add(PHYSICS_OBJECT));
        if physics == 0 {
            return;
        }

        let handle = read_u16(physics.wrapping_add(PHYSICS_HANDLE));
        if handle == INVALID_HANDLE {
            return;
        }

        let manager_slot = lf_checker_rt::relocated(PHYSICS_MANAGER_VA);
        let manager = read_u32(manager_slot);
        let _ = lf_checker_rt::callee_thiscall!(
            UNREGISTER,
            u32,
            manager,
            u32::from(handle),
            CLEAR_FLAG,
        );
    }
});
