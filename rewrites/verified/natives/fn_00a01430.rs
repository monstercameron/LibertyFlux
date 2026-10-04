// original: 0x00a01430 IS_OBJECT_IN_ANGLED_AREA_3D
/// Script native `IS_OBJECT_IN_ANGLED_AREA_3D` (hash 0x5D5A06F7).
///
/// Forwards nine script arguments to the engine: an object handle, seven
/// float bit-patterns (angled-area bounds) and a boolean flag coerced with
/// `arg != 0`. Floats are copied as raw bits, so the forward is bit-exact.
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00a01430(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(8) != 0);

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
            *args.add(7),
            flag,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
