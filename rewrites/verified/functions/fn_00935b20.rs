// original: 0x00935B20 clear_active_network_states

/// Visit each state entry and clear it through the state helper when its
/// active byte is nonzero. The receiver stores a state-table pointer at
/// `+0x14c` and a 16-bit entry count at `+0x150`; entries use a `0x168`
/// stride, with the active byte at `+0x166`. The helper receives the receiver
/// and the zero-based entry index. The routine has no defined return value
/// and uses thiscall.
lf_checker_rt::export!(thiscall, rw_00935b20(this: u32) -> () {
    unsafe {
        const STATE_TABLE_POINTER: u32 = 0x14c;
        const STATE_COUNT: u32 = 0x150;
        const STATE_STRIDE: u32 = 0x168;
        const ACTIVE_FLAG: u32 = 0x166;

        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let state_table = read_u32(this.wrapping_add(STATE_TABLE_POINTER));
        let state_count = u32::from(read_u16(this.wrapping_add(STATE_COUNT)));
        for index in 0..state_count {
            let active_flag = state_table
                .wrapping_add(index.wrapping_mul(STATE_STRIDE))
                .wrapping_add(ACTIVE_FLAG);
            if (active_flag as *const u8).read() != 0 {
                let _ = lf_checker_rt::callee_thiscall!(1, u32, this, index);
            }
        }
    }
});
