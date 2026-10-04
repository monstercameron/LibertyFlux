// original: 0x008ABC90 audio_dispatch_first
/// Resolve the handler for a found record and dispatch the trailing
/// argument to it through its slot-1 entry point. Returns the handler,
/// or null when either lookup misses.
export!(stdcall, rw_008ABC90(key: u32, extra: u32) -> u32 {
    unsafe {
        let found: u32 = callee_stdcall!(1, u32, key);
        if found == 0 {
            return 0;
        }
        let handler: u32 = callee_cdecl!(2, u32, *(found as *const u8) as u32);
        if handler == 0 {
            return 0;
        }
        let vtable = *(handler as *const u32);
        let target = *((vtable as *const u8).add(4) as *const u32);
        let dispatch: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        dispatch(handler, found, extra);
        handler
    }
});
