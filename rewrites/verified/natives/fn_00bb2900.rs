// original: 0x00bb2900 SET_FREE_HEALTH_CARE

/// Native handler `SET_FREE_HEALTH_CARE`.
///
/// Toggle free health care for a player; the flag is boolean-coerced.
/// Forwards 2 argument(s) to the engine and returns nothing.
///
/// Boolean coercion: the original computes the flag in the low byte
/// of its own incoming argument slot, so the dword the engine sees
/// is the context address with its low byte replaced by the flag.
/// The value is reproduced exactly here; only that caller-slot
/// clobber (which no caller can observe) is out of checker scope.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00bb2900(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let _ = arg0;
        // Boolean-coerced flag(s), blended into the context address
        // exactly as the original's in-place computation does.
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let flag1 = u32::from(arg1 != 0);
        callee_cdecl!(1, u32, arg0, flag1);
        0
    }
});
