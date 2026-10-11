// original: 0x00C781F0 phbound_register_static_hooks (proposed)

/// Walk the fixed static registration-pointer array. For each non-null record
/// whose 16-bit type is not the invalid sentinel, copy its integer and float
/// bit fields into the helper's 48-byte request record and invoke the
/// registration helper. Finally validate the process security cookie.
lf_checker_rt::export!(cdecl, rw_phbound_register_static_hooks() -> () {
    unsafe {
        const STATIC_TABLE_START: u32 = 0x0129_5cd8;
        const STATIC_TABLE_END: u32 = 0x012b_4138;
        const SECURITY_COOKIE: u32 = 0x0105_7fb4;
        const RECORD_TYPE: u32 = 0x52;
        const RECORD_ID: u32 = 0x3c;
        const INVALID_RECORD_TYPE: u16 = u16::MAX;
        const REQUEST_SIZE: u32 = 0x30;
        const REGISTER: u32 = 1;
        const CHECK_COOKIE: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        let static_table = lf_checker_rt::relocated(STATIC_TABLE_START);
        let mut record_address = static_table;
        while record_address < static_table.wrapping_add(STATIC_TABLE_END - STATIC_TABLE_START) {
            let record = unsafe { read_u32(record_address) };
            if record != 0 {
                let record_type = unsafe { read_u16(record.wrapping_add(RECORD_TYPE)) };
                if record_type != INVALID_RECORD_TYPE {
                    let signed_type = i32::from(record_type as i16) as u32;
                    let mut request = [0u32; 12];
                    request[0] = unsafe { read_u32(record.wrapping_add(RECORD_ID)) };
                    request[1] = signed_type;
                    request[2] = unsafe { read_u32(record.wrapping_add(0x20)) };
                    request[3] = unsafe { read_u32(record.wrapping_add(0x24)) };
                    request[4] = unsafe { read_u32(record.wrapping_add(0x28)) };
                    request[5] = unsafe { read_u32(record.wrapping_add(0x30)) };
                    request[6] = unsafe { read_u32(record.wrapping_add(0x34)) };
                    request[7] = unsafe { read_u32(record.wrapping_add(0x38)) };
                    request[8] = unsafe { read_u32(record.wrapping_add(0x10)) };
                    request[9] = unsafe { read_u32(record.wrapping_add(0x14)) };
                    request[10] = unsafe { read_u32(record.wrapping_add(0x18)) };
                    request[11] = unsafe { read_u32(record.wrapping_add(0x1c)) };
                    let request_pointer = request.as_mut_ptr() as u32;
                    let _ = lf_checker_rt::callee_cdecl!(
                        REGISTER,
                        u32,
                        request_pointer,
                        REQUEST_SIZE,
                    );
                }
            }
            record_address = record_address.wrapping_add(4);
        }

        let expected_cookie = unsafe {
            lf_checker_rt::global::<u32>(SECURITY_COOKIE).read_volatile()
        };
        let _ = lf_checker_rt::callee_thiscall!(CHECK_COOKIE, u32, expected_cookie);
    }
});
