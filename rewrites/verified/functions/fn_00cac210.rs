// original: 0x00cac210 CTaskComplexMoveGoToPointStandStillAchieveHeading::CTaskComplexMoveGoToPointStandStillAchieveHeading

/// Constructs a go-to-point task that also stores heading fields. The speed
/// word is forwarded unchanged to the move base constructor; the three words
/// at `point_data` are copied to `+0x20..+0x28`. Two input bytes are stored at
/// `+0x38` and `+0x39`, and two supplied f32 words are copied to `+0x30` and
/// `+0x34`. The task and base vtables are installed before returning `this`.
lf_checker_rt::export!(thiscall, rw_00cac210(
    this: u32,
    speed_bits: u32,
    point_data: u32,
    scalar_30_bits: u32,
    scalar_34_bits: u32,
    heading_low: u32,
    heading_high: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_849c;
    const BASE_VTABLE_VA: u32 = 0x00ed_84f4;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed_bits);
    unsafe {
        let point_x = (point_data as *const u32).read_unaligned();
        let point_y = ((point_data + 4) as *const u32).read_unaligned();
        let point_z = ((point_data + 8) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));
        ((this + 0x20) as *mut u32).write_unaligned(point_x);
        ((this + 0x24) as *mut u32).write_unaligned(point_y);
        ((this + 0x28) as *mut u32).write_unaligned(point_z);
        ((this + 0x30) as *mut u32).write_unaligned(scalar_30_bits);
        ((this + 0x34) as *mut u32).write_unaligned(scalar_34_bits);
        ((this + 0x38) as *mut u8).write_unaligned(heading_low as u8);
        ((this + 0x39) as *mut u8).write_unaligned(heading_high as u8);
    }
    this
});
