// original: 0x009dd280 pool_init_stride_80_b
// fn_009dd280: pool allocator, stride 0x80 (thiscall/0).
//
// Same shape as rw_009dcf80 with stride 0x80 and its own vtable.
export!(thiscall, rw_009dd280(this: *mut u8) -> u32 {
    unsafe {
        let count = *(this as *const u32);
        *(this.add(4) as *mut u32) = 0;
        let block = callee_cdecl!(1, u32, {
            let total = (count as u64) * 0x80 + 0x10;
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
            let el = base.wrapping_add(i.wrapping_mul(0x80));
            *(el as *mut u32) = relocated(0xE97D1C);
            *((el.wrapping_add(8)) as *mut u32) = 0;
            i = i.wrapping_add(1);
        }
        *(this.add(8) as *mut u32) = base;
        base.wrapping_add(count.wrapping_mul(0x80))
    }
});
