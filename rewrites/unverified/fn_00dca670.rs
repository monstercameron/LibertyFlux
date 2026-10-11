// original: 0x00DCA670 CTaskComplexCarReactToVehicleCollision::vf1

/// Creates the follow-up collision task when the shared vehicle manager is
/// available.
///
/// `this` is the base collision task in ECX. The routine asks the manager for
/// a destination object. If it is non-null, it passes the task's entity slots,
/// the stored distance as raw `f32` bits, and the embedded vector address to
/// the follow-up constructor. The constructor's EAX value is returned, or the
/// manager query's zero is returned when no destination is available.
lf_checker_rt::export!(thiscall, rw_00dca670(this: u32) -> u32 {
    unsafe {
        const GET_COLLISION_DESTINATION: u32 = 1;
        const CONSTRUCT_FOLLOW_UP: u32 = 2;
        const MANAGER_GLOBAL_FILE_VA: u32 = 0x0167E2A0;
        const ENTITY_A: u32 = 0x18;
        const ENTITY_B: u32 = 0x1c;
        const STORED_DISTANCE: u32 = 0x24;
        const EMBEDDED_VECTOR: u32 = 0x30;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_FILE_VA).read_unaligned();
        let destination = lf_checker_rt::callee_thiscall!(GET_COLLISION_DESTINATION, u32, manager);
        if destination == 0 {
            return destination;
        }

        lf_checker_rt::callee_thiscall!(
            CONSTRUCT_FOLLOW_UP,
            u32,
            manager,
            read_u32(this.wrapping_add(ENTITY_A)),
            read_u32(this.wrapping_add(ENTITY_B)),
            read_u32(this.wrapping_add(STORED_DISTANCE)),
            this.wrapping_add(EMBEDDED_VECTOR)
        )
    }
});
