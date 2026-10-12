// original: 0x0063A880 rage::ptxRule::create

/// Request a rule object through the registry interface in TLS slot zero,
/// using factory arguments `0x140` and `0x10`. A null factory result returns
/// zero. Otherwise initialize the object with the image's relocated
/// `undefined` name pointer and return the constructor's result.
lf_checker_rt::export!(cdecl, rw_0063A880() -> u32 {
    const TLS_REGISTRY_SLOT: usize = 0;
    const REGISTRY_INTERFACE: u32 = 8;
    const FACTORY: u32 = 1;
    const RULE_CONSTRUCTOR: u32 = 2;
    const DEFAULT_NAME: u32 = 0x00F98B64;

    let registry = lf_checker_rt::tls_slot(TLS_REGISTRY_SLOT);
    let interface = unsafe {
        (registry.wrapping_add(REGISTRY_INTERFACE) as *const u32).read_unaligned()
    };
    let object = lf_checker_rt::callee_thiscall!(FACTORY, u32, interface, 0x140u32, 0x10u32, 0u32);
    if object == 0 {
        return 0;
    }

    let default_name = lf_checker_rt::global::<u8>(DEFAULT_NAME) as u32;
    lf_checker_rt::callee_thiscall!(RULE_CONSTRUCTOR, u32, object, default_name)
});
