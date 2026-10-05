// original: 0x0062C3E0 rage::VerletWaterPerturbation::vf5 (symbols)

/// Allocate one object through the thread-local allocator and stamp its vtable.
///
/// Reads the allocator holder from TLS slot 0 (via the array at `fs:[0x2c]`), calls its slot
/// `+8` (thiscall: object, `ALLOC_SIZE`, `0x10`, `0`) to allocate `ALLOC_SIZE`
/// bytes, and on success writes the class vtable pointer at the block start.
/// Returns the block, or null when allocation fails (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0062c3e0() -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const ALLOC_SLOT: u32 = 8;
        const ALLOC_SIZE: u32 = 0x8;
        const ALLOC_ALIGN: u32 = 0x10;
        const VTABLE: u32 = 0xFE246C;
        let holder = lf_checker_rt::tls_slot(TLS_SLOT);
        let obj = ((holder + 8) as *const u32).read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let tgt = ((vtable + ALLOC_SLOT) as *const u32).read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let mem = alloc(obj, ALLOC_SIZE, ALLOC_ALIGN, 0);
        if mem == 0 {
            0
        } else {
            (mem as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
            mem
        }
    }
});
