// original: 0x00981690 audio_pool_alloc_zero_160
/// Allocate `n` records of 0x160 bytes and zero per-record header fields.
///
/// Same shape as [`rw_00981610`] with a smaller stride: the words at
/// +0x0/+0x4/+0x8/+0xc/+0x10/+0x14/+0x18/+0x2c/+0x30/+0x34/+0x38 of each
/// record are zeroed. Returns the block pointer.
export!(stdcall, rw_00981690(n: i32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x160;
        const OFFS: [u32; 11] = [0x0, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18,
                                 0x2c, 0x30, 0x34, 0x38];
        let blk = callee_cdecl!(1, u32, (n as u32).wrapping_mul(STRIDE));
        if n > 0 {
            let mut i = 0u32;
            while i < n as u32 {
                let base = blk.wrapping_add(i.wrapping_mul(STRIDE));
                if base != 0 {
                    for off in OFFS {
                        *((base.wrapping_add(off)) as *mut u32) = 0;
                    }
                }
                i += 1;
            }
        }
        blk
    }
});
