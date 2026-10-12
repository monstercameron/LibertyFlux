// original: 0x005EFBB0 CTaskComplexCarDriveMissionFleeScene::vf1

/// Clone the task through the shared pool and the fifteen-argument builder.
///
/// Allocates a fresh object from the shared pool; a failed allocation
/// returns null. Otherwise scans the source range at `this + 0x14`, builds
/// the clone with the value word at `this + 0x40` and the fixed parameter
/// block, installs the vtable and returns the new object. The original is
/// thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005efbb0(this: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x0167E2A0;
        const VTABLE: u32 = 0x00EB357C;
        const ALLOC_CALLEE: u32 = 1;
        const SCAN_CALLEE: u32 = 2;
        const BUILD_CALLEE: u32 = 3;
        const SCAN_OFFSET: u32 = 0x14;
        const VALUE_OFFSET: u32 = 0x40;
        let pool =
            (lf_checker_rt::global::<u32>(POOL_GLOBAL)).read_unaligned();
        let fresh = lf_checker_rt::callee_thiscall!(ALLOC_CALLEE, u32, pool);
        if fresh == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            SCAN_CALLEE, u32, this.wrapping_add(SCAN_OFFSET));
        let value =
            (this.wrapping_add(VALUE_OFFSET) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(BUILD_CALLEE, u32, fresh, value, fresh, 1, 2, 0x28, 0, 0, 0, 0, 4, 0x14, 0xbf800000u32, 0x1e, 0x14, 1);
        (fresh as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        fresh
    }
});
