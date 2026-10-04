// original: 0x00b8d610 SET_GPS_TEST_IN_3D_FLAG
/// Script native `SET_GPS_TEST_IN_3D_FLAG` (hash 0x28D17798).
///
/// Bool flag.
///
/// Quirk (observed): the handler coerces the boolean argument
/// into the low byte of its own incoming stack slot and pushes
/// the whole dword, so the pushed word's high bytes repeat the
/// context pointer. The engine reads only the low byte (Inferred);
/// the full dword is reproduced here for bit-exact outgoing-call
/// matching.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b8d610(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let quirked = flag;
        callee_cdecl!(1, u32, quirked)
    }
});
