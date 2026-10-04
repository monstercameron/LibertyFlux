// original: 0x0086e8b0 SETTIMERA
/// Script native `SETTIMERA` (hash 0x32501B1E).
///
/// Makes no engine call: reads a script-timer pointer from its global,
/// stores the single script argument into the timer structure at offset
/// 0x1c, and leaves the pointer itself in EAX.
export!(cdecl, rw_0086e8b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let timer = *global::<u32>(0x01BB54DC);
        *((timer + 0x1c) as *mut u32) = *args;
        timer
    }
});
