// original: 0x00D4E810 task_acquire_slot_object (proposed)

// Acquires the slot object for the task and marks it held.
///
/// Takes the argument as a container, passes the word at container `+0x78` in
/// ECX with (0, 4, 4.0, -1) to intercepted callee 1, and stores the result at
/// `+0x18`. A null result sets the byte at `+0x14` instead. Otherwise flag
/// bits 0x8000 then 0x4000 are or-ed into the result's word at `+4`, and the
/// result goes in ECX with (2, a fixed address, the object pointer) to intercepted callee 2.
/// Returns the last callee result (null on the early path).
///
/// Original: 0x00D4E810 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4e810(this: u32, container: u32) -> u32 {
    unsafe {
        const FOUR_BITS: u32 = 0x40800000; // 4.0f
        const FIXED_ADDR: u32 = 0x00B6FE10;
        const ACQUIRE: u32 = 1;
        const ATTACH: u32 = 2;
        const HELD_A: u32 = 0x8000;
        const HELD_B: u32 = 0x4000;
        let w78 = ((container + 0x78) as *const u32).read_unaligned();
        let got: u32 = lf_checker_rt::callee_thiscall!(
            ACQUIRE, u32, w78, 0, 4, FOUR_BITS, 0xFFFFFFFF);
        ((this + 0x18) as *mut u32).write_unaligned(got);
        if got == 0 {
            ((this + 0x14) as *mut u8).write(1);
            return 0;
        }
        let f = ((got + 4) as *const u32).read_unaligned();
        ((got + 4) as *mut u32).write_unaligned(f | HELD_A);
        let got2 = ((this + 0x18) as *const u32).read_unaligned();
        let f2 = ((got2 + 4) as *const u32).read_unaligned();
        ((got2 + 4) as *mut u32).write_unaligned(f2 | HELD_B);
        lf_checker_rt::callee_thiscall!(
            ATTACH, u32, got2, 2, lf_checker_rt::relocated(FIXED_ADDR), this)
    }
});
