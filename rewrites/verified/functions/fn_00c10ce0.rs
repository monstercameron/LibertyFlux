// original: 0x00C10CE0 streaming_request_finalize_asset_metadata

/// Resolve an inactive metadata request and populate the referenced asset
/// handler only when the lookup returns a nonzero handle. The second callee
/// receives the record id, pointer to fields at +0x0c, a dword at +0x4c,
/// byte fields +3 and +2, and the request index. The handler takes six stack
/// arguments and cleans them itself.
lf_checker_rt::export!(thiscall, rw_00c10ce0(this: u32, request_index: u32) -> u32 {
    run_00c10ce0(this, request_index, false);
    0
});
