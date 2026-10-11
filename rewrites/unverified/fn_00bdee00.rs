// original: 0x00BDEE00 CTaskComplexSequence::CTaskComplexSequence

/// Initialize the sequence task and its nested sequence state, then clear its four counters and three one-byte flags. The task object is returned.
lf_checker_rt::export!(thiscall, rw_00bdee00(this: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    let _nested_result = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x14));
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb90fc));
    task.add(0x58).cast::<u32>().write(0);
    task.add(0x5c).cast::<u32>().write(0);
    task.add(0x60).cast::<u32>().write(0);
    task.add(0x64).cast::<u32>().write(0);
    task.add(0x68).write(0);
    task.add(0x6c).cast::<u32>().write(0);
    task.add(0x70).cast::<u32>().write(0);
    task.add(0x74).write(0);
    this
}
});
