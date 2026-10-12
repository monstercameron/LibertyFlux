// original: 0x00B228F0 interior_room_pool_alloc
/// Allocate a room-record pool from the count at `this+0`, zero `this+4`,
/// store the count in the allocator header, initialize each 0x60-byte record
/// through the intercepted field initializer, and publish the first record
/// at `this+8`. The requested byte count is `count*0x60 + 0x10`, saturated to
/// 0xffffffff when either 32-bit operation overflows. Thiscall with no stack
/// arguments and no declared return value.
lf_checker_rt::export!(thiscall, rw_00b228f0(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        unsafe fn run_pool(this: u32, allocator_id: u32, initializer_id: u32, stride: u32) {
            unsafe {
                const RECORD_BYTES: u32 = 0x60;
                const HEADER_BYTES: u32 = 0x10;
                const COUNT_OFFSET: u32 = 0x00;
                const FLAGS_OFFSET: u32 = 0x04;
                const FIRST_RECORD_OFFSET: u32 = 0x08;
                let count = read_u32(this.wrapping_add(COUNT_OFFSET));
                let (product, product_overflow) = count.overflowing_mul(RECORD_BYTES);
                let (allocation_size, add_overflow) = product.overflowing_add(HEADER_BYTES);
                let allocation_size = if product_overflow || add_overflow { u32::MAX } else { allocation_size };
                write_u32(this.wrapping_add(FLAGS_OFFSET), 0);
                let allocation = lf_checker_rt::callee_cdecl!(allocator_id, u32, allocation_size);
                if allocation == 0 {
                    write_u32(this.wrapping_add(FIRST_RECORD_OFFSET), 0);
                    return;
                }
                write_u32(allocation, count);
                let mut record_index = 0u32;
                let final_signed_index = count.wrapping_sub(1) as i32;
                if final_signed_index >= 0 {
                    loop {
                        let record = allocation.wrapping_add(HEADER_BYTES)
                            .wrapping_add(record_index.wrapping_mul(stride));
                        let _ = lf_checker_rt::callee_thiscall!(initializer_id, u32, record);
                        record_index = record_index.wrapping_add(1);
                        if record_index >= count { break; }
                    }
                }
                write_u32(this.wrapping_add(FIRST_RECORD_OFFSET), allocation.wrapping_add(HEADER_BYTES));
            }
        }
        run_pool(this, 1, 2, 0x60);
        0
    }
});
