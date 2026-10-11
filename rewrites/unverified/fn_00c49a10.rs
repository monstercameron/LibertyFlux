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
    const TRACK_COUNT_FIELD: u32 = 0x140;
    const OBJECT_ZERO_FIELDS: [u32; 11] = [
        0x158, 0x160, 0x164, 0x168, 0x170, 0x174, 0x178, 0x180, 0x190, 0x194, 0x198,
    ];
    const OBJECT_SENTINEL_FIELDS: [u32; 2] = [0x144, 0x14c];
    const FIRST_BUFFER_POINTER_FIELD: u32 = 0x154;
    const SECOND_BUFFER_POINTER_FIELD: u32 = 0x15c;
    const INITIALIZED_FLAG_FIELD: u32 = 0x148;
    const WORD_FLAGS_FIELD: u32 = 0x150;
    const BYTE_FLAG_FIELD: u32 = 0x184;
    const SENTINEL_VALUE: u32 = u32::MAX;
    const INITIALIZED_FLAG_VALUE: u32 = 1;
    const WORD_FLAGS_VALUE: u16 = 0x0101;

    let first_buffer = lf_checker_rt::callee_cdecl!(ALLOC_FIRST, u32, BUFFER_BYTES);
    let second_buffer = lf_checker_rt::callee_cdecl!(ALLOC_SECOND, u32, BUFFER_BYTES);

    unsafe {
        (this.wrapping_add(TRACK_COUNT_FIELD) as *mut u32).write_unaligned(0);
        for offset in OBJECT_ZERO_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        for offset in OBJECT_SENTINEL_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(SENTINEL_VALUE);
        }
        (this.wrapping_add(INITIALIZED_FLAG_FIELD) as *mut u32)
            .write_unaligned(INITIALIZED_FLAG_VALUE);
        (this.wrapping_add(WORD_FLAGS_FIELD) as *mut u16).write_unaligned(WORD_FLAGS_VALUE);
        (this.wrapping_add(BYTE_FLAG_FIELD) as *mut u8).write_unaligned(0);

        let first_slot = (this.wrapping_add(FIRST_BUFFER_POINTER_FIELD)) as *mut u32;
        let second_slot = (this.wrapping_add(SECOND_BUFFER_POINTER_FIELD)) as *mut u32;
        first_slot.write_unaligned(first_buffer);
        second_slot.write_unaligned(second_buffer);

        let mut slot_offset = FIRST_SLOT_OFFSET;
        while slot_offset <= LAST_SLOT_OFFSET {
            (first_buffer.wrapping_add(slot_offset) as *mut u32).write_unaligned(0);
            (second_buffer.wrapping_add(slot_offset) as *mut u32).write_unaligned(0);
            slot_offset = slot_offset.wrapping_add(BUFFER_STRIDE);
        }

    }

    (second_buffer & RETURN_LOW_BYTE_MASK) | 1
});
