// original: 0x00b8d470 RENDER_RADIOHUD_SPRITE_IN_MOBILE_PHONE
/// Script native `RENDER_RADIOHUD_SPRITE_IN_MOBILE_PHONE` (hash 0x704D5747).
///
/// Forwards eight script arguments to the engine: five float bit-patterns
/// (sprite coordinates), two integers, and one boolean flag coerced with
/// `arg != 0`. Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00b8d470(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(7) != 0);
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            flag,
        )
    }
});
