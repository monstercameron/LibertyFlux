// original: 0x00bb9b40 TASK_GO_TO_COORD_WHILE_AIMING
/// Native handler `TASK_GO_TO_COORD_WHILE_AIMING`: task a character to walk to a point while aiming: forward handle, speeds, coords, target and flags.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00bb9b40(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        // The original coerces this flag with a byte-wide setnz over the
        // incoming context slot, so the pushed dword keeps the slot's high
        // bytes; reproduce that shape exactly.
        let flag11 = (ctx & 0xFFFF_FF00) | u32::from(*args.add(11) != 0);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8), *args.add(9), *args.add(10), flag11,);
        ans
    }
});
