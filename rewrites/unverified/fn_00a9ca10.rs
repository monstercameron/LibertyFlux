// original: 0x00A9CA10 loader_mark_resource_states
/// Copies each selected resource's status byte into the loader object's
/// result table as a zero-extended dword. The record count and 16-bit index
/// list live behind the context's record pointer; each index selects a
/// resource pointer through the context's resource table. A validator is
/// called once per row with the fixed validator object as `this` and the
/// selected resource as its stack argument. A nonzero low-byte result sets
/// that resource's status byte to `READY_STATUS`. The function returns the
/// record pointer, including when the record count is zero.
lf_checker_rt::export!(thiscall, rw_00A9CA10(this: u32, context: u32) -> u32 {
    unsafe {
        const CONTEXT_RECORD: u32 = 0x40;
        const CONTEXT_RESOURCES: u32 = 0x08;
        const RECORD_INDEX_LIST: u32 = 0x10;
        const RECORD_COUNT: u32 = 0x1a;
        const RESOURCE_STATUS: u32 = 0x09;
        const RESULT_TABLE: u32 = 0x8ec58;
        const READY_STATUS: u8 = 0x1f;
        const VALIDATOR_OBJECT: u32 = 0x01305d30;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { (address as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let record = read_u32(read_u32(context.wrapping_add(CONTEXT_RECORD)));
        let count = read_u16(record.wrapping_add(RECORD_COUNT));
        let indices = read_u32(record.wrapping_add(RECORD_INDEX_LIST));
        let resource_owner = read_u32(context.wrapping_add(CONTEXT_RESOURCES));
        let resource_table = read_u32(resource_owner.wrapping_add(CONTEXT_RESOURCES));
        let validator = lf_checker_rt::relocated(VALIDATOR_OBJECT);

        for row in 0..count {
            let index = read_u16(indices.wrapping_add(row.wrapping_mul(2)));
            let resource = read_u32(resource_table.wrapping_add(index.wrapping_mul(4)));
            let status = read_u8(resource.wrapping_add(RESOURCE_STATUS));
            write_u32(
                this.wrapping_add(RESULT_TABLE)
                    .wrapping_add(row.wrapping_mul(4)),
                u32::from(status),
            );

            let accepted: u8 =
                lf_checker_rt::callee_thiscall!(1, u8, validator, resource);
            if accepted != 0 {
                write_u8(resource.wrapping_add(RESOURCE_STATUS), READY_STATUS);
            }
        }

        record
    }
});
