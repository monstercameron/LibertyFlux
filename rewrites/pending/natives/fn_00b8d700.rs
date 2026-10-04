// original: 0x00b8d700 SET_MENU_ITEM_WITH_2_NUMBERS
/// Script native `SET_MENU_ITEM_WITH_2_NUMBERS` (hash 0x7C4E54ED).
///
/// Forwards six script arguments (menu text ids and two numbers) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8d700(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5),)
    }
});
