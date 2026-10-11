// original: 0x00C78500 phbound_registry_init (proposed)

/// Allocate the registry context using the fixed allocator block size. When
/// allocation succeeds, initialize it with both caller words and the fixed
/// capacity, then publish the helper's returned pointer in the registry
/// global. A failed allocation publishes null. The routine returns no value.
lf_checker_rt::export!(cdecl, rw_phbound_registry_init(first: u32, second: u32) -> () {
    const ALLOCATE: u32 = 1;
    const INITIALIZE: u32 = 2;
    const ALLOCATOR_BYTES: u32 = 0x1c;
    const REGISTRY_CAPACITY: u32 = 0x40;
    const REGISTRY_TABLE: u32 = 0x016e_c7b8;

    let allocation = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, ALLOCATOR_BYTES);
    let registry = if allocation == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(INITIALIZE, u32, allocation, first, second, REGISTRY_CAPACITY)
    };
    unsafe {
        lf_checker_rt::global::<u32>(REGISTRY_TABLE).write_volatile(registry);
    }
});
