// original: 0x00C49A10 camera_cinematic_init_track_buffers

/// Initialize the cinematic camera's fixed fields and two track buffers.
///
/// The object arrives in ECX. Two 0x960-byte buffers are requested through
/// the allocation call, then each buffer receives a zero word every 0x30
/// bytes from offset 0x20 through 0x950. The object stores the returned
/// pointers and the original's sentinel, flag, byte, word, and zero fields.
/// The function returns the second buffer pointer with its low byte replaced
/// by one, as the original's final byte write leaves the rest of EAX intact.
///
/// The allocation calls are outgoing cdecl calls and remain observable through
/// the checker stand-ins. This implementation does not depend on host pointer
/// width; it models the game's 32-bit object and buffer layout explicitly.
lf_checker_rt::export!(thiscall, rw_c49a10(this: u32) -> u32 {
    const ALLOC_FIRST: u32 = 1;
    const ALLOC_SECOND: u32 = 2;
    const BUFFER_BYTES: u32 = 0x960;
    const BUFFER_STRIDE: u32 = 0x30;
    const FIRST_SLOT_OFFSET: u32 = 0x20;
    const LAST_SLOT_OFFSET: u32 = 0x950;
    const RETURN_LOW_BYTE_MASK: u32 = 0xffff_ff00;

    const BUFFER_A: u32 = 0x154;
    const BUFFER_B: u32 = 0x15c;
    const OBJECT_FLAG: u32 = 0x148;
    const FIELD_SENTINEL_A: u32 = 0x144;
    const FIELD_SENTINEL_B: u32 = 0x14c;
    const FIELD_WORD_FLAGS: u32 = 0x150;

    let first_buffer = lf_checker_rt::callee_cdecl!(ALLOC_FIRST, u32, BUFFER_BYTES);
    let second_buffer = lf_checker_rt::callee_cdecl!(ALLOC_SECOND, u32, BUFFER_BYTES);

    unsafe {
        let first_slot = (this.wrapping_add(BUFFER_A)) as *mut u32;
        let second_slot = (this.wrapping_add(BUFFER_B)) as *mut u32;
        first_slot.write_unaligned(first_buffer);
        second_slot.write_unaligned(second_buffer);

        (this.wrapping_add(0x140) as *mut u32).write_unaligned(0);
        (this.wrapping_add(FIELD_SENTINEL_A) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(FIELD_SENTINEL_B) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(0x158) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x160) as *mut u32).write_unaligned(0);
        (this.wrapping_add(OBJECT_FLAG) as *mut u32).write_unaligned(1);
        (this.wrapping_add(FIELD_WORD_FLAGS) as *mut u16).write_unaligned(0x0101);
        (this.wrapping_add(0x184) as *mut u8).write_unaligned(0);

        let mut slot_offset = FIRST_SLOT_OFFSET;
        while slot_offset <= LAST_SLOT_OFFSET {
            (first_buffer.wrapping_add(slot_offset) as *mut u32).write_unaligned(0);
            (second_buffer.wrapping_add(slot_offset) as *mut u32).write_unaligned(0);
            slot_offset = slot_offset.wrapping_add(BUFFER_STRIDE);
        }

        (this.wrapping_add(0x168) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x164) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x178) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x174) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x170) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x180) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x198) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x194) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x190) as *mut u32).write_unaligned(0);
    }

    (second_buffer & RETURN_LOW_BYTE_MASK) | 1
});
