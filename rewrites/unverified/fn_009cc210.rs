// original: 0x009cc210 PRELOAD_STREAM_WITH_START_OFFSET
/// Script native `PRELOAD_STREAM_WITH_START_OFFSET` (hash 0x2B8836A6).
///
/// Forwards two script arguments (a stream name and an offset) to the
/// engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_009cc210(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
