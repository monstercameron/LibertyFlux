// original: 0x00bb28a0 SET_ALL_RANDOM_PEDS_FLEE
/// Native handler `SET_ALL_RANDOM_PEDS_FLEE`: forwards the handle in arg0 and a
/// boolean-coerced arg1 (`(arg1 != 0) as u32`) to its engine function.
/// Note: the original builds the boolean with `setne` into the low byte
/// of its own incoming stack slot, so the dword it pushes also carries
/// the high bytes of the ctx pointer; the same dword is reproduced here
/// explicitly. The dead stack-slot clobber itself is unobservable to
/// real callers (cdecl cleanup) and is not compared (stack check off).
export!(cdecl, rw_00bb28a0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        let flag = if *args.add(1) != 0 { 1u32 } else { 0 };
        let quirky = (ctx as u32 & 0xFFFFFF00) | flag;
        callee_cdecl!(1, u32, *args, quirky);
        0
    }
});
