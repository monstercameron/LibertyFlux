// original: 0x00D5DE30 CPrtSysEntity::vf35

/// Create, initialize, and release an optional entity-side resource.
///
/// The method uses thiscall and consumes four 32-bit stack arguments. Only
/// the first two affect control flow: when the first equals one and the
/// second equals zero, it requests a 12-byte resource. A non-null result is
/// initialized with `this` as its argument; either the initialized result or
/// null is then passed to the release helper. The function returns zero on
/// every path. All helper calls are direct and their arguments and order are
/// observable through the checker call log.
lf_checker_rt::export!(thiscall, rw_cprt_sys_entity_vf35(
    this_ptr: u32,
    selector: u32,
    options: u32,
    unused_argument_3: u32,
    unused_argument_4: u32
) -> u32 {
    const RESOURCE_ALLOCATOR: u32 = 1;
    const RESOURCE_INITIALIZER: u32 = 2;
    const RESOURCE_RELEASE: u32 = 3;
    const RESOURCE_SIZE: u32 = 12;
    const RELEASE_FLAGS: u32 = 0;

    if selector == 1 && options == 0 {
        let resource = lf_checker_rt::callee_cdecl!(
            RESOURCE_ALLOCATOR,
            u32,
            RESOURCE_SIZE,
            RELEASE_FLAGS
        );
        if resource != 0 {
            let initialized = lf_checker_rt::callee_thiscall!(
                RESOURCE_INITIALIZER,
                u32,
                resource,
                this_ptr
            );
            let _ = lf_checker_rt::callee_cdecl!(RESOURCE_RELEASE, u32, initialized);
        } else {
            let _ = lf_checker_rt::callee_cdecl!(RESOURCE_RELEASE, u32, 0u32);
        }
    }
    let _ = (unused_argument_3, unused_argument_4);
    0
});
