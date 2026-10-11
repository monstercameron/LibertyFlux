// original: 0x00cac380 CTaskComplexMoveWaitForTraffic::CTaskComplexMoveWaitForTraffic

/// Constructs the wait-for-traffic task. It calls the move base constructor
/// with a zero word, installs both vtables, clears the task's small leading
/// fields, copies three words from each input record to `+0x40..+0x48` and
/// `+0x50..+0x58`, clears the byte at `+0x60`, and returns `this`.
lf_checker_rt::export!(thiscall, rw_00cac380(
    this: u32,
    approach_data: u32,
    destination_data: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_8774;
    const BASE_VTABLE_VA: u32 = 0x00ed_87cc;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, 0);
    unsafe {
        let approach_x = (approach_data as *const u32).read_unaligned();
        let approach_y = ((approach_data + 4) as *const u32).read_unaligned();
        let approach_z = ((approach_data + 8) as *const u32).read_unaligned();
        let destination_x = (destination_data as *const u32).read_unaligned();
        let destination_y = ((destination_data + 4) as *const u32).read_unaligned();
        let destination_z = ((destination_data + 8) as *const u32).read_unaligned();

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));
        ((this + 0x20) as *mut u32).write_unaligned(0);
        ((this + 0x24) as *mut u32).write_unaligned(0);
        ((this + 0x28) as *mut u16).write_unaligned(0);
        ((this + 0x2c) as *mut u32).write_unaligned(0);
        ((this + 0x30) as *mut u32).write_unaligned(0);
        ((this + 0x34) as *mut u16).write_unaligned(0);
        ((this + 0x40) as *mut u32).write_unaligned(approach_x);
        ((this + 0x44) as *mut u32).write_unaligned(approach_y);
        ((this + 0x48) as *mut u32).write_unaligned(approach_z);
        ((this + 0x50) as *mut u32).write_unaligned(destination_x);
        ((this + 0x54) as *mut u32).write_unaligned(destination_y);
        ((this + 0x58) as *mut u32).write_unaligned(destination_z);
        ((this + 0x60) as *mut u8).write_unaligned(0);
    }
    this
});
