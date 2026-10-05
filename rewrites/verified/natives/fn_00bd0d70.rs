// original: 0x00bd0d70 CLEAR_OBJECT_LAST_WEAPON_DAMAGE
/// Script native `CLEAR_OBJECT_LAST_WEAPON_DAMAGE` (hash 0x15F11BAB).
///
/// Forwards one script argument (an object handle) to the engine.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
/// No return slot is written.
export!(cdecl, rw_00bd0d70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
