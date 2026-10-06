// original: 0x00bd7f90 IS_SPHERE_VISIBLE_TO_ANOTHER_MACHINE
/// Test whether a sphere is visible to another machine.
///
/// Builds a position vector from script arguments 0-2, forwards it to the engine implementation, and stores the low byte of the answer in the return slot.
///
/// The engine call takes a pointer to a caller-frame vector in ECX; the
/// contract skips the address and snapshots the three pointed-to words.
export!(cdecl, rw_00bd7f90(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let vec = [*args, *args.add(1), *args.add(2)];
        let answer = callee_thiscall!(1, u32, vec.as_ptr() as u32);
        let slot = *(ctx as *const u32);
        *(slot as *mut u32) = answer & 0xff;
        slot
    }
});
