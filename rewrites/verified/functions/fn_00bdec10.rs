// original: 0x00BDEC10 TaskAttachToPedIntelligence

/// Initialize task attachment state from four stack words, preserving the upper task flags and clearing their low five bits.
/// The final stack word is copied as raw floating-point bits; the object pointer is returned.
lf_checker_rt::export!(thiscall, rw_00bdec10(this: u32, task_arg0: u32, task_arg1: u32, task_arg2: u32, weight_bits: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    let flags = task.add(0x28).cast::<u32>();
    flags.write(flags.read() & !0x1f);
    task.add(0x14).cast::<u32>().write(task_arg0);
    task.add(0x18).cast::<u32>().write(task_arg1);
    task.add(0x1c).cast::<u32>().write(task_arg2);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb8b2c));
    task.add(0x20).cast::<u32>().write(weight_bits);
    task.add(0x24).cast::<u32>().write(0);
    this
}
});
