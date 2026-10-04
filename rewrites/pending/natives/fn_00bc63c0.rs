// original: 0x00bc63c0 GET_STATION_NAME
/// Script native `GET_STATION_NAME` (hash 0x46F87F55).
///
/// Checks the vehicle is a train, then returns the station-name string.
///
/// Script arguments: args: Train train, uint station.
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle) to the engine routine.
/// The engine's full-word answer is stored into the return slot.
export!(cdecl, rw_00bc63c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const *mut u32);
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, arg0, arg1, );
        *slot = answer;
        answer
    }
});
