// original: 0x00C1FAA0 streaming_lookup_named_entry

/// Resolve the fixed streaming label and pass the resulting handle to the
/// second lookup routine. Both collaborators receive one cdecl argument; the
/// second collaborator's EAX result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_00c1faa0() -> u32 {
    unsafe {
        const LABEL_VA: u32 = 0x00EC53A0;
        let label_handle = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            lf_checker_rt::relocated(LABEL_VA),
        );
        lf_checker_rt::callee_cdecl!(2, u32, label_handle)
    }
});
