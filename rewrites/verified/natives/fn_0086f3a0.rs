// original: 0x0086f3a0 SHIFT_RIGHT
/// Script native `SHIFT_RIGHT` (hash 0x64DD173C).
///
/// Pure handler with no engine call: arithmetic-shifts the first script
/// argument right by the second (x86 `sar` semantics: the count is the low
/// 5 bits) and stores the result into the return slot.
export!(cdecl, rw_0086f3a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let value = *args as i32;
        let count = *args.add(1);
        *slot = value.wrapping_shr(count & 31) as u32;
        slot as u32
    }
});
