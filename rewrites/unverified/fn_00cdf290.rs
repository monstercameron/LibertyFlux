// original: 0x00CDF290 CTaskComplexNM::vf19

/// Update the complex NaturalMotion task for its ped. If the ped's current
/// behaviour is not already the requested behaviour, the method asks the
/// task helper to start it and notifies the ped when that helper succeeds.
/// A pending result in this task's +0x28 slot is returned and cleared.
/// Otherwise the method looks up the shared pool through the manager global
/// and creates the next task with two duration words and two -1.0 float
/// arguments. The two behaviour pointer comparisons are equality checks.
lf_checker_rt::export!(thiscall, rw_00cdf290(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_CURRENT_BEHAVIOUR: u32 = 0x38;
        const PED_REQUESTED_BEHAVIOUR: u32 = 0x7b4;
        const TASK_PENDING_RESULT: u32 = 0x28;
        const MANAGER_FILE_VA: u32 = 0x167e_2a0;
        const FIRST_DURATION: u32 = 0x7d0;
        const SECOND_DURATION: u32 = 0x1f40;
        const NEGATIVE_ONE_BITS: u32 = 0xbf80_0000;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let current_behaviour = unsafe { read_u32(ped.wrapping_add(PED_CURRENT_BEHAVIOUR)) };
        let requested_behaviour =
            unsafe { read_u32(ped.wrapping_add(PED_REQUESTED_BEHAVIOUR)) };
        if requested_behaviour != 0
            && !(current_behaviour != 0 && current_behaviour == requested_behaviour)
        {
            let started = lf_checker_rt::callee_thiscall!(1, u32, this, ped, 0) as u8;
            if started != 0 {
                let _notify_ped =
                    lf_checker_rt::callee_thiscall!(2, u32, ped, 1, 1);
            }
        }

        let pending_result = unsafe { read_u32(this.wrapping_add(TASK_PENDING_RESULT)) };
        if pending_result != 0 {
            unsafe { write_u32(this.wrapping_add(TASK_PENDING_RESULT), 0) };
            return pending_result;
        }

        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_FILE_VA).read_unaligned() };
        let pool = lf_checker_rt::callee_thiscall!(3, u32, manager);
        if pool == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            4,
            u32,
            pool,
            FIRST_DURATION,
            SECOND_DURATION,
            NEGATIVE_ONE_BITS,
            NEGATIVE_ONE_BITS,
        )
    }
});
