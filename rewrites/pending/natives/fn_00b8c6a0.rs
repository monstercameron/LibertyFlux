// original: 0x00b8c6a0 GET_BLIP_COORDS
/// Script native `GET_BLIP_COORDS` (hash 0x4C1E75DB).
///
/// Multi-output native: the second script argument points at a record of
/// three words (an integer plus two float bit-patterns). The handler files
/// the record pointer and a copy of the three words into the call context's
/// output areas at positions derived from the context's output counter
/// (`ctx+0x0c`), hands the engine the blip handle plus this call's output
/// slot, and bumps the counter.
export!(cdecl, rw_00b8c6a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let handle = *args;
        let record = *args.add(1) as *const u32;
        let count = *(ctx.add(0x0c) as *const u32);
        let first = *record;
        let second = *record.add(1);
        let third = *record.add(2);
        *(ctx.add(0x10 + count.wrapping_mul(4) as usize) as *mut u32) = record as u32;
        let doubled = count.wrapping_mul(2).wrapping_add(4);
        let out = ctx.add(doubled.wrapping_mul(8) as usize) as *mut u32;
        *out = first;
        *out.add(1) = second;
        *out.add(2) = third;
        let engine_out = ctx.add(count.wrapping_add(2).wrapping_mul(16) as usize);
        *(ctx.add(0x0c) as *mut u32) = count.wrapping_add(1);
        callee_cdecl!(1, u32, handle, engine_out as u32)
    }
});
