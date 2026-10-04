// original: 0x00bd0f90 GET_NUMBER_OF_STICKY_BOMBS_STUCK_TO_VEHICLE
/// Script native `GET_NUMBER_OF_STICKY_BOMBS_STUCK_TO_VEHICLE` (hash 0x285D1184).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores
/// its full 32-bit answer (a count) into the return slot. Unlike the boolean
/// natives, this handler keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00bd0f90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
