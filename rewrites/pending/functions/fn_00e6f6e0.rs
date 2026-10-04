// original: 0x00E6F6E0 timer_table_notify_2490
/// Notify the shared timer hook about block 0x2490.
///
/// Forwards the block address to the engine hook held in the shared dispatch
/// slot and returns the hook's answer.
export!(cdecl, rw_00e6f6e0() -> u32 {
    unsafe {
        const DISPATCH_SLOT: u32 = 0x00E731D0;
        const BLOCK: u32 = 0x019F2490;
        let target = *global::<u32>(DISPATCH_SLOT);
        let hook: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        hook(relocated(BLOCK))
    }
});
