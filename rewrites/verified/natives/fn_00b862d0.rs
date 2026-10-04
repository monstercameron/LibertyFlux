// original: 0x00b862d0 ALLOCATE_SCRIPT_TO_OBJECT
/// Script native `ALLOCATE_SCRIPT_TO_OBJECT` (hash 0x71C30148).
///
/// Forwards five script arguments (an object handle, integers, and one float
/// bit-pattern) to the engine. Floats are copied as raw bits, so the forward
/// is bit-exact. No return slot is written.
export!(cdecl, rw_00b862d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
