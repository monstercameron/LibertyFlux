// original: 0x00CF2A60 phMaterialMgrGta::create

/// Allocate and publish a material manager, or clear the global on failure.
///
/// The allocator receives the object size `0xa8`. A null allocation clears
/// the manager global and returns zero. A successful allocation is passed to
/// the manager constructor as `this`; its result is stored in the global and
/// returned in EAX.
lf_checker_rt::export!(cdecl, rw_fn_00cf2a60() -> u32 {
    const MANAGER_SIZE: u32 = 0xa8;
    const GLOBAL_MANAGER_VA: u32 = 0x018b_8968;
    const ALLOCATOR_CALLEE: u32 = 1;
    const CONSTRUCTOR_CALLEE: u32 = 2;
    let allocation = lf_checker_rt::callee_cdecl!(ALLOCATOR_CALLEE, u32, MANAGER_SIZE);
    if allocation == 0 {
        unsafe { lf_checker_rt::global::<u32>(GLOBAL_MANAGER_VA).write_unaligned(0); }
        0
    } else {
        let manager = lf_checker_rt::callee_thiscall!(CONSTRUCTOR_CALLEE, u32, allocation);
        unsafe { lf_checker_rt::global::<u32>(GLOBAL_MANAGER_VA).write_unaligned(manager); }
        manager
    }
});
