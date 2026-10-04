// original: 0x00b8bb00 ADD_BLIP_FOR_RADIUS
/// Script native handler `ADD_BLIP_FOR_RADIUS` (hash 0x21804D1A).
///
/// Forwards script arguments 0..4 to the engine worker and returns its answer.
/// Floating-point arguments are forwarded as raw bits, so they match bit-exactly.
export!(cdecl, rw_00b8bb00(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1, a2, a3, a4);
        answer
    }
});
