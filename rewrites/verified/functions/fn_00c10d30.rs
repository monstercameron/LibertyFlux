// original: 0x00C10D30 streaming_request_submit_asset_stream

/// Resolve the asset id in an inactive stream record. On a nonzero handle,
/// the stream handler receives the record id, a pointer at +0x14, the header
/// dword at +0x10, four stream dwords, and the request index. It consumes
/// eight stack arguments.
lf_checker_rt::export!(thiscall, rw_00c10d30(this: u32, request_index: u32) -> u32 {
    run_00c10d30(this, request_index, false);
    0
});
