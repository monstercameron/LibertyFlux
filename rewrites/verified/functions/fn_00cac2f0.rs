// original: 0x00cac2f0 CTaskComplexMoveToShelterAndWait::CTaskComplexMoveToShelterAndWait_2

/// Constructs the alternate move-to-shelter-and-wait task. The first f32 word
/// is forwarded to the move base constructor and retained at `+0x20`; the
/// second f32 word is stored at `+0x24` and the third input word at `+0x28`.
/// The task and base vtables are installed,
/// fixed zero fields are initialized, a 16-bit field at `+0x3c` is cleared,
/// and flag bit 0 is set while bit 1 is cleared at `+0x50`.
lf_checker_rt::export!(thiscall, rw_00cac2f0(
    this: u32,
    speed_bits: u32,
    field_24_bits: u32,
    field_28: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_8d4c;
    const BASE_VTABLE_VA: u32 = 0x00ed_8da4;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed_bits);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));
        ((this + 0x20) as *mut u32).write_unaligned(speed_bits);
        ((this + 0x24) as *mut u32).write_unaligned(field_24_bits);
        ((this + 0x28) as *mut u32).write_unaligned(field_28);
        ((this + 0x2c) as *mut u32).write_unaligned(0);
        ((this + 0x30) as *mut u32).write_unaligned(0);
        ((this + 0x34) as *mut u32).write_unaligned(0);
        ((this + 0x38) as *mut u32).write_unaligned(0);
        ((this + 0x3c) as *mut u16).write_unaligned(0);
        ((this + 0x40) as *mut u32).write_unaligned(0);
        ((this + 0x44) as *mut u32).write_unaligned(0);
        ((this + 0x48) as *mut u32).write_unaligned(0);
        let old_flags = ((this + 0x50) as *const u32).read_unaligned();
        ((this + 0x50) as *mut u32).write_unaligned((old_flags & !2) | 1);
    }
    this
});
