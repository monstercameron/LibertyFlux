// original: 0x00D09B10 CTaskComplexBeArrestedAndDrivenAway::vf19

/// Start the arrested-and-driven-away sequence for a ped.
///
/// Does nothing when the state word at `+0x14` is zero. Otherwise marks the
/// ped arrested, arms its vehicle slot (patched thiscall callee), takes a
/// task slot from the task manager (patched callee), and when one is free
/// enqueues the drive-away subtask with the fixed descriptor (patched
/// thiscall callee). Returns the enqueue answer, or zero on the early paths.
///
/// Original: 0x00D09B10 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d09b10(this: u32, a1: u32) -> u32 {
    unsafe {
        const ARM: u32 = 1;
        const ALLOC: u32 = 2;
        const ENQUEUE: u32 = 3;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        const STATE: u32 = 0x14;
        const SEATED: u32 = 0x18;
        const ARRESTED: u32 = 0xA70;
        const VEHICLE_SLOT: u32 = 0x2B0;
        const ONE_BITS: u32 = 0x3F800000; // 1.0f
        const SPEED_BITS: u32 = 0x41000000; // 8.0f

        if ((this + STATE) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        ((a1 + ARRESTED) as *mut u32).write_unaligned(1);
        lf_checker_rt::callee_thiscall!(ARM, u32, a1.wrapping_add(VEHICLE_SLOT), a1, 1);
        ((this + SEATED) as *mut u32).write_unaligned(1);
        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(ENQUEUE, u32, slot,
            0x16, 0x126, SPEED_BITS, 1, ONE_BITS, 0)
    }
});
