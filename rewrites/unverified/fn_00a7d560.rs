// original: 0x00A7D560 CComplexOnFireInfo::CComplexOnFireInfo
/// Default-construct CComplexOnFireInfo by calling the shared base constructor, installing the relocated class vtable, and returning the same object.
lf_checker_rt::export!(thiscall, rw_00a7d560(this: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA1A6C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE)); }
    this
});
