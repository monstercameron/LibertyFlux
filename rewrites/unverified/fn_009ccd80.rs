// original: 0x009ccd80 NativeImpl_GET_PLAYER_HAS_TRACKS

/// Return one when the signed audio track count stored in shared state is greater than two; otherwise return zero.
lf_checker_rt::export!(cdecl, rw_009ccd80() -> al {
    const TRACK_COUNT_VA: u32 = 0x011D7680;
    let track_count = unsafe { lf_checker_rt::global::<i32>(TRACK_COUNT_VA).read_unaligned() };
    u32::from(track_count > 2)
});
