// original: 0x009dd180 pool_init_stride_70_wide
// fn_009dd180: pool allocator, stride 0x70, wide init (thiscall/0).
//
// Same allocation shape as rw_009dcf80; each element gets its vtable, two
// zero words and a trailing all-ones word. Returns base + 8 + count*stride,
// matching the original's bump pointer (it starts 8 past the element base),
// except for an empty pool, where the original skips the pointer setup and
// returns the block itself.
export!(thiscall, rw_009dd180(this: *mut u8) -> u32 {
    unsafe {
        let count = *(this as *const u32);
        *(this.add(4) as *mut u32) = 0;
        let block = callee_cdecl!(1, u32, {
            let total = (count as u64) * 0x70 + 0x10;
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
            let el = base.wrapping_add(i.wrapping_mul(0x70));
            *(el as *mut u32) = relocated(0xE97CDC);
            *((el.wrapping_add(8)) as *mut u32) = 0;
            *((el.wrapping_add(0x60)) as *mut u32) = 0;
            *((el.wrapping_add(0x64)) as *mut u32) = 0xFFFFFFFF;
            i = i.wrapping_add(1);
        }
        *(this.add(8) as *mut u32) = base;
        if count == 0 {
            block
        } else {
            base.wrapping_add(8).wrapping_add(count.wrapping_mul(0x70))
        }
    }
});
