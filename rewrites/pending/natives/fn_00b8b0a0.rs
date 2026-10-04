// original: 0x00b8b0a0 GET_NETWORK_RESTART_NODE_DEBUG
/// Script native `GET_NETWORK_RESTART_NODE_DEBUG` (hash 0x6629119D).
///
/// Copies one node record through the call context, appends it to the context's inline arrays, bumps the stored count, then calls the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8b0a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let node = *args.add(1) as *const u32;
        let count = *(ctx.add(0xc) as *const u32);
        let w0 = *node;
        let w1 = *node.add(1);
        let w2 = *node.add(2);
        *((ctx.add(0x10) as *mut u32).add(count as usize)) = node as u32;
        let base = (ctx as *mut u8).add(count as usize * 16 + 32);
        *(base as *mut u32) = w0;
        *(base.add(4) as *mut u32) = w1;
        *(base.add(8) as *mut u32) = w2;
        *(ctx.add(0xc) as *mut u32) = count.wrapping_add(1);
        let buf = (ctx as u32).wrapping_add((count as usize + 2).wrapping_mul(16) as u32);
        let answer = callee_cdecl!(1, u32, *args, buf, *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
