// original: 0x00ba2830 SET_SCRIPTED_CONVERSION_CENTRE
/// Script native `SET_SCRIPTED_CONVERSION_CENTRE` (hash 0x40F61D4A).
///
/// Builds a three-word coordinate vector from the script arguments on the
/// stack and passes its address to the engine in ECX (thiscall with no stack
/// arguments). No return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00ba2830(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let point = [*args, *args.add(1), *args.add(2)];
        callee_thiscall!(1, u32, point.as_ptr() as u32)
    }
});
