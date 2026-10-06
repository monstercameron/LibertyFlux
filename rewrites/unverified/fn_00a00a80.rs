// original: 0x00a00a80 FIND_NEAREST_COLLECTABLE_BIN_BAGS
/// Find the nearest collectable bin bags.
///
/// Builds a position vector from script arguments 0-2 and forwards it to the engine implementation.
///
/// The engine call takes a pointer to a caller-frame vector in ECX; the
/// contract skips the address and snapshots the three pointed-to words.
export!(cdecl, rw_00a00a80(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let vec = [*args, *args.add(1), *args.add(2)];
        callee_thiscall!(1, u32, vec.as_ptr() as u32)
    }
});
