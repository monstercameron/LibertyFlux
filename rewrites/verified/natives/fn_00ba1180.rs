// original: 0x00ba1180 SET_CHAR_COMPONENT_VARIATION
/// Script native `SET_CHAR_COMPONENT_VARIATION` (hash 0x71A52973).
///
/// Forwards four script arguments (a ped handle, a component slot and two
/// variation selectors) to the engine. No return slot is written.
export!(cdecl, rw_00ba1180(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
