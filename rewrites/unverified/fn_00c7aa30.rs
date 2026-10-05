// original: 0x00c7aa30 scenario_slot_register

/// Register an argument pair in the next free global scenario slot.
/// Reads the slot count: when count+1 is below `CAP` (20) the pair is
/// handed to the slot-fill callee with the slot address
/// (`SLOTS` + count*32) in ECX, then count+1 is stored back. Otherwise
/// nothing happens. The count comparison is signed; EAX on return is
/// whatever the last step left (entry EAX when skipped), so no return
/// channel is compared.
/// Original: 0x00c7aa30 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00c7aa30(a0: u32, a1: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x016F80C4;
        const SLOTS: u32 = 0x016F80D0;
        const SLOT_STRIDE: u32 = 32;
        const CAP: i32 = 0x14;
        const FILL: u32 = 1;
        let count = lf_checker_rt::global::<i32>(COUNT).read();
        let next = count.wrapping_add(1);
        if next < CAP {
            let slot = lf_checker_rt::relocated(SLOTS).wrapping_add((count as u32).wrapping_mul(SLOT_STRIDE));
            lf_checker_rt::callee_thiscall!(FILL, u32, slot, a0, a1);
            lf_checker_rt::global::<i32>(COUNT).write(next);
        }
        0
    }
});
