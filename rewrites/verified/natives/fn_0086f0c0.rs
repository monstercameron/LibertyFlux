// original: 0x0086f0c0 PRINTSTRING
/// Script native `PRINTSTRING` (hash 0x616F492C).
///
/// Forwards one script argument (a string pointer) to the debug-print
/// routine held in the engine's data-table slot, calling it through that
/// slot exactly like the original. No return slot is written.
export!(cdecl, rw_0086f0c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let target = *global::<u32>(0x110B720);
        let print: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        print(*args)
    }
});
