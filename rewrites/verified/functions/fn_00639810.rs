// original: 0x00639810 rage::ptxRule::~ptxRule

/// Replace the object's vtable with the rule vtable, release each non-null
/// resource pointer at offsets `+8` and `+0x124` through interface slot `+0xC`
/// from TLS slot zero, then install the destructor vtable. When flag bit zero
/// is set, make the same interface call once more with `this`. The object is
/// in ECX; the flags word is the one stack argument, removed by the callee.
lf_checker_rt::export!(thiscall, rw_00639810(this: u32, flags: u32) -> u32 {
    const RELEASE_RESOURCE: u32 = 1;
    const TLS_REGISTRY_SLOT: usize = 0;
    const FIRST_RESOURCE: u32 = 8;
    const SECOND_RESOURCE: u32 = 0x124;
    const REGISTRY_INTERFACE: u32 = 8;
    const RULE_VTABLE: u32 = 0x00FE279C;
    const DESTRUCTOR_VTABLE: u32 = 0x00E86AFC;

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(RULE_VTABLE));

        let first_resource = (this.wrapping_add(FIRST_RESOURCE) as *const u32).read_unaligned();
        if first_resource != 0 {
            let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
            let interface = (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32)
                .read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE, u32, interface, first_resource);
        }

        let second_resource = (this.wrapping_add(SECOND_RESOURCE) as *const u32).read_unaligned();
        if second_resource != 0 {
            let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
            let interface = (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32)
                .read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE, u32, interface, second_resource);
        }

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(DESTRUCTOR_VTABLE));

        if flags & 1 != 0 {
            let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
            let interface = (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32)
                .read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_RESOURCE, u32, interface, this);
        }

        this
    }
});
