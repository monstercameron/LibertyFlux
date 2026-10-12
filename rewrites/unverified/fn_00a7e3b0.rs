// original: 0x00A7E3B0 CStatusAndTargetTaskInfo::CStatusAndTargetTaskInfo_3
/// Destroy the CStatusAndTargetTaskInfo link. When its stored target at +0x18 is non-null, call the detach helper with the stored pointer and the address of that field, then clear the field. Install the base task-info vtable and tail-call the base destructor, forwarding its return value.
lf_checker_rt::export!(thiscall, rw_00a7e3b0(this: u32) -> u32 {
    const DETACH_REFERENCE: u32 = 10;
    const BASE_DESTRUCTOR: u32 = 14;
    const STATUS_VTABLE: u32 = 0x00EA1284;
    const TASK_INFO_VTABLE: u32 = 0x00EA105C;
    let target = unsafe { (this.wrapping_add(0x18) as *const u32).read_unaligned() };
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(STATUS_VTABLE)); }
    if target != 0 {
        let _ = lf_checker_rt::callee_thiscall!(DETACH_REFERENCE, u32, target, this.wrapping_add(0x18));
        unsafe { (this.wrapping_add(0x18) as *mut u32).write_unaligned(0); }
    }
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_INFO_VTABLE)); }
    lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
});
