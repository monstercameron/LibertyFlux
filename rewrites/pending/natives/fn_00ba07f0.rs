// original: 0x00ba07f0 LOCATE_CHAR_IN_CAR_OBJECT_2D
/// Native handler `LOCATE_CHAR_IN_CAR_OBJECT_2D`: locate a character in a vehicle near an object in 2D: forward handles, coords and flag, return 0/1.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00ba07f0(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        // The original coerces this flag with a byte-wide setnz over the
        // incoming context slot, so the pushed dword keeps the slot's high
        // bytes; reproduce that shape exactly.
        let flag4 = (ctx & 0xFFFF_FF00) | u32::from(*args.add(4) != 0);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), flag4,);
        // The engine returns a byte-wide boolean; the handler stores it
        // zero-extended to 32 bits.
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
