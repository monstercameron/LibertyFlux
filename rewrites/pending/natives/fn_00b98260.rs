// original: 0x00b98260 GET_BUFFERED_ASCII
/// Script native `GET_BUFFERED_ASCII` (hash 0x21F43531).
///
/// Forwards two script arguments to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b98260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
