// original: 0x00DCA6F0 CTaskComplexCarReactToVehicleCollision::vf20

/// Controls a collision task's current subtask.
///
/// `this` is the task in ECX and `context` is its one stack argument. The
/// current subtask's virtual type is queried first. Only type `0x76c` asks the
/// helper whether the task may continue; a zero low byte forwards the call to
/// the task's virtual control slot. Each path returns the EAX value left by
/// its last call, or the subtask type when the selected type does not match.
lf_checker_rt::export!(thiscall, rw_00dca6f0(this: u32, context: u32) -> u32 {
    unsafe {
        const TASK_TYPE: u32 = 0x76c;
        const CURRENT_SUBTASK: u32 = 0x08;
        const HELPER_ARGUMENT: u32 = 0x20;
        const GET_SUBTASK_TYPE: u32 = 1;
        const CAN_CONTINUE: u32 = 2;
        const CONTROL_SUBTASK: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let subtask = read_u32(this.wrapping_add(CURRENT_SUBTASK));
        let subtask_type = lf_checker_rt::callee_thiscall!(GET_SUBTASK_TYPE, u32, subtask);
        if subtask_type != TASK_TYPE {
            return subtask;
        }

        let helper_flag = read_u32(this.wrapping_add(HELPER_ARGUMENT));
        let helper_result = lf_checker_rt::callee_cdecl!(CAN_CONTINUE, u32, context, helper_flag);
        if helper_result as u8 != 0 {
            return subtask;
        }

        lf_checker_rt::callee_thiscall!(CONTROL_SUBTASK, u32, this, context)
    }
});
