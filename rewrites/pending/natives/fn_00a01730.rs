// original: 0x00a01730 LOOK_AT_NEARBY_ENTITY_WITH_SPECIAL_ATTRIBUTE
/// Script native `LOOK_AT_NEARBY_ENTITY_WITH_SPECIAL_ATTRIBUTE`
/// (hash 0x6EB639E8).
///
/// The second script argument points at three floats. As in
/// `GET_FURTHEST_NETWORK_RESTART_NODE`, the handler stashes the pointer and
/// copies the triple into context scratch (indexed by `ctx+12`), then calls
/// the engine with the first argument, the scratch pointer and the
/// remaining four arguments, and stores the low byte of the answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00a01730(ctx: *mut u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let triple = *args.add(1) as *const f32;
        let index = *(ctx.add(12) as *const u32);
        *(ctx.add(0x10 + index as usize * 4) as *mut u32) = triple as u32;
        let scratch = ctx.add((index as usize + 2) * 16);
        *(scratch as *mut f32) = *triple;
        *(scratch.add(4) as *mut f32) = *triple.add(1);
        *(scratch.add(8) as *mut f32) = *triple.add(2);
        *(ctx.add(12) as *mut u32) = index + 1;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            scratch as u32,
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5)
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
