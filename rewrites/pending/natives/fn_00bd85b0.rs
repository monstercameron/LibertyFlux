// original: 0x00bd85b0 NETWORK_HAS_STRICT_NAT
/// Test whether the network path uses strict NAT.
///
/// Takes no arguments; stores the engine's boolean answer (low byte only) in
/// the return slot. Returns the return-slot address.
export!(cdecl, rw_00bd85b0(ctx: u32) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const u32) as *mut u32;
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
