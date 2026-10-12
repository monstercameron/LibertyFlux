// original: 0x00A7D5B0 CComplexPickUpAndCarryObjectTaskInfo::CComplexPickUpAndCarryObjectTaskInfo_2
/// Default-construct CComplexPickUpAndCarryObjectTaskInfo through its linked default constructor, install the relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d5b0(this: u32) -> u32 {
    const BASE_CTOR: u32 = 5;
    const VTABLE: u32 = 0x00EA169C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE)); }
    this
});
