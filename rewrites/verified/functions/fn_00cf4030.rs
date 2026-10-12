// original: 0x00CF4030 CTaskSimpleClimb::CTaskSimpleClimb

/// Initializes the climb task's base vtable, source position, float scale,
/// five state words, sentinel fields, and packed flags. The source vector is
/// the second stack argument at the layout supplied by the caller; the scale,
/// low flag bit, and trailing word occupy the next three argument words after
/// the first constructor flag. The receiver's high flag nibble is preserved,
/// while the input's low bit and the later helper result set the low flag bits.
/// When the first global float is negative, three random-value helper calls
/// fill the three global floats; the first two have their sign bit cleared by
/// an image mask, and the third comes from the local output buffer. The
/// method is thiscall with five stack arguments and returns the receiver.
lf_checker_rt::export!(
    thiscall,
    rw_00cf4030(
        this: u32,
        constructor_flag: u32,
        source_position: u32,
        scale: f32,
        packed_flag: u32,
        trailing_word: u32
    ) -> u32 {
        unsafe {
            const VTABLE_VA: u32 = 0x00ed_f154;
            const FLOAT_MASK_VA: u32 = 0x00fe_8f80;
            const RANDOM_STATE_VA: u32 = 0x0105_3920;
            const RANDOM_X_VA: u32 = 0x0105_3920;
            const RANDOM_Y_VA: u32 = 0x0105_3924;
            const RANDOM_Z_VA: u32 = 0x0105_3928;
            const SOURCE_X: u32 = 0;
            const SOURCE_Y: u32 = 4;
            const SOURCE_Z: u32 = 8;
            const GLOBAL_VTABLE: u32 = 0;
            const BASE_STATE: u32 = 0x14;
            const POSITION_X: u32 = 0x20;
            const POSITION_Y: u32 = 0x24;
            const POSITION_Z: u32 = 0x28;
            const SCALE: u32 = 0x30;
            const ZERO_FIELDS: [u32; 8] = [0x40, 0x44, 0x48, 0x50, 0x54, 0x58, 0x60, 0x64];
            const ARGUMENT_FIELD: u32 = 0x68;
            const MORE_ZERO_FIELDS: [u32; 4] = [0x6c, 0x70, 0x80, 0x84];
            const SENTINELS: [u32; 3] = [0x74, 0x78, 0x7c];
            const BYTE_SENTINEL: u32 = 0x88;
            const PACKED_FLAGS: u32 = 0x89;
            const CALLEE_BASE_CONSTRUCTOR: u32 = 1;
            const CALLEE_FINISH_FLAGS: u32 = 2;
            const CALLEE_RANDOM_VALUE: u32 = 3;
            const RANDOM_SEED_BITS: u32 = 0x3f80_0000;
            const RANDOM_GROUP: u32 = 9;
            const RANDOM_FIRST_KIND: u32 = 0x7b;
            const RANDOM_SECOND_KIND: u32 = 0x7c;
            const RANDOM_THIRD_KIND: u32 = 0x69;

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
                unsafe { (address as *const u8).read_unaligned() }
            }

            #[inline(always)]
            unsafe fn write_byte(address: u32, value: u8) {
                unsafe { (address as *mut u8).write_unaligned(value) }
            }

            let _ = lf_checker_rt::callee_thiscall!(CALLEE_BASE_CONSTRUCTOR, u32, this);
            unsafe { write_word(this + GLOBAL_VTABLE, lf_checker_rt::relocated(VTABLE_VA)) };
            unsafe { write_word(this + BASE_STATE, 0) };
            unsafe { write_word(this + POSITION_X, read_word(source_position + SOURCE_X)) };
            unsafe { write_word(this + POSITION_Y, read_word(source_position + SOURCE_Y)) };
            unsafe { write_word(this + POSITION_Z, read_word(source_position + SOURCE_Z)) };
            unsafe { write_word(this + SCALE, scale.to_bits()) };
            unsafe { write_word(this + ARGUMENT_FIELD, trailing_word) };
            for offset in ZERO_FIELDS {
                unsafe { write_word(this + offset, 0) };
            }
            for offset in MORE_ZERO_FIELDS {
                unsafe { write_word(this + offset, 0) };
            }
            for offset in SENTINELS {
                unsafe { write_word(this + offset, u32::MAX) };
            }
            unsafe { write_byte(this + BYTE_SENTINEL, u8::MAX) };
            let preserved_high_flags = unsafe { read_byte(this + PACKED_FLAGS) } & 0xf0;
            unsafe { write_byte(this + PACKED_FLAGS, preserved_high_flags | (packed_flag as u8 & 1)) };

            let finish_flags =
                lf_checker_rt::callee_thiscall!(CALLEE_FINISH_FLAGS, u32, this, constructor_flag);
            if finish_flags != 0 {
                let flags = unsafe { read_byte(this + PACKED_FLAGS) };
                unsafe { write_byte(this + PACKED_FLAGS, flags | 8) };
            }

            let initial_random_state = unsafe {
                f32::from_bits(read_word(lf_checker_rt::relocated(RANDOM_STATE_VA)))
            };
            if !(0.0f32 > initial_random_state) {
                return this;
            }

            // These words model the original's caller-frame output buffers.
            // The first two calls reuse one four-word area; the third uses a
            // separate area. The contract skips each pointer and snapshots its
            // four pointed-to words at every call.
            let mut local_scratch = [0u32; 12];
            let float_mask = unsafe { read_word(lf_checker_rt::relocated(FLOAT_MASK_VA)) };

            let first_out = unsafe { local_scratch.as_mut_ptr().add(8) as u32 };
            let first_pointer = lf_checker_rt::callee_cdecl!(
                CALLEE_RANDOM_VALUE,
                u32,
                first_out,
                RANDOM_SEED_BITS,
                RANDOM_GROUP,
                RANDOM_FIRST_KIND
            );
            let first_bits = unsafe { read_word(first_pointer) } & float_mask;
            unsafe { write_word(lf_checker_rt::relocated(RANDOM_X_VA), first_bits) };

            let second_out = unsafe { local_scratch.as_mut_ptr().add(8) as u32 };
            let second_pointer = lf_checker_rt::callee_cdecl!(
                CALLEE_RANDOM_VALUE,
                u32,
                second_out,
                RANDOM_SEED_BITS,
                RANDOM_GROUP,
                RANDOM_SECOND_KIND
            );
            let second_bits = unsafe { read_word(second_pointer) } & float_mask;
            unsafe { write_word(lf_checker_rt::relocated(RANDOM_Y_VA), second_bits) };

            let third_out = unsafe { local_scratch.as_mut_ptr().add(4) as u32 };
            let _ = lf_checker_rt::callee_cdecl!(
                CALLEE_RANDOM_VALUE,
                u32,
                third_out,
                RANDOM_SEED_BITS,
                RANDOM_GROUP,
                RANDOM_THIRD_KIND
            );
            unsafe { write_word(lf_checker_rt::relocated(RANDOM_Z_VA), local_scratch[5]) };

            this
        }
    }
);
