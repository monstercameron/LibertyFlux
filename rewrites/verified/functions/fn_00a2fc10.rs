// original: 0x00A2FC10 initialize_emergency_allocator_state

/// Initializes the fields of an emergency allocator state object.
///
/// The thiscall entry receives the object in ECX and returns that same pointer.
/// It installs the relocated image vtable, clears the documented counters and
/// links, applies masks to the existing state words at offsets 0x28, 0x54,
/// 0x5C, and 0x5E, and fills the fixed sentinel fields. A no-argument helper
/// supplies the low 16 bits stored at offset 0x2C. The object uses the
/// original 32-bit layout; every offset below is a byte offset.
lf_checker_rt::export!(thiscall, rw_00a2fc10(state: u32) -> u32 {
    const VTABLE_ADDRESS: u32 = 0x00E9_C454;
    const VTABLE_OFFSET: usize = 0x00;
    const FIELD_TAG_OFFSET: usize = 0x24;
    const FIELD_TAG_VALUE: u32 = 0x8000_0020;
    const STATE_FLAGS: usize = 0x28;
    const STATE_CONTROL_OFFSET: usize = 0x2E;
    const STATE_WORD_5C: usize = 0x5C;
    const STATE_WORD_5E: usize = 0x5E;
    const STATE_HIGH_BYTE: usize = 0x54;
    const HELPER_VALUE_OFFSET: usize = 0x2C;
    const VALIDATION_WORD_OFFSET: usize = 0x40;
    const BOUND_WORD_A_OFFSET: usize = 0x42;
    const BOUND_WORD_B_OFFSET: usize = 0x44;
    const ALL_BITS_WORD_OFFSET: usize = 0x48;
    const CONTROL_WORD_OFFSET: usize = 0x61;
    const CONTROL_BYTE_OFFSET: usize = 0x63;
    const FINAL_WORD_OFFSET: usize = 0x58;
    const FINAL_BYTE_OFFSET: usize = 0x60;
    const FINAL_TAG_OFFSET: usize = 0x08;
    const FINAL_WORD_A_OFFSET: usize = 0x64;
    const FINAL_WORD_B_OFFSET: usize = 0x68;
    const ZEROED_WORD_OFFSETS: [usize; 9] =
        [0x10, 0x14, 0x18, 0x1C, 0x6C, 0x20, 0x3C, 0x38, 0x34];
    const POST_HELPER_ZEROED_WORD_OFFSETS: [usize; 3] = [0x30, 0x4C, 0x50];
    const FLAGS_KEEP_MASK: u32 = 0xC00C_880D;
    const FLAGS_SET_MASK: u32 = 0x0000_080C;
    const FLAGS_CLEAR_AFTER_HELPER: u32 = 0x0008_0000;
    const WORD_5C_KEEP_MASK: u16 = 0xC000;
    const WORD_5E_CLEAR_BIT_ZERO_MASK: u16 = 0xFFFE;
    const WORD_5E_CLEAR_BIT_ONE_MASK: u16 = 0xFFFD;
    const VALIDATION_WORD_VALUE: u16 = 0x003F;
    const CONTROL_WORD_VALUE: u16 = 0xFF00;
    const CONTROL_BYTE_VALUE: u8 = 0xFF;
    const FINAL_WORD_VALUE: u32 = 0xFF00_0000;
    const FINAL_TAG_VALUE: u32 = 0x4000_0000;
    const HELPER_CLOCK: u32 = 1;

    unsafe {
        let object = state as *mut u8;
        let relocated_vtable =
            lf_checker_rt::global::<u32>(VTABLE_ADDRESS) as usize as u32;
        object
            .add(VTABLE_OFFSET)
            .cast::<u32>()
            .write_unaligned(relocated_vtable);

        for offset in ZEROED_WORD_OFFSETS {
            object.add(offset).cast::<u32>().write_unaligned(0);
        }

        let original_flags = object.add(STATE_FLAGS).cast::<u32>().read_unaligned();
        let initialized_flags = (original_flags & FLAGS_KEEP_MASK) | FLAGS_SET_MASK;
        object
            .add(STATE_FLAGS)
            .cast::<u32>()
            .write_unaligned(initialized_flags);

        let original_5c = object.add(STATE_WORD_5C).cast::<u16>().read_unaligned();
        object
            .add(STATE_WORD_5C)
            .cast::<u16>()
            .write_unaligned(original_5c & WORD_5C_KEEP_MASK);
        let original_5e = object.add(STATE_WORD_5E).cast::<u16>().read_unaligned();
        object
            .add(STATE_WORD_5E)
            .cast::<u16>()
            .write_unaligned(original_5e & WORD_5E_CLEAR_BIT_ZERO_MASK);

        object
            .add(FIELD_TAG_OFFSET)
            .cast::<u32>()
            .write_unaligned(FIELD_TAG_VALUE);
        object
            .add(STATE_CONTROL_OFFSET)
            .cast::<u16>()
            .write_unaligned(u16::MAX);
        object
            .add(VALIDATION_WORD_OFFSET)
            .cast::<u16>()
            .write_unaligned(VALIDATION_WORD_VALUE);
        object
            .add(BOUND_WORD_A_OFFSET)
            .cast::<u16>()
            .write_unaligned(u16::MAX);
        object
            .add(BOUND_WORD_B_OFFSET)
            .cast::<u16>()
            .write_unaligned(u16::MAX);
        object
            .add(ALL_BITS_WORD_OFFSET)
            .cast::<u32>()
            .write_unaligned(u32::MAX);

        let clock_value = lf_checker_rt::callee_cdecl!(HELPER_CLOCK, u32,);
        object
            .add(HELPER_VALUE_OFFSET)
            .cast::<u16>()
            .write_unaligned(clock_value as u16);

        let high_byte = object.add(STATE_HIGH_BYTE).cast::<u32>().read_unaligned();
        object
            .add(STATE_HIGH_BYTE)
            .cast::<u32>()
            .write_unaligned(high_byte & 0xFF00_0000);
        let flags_after_helper = object
            .add(STATE_FLAGS)
            .cast::<u32>()
            .read_unaligned();
        object
            .add(STATE_FLAGS)
            .cast::<u32>()
            .write_unaligned(flags_after_helper & !FLAGS_CLEAR_AFTER_HELPER);
        let word_5e_after_helper = object
            .add(STATE_WORD_5E)
            .cast::<u16>()
            .read_unaligned();
        object
            .add(STATE_WORD_5E)
            .cast::<u16>()
            .write_unaligned(word_5e_after_helper & WORD_5E_CLEAR_BIT_ONE_MASK);
        for offset in POST_HELPER_ZEROED_WORD_OFFSETS {
            object.add(offset).cast::<u32>().write_unaligned(0);
        }
        object
            .add(CONTROL_WORD_OFFSET)
            .cast::<u16>()
            .write_unaligned(CONTROL_WORD_VALUE);
        object.add(CONTROL_BYTE_OFFSET).write(CONTROL_BYTE_VALUE);
        object
            .add(FINAL_WORD_OFFSET)
            .cast::<u32>()
            .write_unaligned(FINAL_WORD_VALUE);
        object.add(FINAL_BYTE_OFFSET).write(CONTROL_BYTE_VALUE);
        object
            .add(FINAL_TAG_OFFSET)
            .cast::<u32>()
            .write_unaligned(FINAL_TAG_VALUE);
        object
            .add(FINAL_WORD_A_OFFSET)
            .cast::<u32>()
            .write_unaligned(u32::MAX);
        object
            .add(FINAL_WORD_B_OFFSET)
            .cast::<u32>()
            .write_unaligned(u32::MAX);
        state
    }
});
