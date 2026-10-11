// original: 0x00BDEC50 CTaskComplexInWater::CTaskComplexInWater

/// Initialize the in-water task's configuration: copy two words and the low byte of a third argument, clear its state fields, set the water threshold to -10.0, and set the mode word to 3. The object pointer is returned.
lf_checker_rt::export!(thiscall, rw_00bdec50(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.add(0x0).cast::<u32>().write(lf_checker_rt::relocated(0x00eb92ec));
    task.add(0x20).cast::<u32>().write(0);
    task.add(0x24).cast::<u32>().write(0);
    task.add(0x28).cast::<u32>().write(0);
    task.add(0x30).cast::<u32>().write(0);
    task.add(0x34).cast::<u32>().write(0);
    task.add(0x38).cast::<u32>().write(0);
    task.add(0x40).cast::<u32>().write(0);
    task.add(0x44).cast::<u32>().write(arg0);
    task.add(0x48).cast::<u32>().write(arg1);
    task.add(0x74).write(arg2 as u8);
    task.add(0x54).cast::<u32>().write(0xc1200000);
    task.add(0x58).cast::<u32>().write(0);
    task.add(0x5c).cast::<u32>().write(0);
    task.add(0x60).cast::<u32>().write(0);
    task.add(0x64).cast::<u32>().write(0);
    task.add(0x6c).cast::<u32>().write(0);
    task.add(0x68).cast::<u16>().write(0);
    task.add(0x70).cast::<u32>().write(3);
    this
}
});
