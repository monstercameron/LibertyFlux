// original: 0x00bc5fe0 GET_NEXT_STATION_FOR_TRAIN
/// Script native `GET_NEXT_STATION_FOR_TRAIN` (hash 0x4835637D).
///
/// Forwards one script argument (a train handle) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bc5fe0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
