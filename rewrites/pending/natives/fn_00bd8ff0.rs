// original: 0x00bd8ff0 REQUEST_CONTROL_OF_NETWORK_ID
/// Ask the network layer for control of a network id.
///
/// Forwards the id from script argument 0 to the engine, keeps only the low
/// byte of its boolean answer (the original uses `movzx`, so high bytes from
/// a nonzero answer are dropped) and stores it in the return slot. Returns
/// the return-slot address, as the original leaves it in `eax`.
export!(cdecl, rw_00bd8ff0(ctx: u32) -> u32 {
    unsafe {
        let base = ctx as *const u32;
        let ret_slot = *base as *mut u32;
        let args = *base.add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
