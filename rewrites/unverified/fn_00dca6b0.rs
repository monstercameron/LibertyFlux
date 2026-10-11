// original: 0x00DCA6B0 CTaskComplexCarReactToVehicleCollisionGetOut::vf1

/// Creates a get-out collision follow-up when the shared vehicle manager
/// supplies a destination object.
///
/// `this` is the task in ECX. The manager query returns the constructor's
/// object pointer; when it is non-null, the task's entity slots, embedded
/// vector pointer, stored distance bits, and mode byte are forwarded in that
/// order. The constructor result is returned, or zero when the query is null.
lf_checker_rt::export!(thiscall, rw_00dca6b0(this: u32) -> u32 {
    unsafe {
        const GET_COLLISION_DESTINATION: u32 = 1;
        const CONSTRUCT_GET_OUT_TASK: u32 = 2;
        const MANAGER_GLOBAL_FILE_VA: u32 = 0x0167E2A0;
        const ENTITY_A: u32 = 0x18;
        const ENTITY_B: u32 = 0x1c;
        const EMBEDDED_VECTOR: u32 = 0x30;
        const STORED_DISTANCE: u32 = 0x40;
        const MODE: u32 = 0x44;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_FILE_VA).read_unaligned();
        let destination = lf_checker_rt::callee_thiscall!(GET_COLLISION_DESTINATION, u32, manager);
        if destination == 0 {
            return destination;
        }

        let mode = (this.wrapping_add(MODE) as *const u8).read();
        lf_checker_rt::callee_thiscall!(
            CONSTRUCT_GET_OUT_TASK,
            u32,
            destination,
            read_u32(this.wrapping_add(ENTITY_A)),
            read_u32(this.wrapping_add(ENTITY_B)),
            this.wrapping_add(EMBEDDED_VECTOR),
            read_u32(this.wrapping_add(STORED_DISTANCE)),
            mode as u32
        )
    }
});
