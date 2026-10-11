// original: 0x00A82CC0 CComplexPickUpAndCarryObjectTaskInfo::vf18

/// Map a compact task kind to one of three stable descriptor pointers.
///
/// `this` is unused. The single stack argument is compared as an unsigned
/// 32-bit value: kinds zero, one and two select descriptors at image offsets
/// `0x00EA16EC`, `0x00EA16F4` and `0x00EA16FC`; every other value returns
/// zero. The returned pointer is relocated with the worker's image base.
lf_checker_rt::export!(thiscall, rw_00a82cc0(this: u32, task_kind: u32) -> u32 {
    const DESCRIPTOR_KIND_ZERO: u32 = 0x00EA16EC;
    const DESCRIPTOR_KIND_ONE: u32 = 0x00EA16F4;
    const DESCRIPTOR_KIND_TWO: u32 = 0x00EA16FC;

    let file_address = match task_kind {
        0 => DESCRIPTOR_KIND_ZERO,
        1 => DESCRIPTOR_KIND_ONE,
        2 => DESCRIPTOR_KIND_TWO,
        _ => return 0,
    };
    let _ = this;
    lf_checker_rt::relocated(file_address)
});
