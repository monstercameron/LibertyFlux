// original: 0x009cb9d0 CAN_BE_DESCRIBED_AS_A_CAR
/// Script native `CAN_BE_DESCRIBED_AS_A_CAR` (hash 0x79103802).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_009cb9d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
