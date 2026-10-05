// original: 0x008701E0 crmt_sync_array_init (proposed)

/// Initialise the entry array and create the synchronisation handles at `this`.
///
/// Clears the words at `this` + 4 and +8, runs the entry initialiser
/// (callee 1) over 256 slots of 44 bytes from `this` + 0xC (the counter runs
/// 255 down to 0 compared as signed), runs the pair creator (callee 2) on
/// the sub-object at `this` + 0x2C10, then creates the handles: a mutex
/// stored at +0x3C30, two semaphores with maximum 0x7FFF stored at +0x3C34
/// and +0x3C38 with the counters at +0x3C2C and +0x3C3C cleared, and a final
/// mutex stored at +0. Returns `this`.
///
/// Original: 0x008701E0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008701e0(this: u32) -> u32 {
    const ENTRY_INIT: u32 = 1;
    const PAIR_CREATE: u32 = 2;
    const MUTEX_SLOT: u32 = 0xe73180;
    const SEMAPHORE_SLOT: u32 = 0xe73194;
    const ENTRY_COUNT: u32 = 256;
    const ENTRY_STRIDE: u32 = 0x2c;
    unsafe {
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 8) as *mut u32).write_unaligned(0);
        let mut slot = this + 0x0c;
        let mut remaining = ENTRY_COUNT;
        while remaining != 0 {
            lf_checker_rt::callee_thiscall!(ENTRY_INIT, u32, slot);
            slot += ENTRY_STRIDE;
            remaining -= 1;
        }
        lf_checker_rt::callee_thiscall!(PAIR_CREATE, u32, this + 0x2c10);
        let create_mutex = (lf_checker_rt::global::<u32>(MUTEX_SLOT).read_unaligned()) as usize;
        let create_mutex: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(create_mutex);
        let create_semaphore =
            (lf_checker_rt::global::<u32>(SEMAPHORE_SLOT).read_unaligned()) as usize;
        let create_semaphore: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(create_semaphore);
        let first = create_mutex(0, 0, 0);
        ((this + 0x3c30) as *mut u32).write_unaligned(first);
        ((this + 0x3c2c) as *mut u32).write_unaligned(0);
        let second = create_semaphore(0, 0, 0x7fff, 0);
        ((this + 0x3c34) as *mut u32).write_unaligned(second);
        let third = create_semaphore(0, 0, 0x7fff, 0);
        ((this + 0x3c38) as *mut u32).write_unaligned(third);
        ((this + 0x3c3c) as *mut u32).write_unaligned(0);
        let last = create_mutex(0, 0, 0);
        (this as *mut u32).write_unaligned(last);
        this
    }
});
