// original: 0x006615D0 rage::snDropGamersTask::set_gamer_list
/// Stores the pending state, pair-list pointer, copied-pair count, and request identifier on `this`. It copies at most 32 signed-counted 8-byte key pairs. For each copied pair, it searches the context's signed-counted pointer table for both matching key words and ORs bit 3 into the matching record's flag byte. The result is a boolean returned in AL; upper EAX is path-dependent. The layout is inherently 32-bit.
lf_checker_rt::export!(thiscall, rw_006615d0(
    this: u32,
    context: u32,
    pending_state: u32,
    pairs: u32,
    source_pairs: u32,
    pair_count: i32,
    request_id: u32
) -> u32 {
    unsafe {
        const STATE_FIELD: u32 = 0x90;
        const PAIRS_FIELD: u32 = 0x94;
        const STORED_COUNT_FIELD: u32 = 0x398;
        const REQUEST_ID_FIELD: u32 = 0x39c;
        const STORED_PAIRS: u32 = 0x98;
        const MAX_STORED_PAIRS: i32 = 32;
        const CONTEXT_PAIRS: u32 = 0x1d94;
        const CONTEXT_PAIR_COUNT: u32 = 0x1e14;
        const KEY_FIRST: u32 = 0x40;
        const KEY_SECOND: u32 = 0x44;
        const ACTIVE_FLAGS: u32 = 0x80;
        const ACTIVE_BIT: u8 = 8;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_byte(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        unsafe { write_word(this.wrapping_add(STATE_FIELD), pending_state) };
        unsafe { write_word(this.wrapping_add(PAIRS_FIELD), pairs) };
        unsafe { write_word(this.wrapping_add(STORED_COUNT_FIELD), 0) };
        unsafe { write_word(this.wrapping_add(REQUEST_ID_FIELD), request_id) };

        if pair_count <= 0 {
            return 1;
        }

        let mut item_index = 0i32;
        while item_index < pair_count && item_index < MAX_STORED_PAIRS {
            let source = source_pairs.wrapping_add((item_index as u32).wrapping_mul(8));
            let key_first = unsafe { read_word(source) };
            let key_second = unsafe { read_word(source.wrapping_add(4)) };
            let stored_count = unsafe { read_word(this.wrapping_add(STORED_COUNT_FIELD)) };
            let destination = this
                .wrapping_add(STORED_PAIRS)
                .wrapping_add(stored_count.wrapping_mul(8));
            unsafe { write_word(destination, key_first) };
            unsafe { write_word(destination.wrapping_add(4), key_second) };
            unsafe { write_word(this.wrapping_add(STORED_COUNT_FIELD), stored_count.wrapping_add(1)) };

            let context_count = unsafe { read_word(context.wrapping_add(CONTEXT_PAIR_COUNT)) as i32 };
            if context_count > 0 {
                let mut context_index = 0i32;
                while context_index < context_count {
                    let context_pair = unsafe {
                        read_word(context.wrapping_add(CONTEXT_PAIRS + (context_index as u32).wrapping_mul(4)))
                    };
                    let first_matches = unsafe { read_word(context_pair.wrapping_add(KEY_FIRST)) } == key_first;
                    let second_matches = unsafe { read_word(context_pair.wrapping_add(KEY_SECOND)) } == key_second;
                    if first_matches && second_matches {
                        if context_pair != 0 {
                            let flags_address = context_pair.wrapping_add(ACTIVE_FLAGS);
                            let flags = unsafe { read_byte(flags_address) };
                            unsafe { write_byte(flags_address, flags | ACTIVE_BIT) };
                        }
                        break;
                    }
                    context_index += 1;
                }
            }
            item_index += 1;
        }

        1
    }
});
