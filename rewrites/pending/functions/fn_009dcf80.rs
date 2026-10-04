// original: 0x009dcf80 pool_init_stride_60
// fn_009dcf80: pool allocator, stride 0x60 (thiscall/0).
//
// Reads the element count from `this+0`, allocates `count * stride + 16`
// bytes through the game's allocator, and on success stores the count at the
// block head, the element base at `this+8`, and stamps every element with its
// vtable pointer and a zero status word. On allocation failure stores null.
// Returns the end pointer (base + count * stride), or null on failure.
export!(thiscall, rw_009dcf80(this: *mut u8) -> u32 {
    unsafe {
        let count = *(this as *const u32);
        *(this.add(4) as *mut u32) = 0;
        let block = callee_cdecl!(1, u32, {
            let total = (count as u64) * 0x60 + 0x10;
            if total > u32::MAX as u64 { u32::MAX } else { total as u32 }
        });
        if block == 0 {
            *(this.add(8) as *mut u32) = 0;
            return 0;
        }
        *(block as *mut u32) = count;
        let base = block.wrapping_add(0x10);
        let mut i = 0u32;
        while i < count {
            let el = base.wrapping_add(i.wrapping_mul(0x60));
            *(el as *mut u32) = relocated(0xE97B9C);
            *((el.wrapping_add(8)) as *mut u32) = 0;
            i = i.wrapping_add(1);
        }
        *(this.add(8) as *mut u32) = base;
        base.wrapping_add(count.wrapping_mul(0x60))
    }
});
