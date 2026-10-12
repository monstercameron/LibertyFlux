// original: 0x00A7D320 CComplexGangDrivebyTaskInfo::CComplexGangDrivebyTaskInfo_2
/// Default-initialize CComplexGangDrivebyTaskInfo after its base constructor. It clears the fields at +0x18, +0x1c, +0x20, +0x24 and +0x28, clears the two bytes at +0x31, installs the relocated vtable, and returns `this`.
lf_checker_rt::export!(thiscall, rw_00a7d320(this: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA13FC;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x18) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x31) as *mut u16).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
