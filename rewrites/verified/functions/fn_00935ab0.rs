// original: 0x00935AB0 set_network_record_state_for_key

/// Set a one-byte field on active records that have the requested key. The
/// receiver stores a record-array pointer at `+0x144` and a 16-bit count at
/// `+0x148`; records use a `0x22c` stride. A record is eligible when its byte
/// at `+0x226` is nonzero and its key byte at `+0x229` equals the first stack
/// argument as a full 32-bit value. The second stack argument is truncated to
/// a byte and written at `+0x225`. Two receiver helpers run before the scan,
/// even when the key is zero. The routine has no defined return value and
/// uses thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00935ab0(this: u32, target_key: u32, state: u32) -> () {
    unsafe {
        const RECORDS_POINTER: u32 = 0x144;
        const RECORD_COUNT: u32 = 0x148;
        const RECORD_STRIDE: u32 = 0x22c;
        const ACTIVE_FLAG: u32 = 0x226;
        const RECORD_KEY: u32 = 0x229;
        const STATE_FIELD: u32 = 0x225;

        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this);

        if target_key != 0 {
            let records = read_u32(this.wrapping_add(RECORDS_POINTER));
            let record_count = u32::from(read_u16(this.wrapping_add(RECORD_COUNT)));
            for index in 0..record_count {
                let record = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
                let active = read_u8(record.wrapping_add(ACTIVE_FLAG));
                let key = u32::from(read_u8(record.wrapping_add(RECORD_KEY)));
                if active != 0 && key == target_key {
                    write_u8(record.wrapping_add(STATE_FIELD), state as u8);
                }
            }
        }
    }
});
