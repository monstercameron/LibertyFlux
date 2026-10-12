// original: 0x00A7D670 CMedicTaskInfo::CMedicTaskInfo
/// Construct CMedicTaskInfo through its linked base constructor. Store the two target pointers at +0x1c and +0x20, set only bit 0 of the existing word at +0x24 from the low bit of `flags`, and install the relocated vtable. For each non-null target, call the reference-registration helper with that target and the address of its stored pointer. Return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d670(this: u32, base_value: u32, first_target: u32, second_target: u32, flags: u32) -> u32 {
    const LINKED_CTOR: u32 = 7;
    const REGISTER_REFERENCE: u32 = 9;
    const VTABLE: u32 = 0x00EA11F4;
    const FIRST_TARGET_OFFSET: u32 = 0x1C;
    const SECOND_TARGET_OFFSET: u32 = 0x20;
    const FLAGS_OFFSET: u32 = 0x24;
    let _ = lf_checker_rt::callee_thiscall!(LINKED_CTOR, u32, this, base_value);
    unsafe {
        (this.wrapping_add(SECOND_TARGET_OFFSET) as *mut u32).write_unaligned(second_target);
        let old_flags = (this.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned();
        let changed_low_bit = (flags as u8 as u32 ^ old_flags) & 1;
        (this.wrapping_add(FLAGS_OFFSET) as *mut u32).write_unaligned(old_flags ^ changed_low_bit);
        (this.wrapping_add(FIRST_TARGET_OFFSET) as *mut u32).write_unaligned(first_target);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    if first_target != 0 {
        let _ = lf_checker_rt::callee_thiscall!(REGISTER_REFERENCE, u32, first_target, this.wrapping_add(FIRST_TARGET_OFFSET));
    }
    if second_target != 0 {
        let _ = lf_checker_rt::callee_thiscall!(REGISTER_REFERENCE, u32, second_target, this.wrapping_add(SECOND_TARGET_OFFSET));
    }
    this
});
