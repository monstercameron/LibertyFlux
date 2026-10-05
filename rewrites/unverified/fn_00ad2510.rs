// original: 0x00AD2510 audio_chan_preset_e (proposed)

/// Apply audio channel preset E: eleven (channel, value) pairs through the channel-parameter helper.
///
/// Each pair is sent as (channel, value) to the game's channel-parameter
/// helper (cdecl, two words); several values are also mirrored into the
/// preset's global words. Takes no arguments (cdecl/0); returns the last
/// helper answer.
lf_checker_rt::export!(cdecl, rw_00ad2510() -> u32 {
    unsafe {
        const SET_PARAM: u32 = 1;
        let mut r: u32;
        lf_checker_rt::global::<u32>(0x0106B354).write(7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0xFu32, 7u32);
        lf_checker_rt::global::<u32>(0x0106B358).write(7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x10u32, 7u32);
        lf_checker_rt::global::<u32>(0x0106B35C).write(7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x11u32, 7u32);
        lf_checker_rt::global::<u32>(0x0106B360).write(15u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x12u32, 15u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0xFu32, 7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x10u32, 7u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x11u32, 7u32);
        lf_checker_rt::global::<u32>(0x0106B360).write(15u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x12u32, 15u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x8u32, 0u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x7u32, 0u32);
        r = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, 0x2u32, 0u32);
        r
    }
});
