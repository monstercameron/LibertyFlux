// original: 0x00cac270 CTaskComplexMoveToShelterAndWait::CTaskComplexMoveToShelterAndWait

/// Constructs a move-to-shelter-and-wait task. The speed word is forwarded
/// unchanged to the move base constructor; the supplied integer is stored at
/// `+0x28`, and the speed word is also retained at `+0x20`. The task and base
/// vtables are installed, fixed zero and one fields are initialized, a
/// 16-bit field at `+0x3c` is cleared, three words are copied from
/// `shelter_data` to `+0x40..+0x48`, and flag bits 0 and 1 are set at `+0x50`.
lf_checker_rt::export!(thiscall, rw_00cac270(
    this: u32,
    speed_bits: u32,
    shelter_data: u32,
    shelter_id: u32
) -> u32 {
    const MOVE_BASE_CTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ed_8d4c;
    const BASE_VTABLE_VA: u32 = 0x00ed_8da4;

    let _ = lf_checker_rt::callee_thiscall!(MOVE_BASE_CTOR, u32, this, speed_bits);
    unsafe {
        let shelter_x = (shelter_data as *const u32).read_unaligned();
        let shelter_y = ((shelter_data + 4) as *const u32).read_unaligned();
        let shelter_z = ((shelter_data + 8) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_VTABLE_VA));
        ((this + 0x20) as *mut u32).write_unaligned(speed_bits);
        ((this + 0x24) as *mut u32).write_unaligned(0);
        ((this + 0x28) as *mut u32).write_unaligned(shelter_id);
        ((this + 0x2c) as *mut u32).write_unaligned(1);
        ((this + 0x30) as *mut u32).write_unaligned(0);
        ((this + 0x34) as *mut u32).write_unaligned(0);
        ((this + 0x38) as *mut u32).write_unaligned(0);
        ((this + 0x3c) as *mut u16).write_unaligned(0);
        ((this + 0x40) as *mut u32).write_unaligned(shelter_x);
        ((this + 0x44) as *mut u32).write_unaligned(shelter_y);
        ((this + 0x48) as *mut u32).write_unaligned(shelter_z);
        let flags = ((this + 0x50) as *const u32).read_unaligned();
        ((this + 0x50) as *mut u32).write_unaligned(flags | 3);
    }
    this
});
