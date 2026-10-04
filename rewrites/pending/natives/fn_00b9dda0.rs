// original: 0x00b9dda0 ADD_ARMOUR_TO_CHAR
/// Script native `ADD_ARMOUR_TO_CHAR` (hash 0x1C623537).
///
/// Forwards two script arguments (a character handle and an armour amount)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b9dda0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
