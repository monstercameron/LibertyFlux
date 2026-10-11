// original: 0x00cabe90 CTaskComplexMoveGoToPointAndStandStill::CTaskComplexMoveGoToPointAndStandStill_2

/// Constructs the alternate go-to-point-and-stand-still task. The supplied
/// speed word is forwarded unchanged to the move base constructor. The low
/// two input flag bits replace bits 0 and 1 while other stored flags are
/// retained except bit 4. The task starts with a fixed 2.0 scalar at `+0x34`
/// and the supplied word at `+0x38`. If any of the first three point
/// coordinates differs from the pointed-to vector, including an unordered
/// NaN comparison, all four vector words are copied to `+0x20..+0x2c` and
/// flag bit 4 is set. The scalar at `+0x30` is copied only when unequal or
/// unordered. Signed zero compares equal; the float words undergo no
/// arithmetic.
lf_checker_rt::export!(thiscall, rw_00cabe90(
    this: u32,
    speed_bits: u32,
    point_data: u32,
    scalar_30_bits: u32,
    scalar_38_bits: u32,
    flag_low: u32,
    flag_high: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_837c;
    const BASE_VTABLE_VA: u32 = 0x00ed_83d4;
    const FLAGS: u32 = 0x3c;
    const DEFAULT_SCALAR_BITS: u32 = 0x4000_0000;
    const POINT_CHANGED: u32 = 0x04;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed_bits);

    let previous_flags = unsafe { ((this + FLAGS) as *const u32).read_unaligned() };
    let new_flags = (previous_flags & 0xffff_ffe4)
        | ((flag_high & 1) << 1)
        | (flag_low & 1);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));
        ((this + FLAGS) as *mut u32).write_unaligned(new_flags);
        ((this + 0x34) as *mut u32).write_unaligned(DEFAULT_SCALAR_BITS);
        ((this + 0x38) as *mut u32).write_unaligned(scalar_38_bits);
    }

    let point_x = unsafe { (point_data as *const u32).read_unaligned() };
    let point_y = unsafe { ((point_data + 4) as *const u32).read_unaligned() };
    let point_z = unsafe { ((point_data + 8) as *const u32).read_unaligned() };
    let old_x = unsafe { ((this + 0x20) as *const u32).read_unaligned() };
    let old_y = unsafe { ((this + 0x24) as *const u32).read_unaligned() };
    let old_z = unsafe { ((this + 0x28) as *const u32).read_unaligned() };
    let point_changed = f32::from_bits(old_x) != f32::from_bits(point_x)
        || f32::from_bits(old_y) != f32::from_bits(point_y)
        || f32::from_bits(old_z) != f32::from_bits(point_z);
    if point_changed {
        let point_w = unsafe { ((point_data + 12) as *const u32).read_unaligned() };
        unsafe {
            ((this + 0x20) as *mut u32).write_unaligned(point_x);
            ((this + 0x24) as *mut u32).write_unaligned(point_y);
            ((this + 0x28) as *mut u32).write_unaligned(point_z);
            ((this + 0x2c) as *mut u32).write_unaligned(point_w);
            let flags = ((this + FLAGS) as *const u32).read_unaligned();
            ((this + FLAGS) as *mut u32).write_unaligned(flags | POINT_CHANGED);
        }
    }

    let previous_scalar = unsafe { ((this + 0x30) as *const u32).read_unaligned() };
    if f32::from_bits(previous_scalar) != f32::from_bits(scalar_30_bits) {
        unsafe { ((this + 0x30) as *mut u32).write_unaligned(scalar_30_bits) };
    }
    this
});
