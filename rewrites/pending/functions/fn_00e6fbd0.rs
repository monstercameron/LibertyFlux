// original: 0x00E6FBD0 timer_table_notify_923c
/// Notify the shared timer hook about block 0x923c.
///
/// Forwards the block address to the engine hook held in the shared dispatch
/// slot and returns the hook's answer.
export!(cdecl, rw_00e6fbd0() -> u32 {
    unsafe {
        const DISPATCH_SLOT: u32 = 0x00E731D0;
        const BLOCK: u32 = 0x019F923C;
        let target = *global::<u32>(DISPATCH_SLOT);
        let hook: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        hook(relocated(BLOCK))
    }
});
