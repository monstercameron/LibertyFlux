// original: 0x00b9e440 CAN_CREATE_RANDOM_CHAR
/// Script native handler `CAN_CREATE_RANDOM_CHAR` (hash 0x5CD64D63).
///
/// Forwards script arguments 0..1 to the engine worker and stores the low byte of its answer (zero-extended) into the return slot.
/// Boolean arguments are coerced to 0/1; the original builds the pushed
/// word inside its own incoming stack slot, so the high bytes repeat the
/// context pointer and are reproduced from `ctx` here.
export!(cdecl, rw_00b9e440(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let q1 = (ctx & !0xFF) | u32::from(a1 != 0);
        // Low byte only: the original's upper bytes here are caller-register
        // leftovers, masked out of the call comparison by the contract.
        let b0 = u32::from(a0 != 0);
        let answer: u32 = callee_cdecl!(1, u32, b0, q1);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
