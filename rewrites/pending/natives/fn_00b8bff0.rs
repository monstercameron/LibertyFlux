// original: 0x00b8bff0 DISPLAY_AMMO
/// Script native `DISPLAY_AMMO` (hash 0x2E115B4B).
///
/// Forwards one boolean script argument to the engine. The argument is
/// /// coerced with `arg != 0`.
///
/// /// Quirk (observed): the handler coerces the flag into the low byte
/// /// of its own incoming stack slot and pushes the whole dword, so the
/// /// pushed word's high bytes repeat the context pointer. The engine
/// /// reads only the low byte (Inferred); the full dword is reproduced
/// /// here for bit-exact outgoing-call matching. No return slot is
/// /// written.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b8bff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        // v2 port: push the bare flag; the slot-residue high bytes are masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, quirked)
    }
});
