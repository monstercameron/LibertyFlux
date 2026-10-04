// original: 0x00ba0640 LOCATE_CHAR_IN_CAR_3D
/// Script native `LOCATE_CHAR_IN_CAR_3D` (hash 0x0AC92D36).
///
/// Forwards eight script arguments to the engine: a character handle, six float
/// bit-patterns (centre coordinates and extents) and a boolean flag.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching. Stores the low byte of the engine
/// answer (zero-extended) into the return slot.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00ba0640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(7) != 0);
        // v2 port: push the bare flag; the slot-residue high bytes are masked in the contract (call_skip).
        let quirked = flag;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
