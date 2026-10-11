// original: 0x00C10B50 streaming_request_submit_if_ready

/// Queue one streaming request when the record is eligible. A non-sentinel
/// asset id is resolved by a two-argument lookup; the submitter then receives
/// the id at +0x0c, the payload at +0x10, the resolved handle, a boolean from
/// byte +1, and the original argument. Byte +2 suppresses the whole operation.
lf_checker_rt::export!(thiscall, rw_00c10b50(this: u32, request_index: u32) -> u32 {
    unsafe {
        const LOOKUP_ID: u32 = 1;
        const SUBMIT_ID: u32 = 2;
        const SUBMITTER_VA: u32 = 0x0123_8898;
        const NO_ASSET: u32 = u32::MAX;
        let state = ((this + 2) as *const u8).read();
        if state == 0 {
            let asset_id = ((this + 8) as *const u32).read_unaligned();
            let handle = if asset_id == NO_ASSET {
                0
            } else {
                lf_checker_rt::callee_cdecl!(LOOKUP_ID, u32, 1u32, asset_id)
            };
            let active = (((this + 1) as *const u8).read() != 0) as u32;
            let record_id = ((this + 0x0c) as *const u32).read_unaligned();
            let payload = this + 0x10;
            let _ = lf_checker_rt::callee_thiscall!(
                SUBMIT_ID, u32, lf_checker_rt::relocated(SUBMITTER_VA),
                record_id, payload, handle, active, request_index
            );
        }
    }
    0
});
