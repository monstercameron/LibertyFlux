// original: 0x00bb88f0 ADD_PED_QUEUE
/// Script native `ADD_PED_QUEUE` (hash 0x4E043F3C).
///
/// Forwards eight script arguments to the engine: five float bit-patterns
/// (position and bounds) followed by three integers. Floats are copied as
/// raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bb88f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
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
            *args.add(7),
        )
    }
});
