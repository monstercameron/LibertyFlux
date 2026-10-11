// original: 0x00BDEFC0 CTaskComplexSitIdle::CTaskComplexSitIdle

/// Initialize the sit-idle task by copying six constructor words into its task slots and clearing the remaining counters and small flags. The sixth input contributes only its low byte. The object pointer is returned.
lf_checker_rt::export!(thiscall, rw_00bdefc0(this: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32, arg5: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb8dd4));
    task.add(0x14).cast::<u32>().write(arg1);
    task.add(0x18).cast::<u32>().write(arg2);
    task.add(0x1c).cast::<u32>().write(0);
    task.add(0x20).cast::<u32>().write(0);
    task.add(0x24).cast::<u32>().write(0);
    task.add(0x28).cast::<u16>().write(0);
    task.add(0x2c).cast::<u32>().write(arg0);
    task.add(0x30).cast::<u32>().write(arg4);
    task.add(0x34).cast::<u32>().write(arg3);
    task.add(0x38).write(arg5 as u8);
    task.add(0x39).cast::<u16>().write_unaligned(0);
    task.add(0x3c).cast::<u16>().write_unaligned(0);
    task.add(0x3e).write(0);
    this
}
});
