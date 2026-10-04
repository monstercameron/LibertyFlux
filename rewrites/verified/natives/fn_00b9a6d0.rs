// original: 0x00b9a6d0 GET_COORDINATES_FOR_NETWORK_RESTART_NODE
/// Script native `GET_COORDINATES_FOR_NETWORK_RESTART_NODE` (hash 0x2EAA3C4A).
///
/// Takes three script arguments (a node index, a vector out-pointer, and a
/// flags word). Copies the three words at the out-pointer into the context's
/// return area at `ctx + (n + 2) * 16`, where `n` is the running index kept
/// at `ctx+0x0c`; records the out-pointer itself at `ctx + n * 4 + 0x10`;
/// bumps the index; then calls the engine with the node index, a pointer to
/// the copied vector, and the flags word. The vector words are copied as raw
/// bits, so float coordinates survive bit-exactly. No return slot is written
/// by the handler itself.
export!(cdecl, rw_00b9a6d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let node = *args;
        let out_ptr = *args.add(1);
        let flags = *args.add(2);
        let n = *(ctx.add(0x0c) as *const u32);
        let dest = (ctx as u32)
            .wrapping_add(n.wrapping_add(2).wrapping_mul(16)) as *mut u32;
        *(ctx.add(n.wrapping_mul(4).wrapping_add(0x10) as usize) as *mut u32) =
            out_ptr;
        let src = out_ptr as *const u32;
        *dest = *src;
        *dest.add(1) = *src.add(1);
        *dest.add(2) = *src.add(2);
        *(ctx.add(0x0c) as *mut u32) = n.wrapping_add(1);
        callee_cdecl!(1, u32, node, dest as u32, flags)
    }
});
