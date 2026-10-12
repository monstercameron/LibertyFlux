// original: 0x00A7D490 CComplexNewExitVehicleTaskInfo::CComplexNewExitVehicleTaskInfo_2
/// Default-construct CComplexNewExitVehicleTaskInfo through its extended base constructor. It clears bit 0 of the existing word at +0x30, clears the word at +0x38 and dword at +0x34, installs the relocated vtable, and returns `this`.
lf_checker_rt::export!(thiscall, rw_00a7d490(this: u32) -> u32 {
    const BASE_CTOR: u32 = 2;
    const VTABLE: u32 = 0x00EA15FC;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        let flags = (this.wrapping_add(0x30) as *const u32).read_unaligned();
        (this.wrapping_add(0x30) as *mut u32).write_unaligned(flags & !1);
        (this.wrapping_add(0x38) as *mut u16).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(0);
    }
    this
});
