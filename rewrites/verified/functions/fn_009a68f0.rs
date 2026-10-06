// original: 0x009A68F0 audio_reset_state (proposed)

/// Releases the audio handle when present and resets the state block.
///
/// No arguments. When the handle global at `HANDLE` is non-zero it is
/// released through callee 1 (thiscall on the handle, one zero word);
/// then five state dwords are reset (`STATE0` to 0, the next three to -1,
/// `STATE4` to 0) and 1 is returned in the low byte. The upper return
/// bytes are caller leftovers, so the contract compares the low byte only.
lf_checker_rt::export!(cdecl, rw_009a68f0() -> u32 {
    unsafe {
        const RELEASER: u32 = 1;
        const HANDLE: u32 = 0x001288524;
        const STATE0: u32 = 0x001288528;
        const STATE1: u32 = 0x001288530;
        const STATE2: u32 = 0x001288534;
        const STATE3: u32 = 0x001288538;
        const STATE4: u32 = 0x00128853C;
        let handle = (lf_checker_rt::global::<u32>(HANDLE)).read_unaligned();
        if handle != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(RELEASER, u32, handle, 0);
        }
        lf_checker_rt::global::<u32>(STATE0).write_unaligned(0);
        lf_checker_rt::global::<u32>(STATE1).write_unaligned(0xFFFF_FFFF);
        lf_checker_rt::global::<u32>(STATE2).write_unaligned(0xFFFF_FFFF);
        lf_checker_rt::global::<u32>(STATE3).write_unaligned(0xFFFF_FFFF);
        lf_checker_rt::global::<u32>(STATE4).write_unaligned(0);
    }
    1
});
