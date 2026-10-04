// original: 0x00bc6e40 IS_VEH_DRIVEABLE
/// Script native `IS_VEH_DRIVEABLE` (hash 0x17BC668D).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6e40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
