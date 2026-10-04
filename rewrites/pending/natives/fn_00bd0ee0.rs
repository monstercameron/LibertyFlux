// original: 0x00bd0ee0 GET_IS_STICKY_BOMB_STUCK_TO_VEHICLE
/// Script native `GET_IS_STICKY_BOMB_STUCK_TO_VEHICLE` (hash 0x29BF0233).
///
/// Forwards one script argument (a vehicle handle) to the engine and
/// /// stores the low byte of its answer (zero-extended) into the
/// /// return slot.
export!(cdecl, rw_00bd0ee0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
