// original: 0x00637B20 rage::ptxTimeLine::~ptxTimeLine__deleting

/// Run the class-specific destructor helper, then, when the input's low flag
/// bit is set and `this` is non-null, remove its reference through the
/// interface stored in TLS slot zero. The object is in ECX and the flags word
/// is one stack argument. Return the original object pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00637B20(this: u32, flags: u32) -> u32 {
    const DESTROY_OBJECT: u32 = 1;
    const REMOVE_REFERENCE: u32 = 2;
    const TLS_REGISTRY_SLOT: usize = 0;
    const REGISTRY_INTERFACE: u32 = 8;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(DESTROY_OBJECT, u32, this);

        if flags & 1 != 0 && this != 0 {
            let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
            let interface = (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32)
                .read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(REMOVE_REFERENCE, u32, interface, this);
        }

        this
    }
});
