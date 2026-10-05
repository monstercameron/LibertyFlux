// original: 0x00AF8E20 veh_zone_add_ref (proposed)

/// Register this object's zone link with the zone registry.
///
/// Passes the address of the link field at `this + 8` to the registry
/// (callee 1, whose context is the fixed registry object). Nothing is
/// returned.
///
/// Original: 0x00AF8E20 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF8E20(this: u32) -> u32 {
    unsafe {
        const REGISTRY: u32 = 1;
        const REGISTRY_OBJ: u32 = 0x116BFF0;
        const LINK: u32 = 8;
        let registry = lf_checker_rt::relocated(REGISTRY_OBJ);
        lf_checker_rt::callee_thiscall!(REGISTRY, u32, registry, this + LINK);
        0
    }
});
