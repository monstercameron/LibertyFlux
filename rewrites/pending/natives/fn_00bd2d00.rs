// original: 0x00bd2d00 GET_MAP_AREA_FROM_COORDS
/// Script native `GET_MAP_AREA_FROM_COORDS` (hash 0x5ED33D46).
///
/// Builds a three-word coordinate vector from the script arguments on the
/// stack and passes its address to the engine in ECX (thiscall with no stack
/// arguments), then stores the full 32-bit answer into the return slot. The
/// answer is the exit value.
export!(cdecl, rw_00bd2d00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let point = [*args, *args.add(1), *args.add(2)];
        let answer = callee_thiscall!(1, u32, point.as_ptr() as u32);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
