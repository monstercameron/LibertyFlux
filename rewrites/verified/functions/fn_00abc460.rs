// original: 0x00ABC460 input_ui_build_extended_record

/// Build an extended record from twelve source words, pointed-to values, and
/// one direct word.
///
/// The destination arrives in ECX. Seven stack arguments provide a direct
/// word, a pointer to another word, a 0x3c-byte source record, pointers to two
/// words, a byte, and one final word. The routine updates a 14-bit key using
/// a process-wide generation counter, increments that counter, writes a
/// relocated vtable pointer, copies the twelve source words into their record
/// slots, and stores the remaining values. It returns the destination in EAX
/// and removes all seven stack arguments.
lf_checker_rt::export!(thiscall, rw_00abc460(destination: u32, first_value: u32, pointed_value: u32, source: u32, fourth_pointer: u32, fifth_pointer: u32, byte_pointer: u32, final_pointer: u32) -> u32 {
    unsafe {
        const GLOBAL_COUNTER_VA: u32 = 0x010327A0;
        const VTABLE_VA: u32 = 0x00EA5C04;
        const KEY_MASK: u32 = 0x3fff;
        const SOURCE_OFFSETS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        const DESTINATION_OFFSETS: [u32; 12] = [0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38, 0x40, 0x44, 0x48];

        let generation_counter = lf_checker_rt::global::<u32>(GLOBAL_COUNTER_VA);
        let prior_generation = generation_counter.read_unaligned();
        let key_slot = destination.wrapping_add(4) as *mut u32;
        let prior_key = key_slot.read_unaligned();
        key_slot.write_unaligned(prior_key ^ ((prior_key ^ prior_generation) & KEY_MASK));
        generation_counter.write_unaligned(prior_generation.wrapping_add(1));

        (destination as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        (destination.wrapping_add(8) as *mut u32).write_unaligned(first_value);
        let pointed_word = (pointed_value as *const u32).read_unaligned();
        (destination.wrapping_add(0x0c) as *mut u32).write_unaligned(pointed_word);
        for index in 0..SOURCE_OFFSETS.len() {
            let word = (source.wrapping_add(SOURCE_OFFSETS[index]) as *const u32).read_unaligned();
            (destination.wrapping_add(DESTINATION_OFFSETS[index]) as *mut u32).write_unaligned(word);
        }
        let fourth = (fourth_pointer as *const u32).read_unaligned();
        let fifth = (fifth_pointer as *const u32).read_unaligned();
        let byte = (byte_pointer as *const u8).read();
        let final_word = (final_pointer as *const u32).read_unaligned();
        (destination.wrapping_add(0x50) as *mut u32).write_unaligned(fourth);
        (destination.wrapping_add(0x54) as *mut u32).write_unaligned(fifth);
        (destination.wrapping_add(0x58) as *mut u8).write(byte);
        (destination.wrapping_add(0x5c) as *mut u32).write_unaligned(final_word);
        destination
    }
});
