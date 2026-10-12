// original: 0x0063B880 rage::ptxModel::vf23

/// Access the drawable-pool span through the nested pool pointer at `this + 0x170`. The pool layout stores the start word at `+0x6c` and end word at `+0x70`; the returned difference uses wrapping 32-bit arithmetic.
lf_checker_rt::export!(thiscall, rw_0063b880(this: *mut u8) -> u32 {
    unsafe {
        const DRAWABLE_POOL_PTR: usize = 0x170;
        const POOL_START: usize = 0x6C;
        const POOL_END: usize = 0x70;
        let pool_address = (this.add(DRAWABLE_POOL_PTR) as *const u32).read_unaligned();
        let pool = pool_address as usize as *const u8;
        let start = (pool.add(POOL_START) as *const u32).read_unaligned();
        let end = (pool.add(POOL_END) as *const u32).read_unaligned();
        end.wrapping_sub(start)
    }
});
