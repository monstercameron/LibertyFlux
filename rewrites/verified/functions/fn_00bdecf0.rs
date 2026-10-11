// original: 0x00BDECF0 CTaskComplexMoveSequence::CTaskComplexMoveSequence

/// Initialize a move-sequence task, including its scalar base state and nested vector subobject. The base initializer receives the exact 1.0f bits; task-owned counters and flags are cleared, and the object pointer is returned.
lf_checker_rt::export!(thiscall, rw_00bdecf0(this: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this, 0x3f800000);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb9154));
    task.add(0x14).cast::<u32>().write(lf_checker_rt::relocated(0x00eb91ac));
    let _nested_result = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x20));
    for offset in [0x64usize, 0x68, 0x6c, 0x70] {
        task.add(offset).cast::<u32>().write(0);
    }
    task.add(0x74).write(0);
    this
}
});
