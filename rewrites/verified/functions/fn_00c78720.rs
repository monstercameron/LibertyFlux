// original: 0x00C78720 phbound_install_default_resource (proposed)

/// Resolve the default resource key, then register the resolved pointer with
/// three helpers in order. The first key is an image-relative string address;
/// the final helper receives the resource pointer returned by the resolver.
lf_checker_rt::export!(cdecl, rw_phbound_install_default_resource() -> () {
    const RESOURCE_KEY: u32 = 0x00ed_43a4;
    const RESOLVE_KEY: u32 = 1;
    const APPLY_RESOURCE: u32 = 2;
    const INITIALIZE_RESOURCE: u32 = 3;
    const FINISH_RESOURCE: u32 = 4;

    let key = lf_checker_rt::relocated(RESOURCE_KEY);
    let resource = lf_checker_rt::callee_cdecl!(RESOLVE_KEY, u32, key);
    let _ = lf_checker_rt::callee_cdecl!(APPLY_RESOURCE, u32, resource, 0x12);
    let _ = lf_checker_rt::callee_cdecl!(INITIALIZE_RESOURCE, u32, 1);
    let _ = lf_checker_rt::callee_cdecl!(FINISH_RESOURCE, u32, resource);
});
