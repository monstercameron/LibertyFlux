// original: 0x00bd80e0 NETWORK_ADVERTISE_SESSION
/// Script native `NETWORK_ADVERTISE_SESSION` (hash 0x1B9E5D07).
///
/// Forwards one boolean script argument to the engine, coerced with
/// `arg != 0`, and stores the low byte of its answer (zero-extended) into
/// the return slot.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching.
export!(cdecl, rw_00bd80e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(1, u32, quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
