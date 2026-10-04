// original: 0x008ABCD0 audio_dispatch_second
/// Twin of the first dispatcher through the sibling lookup routine.
export!(stdcall, rw_008ABCD0(key: u32, extra: u32) -> u32 {
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
