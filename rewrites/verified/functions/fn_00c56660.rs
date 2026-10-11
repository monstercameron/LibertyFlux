// original: 0x00c56660 CEventSeenCop::~CEventSeenCop
/// Allocate a CEventSeenCop object through the manager, return zero on allocation failure, initialize its CEventAcquaintancePed base from the source value at +0x18, install the derived vtable, and return the fresh object. This thiscall has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c56660(source: u32) -> u32 {
    const MANAGER_GLOBAL_VA: u32 = 0x016669c8;
    const SOURCE_REFERENCE: u32 = 0x18;
    const ALLOCATE_EVENT: u32 = 1;
    const INITIALIZE_BASE: u32 = 2;
    unsafe {
        let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA));
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOCATE_EVENT, u32, manager);
        if fresh == 0 {
            return 0;
        }
        let source_reference = core::ptr::read_unaligned(source.wrapping_add(SOURCE_REFERENCE) as *const u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(INITIALIZE_BASE, u32, fresh, source_reference);
        core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(0x00ecace4));
        fresh
    }
});
