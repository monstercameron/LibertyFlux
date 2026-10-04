// original: 0x00bde3a0 GET_WATER_HEIGHT_NO_WAVES
/// Script native `GET_WATER_HEIGHT_NO_WAVES` (hash 0x67C82864).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (coordinates) and one integer word; then stores the low byte of the
/// engine answer (zero-extended) into the return slot. The original moves
/// the floats through vector registers with stack scratch space, but only
/// copies bit patterns onto the argument stack, so plain forwarding is
/// bit-exact.
export!(cdecl, rw_00bde3a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
