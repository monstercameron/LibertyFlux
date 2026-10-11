// original: 0x00BC83C0 Vehicle_AreaSearchHelper

/// Decide whether an area-search request is enabled, then test the primary
/// candidate and up to eight queued candidates through the two search helpers.
///
/// The cdecl arguments are a search-state pointer, a signed selector, and a
/// packed option word. The state fields used here are the mode at `+0x1304`,
/// the state flags at `+0x0f1f`, the signed selector at `+0x2e`, the primary
/// candidate at `+0x0f50`, and eight candidate slots beginning at `+0x0f54`.
/// Only modes zero and one are accepted. A nonnegative selector must match
/// the signed 16-bit state selector. The option word supplies bits zero
/// through eleven to the search helpers; the original also uses the third
/// stack argument's low byte as scratch while building those values.
///
/// The area-policy helper receives bits one, six through nine and fixed zero
/// words. If it accepts, the function clears one shared search-state byte and
/// asks the candidate helper about every non-null candidate in order. The
/// result is one only when the policy helper accepts and every tested
/// candidate helper returns nonzero. The candidate helper's stack arguments
/// whose upper bytes came from uninitialized local storage are compared on
/// their low byte by this proof contract.
///
/// The raw object layout is not identified by the merged structure data, so
/// the offsets below name only the fields established from this function.
/// Calling convention: cdecl, three stack arguments, low-byte boolean result.
lf_checker_rt::export!(cdecl, rw_00BC83C0(search_state: u32, selector: i32, options: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x1304;
        const STATE_FLAGS: u32 = 0x0f1f;
        const STORED_SELECTOR: u32 = 0x2e;
        const PRIMARY_CANDIDATE: u32 = 0x0f50;
        const QUEUED_CANDIDATES: u32 = 0x0f54;
        const QUEUED_CANDIDATE_COUNT: u32 = 8;
        const AREA_POLICY_CALLEE: u32 = 1;
        const SEARCH_STATE_BYTE: u32 = 0x011e_66c8;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn test_candidate(
            candidate: u32,
            bit1: u32,
            bit2: u32,
            bit5: u32,
            bit6: u32,
            bit11: u32,
            options: u32,
        ) -> bool {
            const CALLEE_ID: u32 = 2;
            let result = lf_checker_rt::callee_cdecl!(
                CALLEE_ID,
                u32,
                candidate,
                bit2,
                bit1,
                1u32.wrapping_sub(bit6),
                0,
                (options & !0xff) | ((options >> 4) & 1),
                bit5,
                bit11,
            );
            result as u8 != 0
        }

        let flag_bit = |index: u32| (options >> index) & 1;
        let bit0 = flag_bit(0);
        let bit1 = flag_bit(1);
        let bit2 = flag_bit(2);
        let bit5 = flag_bit(5);
        let bit6 = flag_bit(6);
        let bit7 = flag_bit(7);
        let bit8 = flag_bit(8);
        let bit9 = flag_bit(9);
        let bit10 = flag_bit(10);
        let bit11 = flag_bit(11);

        let mode = read_u32(search_state.wrapping_add(MODE));
        if mode != 0 && mode != 1 {
            return 0;
        }

        let state_flags = read_u8(search_state.wrapping_add(STATE_FLAGS));
        if state_flags & 0x20 != 0 {
            if bit0 == 0 && selector < 0 {
                return 0;
            }
        } else if bit10 != 0 {
            return 0;
        }

        let stored_selector = read_u16(search_state.wrapping_add(STORED_SELECTOR)) as i16;
        if selector >= 0 && i32::from(stored_selector) != selector {
            return 0;
        }

        let policy_result = lf_checker_rt::callee_thiscall!(
            AREA_POLICY_CALLEE,
            u32,
            search_state,
            1u32.wrapping_sub(bit1),
            bit7,
            1u32.wrapping_sub(bit6),
            bit8,
            bit9,
            0,
            0,
        );
        if policy_result as u8 == 0 {
            return 0;
        }

        core::ptr::write_volatile(lf_checker_rt::global::<u8>(SEARCH_STATE_BYTE), 0);

        let primary = read_u32(search_state.wrapping_add(PRIMARY_CANDIDATE));
        if primary != 0 && !test_candidate(primary, bit1, bit2, bit5, bit6, bit11, options) {
            return 0;
        }

        let mut index = 0;
        while index < QUEUED_CANDIDATE_COUNT {
            let candidate = read_u32(
                search_state
                    .wrapping_add(QUEUED_CANDIDATES)
                    .wrapping_add(index.wrapping_mul(4)),
            );
            if candidate != 0 && !test_candidate(candidate, bit1, bit2, bit5, bit6, bit11, options) {
                return 0;
            }
            index += 1;
        }

        1
    }
});
