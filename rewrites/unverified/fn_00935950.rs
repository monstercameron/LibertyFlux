// original: 0x00935950 clear_network_record_state

/// Clear the state byte for one network record. The receiver has record and
/// state-table pointers at `+0x144` and `+0x14c`, with a 16-bit record count
/// at `+0x148`. Record keys use a `0x22c` stride; state entries use a `0x168`
/// stride and their active byte is at `+0x166`. When the selected entry is
/// active, the routine queries the record keys and calls its record and state
/// helpers in order. It clears the selected active byte and returns the
/// state-table pointer in EAX, then removes its
/// one stack argument (`thiscall`).
lf_checker_rt::export!(thiscall, rw_00935950(this: u32, state_index: u32) -> u32 {
    unsafe {
        const RECORDS_POINTER: u32 = 0x144;
        const RECORD_COUNT: u32 = 0x148;
        const STATE_TABLE_POINTER: u32 = 0x14c;
        const RECORD_STRIDE: u32 = 0x22c;
        const RECORD_STATE_STRIDE: u32 = 0x168;
        const ACTIVE_FLAG: u32 = 0x166;

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
        let state_table = read_u32(this.wrapping_add(STATE_TABLE_POINTER));
        let state_flag = state_table
            .wrapping_add(state_index.wrapping_mul(RECORD_STATE_STRIDE))
            .wrapping_add(ACTIVE_FLAG);

        if read_u8(state_flag) != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32,);
            let record_array = read_u32(this.wrapping_add(RECORDS_POINTER));
            let record_count = u32::from(read_u16(this.wrapping_add(RECORD_COUNT)));

            for record_index in 0..record_count {
                let record = record_array.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
                if read_u32(record) == state_index {
                    let _ = lf_checker_rt::callee_thiscall!(3, u32, this, record_index);
                }
            }

            let _ = lf_checker_rt::callee_thiscall!(4, u32, this, state_index);
        }

        write_u8(state_flag, 0);
        state_table
    }
});
