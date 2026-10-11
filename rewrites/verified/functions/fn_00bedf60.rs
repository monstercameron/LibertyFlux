// original: 0x00BEDF60 toggle_task_option_bit

/// Apply one of eight task-option updates. Case zero toggles bit 6 of byte
/// `this + 0x55`; case one replaces bit 7 of that byte from the input's low
/// bit; cases two through seven toggle bits zero through five of byte
/// `this + 0x56`. The case selector is an unsigned stack argument, and values
/// above seven leave memory unchanged and return the selector in EAX. For the
/// active cases, EAX returns the shifted input bit or the toggle mask produced
/// by the selected update. The original is a thiscall with two stack arguments.
lf_checker_rt::export!(thiscall, rw_00BEDF60(this: u32, input_bits: u32, action: u32) -> u32 {
    const FIRST_OPTION: u32 = 0x55;
    const SECOND_OPTION: u32 = 0x56;
    const CASE_TOGGLE_SIXTH: u32 = 0;
    const CASE_SET_SEVENTH: u32 = 1;
    const CASE_TOGGLE_FIRST: u32 = 2;
    const LAST_ACTION: u32 = 7;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        match action {
            CASE_TOGGLE_SIXTH => {
                let current = read_u8(this + FIRST_OPTION);
                let shifted_input = (input_bits as u8).wrapping_shl(6);
                let toggle = (shifted_input ^ current) & 0x40;
                write_u8(this + FIRST_OPTION, current ^ toggle);
                u32::from(toggle)
            }
            CASE_SET_SEVENTH => {
                let current = read_u8(this + FIRST_OPTION) & 0x7F;
                let shifted_input = (input_bits as u8).wrapping_shl(7);
                write_u8(this + FIRST_OPTION, current | shifted_input);
                u32::from(shifted_input)
            }
            CASE_TOGGLE_FIRST..=LAST_ACTION => {
                let bit = action - CASE_TOGGLE_FIRST;
                let mask = 1u8 << bit;
                let current = read_u8(this + SECOND_OPTION);
                let shifted_input = (input_bits as u8).wrapping_shl(bit);
                let toggle = (shifted_input ^ current) & mask;
                write_u8(this + SECOND_OPTION, current ^ toggle);
                u32::from(toggle)
            }
            _ => action,
        }
    }
});
