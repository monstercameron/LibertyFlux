// original: 0x005cdd40 pick_loading_screen
/// Query the display mode, then hand off to the loading-screen resolver.
///
/// Calls the widescreen probe with the display-config object and tail-calls
/// the resolver with the probe's answer byte. Only the low byte of the probe
/// answer is meaningful: the original keeps the config address in the upper
/// bytes (an upshot of writing CL after a callee that preserves ECX), but the
/// resolver reads only CL, so this passes the byte zero-extended.
export!(cdecl, rw_005cdd40() -> u32 {
    unsafe {
        const DISPLAY_CONFIG: u32 = 0x0118D7F0;
        let is_widescreen: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(0) as usize);
        let resolve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let flag = is_widescreen(relocated(DISPLAY_CONFIG));
        resolve(flag & 0xFF)
    }
});
