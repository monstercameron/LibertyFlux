// original: 0x00b9a730 GET_FURTHEST_NETWORK_RESTART_NODE
/// Script native `GET_FURTHEST_NETWORK_RESTART_NODE` (hash 0x2FEF2477).
///
/// The first script argument points at three floats. The handler stashes
/// that pointer and copies the triple into the call context's own scratch
/// area (slot selected by the index word at `ctx+12`, which it increments),
/// then calls the engine with the scratch pointer and the second script
/// argument, and stores the low byte of the answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00b9a730(ctx: *mut u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let triple = *args as *const f32;
        let index = *(ctx.add(12) as *const u32);
        *(ctx.add(0x10 + index as usize * 4) as *mut u32) = triple as u32;
        let scratch = ctx.add((index as usize + 2) * 16);
        *(scratch as *mut f32) = *triple;
        *(scratch.add(4) as *mut f32) = *triple.add(1);
        *(scratch.add(8) as *mut f32) = *triple.add(2);
        *(ctx.add(12) as *mut u32) = index + 1;
        let answer = callee_cdecl!(1, u32, scratch as u32, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
