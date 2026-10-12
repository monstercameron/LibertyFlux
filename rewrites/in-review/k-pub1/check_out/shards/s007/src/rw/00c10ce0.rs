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

fn run_00c10ce0(this: u32, request_index: u32, call_on_null_handle: bool) {
    unsafe {
    const LOOKUP_ID: u32 = 1;
    const METADATA_ID: u32 = 2;
    const NO_ASSET: u32 = u32::MAX;

    let state = ((this + 1) as *const u8).read();
    let asset_id = ((this + 0x58) as *const u32).read_unaligned();
    if state != 0 || asset_id == NO_ASSET {
        return;
    }

    let handle = __lf_callee_cdecl!(LOOKUP_ID, u32, 1u32, asset_id);
    if handle != 0 || call_on_null_handle {
        let record_id = ((this + 8) as *const u32).read_unaligned();
        let record_fields = this + 0x0c;
        let field_4c = ((this + 0x4c) as *const u32).read_unaligned();
        let low_mode = ((this + 3) as *const u8).read() as u32;
        let class = ((this + 2) as *const u8).read() as u32;
        let _ = __lf_callee_thiscall!(
            METADATA_ID,
            u32,
            handle.wrapping_add(0x570),
            record_id,
            record_fields,
            field_4c,
            low_mode,
            class,
            request_index
        );
    }
    }
}

    const LOOKUP_ID: u32 = 1;

    const METADATA_ID: u32 = 2;

    const NO_ASSET: u32 = u32::MAX;
