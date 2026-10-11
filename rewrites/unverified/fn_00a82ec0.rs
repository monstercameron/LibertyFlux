// original: 0x00A82EC0 task_registry_initialize

/// Create the task registry and publish the builder's result.
///
/// The allocator is asked for `0x1C` bytes. If it returns null, the registry
/// global at file VA `0x012FB1AC` is set to zero. Otherwise a thiscall builder
/// receives size `0x460`, the relocated descriptor at file VA `0x00EA03C8`,
/// and capacity `0x50`; its return is written to the registry global.
lf_checker_rt::export!(cdecl, rw_00a82ec0() -> u32 {
    const ALLOCATOR: u32 = 1;
    const REGISTRY_BUILDER: u32 = 2;
    const REGISTRY_OBJECT_SIZE: u32 = 0x1C;
    const REGISTRY_TABLE_SIZE: u32 = 0x460;
    const REGISTRY_CAPACITY: u32 = 0x50;
    const REGISTRY_DESCRIPTOR: u32 = 0x00EA03C8;
    const REGISTRY_GLOBAL: u32 = 0x012FB1AC;

    let allocation = lf_checker_rt::callee_cdecl!(ALLOCATOR, u32, REGISTRY_OBJECT_SIZE);
    let registry = if allocation == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(
            REGISTRY_BUILDER,
            u32,
            allocation,
            REGISTRY_TABLE_SIZE,
            lf_checker_rt::relocated(REGISTRY_DESCRIPTOR),
            REGISTRY_CAPACITY,
        )
    };
    unsafe { lf_checker_rt::global::<u32>(REGISTRY_GLOBAL).write(registry) };
    0
});
