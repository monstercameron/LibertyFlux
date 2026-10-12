// original: 0x005C43C0 ped_task_global_category_bit

/// Queries the fixed ped-task service object and returns bit 2 of the
/// service record's byte at offset 0x13c. The service lookup is a thiscall
/// with no stack arguments; the result is read as a byte and shifted to bit 0.
lf_checker_rt::export!(cdecl, rw_005c43c0() -> u32 {
    unsafe {
        const SERVICE_OBJECT: u32 = 0x0103e498;
        const SERVICE_LOOKUP_CALLEE: u32 = 1;
        const SERVICE_FLAGS_OFFSET: u32 = 0x13c;
        const CATEGORY_PRESENT_BIT: u8 = 1 << 2;

        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let service_record: u32 =
            lf_checker_rt::callee_thiscall!(SERVICE_LOOKUP_CALLEE, u32, lf_checker_rt::relocated(SERVICE_OBJECT));
        u32::from((read_byte(service_record + SERVICE_FLAGS_OFFSET) & CATEGORY_PRESENT_BIT) != 0)
    }
});
