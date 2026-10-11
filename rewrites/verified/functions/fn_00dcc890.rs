// original: 0x00DCC890 CTaskComplexMove_StepAwayFromCollisionObjects::vf20
/// Advances a step-away task when it is active, refreshes its start time
/// when requested, and checks the signed sum of start time and duration
/// against the signed current time. If due and the child is not already
/// flagged, the method calls slot five in the child's function table with
/// the original argument, one, and zero; a nonzero low byte sets child flag 2.
lf_checker_rt::export!(thiscall, rw_00dcc890(task: u32, action: u32) -> u32 {
    const CHILD_OFFSET: usize = 8;
    const DEADLINE_OFFSET: usize = 0x30;
    const DURATION_OFFSET: usize = 0x34;
    const ACTIVE_OFFSET: usize = 0x38;
    const REFRESH_OFFSET: usize = 0x39;
    const CHILD_FLAGS_OFFSET: usize = 0x0c;
    const CALLBACK_SLOT_OFFSET: usize = 0x14;
    const CURRENT_TIME_VA: u32 = 0x011735b4;

    // SAFETY: the checker contract provides a readable task object.
    let bytes = task as *mut u8;
    let active = unsafe { bytes.add(ACTIVE_OFFSET).read() != 0 };
    let child = unsafe { (bytes.add(CHILD_OFFSET) as *mut u32).read() };
    if !active {
        return child;
    }
    let refresh = unsafe { bytes.add(REFRESH_OFFSET).read() != 0 };
    // SAFETY: this is the original's relocated current-time global.
    let now = unsafe { lf_checker_rt::global::<u32>(CURRENT_TIME_VA).read() };
    if refresh {
        // SAFETY: the checker contract supplies a writable task object.
        unsafe {
            (bytes.add(DEADLINE_OFFSET) as *mut u32).write(now);
            bytes.add(REFRESH_OFFSET).write(0);
        }
    }
    let start = unsafe { (bytes.add(DEADLINE_OFFSET) as *const u32).read() };
    let duration = unsafe { (bytes.add(DURATION_OFFSET) as *const u32).read() };
    let deadline = start.wrapping_add(duration);
    if (deadline as i32) > (now as i32) {
        return child;
    }
    let flags = unsafe { (child.wrapping_add(CHILD_FLAGS_OFFSET as u32) as *const u32).read() };
    if flags & 1 != 0 {
        return 0;
    }
    // SAFETY: the contract plants recorder stub 1 into this fabricated table.
    let table = unsafe { (child as *const u32).read() };
    let callback_address = unsafe { (table.wrapping_add(CALLBACK_SLOT_OFFSET as u32) as *const u32).read() };
    let callback: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(callback_address as usize) };
    let answer = callback(child, action, 1, 0);
    if answer as u8 != 0 {
        // SAFETY: the child object is writable in the contract.
        unsafe { (child.wrapping_add(CHILD_FLAGS_OFFSET as u32) as *mut u32).write(flags | 2); }
        0
    } else {
        child
    }
});
