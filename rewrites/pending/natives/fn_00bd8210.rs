// original: 0x00bd8210 NETWORK_DID_INVITE_FRIEND
/// Script native `NETWORK_DID_INVITE_FRIEND` (hash 0x3CAA1340).
///
/// Script arguments: 1 word(s).
///
/// Forwards arg0 (integer/handle) to the engine routine.
/// The engine's byte answer is zero-extended into the return slot.
export!(cdecl, rw_00bd8210(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const *mut u32);
        let arg0 = *args.add(0);
        let answer: u32 = callee_cdecl!(1, u32, arg0, );
        *slot = answer & 0xFF;
        slot as u32
    }
});
