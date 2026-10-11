// original: 0x00DCAB40 CTaskComplexCarReactToVehicleCollision::vf18

/// Starts the fallback collision-response task when the current subtask is
/// not the target drive-out task.
///
/// `this` is the collision task in ECX and `context` is its one stack
/// argument. The current subtask's virtual type is queried first. Type
/// `0x76c` is returned unchanged; every other type constructs the fallback
/// response using kind `0x516` and the supplied context.
lf_checker_rt::export!(thiscall, rw_00dcab40(this: u32, context: u32) -> u32 {
    unsafe {
        const CURRENT_SUBTASK: u32 = 0x08;
        const EXPECTED_TYPE: u32 = 0x76c;
        const TASK_KIND: u32 = 0x516;
        const GET_SUBTASK_TYPE: u32 = 1;
        const CREATE_FALLBACK: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let subtask = read_u32(this.wrapping_add(CURRENT_SUBTASK));
        let subtask_type = lf_checker_rt::callee_thiscall!(GET_SUBTASK_TYPE, u32, subtask);
        if subtask_type == EXPECTED_TYPE {
            return subtask_type;
        }

        lf_checker_rt::callee_thiscall!(CREATE_FALLBACK, u32, this, TASK_KIND, context)
    }
});
