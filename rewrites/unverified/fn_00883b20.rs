// original: 0x00883b20 streaming_find_empty_id_and_clear_buffer

/// Register an identifier in the first open slot. The object has eight
/// 16-bit slots at byte offset 0x4c; a slot holding all-ones bits is open.
/// When a slot is available, three scripted helpers create and validate a
/// backing buffer; a successful validation stores the low 16 bits of the
/// second helper's returned value and clears the buffer's active and
/// overflow words. The function returns the selected slot, or all-ones
/// bits if no slot is open or validation fails.
///
/// Thiscall with one stack argument; the caller's argument slot is
/// temporarily overwritten with the helper's result before the validator
/// call, so the proof does not compare stack memory.
const ENTRY_SLOTS_OFFSET: u32 = 0x4c;
const ENTRY_SLOT_COUNT: u32 = 8;
const BUFFER_BASE_OFFSET: u32 = 8;
const BUFFER_END_OFFSET: u32 = 12;
const BUFFER_MODE_OFFSET: u32 = 16;
const BUFFER_MODE_MASK: u32 = 0x0f;

#[inline(always)]
unsafe fn read_stream_u16(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_stream_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_stream_u16(address: u32, value: u16) {
    unsafe { (address as *mut u16).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_stream_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

unsafe fn register_streaming_entry(this: u32, requested_id: u32) -> u32 {
    let mut available_slot = None;
    for slot in 0..ENTRY_SLOT_COUNT {
        let slot_id = unsafe { read_stream_u16(this.wrapping_add(ENTRY_SLOTS_OFFSET + slot * 2)) };
        if slot_id == u16::MAX {
            available_slot = Some(slot);
            break;
        }
    }
    let Some(slot) = available_slot else {
        return u32::MAX;
    };

    // The original calls the first two helpers with cdecl stack arguments.
    let buffer_info = lf_checker_rt::callee_cdecl!(1, u32, this, requested_id);
    let validated_buffer = lf_checker_rt::callee_cdecl!(2, u32, buffer_info);
    // The original writes this value into its incoming argument slot and then
    // forwards it as the validator's sole stack argument.
    let accepted = lf_checker_rt::callee_thiscall!(3, u32, requested_id, validated_buffer);
    if accepted as u8 == 0 {
        return u32::MAX;
    }

    unsafe {
        write_stream_u16(
            this.wrapping_add(ENTRY_SLOTS_OFFSET + slot * 2),
            validated_buffer as u16,
        );
    }

    let mode = unsafe { read_stream_u32(buffer_info.wrapping_add(BUFFER_MODE_OFFSET)) } & BUFFER_MODE_MASK;
    if mode != 0 {
        let buffer_base = unsafe { read_stream_u32(buffer_info.wrapping_add(BUFFER_BASE_OFFSET)) };
        let buffer_end = if mode == 1 {
            buffer_base.wrapping_add(4)
        } else {
            unsafe { read_stream_u32(buffer_info.wrapping_add(BUFFER_END_OFFSET)) }
        };
        let overflow_offset = buffer_end.wrapping_sub(buffer_base);
        for word_index in 0..mode {
            unsafe {
                write_stream_u32(
                    buffer_base
                        .wrapping_add(overflow_offset)
                        .wrapping_add(word_index.wrapping_mul(4)),
                    0,
                );
                write_stream_u32(buffer_base.wrapping_add(word_index.wrapping_mul(4)), 0);
            }
        }
    }

    slot
}

lf_checker_rt::export!(thiscall, rw_00883b20(this: u32, requested_id: u32) -> u32 {
    unsafe { register_streaming_entry(this, requested_id) }
});
