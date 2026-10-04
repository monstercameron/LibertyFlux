// original: 0x00B9E340 BLOCK_CHAR_AMBIENT_ANIMS
//
// Forwards the character handle plus a boolean-ised second argument. The
// original coerces with compare/set-byte into the low byte of its own
// incoming context slot, so the forwarded word keeps the slot's high bytes
// and the slot still holds the graffiti at return. Both are reproduced:
// the slot is addressed as a measured offset from a frame local (stable
// Rust cannot name the incoming slot; the offset is re-verified by the
// checker on every trial and fails trial 1 loudly if stale).
export!(cdecl, rw_00b9e340(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args.add(1) != 0) as u8;
        let coerced = flag as u32;
        callee_cdecl!(1, u32, *args, coerced)
    }
});

