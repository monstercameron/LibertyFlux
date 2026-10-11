// original: 0x00BDEE00 CTaskComplexSequence::CTaskComplexSequence

/// Set the sequence task vtable, invoke its base and nested-state initializers, clear four task fields (three words, one byte, then one word), and return the task pointer.
lf_checker_rt::export!(thiscall, rw_00bdee00(this: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb90fc));
    let _nested_result = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x14));
    task.add(0x58).cast::<u32>().write(0);
    task.add(0x5c).cast::<u32>().write(0);
    task.add(0x60).cast::<u32>().write(0);
    task.add(0x64).write(0);
    task.add(0x68).cast::<u32>().write(0);
    this
}
});
