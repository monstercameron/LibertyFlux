// original: 0x00a1d8c0 cam_target_set_pair (proposed)

/// Stores a target float pair and marks it dirty.
///
/// `this` points to a record with two float slots at `+SLOT0_OFF` and
/// `+SLOT1_OFF` and a flag byte at `+FLAG_OFF`. The two float arguments
/// (`f0`, `f1`, passed as bits) are stored into the slots and the dirty
/// bit is set in the flag byte. Returns nothing.
///
/// Original: 0x00a1d8c0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a1d8c0(this: u32, f0: u32, f1: u32) -> u32 {
    unsafe {
        const SLOT0_OFF: u32 = 0x324;
        const SLOT1_OFF: u32 = 0x328;
        const FLAG_OFF: u32 = 0x38d;
        const DIRTY_BIT: u8 = 8;
        let flag = (this + FLAG_OFF) as *mut u8;
        flag.write(flag.read() | DIRTY_BIT);
        ((this + SLOT0_OFF) as *mut u32).write_unaligned(f0);
        ((this + SLOT1_OFF) as *mut u32).write_unaligned(f1);
        0
    }
});
