// original: 0x00981610 audio_pool_alloc_zero_360
/// Allocate `n` records of 0x360 bytes and zero per-record header fields.
///
/// Allocates through the game's allocator (cdecl/1, stubbed by the checker).
/// For non-positive `n` the block is returned uninitialized. Otherwise each
/// record's words at +0x0/+0x30/+0x34/+0x40/+0x44, sixteen words from +0x74
/// with stride 0x30, and the byte at +0x350 are zeroed. Records are skipped
/// when the computed base is null. Returns the block pointer.
export!(stdcall, rw_00981610(n: i32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x360;
        let blk = callee_cdecl!(1, u32, (n as u32).wrapping_mul(STRIDE));
        if n > 0 {
            let mut i = 0u32;
            while i < n as u32 {
                let base = blk.wrapping_add(i.wrapping_mul(STRIDE));
                if base != 0 {
                    let w = |off: u32| (base.wrapping_add(off)) as *mut u32;
                    *w(0x0) = 0;
                    *w(0x30) = 0;
                    *w(0x34) = 0;
                    *w(0x40) = 0;
                    *w(0x44) = 0;
                    let mut k = 0u32;
                    while k < 16 {
                        *w(0x74 + k * 0x30) = 0;
                        k += 1;
                    }
                    *((base.wrapping_add(0x350)) as *mut u8) = 0;
                }
                i += 1;
            }
        }
        blk
    }
});
