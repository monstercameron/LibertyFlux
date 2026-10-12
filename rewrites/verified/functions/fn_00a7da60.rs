// original: 0x00A7DA60 CScriptAnimTaskInfo::CScriptAnimTaskInfo
/// Construct the parameterized CScriptAnimTaskInfo form with its base constructor, relocated vtable, zero dwords at +0x14 through +0x20, and a zero byte at +0x24. Returns `this`.
lf_checker_rt::export!(thiscall, rw_00a7da60(this: u32) -> u32 {
    const BASE_CTOR: u32 = 6;
    const VTABLE: u32 = 0x00EA19DC;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u8).write(0);
    }
    this
});
