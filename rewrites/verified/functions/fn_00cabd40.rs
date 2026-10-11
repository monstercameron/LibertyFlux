// original: 0x00cabd40 CTaskComplexMoveGetToPointContinuous::CTaskComplexMoveGetToPointContinuous

/// Constructs a continuous move-to-point task. The supplied speed word is
/// forwarded unchanged to the move base constructor. Three f32 words are
/// copied to the point fields at `+0x20..+0x28`; the three words addressed by
/// `point_data` are copied to `+0x30..+0x38`. The two task vtables are then
/// installed, and the task pointer is returned.
lf_checker_rt::export!(thiscall, rw_00cabd40(
    this: u32,
    speed_bits: u32,
    point_data: u32,
    field_24_bits: u32,
    field_20_bits: u32,
    field_28_bits: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_8b0c;
    const BASE_VTABLE_VA: u32 = 0x00ed_8b64;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed_bits);
    unsafe {
        ((this + 0x20) as *mut u32).write_unaligned(field_20_bits);
        ((this + 0x24) as *mut u32).write_unaligned(field_24_bits);
        ((this + 0x28) as *mut u32).write_unaligned(field_28_bits);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));

        let point_x = (point_data as *const u32).read_unaligned();
        let point_y = ((point_data + 4) as *const u32).read_unaligned();
        let point_z = ((point_data + 8) as *const u32).read_unaligned();
        ((this + 0x30) as *mut u32).write_unaligned(point_x);
        ((this + 0x34) as *mut u32).write_unaligned(point_y);
        ((this + 0x38) as *mut u32).write_unaligned(point_z);
    }
    this
});
