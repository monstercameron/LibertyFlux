// original: 0x005e6d80 SET_MOBILE_PHONE_SCALE
/// Script native `SET_MOBILE_PHONE_SCALE` (hash 0x61C921EF).
///
/// Stores its single float script argument (a phone UI scale factor) as raw
/// bits into the engine's scale global. No engine call is made and no return
/// slot is written.
export!(cdecl, rw_005e6d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        *lf_rn35_rt::global::<u32>(0x018B6EEC) = *args;
        args as u32
    }
});
