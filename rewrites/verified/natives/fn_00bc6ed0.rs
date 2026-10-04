// original: 0x00bc6ed0 LOCATE_CAR_2D
/// Script native `LOCATE_CAR_2D` (hash 0x36F70AF6).
///
/// Forwards 6 script arguments (a vehicle handle, four float coordinates and a boolean flag) to the engine.
///
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
///
/// Float arguments are copied as raw bit patterns, so the forward
/// is bit-exact.
///
/// Quirk (observed): the handler coerces the boolean argument into
/// the low byte of its own incoming stack slot and pushes the whole
/// dword, so the pushed word's high bytes repeat the context pointer.
/// The engine reads only the low byte (Inferred); the full dword is
/// reproduced here for bit-exact outgoing-call matching.
/// The return slot holds the zero-extended engine answer.
///
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00bc6ed0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);
        // v2 port: push the bare flag; the slot-residue high bytes are masked in the contract (call_skip).
        let quirked = flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            quirked,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
