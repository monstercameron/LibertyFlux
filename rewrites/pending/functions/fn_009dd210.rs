// original: 0x009dd210 pool_init_stride_3d0_constructed
// fn_009dd210: pool allocator, stride 0x3d0, constructed elements (thiscall/0).
//
// Same allocation shape as rw_009dcf80, but each element is constructed by a
// (stubbed) thiscall instead of stamped inline. Returns the last construction
// result, or the block pointer when the pool is empty.
export!(thiscall, rw_009dd210(this: *mut u8) -> u32 {
    unsafe {
        let count = *(this as *const u32);
        *(this.add(4) as *mut u32) = 0;
        let block = callee_cdecl!(1, u32, {
            let total = (count as u64) * 0x3D0 + 0x10;
            if total > u32::MAX as u64 { u32::MAX } else { total as u32 }
        });
        if block == 0 {
            *(this.add(8) as *mut u32) = 0;
            return 0;
        }
        *(block as *mut u32) = count;
        let base = block.wrapping_add(0x10);
        let construct: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let mut last = block;
        let mut i = 0u32;
        while i < count {
            last = construct(base.wrapping_add(i.wrapping_mul(0x3D0)));
            i = i.wrapping_add(1);
        }
        *(this.add(8) as *mut u32) = base;
        last
    }
});
