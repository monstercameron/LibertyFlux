// original: 0x00b93ea0 ATAN2
/// Script native `ATAN2` (hash 0x10A1449C).
///
/// Forwards two script arguments (the y and x operands, as float
/// bit-patterns) to the engine and stores its x87 floating-point answer
/// into the return slot.
export!(cdecl, rw_00b93ea0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = answer;
        slot as u32
    }
});
