// original: 0x00A7D650 CJackTaskInfo::CJackTaskInfo
/// Construct CJackTaskInfo by running the base constructor, storing the low byte of `mode` at +0x14, installing the relocated vtable, and returning `this`.
lf_checker_rt::export!(thiscall, rw_00a7d650(this: u32, mode: u32) -> u32 {
    const BASE_CTOR: u32 = 6;
    const VTABLE: u32 = 0x00EA1654;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(0x14) as *mut u8).write(mode as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
