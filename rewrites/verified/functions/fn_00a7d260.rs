// original: 0x00A7D260 CComplexFallAndGetUpTaskInfo::CComplexFallAndGetUpTaskInfo
/// Construct CComplexFallAndGetUpTaskInfo by running its base constructor, copying the full state word to +0x18, installing the relocated vtable, and returning `this`.
lf_checker_rt::export!(thiscall, rw_00a7d260(this: u32, state: u32) -> u32 {
    const BASE_CTOR: u32 = 1;
    const VTABLE: u32 = 0x00EA135C;
    const STATE_OFFSET: u32 = 0x18;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this.wrapping_add(STATE_OFFSET) as *mut u32).write_unaligned(state);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
