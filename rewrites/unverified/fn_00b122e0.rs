// original: 0x00B122E0 decrement_indexed_resource_reference

/// Decrement the count at offset four in the resource record selected by the
/// signed 32-bit index and manager stride. A high bit in the indexed flag is
/// an invalid sentinel: the original faults while attempting the same null
/// record update, which this implementation preserves. The manager pointer is
/// read from a relocated global; this is a cdecl function with one argument.
lf_checker_rt::export!(cdecl, rw_00b122e0(index: u32) -> u32 {
    const MANAGER_VA: u32 = 0x016F_B694;

    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let manager_slot = lf_checker_rt::global::<u32>(MANAGER_VA) as u32;
        let manager = read_u32(manager_slot);
        let flags = read_u32(manager.wrapping_add(4));
        if read_u8(flags.wrapping_add(index)) & 0x80 != 0 {
            let invalid_count = 4usize as *mut u32;
            let value = invalid_count.read_volatile();
            invalid_count.write_volatile(value.wrapping_sub(1));
            return 0;
        }
        let stride = read_u32(manager.wrapping_add(12));
        let records = read_u32(manager);
        let record = records.wrapping_add(stride.wrapping_mul(index));
        let count_address = record.wrapping_add(4);
        let count = read_u32(count_address);
        write_u32(count_address, count.wrapping_sub(1));
        record
    }
});
