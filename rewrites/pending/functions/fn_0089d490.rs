// original: 0x0089d490 guarded_audio_forward
/// Guarded forward: when the global flag is zero return zero, otherwise call
/// the audio helper (thiscall/1) with the fixed audio object and the flag.
export!(cdecl, rw_0089d490() -> u32 {
    unsafe {
        const FLAG: u32 = 0x115F824;
        const AUDIO_OBJ: u32 = 0x115D9A0;
        let flag: u32 = *global::<u32>(FLAG);
        if flag == 0 {
            return 0;
        }
        let target: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        target(relocated(AUDIO_OBJ), flag)
    }
});
