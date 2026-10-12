// original: 0x00DB63C0 input_get_button_state_mask

/// Read the receiver's input-state pointer and return the first matching
/// activity mask. The seven byte-pair XOR values are tested in order against
/// the unsigned boundary 0x7F. A high first-pair result followed by a low
/// second-pair result returns 0x40000; later pairs return 0x100, 0x200,
/// 0x800 or 0x400 in order, and the last pair contributes 4. The receiver is
/// in ECX; this method has no stack arguments and returns the full mask in EAX.
lf_checker_rt::export!(thiscall, rw_00db63c0(this: u32) -> u32 {
    unsafe {
        const FIRST_HIGH: u32 = 0x314e;
        const FIRST_BASE: u32 = 0x314c;
        const SECOND_HIGH: u32 = 0x314f;
        const PAIR_A_HIGH: u32 = 0x2aae;
        const PAIR_A_BASE: u32 = 0x2aac;
        const PAIR_B_HIGH: u32 = 0x2a9e;
        const PAIR_B_BASE: u32 = 0x2a9c;
        const PAIR_C_HIGH: u32 = 0x2abe;
        const PAIR_C_BASE: u32 = 0x2abc;
        const PAIR_D_HIGH: u32 = 0x2ace;
        const PAIR_D_BASE: u32 = 0x2acc;
        const FINAL_HIGH: u32 = 0x2b6e;
        const FINAL_BASE: u32 = 0x2b6c;
        const LIMIT: u8 = 0x7f;
        const FIRST_LIMIT: u8 = 0x7f;

        #[inline(always)]
        unsafe fn read_byte(base: u32, offset: u32) -> u8 {
            unsafe { (base.wrapping_add(offset) as *const u8).read() }
        }

        let state = (this as *const u32).read_unaligned();
        let first = read_byte(state, FIRST_HIGH) ^ read_byte(state, FIRST_BASE);
        if first > FIRST_LIMIT {
            let second = read_byte(state, SECOND_HIGH) ^ read_byte(state, FIRST_BASE);
            if second <= LIMIT {
                return 0x40000;
            }
        }

        let checks = [
            (PAIR_A_HIGH, PAIR_A_BASE, 0x100),
            (PAIR_B_HIGH, PAIR_B_BASE, 0x200),
            (PAIR_C_HIGH, PAIR_C_BASE, 0x800),
            (PAIR_D_HIGH, PAIR_D_BASE, 0x400),
        ];
        for (high, base, mask) in checks {
            if (read_byte(state, high) ^ read_byte(state, base)) > LIMIT {
                return mask;
            }
        }
        if (read_byte(state, FINAL_HIGH) ^ read_byte(state, FINAL_BASE)) > LIMIT {
            4
        } else {
            0
        }
    }
});
