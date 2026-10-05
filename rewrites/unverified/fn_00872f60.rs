// original: 0x00872F60 crmt_global_pool_lazy_init

/// Lazily initialise the global block pool (base at 0x1b4af2c, count at +0x30, capacity at +0x32): when capacity is zero, set capacity to 0x17, allocate a 0x5c-byte block through the thread manager, set count to 0x17 and store the block. When capacity is already set, just reset count to 0x17. Returns the block, or 0x17 on the already-initialised path.
///
/// Original: 0x00872F60 (stdcall, one ignored stack word).
lf_checker_rt::export!(stdcall, rw_00872f60(_arg: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const ALLOC_SLOT: u32 = 8;
    const POOL_BASE: u32 = 0x001b4af2c;
    const POOL_COUNT: u32 = 0x001b4af30;
    const POOL_CAP: u32 = 0x001b4af32;
    const INIT_TAG: u16 = 0x17;
    const BLOCK_SIZE: u32 = 0x5c;
    unsafe {
        let cap = lf_checker_rt::global::<u16>(POOL_CAP).read_unaligned();
        if cap != 0 {
            lf_checker_rt::global::<u16>(POOL_COUNT).write_unaligned(INIT_TAG);
            return INIT_TAG as u32;
        }
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        lf_checker_rt::global::<u16>(POOL_CAP).write_unaligned(INIT_TAG);
        let fresh = alloc(manager, BLOCK_SIZE, 0x10, 0);
        lf_checker_rt::global::<u16>(POOL_COUNT).write_unaligned(INIT_TAG);
        lf_checker_rt::global::<u32>(POOL_BASE).write_unaligned(fresh);
        fresh
    }
});
