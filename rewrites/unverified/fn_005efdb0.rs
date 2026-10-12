// original: 0x005EFDB0 CEventAcquaintancePedHate::vf21

/// Clone the event through the shared pool, seeding from word 0x18.
///
/// Allocates a fresh object from the shared pool; a failed allocation
/// returns null. Otherwise initialises it with the seed word at `this + 0x18`,
/// installs the vtable and returns the new object. The original is thiscall
/// with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005efdb0(this: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x016669C8;
        const VTABLE: u32 = 0x00E98734;
        const ALLOC_CALLEE: u32 = 1;
        const INIT_CALLEE: u32 = 2;
        const SEED_OFFSET: u32 = 0x18;
        let pool =
            (lf_checker_rt::global::<u32>(POOL_GLOBAL)).read_unaligned();
        let fresh = lf_checker_rt::callee_thiscall!(ALLOC_CALLEE, u32, pool);
        if fresh == 0 {
            return 0;
        }
        let seed =
            (this.wrapping_add(SEED_OFFSET) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, fresh, seed);
        (fresh as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        fresh
    }
});
