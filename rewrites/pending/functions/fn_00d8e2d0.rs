// original: 0x00d8e2d0 audio_singleton_alloc
/// Allocate the 16-byte shared audio object and publish it globally.
///
/// On allocation failure the global slot is cleared instead. Returns the
/// constructor's answer, or zero when nothing was allocated.
lf_rs89_rt::export!(cdecl, rw_00d8e2d0() -> u32 {
    unsafe {
        let slot = lf_rs89_rt::global::<u32>(0x179F934);
        let block: u32 = lf_rs89_rt::callee_cdecl!(1, u32, 0x10);
        if block == 0 {
            *slot = 0;
            0
        } else {
            let built: u32 = lf_rs89_rt::callee_thiscall!(2, u32, block);
            *slot = built;
            built
        }
    }
});
