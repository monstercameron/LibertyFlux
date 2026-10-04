// original: 0x008D3C20 DispatchSharedGlobalCalls
/// Forwards the shared global through two manager calls with their selector
/// globals, then runs the final single-argument call and returns its answer.
export!(cdecl, rw_008D3C20() -> u32 {
    unsafe {
        let shared = *global::<u32>(0x17ed954);
        callee_thiscall!(
            0,
            u32,
            *global::<u32>(0x11736c4),
            *global::<u32>(0x11736dc),
            shared
        );
        callee_thiscall!(
            1,
            u32,
            *global::<u32>(0x11736c4),
            *global::<u32>(0x11736e0),
            shared
        );
        callee_cdecl!(2, u32, *global::<u32>(0x17ed954))
    }
});
