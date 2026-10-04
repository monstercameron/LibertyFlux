// original: 0x00bdabc0 NETWORK_GET_FIND_RESULT
/// Script native `NETWORK_GET_FIND_RESULT` (hash 0x282D2CAA).
///
/// Copies one network-find result into a script buffer. Forwards two values
/// to the engine: the result index and the script buffer pointer advanced
/// past a one-word header (`args[1] + 4`). No return slot is written.
export!(cdecl, rw_00bdabc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let buf = (*args.add(1)).wrapping_add(4);
        callee_cdecl!(1, u32, *args, buf)
    }
});
