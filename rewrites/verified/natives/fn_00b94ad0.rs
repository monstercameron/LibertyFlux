// original: 0x00b94ad0 IS_SNIPER_BULLET_IN_AREA
/// Script native `IS_SNIPER_BULLET_IN_AREA` (hash 0x6E435BDE).
///
/// Forwards two corner positions (six floats passed as raw bits) to the
/// engine and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00b94ad0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
