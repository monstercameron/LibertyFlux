// original: 0x00B22960 interior_portal_pool_alloc
/// Allocate a portal-record pool from `this[0]`, then initialize each
/// 0x60-byte record through the intercepted portal-field initializer. The
/// allocation header and published first-record pointer follow the same
/// overflow and failure rules as the room pool.
lf_checker_rt::export!(thiscall, rw_00b22960(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        unsafe fn run_pool(this: u32, allocator_id: u32, initializer_id: u32, stride: u32) {
            unsafe {
                let count = read_u32(this);
                let (product, product_overflow) = count.overflowing_mul(0x60);
                let (allocation_size, add_overflow) = product.overflowing_add(0x10);
                let allocation_size = if product_overflow || add_overflow { u32::MAX } else { allocation_size };
                write_u32(this.wrapping_add(4), 0);
                let allocation = lf_checker_rt::callee_cdecl!(allocator_id, u32, allocation_size);
                if allocation == 0 {
                    write_u32(this.wrapping_add(8), 0);
                    return;
                }
                write_u32(allocation, count);
                let mut record_index = 0u32;
                if count.wrapping_sub(1) as i32 >= 0 {
                    loop {
                        let record = allocation.wrapping_add(0x10).wrapping_add(record_index.wrapping_mul(stride));
                        let _ = lf_checker_rt::callee_thiscall!(initializer_id, u32, record);
                        record_index = record_index.wrapping_add(1);
                        if record_index >= count { break; }
                    }
                }
                write_u32(this.wrapping_add(8), allocation.wrapping_add(0x10));
            }
        }
        run_pool(this, 1, 2, 0x60);
        0
    }
});
