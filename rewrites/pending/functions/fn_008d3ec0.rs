// original: 0x008D3EC0 TeardownManagerGlobals
/// Tears down the globals-held objects (releasing each live one through its
/// own path and clearing its slot), re-emits the two manager calls, then
/// releases the manager pair itself when live. Returns 1.
export!(cdecl, rw_008D3EC0() -> u8 {
    unsafe {
        let a = *global::<u32>(0x11736f0);
        if a != 0 {
            let vtable = *(a as *const u32);
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*(vtable as *const u32));
            free(a, 1);
            *global::<u32>(0x11736f0) = 0;
        }
        let e = *global::<u32>(0x11736ec);
        if e != 0 {
            callee_cdecl!(1, u32, e);
            *global::<u32>(0x11736ec) = 0;
        }
        let mgr = *global::<u32>(0x11736c4);
        callee_thiscall!(2, u32, mgr, *global::<u32>(0x11736dc), 0);
        callee_thiscall!(3, u32, *global::<u32>(0x11736c4), *global::<u32>(0x11736e0), 0);
        let mgr2 = *global::<u32>(0x11736c4);
        if mgr2 != 0 {
            let vtable = *(mgr2 as *const u32);
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(*(vtable as *const u32));
            free(mgr2, 1);
            *global::<u32>(0x11736c4) = 0;
            *global::<u32>(0x11736c8) = 0;
        }
        1
    }
});
