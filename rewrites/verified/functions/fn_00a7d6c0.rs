// original: 0x00A7D6C0 CMedicTaskInfo::CMedicTaskInfo_2
/// Default-construct CMedicTaskInfo through the linked base constructor with a zero mode. Clear bit 0 of the existing word at +0x24, install the relocated vtable, clear the pointers at +0x1c and +0x20, and return `this`.
lf_checker_rt::export!(thiscall, rw_00a7d6c0(this: u32) -> u32 {
    const LINKED_CTOR: u32 = 7;
    const VTABLE: u32 = 0x00EA11F4;
    let _ = lf_checker_rt::callee_thiscall!(LINKED_CTOR, u32, this, 0);
    unsafe {
        let flags = (this.wrapping_add(0x24) as *const u32).read_unaligned();
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(flags & !1);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
    }
    this
});
