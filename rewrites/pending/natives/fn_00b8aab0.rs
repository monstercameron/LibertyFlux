// original: 0x00b8aab0 GET_CUTSCENE_PED_POSITION
/// Script native `GET_CUTSCENE_PED_POSITION` (hash 0x366B549F).
///
/// Reads a ped handle and a pointer to a 3-word position vector from
/// the argument array, records both inside the call context (the
/// pointer at a slot selected by the context's own entry counter, the
/// vector contents at a second slot derived from the same counter),
/// bumps the counter, and forwards the handle plus a pointer to the
/// copied vector to the engine worker. No return slot is written.
export!(cdecl, rw_00b8aab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let handle = *args;
        let vec = *args.add(1) as *const u32;
        let index = *(ctx.add(0x0c) as *const u32);
        // Record the vector pointer at slot `index`.
        *(ctx.add(0x10).add((index * 4) as usize) as *mut u32) = vec as u32;
        // Copy the three vector words to slot `index + 2`.
        let dest = ctx.add(((index + 2) * 16) as usize) as *mut u32;
        *dest = *vec;
        *dest.add(1) = *vec.add(1);
        *dest.add(2) = *vec.add(2);
        // Bump the context entry counter.
        *(ctx.add(0x0c) as *mut u32) = index.wrapping_add(1);
        callee_cdecl!(1, u32, handle, dest as u32)
    }
});
