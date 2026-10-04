// original: 0x00E6FBE0 timer_table_notify_8124
/// Notify the shared timer hook about block 0x8124.
///
/// Forwards the block address to the engine hook held in the shared dispatch
/// slot and returns the hook's answer.
export!(cdecl, rw_00e6fbe0() -> u32 {
    unsafe {
        const DISPATCH_SLOT: u32 = 0x00E731D0;
        const BLOCK: u32 = 0x019F8124;
        let target = *global::<u32>(DISPATCH_SLOT);
        let hook: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        hook(relocated(BLOCK))
    }
});
