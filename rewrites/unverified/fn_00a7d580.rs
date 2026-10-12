// original: 0x00A7D580 CComplexPickUpAndCarryObjectTaskInfo::CComplexPickUpAndCarryObjectTaskInfo
/// Construct CComplexPickUpAndCarryObjectTaskInfo by calling its linked base constructor with a zero mode and `link_value`. Store `field_value` at +0x24, install the relocated vtable, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d580(this: u32, link_value: u32, field_value: u32) -> u32 {
    const LINKED_CTOR: u32 = 4;
    const VTABLE: u32 = 0x00EA169C;
    const FIELD_VALUE_OFFSET: u32 = 0x24;
    let _ = lf_checker_rt::callee_thiscall!(LINKED_CTOR, u32, this, 0, link_value);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(FIELD_VALUE_OFFSET) as *mut u32).write_unaligned(field_value);
    }
    this
});
