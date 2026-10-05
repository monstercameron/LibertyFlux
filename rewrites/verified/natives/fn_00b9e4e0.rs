// original: 0x00b9e4e0 CLEAR_CHAR_LAST_DAMAGE_BONE
/// Script native `CLEAR_CHAR_LAST_DAMAGE_BONE` (hash 0x1A013092).
///
/// Forwards one script argument (a character handle) to the engine. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b9e4e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
