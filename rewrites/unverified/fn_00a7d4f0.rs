// original: 0x00A7D4F0 CComplexNewGetInVehicleTaskInfo::CComplexNewGetInVehicleTaskInfo_2
/// Default-construct CComplexNewGetInVehicleTaskInfo. Its extended base constructor runs first; the relocated class vtable and a zero byte at +0x30 complete initialization.
lf_checker_rt::export!(thiscall, rw_00a7d4f0(this: u32) -> u32 {
    const BASE_CTOR: u32 = 2;
    const VTABLE: u32 = 0x00EA149C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x30) as *mut u8).write(0);
    }
    this
});
