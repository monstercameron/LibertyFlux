// original: 0x00C11300 streaming_request_release_asset_reference

/// Release an asset reference for an inactive request. The helper is skipped
/// when byte +1 is nonzero or the asset id at +8 is u32::MAX; otherwise it is
/// called with selector 1 and the asset id.
lf_checker_rt::export!(thiscall, rw_00c11300(this: u32) -> u32 {
    unsafe {
        const RELEASE_ID: u32 = 1;
        const NO_ASSET: u32 = u32::MAX;
        let active = ((this + 1) as *const u8).read();
        let asset_id = ((this + 8) as *const u32).read_unaligned();
        if active == 0 && asset_id != NO_ASSET {
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_ID, u32, 1u32, asset_id);
        }
    }
    0
});
