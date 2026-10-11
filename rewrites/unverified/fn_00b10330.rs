// original: 0x00B10330 increment_indexed_resource_reference

/// Increment the reference count in the resource record selected by `index`.
/// The manager is a relocated game global containing a records pointer, a
/// byte-flag pointer and a byte stride. The high bit in the indexed flag is an
/// invalid sentinel: the original faults while attempting to touch offset 4
/// from null, so the rewrite reproduces that fault. Otherwise the count word
/// at record offset 4 is incremented with 32-bit wrapping and the record
/// address is returned. The function is cdecl with one stack argument.
lf_checker_rt::export!(cdecl, rw_00b10330(index: u32) -> u32 {
    const MANAGER_VA: u32 = 0x016F_B694;
    const RECORDS_FIELD: u32 = 0;
    const FLAGS_FIELD: u32 = 4;
    const STRIDE_FIELD: u32 = 0x0C;
    const COUNT_FIELD: u32 = 4;
    const INVALID_COUNT_WORD: usize = 1;
    const INVALID_FLAG: u8 = 0x80;

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let manager = read_u32(lf_checker_rt::global::<u32>(MANAGER_VA) as u32);
        let flags = read_u32(manager.wrapping_add(FLAGS_FIELD));
        if read_u8(flags.wrapping_add(index)) & INVALID_FLAG != 0 {
            let invalid_count = core::ptr::null_mut::<u32>().wrapping_add(INVALID_COUNT_WORD);
            let old = core::ptr::read_volatile(invalid_count);
            core::ptr::write_volatile(invalid_count, old.wrapping_add(1));
            return 0;
        }

        let records = read_u32(manager.wrapping_add(RECORDS_FIELD));
        let stride = read_u32(manager.wrapping_add(STRIDE_FIELD));
        let record = records.wrapping_add(stride.wrapping_mul(index));
        let count_address = record.wrapping_add(COUNT_FIELD);
        write_u32(count_address, read_u32(count_address).wrapping_add(1));
        record
    }
});
