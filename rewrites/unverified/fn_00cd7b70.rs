// original: 0x00CD7B70 CTaskComplexSeekEntityAnyMeans<CEntitySeekPosCalculatorXYOffset>::vf20

/// Construct the nested seek task when the object has no active handle. For
/// an active handle, update its timer state and, once the signed sum of the
/// stored time and duration is no greater than the current signed timer,
/// resolve a new three-float vector and copy it to the associated entity.
/// The timer addition wraps as a 32-bit word, and the routine returns either
/// the new task pointer or the stored entity pointer.
///
/// The object stores its handle at `0x14`, entity pointer at `0x08`, timer
/// words at `0x20` and `0x24`, period at `0x18`, and state bytes at `0x28`
/// and `0x29`. The pending byte refreshes the stored time and clears itself.
/// The vector helper writes three raw float words through a temporary stack
/// pointer before those words are copied to entity offsets `0x18` through
/// `0x20`.
///
/// Calling convention: thiscall with one 32-bit stack argument. The timer
/// comparison is signed; the helper's three output words are copied bit for
/// bit without floating-point arithmetic.
lf_checker_rt::export!(thiscall, rw_00cd7b70(this: u32, request: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const CURRENT_TIME: u32 = 0x0117_35B4;
    const CHILD_TASK_VTABLE: u32 = 0x00E8_9044;
    const ACTIVE_HANDLE: u32 = 0x14;
    const CHILD_TASK_STATE: u32 = 0x1A;
    const ENTITY_POINTER: u32 = 0x08;
    const PERIOD: u32 = 0x18;
    const STORED_TIME: u32 = 0x20;
    const DURATION: u32 = 0x24;
    const UPDATE_STATE: u32 = 0x28;
    const REFRESH_PENDING: u32 = 0x29;
    const FIND_PED: u32 = 1;
    const BASE_CONSTRUCTOR: u32 = 2;
    const RESOLVE_VECTOR: u32 = 3;

    unsafe {
        let active_handle = ((this + ACTIVE_HANDLE) as *const u32).read_unaligned();
        if active_handle == 0 {
            let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
            let task = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
            if task == 0 {
                0
            } else {
                let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, task);
                (task as *mut u32).write_unaligned(lf_checker_rt::relocated(CHILD_TASK_VTABLE));
                ((task + ACTIVE_HANDLE) as *mut u32).write_unaligned(0);
                ((task + PERIOD) as *mut u16).write_unaligned(1);
                ((task + CHILD_TASK_STATE) as *mut u8).write(1);
                task
            }
        } else {
            let state = ((this + UPDATE_STATE) as *const u8).read();
            if state != 0 {
                let refresh = ((this + REFRESH_PENDING) as *const u8).read();
                if refresh != 0 {
                    let now = lf_checker_rt::global::<u32>(CURRENT_TIME).read();
                    ((this + STORED_TIME) as *mut u32).write_unaligned(now);
                    ((this + REFRESH_PENDING) as *mut u8).write(0);
                }

                let duration = ((this + DURATION) as *const u32).read_unaligned();
                let stored_time = ((this + STORED_TIME) as *const u32).read_unaligned();
                let now = lf_checker_rt::global::<u32>(CURRENT_TIME).read();
                let deadline = duration.wrapping_add(stored_time);
                if (deadline as i32) <= (now as i32) {
                    let period = ((this + PERIOD) as *const u32).read_unaligned();
                    ((this + DURATION) as *mut u32).write_unaligned(period);
                    ((this + STORED_TIME) as *mut u32).write_unaligned(now);
                    ((this + UPDATE_STATE) as *mut u8).write(1);

                    let entity = ((this + ENTITY_POINTER) as *const u32).read_unaligned();
                    let mut vector_words = [0u32; 3];
                    let _ = lf_checker_rt::callee_thiscall!(
                        RESOLVE_VECTOR,
                        u32,
                        this,
                        request,
                        vector_words.as_mut_ptr() as usize as u32
                    );
                    ((entity + 0x18) as *mut u32).write_unaligned(vector_words[0]);
                    ((entity + 0x1C) as *mut u32).write_unaligned(vector_words[1]);
                    ((entity + 0x20) as *mut u32).write_unaligned(vector_words[2]);
                }
            }
            ((this + ENTITY_POINTER) as *const u32).read_unaligned()
        }
    }
});
