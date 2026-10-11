// original: 0x00CC82B0 initialize_physics_resource_slot_2

/// Initializes one physics resource slot. It requests a 0x1c-byte allocation;
/// if allocation fails, it stores zero. Otherwise it calls the setup helper
/// with the allocator result in ECX and this slot's three constant arguments,
/// then stores the helper's return value in the relocated global word.
/// Both callees are scripted stand-ins, and the global is compared as one word.
lf_checker_rt::export!(cdecl, rw_00cc82b0() -> () {
    const ALLOCATE: u32 = 1;
    const SETUP: u32 = 2;
    const GLOBAL_SLOT: u32 = 0x0171c128;
    const ALLOCATION_SIZE: u32 = 0x1c;
    const SETUP_ARG0: u32 = 0x1298;
    const SETUP_ARG1_FILE_VA: u32 = 0xed95d4;
    const SETUP_ARG2: u32 = 0x60;

    let allocation = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, ALLOCATION_SIZE);
    let value = if allocation == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(
            SETUP,
            u32,
            allocation,
            SETUP_ARG0,
            lf_checker_rt::relocated(SETUP_ARG1_FILE_VA),
            SETUP_ARG2
        )
    };
    unsafe {
        lf_checker_rt::global::<u32>(GLOBAL_SLOT).write_unaligned(value);
    }
});
