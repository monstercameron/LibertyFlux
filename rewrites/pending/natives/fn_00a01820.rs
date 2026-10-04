// original: 0x00A01820 PLAYER_IS_NEAR_FIRST_PIGEON
// PLAYER_IS_NEAR_FIRST_PIGEON: slot 0 points at an xyz vector. The handler
// stashes that pointer in the context scratch header, copies the vector
// into scratch slot `scratch + 2`, bumps the scratch index, calls the
// engine with the scratch copy's address, and stores the answer's low byte
// (zero-extended) into the return slot. Returns the slot pointer.
export!(cdecl, rw_fn_a01820(ctx: *mut u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let vecp = *args as *const u32;
        let scratch = *((ctx.add(0x0c)) as *const u32);
        *((ctx.add(0x10).add((scratch as usize).wrapping_mul(4))) as *mut u32) =
            vecp as u32;
        let slot = ctx.add((scratch.wrapping_add(2).wrapping_mul(16)) as usize);
        *(slot as *mut u32) = *vecp;
        *((slot.add(4)) as *mut u32) = *vecp.add(1);
        *((slot.add(8)) as *mut u32) = *vecp.add(2);
        *((ctx.add(0x0c)) as *mut u32) = scratch.wrapping_add(1);
        let ans: u32 = callee_cdecl!(1, u32, slot as u32);
        let ret = *(ctx as *mut *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});
