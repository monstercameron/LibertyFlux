// original: 0x00cfc1a0 CTaskComplexCombatAdvanceSubtask::CTaskComplexCombatAdvanceSubtask
/// Call the combat-subtask base constructor with the task type and install this
/// vtable. Clear offsets 0x40, 0x44, and 0x48, then reset holders at offsets 0x4C
/// and 0x58. Clear the listed state fields from offsets 0x60 through 0x90 and
/// return this object.
lf_checker_rt::export!(thiscall, rw_00cfc1a0(this: *mut u8, task_type: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EDFDB4;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32, task_type);
        *(this as *mut u32) = lf_checker_rt::relocated(TASK_VTABLE);
        *this.add(0x40) = 0;
        *(this.add(0x44) as *mut u32) = 0;
        *(this.add(0x48) as *mut u32) = 0;
        lf_checker_rt::callee_thiscall!(2, u32, this.add(0x4C) as u32);
        lf_checker_rt::callee_thiscall!(3, u32, this.add(0x58) as u32);
        for offset in [0x60usize,0x70,0x74,0x78,0x80,0x84,0x88,0x90] {
            *(this.add(offset) as *mut u32) = 0;
        }
        this as u32
    }
});
