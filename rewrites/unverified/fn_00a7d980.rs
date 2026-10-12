// original: 0x00A7D980 CScenarioTaskInfo::CScenarioTaskInfo_2
/// Default-construct CScenarioTaskInfo through its base constructor. Initialize its state at +0x18 to all ones, clear the documented fields at +0x1c, +0x20, +0x24, +0x30, +0x34 and +0x38, clear the word at +0x40, set +0x44 to all ones, install the relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d980(this: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA1874;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u8).write(0);
        (this.wrapping_add(0x30) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x38) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x40) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x44) as *mut u32).write_unaligned(u32::MAX);
    }
    this
});
