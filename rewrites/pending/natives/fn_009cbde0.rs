// original: 0x009cbde0 IS_LAZLOW_STATION_LOCKED
/// Script native `IS_LAZLOW_STATION_LOCKED` (hash 0x1CB80079).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_009cbde0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
