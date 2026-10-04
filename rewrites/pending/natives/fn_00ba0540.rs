// original: 0x00ba0540 LOCATE_CHAR_ANY_MEANS_OBJECT_2D
/// Native handler `LOCATE_CHAR_ANY_MEANS_OBJECT_2D`.
///
/// Test whether a character is within 2D range of an object; the flag argument is coerced to boolean.
/// Forwards 5 argument(s) to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
///
/// Boolean coercion: the original computes the flag in the low byte
/// of its own incoming argument slot, so the dword the engine sees
/// is the context address with its low byte replaced by the flag.
/// The value is reproduced exactly here; only that caller-slot
/// clobber (which no caller can observe) is out of checker scope.
export!(cdecl, rw_00ba0540(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let arg3 = *args.add(3);
        let arg4 = *args.add(4);
        let _ = arg0;
        // Boolean-coerced flag(s), blended into the context address
        // exactly as the original's in-place computation does.
        let flag4 = (ctx as u32 & 0xFFFF_FF00) | u32::from(arg4 != 0);
        // The original keeps only the low byte of the answer.
        let answer = callee_cdecl!(1, u32, arg0, arg1, arg2, arg3, flag4);
        *(*ctx as *mut u32) = answer & 0xFF;
        0
    }
});
