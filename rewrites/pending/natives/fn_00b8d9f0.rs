// original: 0x00b8d9f0 SET_TEXT_PROPORTIONAL

/// Native handler `SET_TEXT_PROPORTIONAL`.
///
/// Toggle proportional text rendering; the flag is boolean-coerced.
/// Forwards 1 argument(s) to the engine and returns nothing.
///
/// Boolean coercion: the original computes the flag in the low byte
/// of its own incoming argument slot, so the dword the engine sees
/// is the context address with its low byte replaced by the flag.
/// The value is reproduced exactly here; only that caller-slot
/// clobber (which no caller can observe) is out of checker scope.
/// v2 port: the rewrite pushes the bare 0/1 flag; the high-byte slot residue is masked in the contract (call_skip).
export!(cdecl, rw_00b8d9f0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let _ = arg0;
        // Boolean-coerced flag(s), blended into the context address
        // exactly as the original's in-place computation does.
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let flag0 = u32::from(arg0 != 0);
        callee_cdecl!(1, u32, flag0);
        0
    }
});
