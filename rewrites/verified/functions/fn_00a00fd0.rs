// original: 0x00a00fd0 GET_STATE_OF_CLOSEST_DOOR_OF_TYPE
/// Read the state of the closest door of a type.
///
/// Builds a position vector from script arguments 1-3 and forwards it with the door type and the first two vector words (0, 1, 2) to the engine implementation.
///
/// The engine call takes a pointer to a caller-frame vector in ECX; the
/// contract skips the address and snapshots the three pointed-to words.
export!(cdecl, rw_00a00fd0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let vec = [*args.add(1), *args.add(2), *args.add(3)];
        callee_thiscall!(1, u32, vec.as_ptr() as u32, *args, *args.add(1), *args.add(2))
    }
});
