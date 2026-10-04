// original: 0x00bb7250 ACTIVATE_INTERIOR
/// Native handler `ACTIVATE_INTERIOR`: activate an interior: forward the interior id and the activate flag (coerced to 0/1).
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bb7250(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        // The original coerces this flag with a byte-wide setnz over the
        // incoming context slot, so the pushed dword keeps the slot's high
        // bytes; reproduce that shape exactly.
        let flag1 = (ctx & 0xFFFF_FF00) | u32::from(*args.add(1) != 0);
        let ans = callee_cdecl!(1, u32, *args.add(0), flag1,);
        ans
    }
});
