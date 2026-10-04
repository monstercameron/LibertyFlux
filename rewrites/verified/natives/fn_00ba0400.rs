// original: 0x00ba0400 LOCATE_CHAR_ANY_MEANS_CAR_2D
/// Script native `LOCATE_CHAR_ANY_MEANS_CAR_2D` (hash 0x1A455E51).
///
/// Forwards a character handle, a vehicle handle, two coordinate words
/// (copied bit-for-bit, floating point in meaning) and a stack-slot
/// coerced boolean flag, then stores the low byte of the engine answer
/// (zero-extended) into the return slot. Returns the slot pointer.
export!(cdecl, rw_00ba0400(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(4) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
