// original: 0x00b9fa70 IS_CHAR_IN_AREA_3D
/// Script native `IS_CHAR_IN_AREA_3D` (hash 0x44A30283).
///
/// Forwards eight script arguments to the engine: a character handle,
/// /// six float bit-patterns (two opposite corners of a box) and a
/// /// boolean flag coerced with `arg != 0`; then stores the low byte
/// /// of the engine answer (zero-extended) into the return slot.
///
/// /// Quirk (observed): the handler coerces the flag into the low byte
/// /// of its own incoming stack slot and pushes the whole dword, so the
/// /// pushed word's high bytes repeat the context pointer. The engine
/// /// reads only the low byte (Inferred); the full dword is reproduced
/// /// here for bit-exact outgoing-call matching.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b9fa70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(7) != 0);
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
            *args.add(5),
            *args.add(6),
            quirked,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
