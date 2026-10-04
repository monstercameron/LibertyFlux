// original: 0x0086e8f0 SETTIMERC
/// Script native `SETTIMERC` (hash 0x499852DB).
///
/// Stores the first script argument (the new timer value) into the timer-C
/// field of the current script context. The context pointer is read from a
/// game global; the timer field sits 0x24 bytes into the context. Makes no
/// engine calls. Returns the context pointer it read.
export!(cdecl, rw_0086e8f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        /// File VA of the global holding the current script context.
        const CTX_GLOBAL_FILE_VA: u32 = 0x01BB_54DC;
        /// Byte offset of the timer-C field within the script context.
        const TIMER_C_OFFSET: u32 = 0x24;
        let timer_base = *global::<u32>(CTX_GLOBAL_FILE_VA);
        *((timer_base + TIMER_C_OFFSET) as *mut u32) = *args;
        timer_base
    }
});
