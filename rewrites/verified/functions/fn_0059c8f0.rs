// original: 0x0059C8F0 CNetBlenderDummyPed::vf13

/// Query the entity's node manager and write the resulting three-word position.
///
/// `this` contains the entity pointer at `+0xC0`; that entity contains its
/// node manager at `+0x78`. The first helper looks up a key using two zero
/// selectors. A missing key, or either key component equal to `u32::MAX`,
/// zeroes the three-word output. Otherwise the second helper receives the
/// pointer to a local scratch record followed by the key's high and low words;
/// its returned three-word record is copied to `output`. The method is a
/// thiscall with one stack pointer argument and returns that same pointer.
lf_checker_rt::export!(thiscall, rw_0059c8f0(this: u32, output: u32) -> u32 {
    const ENTITY_POINTER: u32 = 0xC0;
    const NODE_MANAGER: u32 = 0x78;
    const KEY_LOW: u32 = 0x0C;
    const KEY_HIGH: u32 = 0x10;
    const MISSING: u32 = u32::MAX;
    const VECTOR_WORDS: u32 = 3;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let entity = read_word(this.wrapping_add(ENTITY_POINTER));
        let manager = read_word(entity.wrapping_add(NODE_MANAGER));
        let key = lf_checker_rt::callee_thiscall!(1, u32, manager, 0u32, 0u32);

        if key == 0 {
            for word in 0..VECTOR_WORDS {
                write_word(output.wrapping_add(word * 4), 0);
            }
            return output;
        }

        let low = read_word(key.wrapping_add(KEY_LOW));
        let high = read_word(key.wrapping_add(KEY_HIGH));
        if low == MISSING || high == MISSING {
            for word in 0..VECTOR_WORDS {
                write_word(output.wrapping_add(word * 4), 0);
            }
            return output;
        }

        let mut scratch = [0u32; 4];
        let scratch_pointer = scratch.as_mut_ptr() as u32;
        let position = lf_checker_rt::callee_cdecl!(2, u32, scratch_pointer, high, low);
        for word in 0..VECTOR_WORDS {
            let value = read_word(position.wrapping_add(word * 4));
            write_word(output.wrapping_add(word * 4), value);
        }
        output
    }
});
