// original: 0x00ABC830 input_ui_build_record

/// Initialize a record with fields through byte offset 0x38.
///
/// The destination is in ECX; the stack arguments are a source pointer, two
/// 32-bit values, and a flag byte. The routine updates the destination key
/// using a 14-bit salt derived from the key and a process-wide generation
/// counter, increments that counter, stores the relocated vtable pointer,
/// copies six source words into the record, and writes the remaining values.
/// It returns the destination in EAX and removes all four stack arguments.
lf_checker_rt::export!(thiscall, rw_00abc830(destination: u32, source: u32, first_value: u32, second_value: u32, flag: u32) -> u32 {
    unsafe {
        const GLOBAL_COUNTER_VA: u32 = 0x010327A0;
        const VTABLE_VA: u32 = 0x00EA5B24;

        let generation_counter = lf_checker_rt::global::<u32>(GLOBAL_COUNTER_VA);
        let prior_generation = generation_counter.read_unaligned();
        let key_slot = destination.wrapping_add(4) as *mut u32;
        let prior_key = key_slot.read_unaligned();
        let salt = (prior_key ^ prior_generation) & 0x3fff;
        key_slot.write_unaligned(prior_key ^ salt);
        generation_counter.write_unaligned(prior_generation.wrapping_add(1));

        (destination as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        let source_offsets = [0u32, 4, 8, 0x10, 0x14, 0x18];
        let destination_offsets = [0x10u32, 0x14, 0x18, 0x20, 0x24, 0x28];
        for index in 0..source_offsets.len() {
            let value = (source.wrapping_add(source_offsets[index]) as *const u32).read_unaligned();
            (destination.wrapping_add(destination_offsets[index]) as *mut u32).write_unaligned(value);
        }
        (destination.wrapping_add(0x30) as *mut u32).write_unaligned(first_value);
        (destination.wrapping_add(0x34) as *mut u32).write_unaligned(second_value);
        (destination.wrapping_add(0x38) as *mut u8).write(flag as u8);
        destination
    }
});
