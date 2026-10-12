// original: 0x00C10D30 streaming_request_submit_asset_stream

/// Resolve the asset id in an inactive stream record. On a nonzero handle,
/// the stream handler receives the record id, a pointer at +0x14, the header
/// dword at +0x10, four stream dwords, and the request index. It consumes
/// eight stack arguments.
lf_checker_rt::export!(thiscall, rw_00c10d30(this: u32, request_index: u32) -> u32 {
    run_00c10d30(this, request_index, false);
    0
});

fn run_00c10d30(this: u32, request_index: u32, call_on_null_handle: bool) {
    unsafe {
    const LOOKUP_ID: u32 = 1;
    const STREAM_ID: u32 = 2;
    const NO_ASSET: u32 = u32::MAX;

    let state = ((this + 1) as *const u8).read();
    let asset_id = ((this + 8) as *const u32).read_unaligned();
    if state != 0 || asset_id == NO_ASSET {
        return;
    }

    let handle = __lf_callee_cdecl!(LOOKUP_ID, u32, 1u32, asset_id);
    if handle != 0 || call_on_null_handle {
        let record_id = ((this + 0x0c) as *const u32).read_unaligned();
        let header = ((this + 0x10) as *const u32).read_unaligned();
        let payload = this + 0x14;
        let stream_114 = ((this + 0x114) as *const u32).read_unaligned();
        let stream_118 = ((this + 0x118) as *const u32).read_unaligned();
        let stream_11c = ((this + 0x11c) as *const u32).read_unaligned();
        let stream_120 = ((this + 0x120) as *const u32).read_unaligned();
        let _ = __lf_callee_thiscall!(
            STREAM_ID,
            u32,
            handle.wrapping_add(0x570),
            record_id,
            payload,
            header,
            stream_114,
            stream_118,
            stream_11c,
            stream_120,
            request_index
        );
    }
    }
}

        const LOOKUP_ID: u32 = 1;

        const NO_ASSET: u32 = u32::MAX;

    const STREAM_ID: u32 = 2;
