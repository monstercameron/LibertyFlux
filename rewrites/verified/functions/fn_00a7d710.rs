// original: 0x00A7D710 CMeleeTaskInfo::CMeleeTaskInfo_2
/// Default-construct CMeleeTaskInfo through its linked default constructor, install the relocated vtable, clear the word at +0x1c, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d710(this: u32) -> u32 {
    const BASE_CTOR: u32 = 8;
    const VTABLE: u32 = 0x00EA123C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
    }
    this
});
