// original: 0x00BE33D0 CTaskComplexSitIdle::CTaskComplexSitIdle_2

/// Return zero when the delegated selector reports completion or task
/// allocation fails. Otherwise allocate through the task factory. When both
/// stored state bytes are nonzero, run the base initializer and install the
/// simple idle task's relocated vtable and zeroed fields. If either byte is
/// zero, forward the six original creation words, including the -1.0 float
/// bit pattern, and return the scripted creation result.
///
/// `this` holds the delegated vtable at offset 0, state bytes at `0x3c` and
/// `0x3d`; the task factory pointer is read from a relocated global. The
/// method uses thiscall with one stack selector and returns a task pointer.
lf_checker_rt::export!(thiscall, rw_00be33d0(this: u32, selector: u32) -> u32 {
    unsafe {
        const TASK_STATE_A: u32 = 0x3c;
        const TASK_STATE_B: u32 = 0x3d;
        const SIT_IDLE_VTABLE: u32 = 0x00eb391c;
        let read_u32 = |address: u32| (address as *const u32).read_unaligned();
        let read_u8 = |address: u32| (address as *const u8).read();
        let write_u32 = |address: u32, value: u32| {
            (address as *mut u32).write_unaligned(value);
        };
        let write_u8 = |address: u32, value: u8| (address as *mut u8).write(value);
        let vtable = read_u32(this);
        let decision_slot = read_u32(vtable + 0x54);
        let decision: extern "thiscall" fn(u32, u32) -> u8 =
            core::mem::transmute(decision_slot as usize);
        if decision(this, selector) != 0 {
            return 0;
        }
        let has_idle_state = read_u8(this + TASK_STATE_A) != 0
            && read_u8(this + TASK_STATE_B) != 0;
        let factory_slot = lf_checker_rt::global::<u32>(0x0167e2a0);
        let factory = read_u32(factory_slot as u32);
        let task = lf_checker_rt::callee_thiscall!(2, u32, factory);
        if task == 0 {
            return 0;
        }
        if !has_idle_state {
            return lf_checker_rt::callee_thiscall!(
                4, u32, task, 0, 0xbf800000, 0, 0, 0, 12
            );
        }
        let _base_init = lf_checker_rt::callee_thiscall!(3, u32, task);
        write_u32(task, lf_checker_rt::relocated(SIT_IDLE_VTABLE));
        write_u32(task + 0x14, 0);
        write_u8(task + 0x18, 0);
        write_u32(task + 0x1c, 0);
        task
    }
});
