// original: 0x00b94630 GET_OFFSET_FROM_INTERIOR_IN_WORLD_COORDS
/// Script native `GET_OFFSET_FROM_INTERIOR_IN_WORLD_COORDS` (hash 0x68966670).
///
/// Takes a handle, three float bit-patterns (an offset) and a pointer to a
/// three-word interior record. Copies the record's three words into the
/// context's return area at the slot selected by the counter at `ctx+0x0c`,
/// records the record pointer just past the argument area, bumps the
/// counter, and calls the engine with the handle, the three offset words
/// by value, and the return-area slot pointer. Floats are copied as raw
/// bits, so the forward is bit-exact.
export!(cdecl, rw_00b94630(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let count = *(ctx.add(0x0c) as *const u32);
        let item = *args.add(4) as *const u32;
        *((ctx.add(0x10) as *mut u32).add(count as usize)) = item as u32;
        let slot = (ctx as *mut u32).add((count as usize + 2) * 4);
        *slot = *item;
        *slot.add(1) = *item.add(1);
        *slot.add(2) = *item.add(2);
        *(ctx.add(0x0c) as *mut u32) = count + 1;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            slot as u32,
        )
    }
});
