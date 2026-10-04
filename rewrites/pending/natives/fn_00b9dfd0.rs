// original: 0x00b9dfd0 ARE_ENEMY_PEDS_IN_AREA
// Rewrite of the ARE_ENEMY_PEDS_IN_AREA native handler.

/// Script native `ARE_ENEMY_PEDS_IN_AREA(ped, x, y, z, radius)`.
///
/// Reads the ped handle and four floats from the call context's argument
/// array, forwards them to the engine proximity check, and stores the
/// zero-extended low byte of its answer in the context's return slot.
export!(cdecl, rw_b9dfd0(ctx: *const u8) -> u32 {
    unsafe {
        let ret_slot = *(ctx as *const *mut u32);
        let args = *((ctx.add(8)) as *const *const u32);
        let answer: u32 = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4)
        );
        *ret_slot = answer & 0xFF;
        0
    }
});
