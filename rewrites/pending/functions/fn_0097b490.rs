// original: 0x0097b490 audio_channel_classify
/// Classify an audio channel id into a routing bucket 0..3.
///
/// Ids 6 and 12 route to bucket 2, ids 7 and 13 to bucket 3; any other id
/// above 16, and any odd id, routes to bucket 1, while the remaining even
/// ids route to bucket 0.
export!(stdcall, rw_0097b490(arg: u32) -> u32 {
    if arg > 0x10 {
        return 1;
    }
    match arg {
        6 | 12 => 2,
        7 | 13 => 3,
        _ => arg & 1,
    }
});
