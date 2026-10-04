// original: 0x00b8cc40 HIGHLIGHT_MENU_ITEM
/// Script native `HIGHLIGHT_MENU_ITEM` (hash 0x1ABE6A4C).
///
/// Forwards two script arguments plus a boolean flag (`arg != 0`) to the
/// engine menu routine. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching.
export!(cdecl, rw_00b8cc40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(2) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, *args.add(1), quirked)
    }
});
