// original: 0x00CF4BB0 CTaskSimpleClimb::vf5

/// Selects whether the climb task can abort, based on a mode, a ped status
/// byte, and virtual predicates on the supplied task object. The slot at
/// `+0x08` is compared as a signed integer against 101; slots at `+0x04` are
/// compared as unsigned bit patterns against 0x3a, 9, and 0x7c. A successful
/// predicate runs three helpers and returns true in AL. Mode 2 takes the
/// common path directly and supplies height -1000.0; other successful paths
/// supply -8.0. The final helper receives a three-word local vector whose
/// words are 0, 0, and the bit pattern for -0.05. The method is thiscall with
/// three stack arguments and returns a byte-sized boolean.
lf_checker_rt::export!(thiscall, rw_00cf4bb0(this: u32, ped: u32, mode: u32, task: u32) -> u32 {
    unsafe {
        const PED_FLAGS: u32 = 0x212;
        const TASK_SUBOBJECT: u32 = 0x10;
        const READY_SLOT: u32 = 0x18;
        const TASK_QUERY_SLOT: u32 = 0x08;
        const TASK_KIND_SLOT: u32 = 0x04;
        const SUBOBJECT_QUERY_SLOT: u32 = 0x0c;
        const MODE_TWO: u32 = 2;
        const MINIMUM_TASK_SCORE: i32 = 0x65;
        const TASK_STATE_IDLE: u32 = 0x3a;
        const TASK_STATE_BUSY: u32 = 9;
        const TASK_STATE_SUPPORTED: u32 = 0x7c;
        const PED_STATE_SUPPORTED: u32 = 0x454;
        const HEIGHT_MODE_TWO: u32 = 0xc47a_0000;
        const HEIGHT_DEFAULT: u32 = 0xc100_0000;
        const CALL_FINISH: u32 = 1;
        const CALL_SET_HEIGHT: u32 = 2;
        const CALL_APPLY_VECTOR: u32 = 3;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn call_virtual(object: u32, slot: u32) -> u32 {
            let table = unsafe { read_word(object) };
            let target = unsafe { read_word(table + slot) };
            let call: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            call(object)
        }

        let mut accepted = mode == MODE_TWO;
        if !accepted && unsafe { read_byte(ped + PED_FLAGS) } != 0 {
            accepted = true;
        }

        if !accepted && task != 0 {
            accepted = (unsafe { call_virtual(task, READY_SLOT) } as u8) != 0;
            if !accepted {
                accepted = (unsafe { call_virtual(task, TASK_QUERY_SLOT) } as i32)
                    >= MINIMUM_TASK_SCORE;
            }
            if !accepted {
                accepted = unsafe { call_virtual(task, TASK_KIND_SLOT) } == TASK_STATE_IDLE;
            }
            if !accepted {
                let task_state = unsafe { call_virtual(task, TASK_KIND_SLOT) };
                if task_state == TASK_STATE_BUSY {
                    let has_blocker =
                        lf_checker_rt::callee_thiscall!(4, u32, task) as u8 != 0;
                    accepted = has_blocker
                        || lf_checker_rt::callee_thiscall!(5, u32, task) as u8 != 0;
                } else if unsafe { call_virtual(task, TASK_KIND_SLOT) } == TASK_STATE_SUPPORTED {
                    let subobject = unsafe { read_word(task + TASK_SUBOBJECT) };
                    if subobject != 0 {
                        accepted = unsafe {
                            call_virtual(subobject, SUBOBJECT_QUERY_SLOT)
                        } == PED_STATE_SUPPORTED;
                    }
                }
            }
        }

        if !accepted {
            return 0;
        }

        let _ = lf_checker_rt::callee_thiscall!(CALL_FINISH, u32, this, ped);
        let height = if mode == MODE_TWO { HEIGHT_MODE_TWO } else { HEIGHT_DEFAULT };
        let _ = lf_checker_rt::callee_thiscall!(CALL_SET_HEIGHT, u32, this, height);
        let mut local_vector = [0u32, 0, 0xbd4c_cccd];
        let vector_pointer = local_vector.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(CALL_APPLY_VECTOR, u32, ped, vector_pointer);
        1
    }
});
