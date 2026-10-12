// original: 0x00889830 audio_voice_manager_initialize (proposed)

/// Allocate and initialize the global voice manager.
///
/// Allocates 0x38 bytes (patched callee); when that fails the global at
/// 0x0115A520 is set to null. Otherwise the manager is constructed in place
/// (patched callee) with the voice count from the global at 0x010300AC and
/// the "voices" name, and the global is set to it. Returns the manager
/// pointer, or null on allocation failure (the original's eax in each case).
///
/// Original: 0x00889830 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00889830() -> u32 {
    unsafe {
        const ALLOCATOR: u32 = 1;
        const MANAGER_CTOR: u32 = 2;
        const MANAGER_SIZE: u32 = 0x38;
        const MANAGER_SLOT: u32 = 0x0115_a520;
        const COUNT_SLOT: u32 = 0x0103_00ac;
        const VOICES_NAME: u32 = 0x00e7_78a8;

        let mem = lf_checker_rt::callee_cdecl!(ALLOCATOR, u32, MANAGER_SIZE);
        if mem == 0 {
            lf_checker_rt::global::<u32>(MANAGER_SLOT).write(0);
            return 0;
        }
        let count = lf_checker_rt::global::<u32>(COUNT_SLOT).read();
        let mgr =
            lf_checker_rt::callee_thiscall!(MANAGER_CTOR, u32, mem, count, lf_checker_rt::relocated(VOICES_NAME));
        lf_checker_rt::global::<u32>(MANAGER_SLOT).write(mgr);
        mgr
    }
});
