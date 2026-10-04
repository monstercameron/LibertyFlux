// original: 0x008aae10 audio_global_notify
/// Notify the audio manager with the global voice handle.
///
/// Passes the dword at the global `0x0115DEA0` plus the constants 1 and 0xc
/// to the helper (stdcall/3, stubbed) and returns its answer.
export!(cdecl, rw_008aae10() -> u32 {
    unsafe {
        let handle = *global::<u32>(0x0115DEA0);
        callee_stdcall!(1, u32, handle, 1, 0xC)
    }
});
