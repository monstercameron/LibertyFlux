// original: 0x00AD23F0 audio_chan_preset_c (proposed)

/// Apply audio channel preset C: four (channel, value) pairs through the channel-parameter helper.
///
/// Each pair is sent as (channel, value) to the game's channel-parameter
/// helper (cdecl, two words); several values are also mirrored into the
/// preset's global words. Takes no arguments (cdecl/0); returns the last
/// helper answer.
lf_checker_rt::export!(cdecl, rw_00ad23f0() -> u32 {
    unsafe {
        const SET_PARAM: u32 = 1;
        let mut r: u32;
        lf_checker_rt::global::<u32>(0x0106B354).write(7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0xFu32, 7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0xFu32, 7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x8u32, 0u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x7u32, 1u32);
        r
    }
});
