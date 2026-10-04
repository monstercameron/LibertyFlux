// original: 0x00b9e340 BLOCK_CHAR_AMBIENT_ANIMS
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
        let probe = 0u32;
        let slot = (core::ptr::addr_of!(probe) as u32).wrapping_add(RW_00B9E340_K);
        *(slot as *mut u8) = flag;
        let coerced = (ctx as u32 & 0xFFFF_FF00) | flag as u32;
        callee_cdecl!(1, u32, *args, coerced)
    }
});

/// Measured bytes from `probe` to the incoming context slot in rw_00b9e340
/// (probe at frame+0x2c, incoming slot at frame+0x80 in this build).
const RW_00B9E340_K: u32 = 0x54;
