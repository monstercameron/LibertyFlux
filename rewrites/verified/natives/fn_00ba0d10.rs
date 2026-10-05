// original: 0x00ba0d10 REMOVE_CHAR_FROM_GROUP
/// Script native `REMOVE_CHAR_FROM_GROUP` (hash 0x649316B7).
///
/// Forwards one script argument (a character handle) to the engine. No
/// return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00ba0d10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
