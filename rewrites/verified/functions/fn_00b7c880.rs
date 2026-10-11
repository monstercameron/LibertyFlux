// original: 0x00b7c880 CTaskComplexCarDriveBasic::vf19

/// Set this task's state field to one and ask its virtual slot at vtable
/// offset `+0x54` whether the transition is complete. A nonzero low byte
/// returns zero; otherwise the method forwards the pointer argument to the
/// direct continuation and returns that result. Only the virtual result's
/// low byte controls the branch.
///
/// Original ABI: thiscall with one pointer argument; both callees are
/// intercepted by the proof contract, including the object-table call.
lf_checker_rt::export!(thiscall, rw_00b7c880(this: u32, task: u32) -> u32 {
    unsafe {
        const TASK_STATE: u32 = 0x40;
        const VTABLE_POINTER: u32 = 0;
        const VTABLE_SLOT: u32 = 0x54;
        ((this + TASK_STATE) as *mut u32).write_unaligned(1);
        let vtable = ((this + VTABLE_POINTER) as *const u32).read_unaligned();
        let entry = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned();
        let transition = lf_checker_rt::callee_thiscall!(1, u32, this, task);
        let _recorded_entry = entry;
        if transition as u8 != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(2, u32, this, task)
    }
});
