// original: 0x00A7D4C0 CComplexNewGetInVehicleTaskInfo::CComplexNewGetInVehicleTaskInfo
/// Construct CComplexNewGetInVehicleTaskInfo by forwarding `mode`, `auxiliary`, a sentinel word, `base_flag` and `extra` to the extended base constructor. The low byte of `kind` is stored at +0x30, the class vtable is installed, and `this` is returned.
lf_checker_rt::export!(thiscall, rw_00a7d4c0(this: u32, base_flag: u32, mode: u32, auxiliary: u32, kind: u32, extra: u32) -> u32 {
    const EXTENDED_CTOR: u32 = 3;
    const VTABLE: u32 = 0x00EA149C;
    const KIND_OFFSET: u32 = 0x30;
    let _ = lf_checker_rt::callee_thiscall!(EXTENDED_CTOR, u32, this, mode, auxiliary, u32::MAX, base_flag, extra);
    unsafe {
        (KIND_OFFSET.wrapping_add(this) as *mut u8).write(kind as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
