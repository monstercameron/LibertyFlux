// original: 0x00bd0d50 CLEAR_CAR_LAST_WEAPON_DAMAGE
/// Script native `CLEAR_CAR_LAST_WEAPON_DAMAGE` (hash 0x31102E20).
///
/// Forwards one script argument (a vehicle handle) to the engine. No return
/// slot is written. (The original cleans its one pushed argument with
/// `(an instruction of the original)`; the effect on the stack pointer is identical to the plain
/// cdecl return here.)
export!(cdecl, rw_00bd0d50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
