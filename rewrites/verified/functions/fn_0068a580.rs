// original: 0x0068A580 animation_track_copy_selected_0068A580 (proposed)

/// Copy selected source tracks into a destination track set.
///
/// Dereferencing `this` gives the destination set, which contains a 16-bit count
/// at `+0x10` and a pointer array at `+0x0c`. The first stack argument points
/// to the source set's pointer array at `+0x0c`; the third points to one
/// 32-bit weight per track. Tracks are 0x30-byte records with flags at `+4`
/// and values beginning at `+0x10`. A source flag bit `0x10` or a weight with
/// zero magnitude skips that track. Target kind 0 copies four words, kind 1
/// copies four words, kind 2 copies one word, and other kinds copy one word;
/// kinds 0, 1, and 2 also replace the target's `0x10` flag from the source.
/// The known caller ignores EAX. This is a thiscall method with three stack
/// arguments and callee cleanup.
lf_checker_rt::export!(thiscall, rw_0068A580(
    this: u32,
    source_set: u32,
    _unused: u32,
    weights: u32
) -> u32 {
    unsafe {
        const DESTINATION_SET_POINTER: u32 = 0;
        const SET_COUNT: u32 = 0x10;
        const TRACK_POINTERS: u32 = 0x0C;
        const TRACK_FLAGS: u32 = 4;
        const TRACK_VALUE: u32 = 0x10;
        const SKIP_FLAG: u8 = 0x10;
        const KIND_MASK: u8 = 0x0F;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            (address as *const u32).read_unaligned()
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            (address as *mut u32).write_unaligned(value);
        }

        let destination_set = read_u32(this.wrapping_add(DESTINATION_SET_POINTER));
        let count = (destination_set.wrapping_add(SET_COUNT) as *const u16)
            .read_unaligned() as u32;
        if count == 0 {
            return 0;
        }

        let destination_tracks = read_u32(destination_set.wrapping_add(TRACK_POINTERS));
        let source_tracks = read_u32(source_set.wrapping_add(TRACK_POINTERS));
        for index in 0..count {
            let slot_offset = index.wrapping_mul(4);
            let destination = read_u32(destination_tracks.wrapping_add(slot_offset));
            let source = read_u32(source_tracks.wrapping_add(slot_offset));
            let weight_bits = read_u32(weights.wrapping_add(slot_offset));
            let source_flags = *((source.wrapping_add(TRACK_FLAGS)) as *const u8);
            if source_flags & SKIP_FLAG != 0 || weight_bits & 0x7FFF_FFFF == 0 {
                continue;
            }

            let target_flags = *((destination.wrapping_add(TRACK_FLAGS)) as *const u8);
            let kind = target_flags & KIND_MASK;
            let words_to_copy = if kind == 0 || kind == 1 { 4 } else { 1 };
            for word in 0..words_to_copy {
                let value_offset = TRACK_VALUE + word * 4;
                let value = read_u32(source.wrapping_add(value_offset));
                write_u32(destination.wrapping_add(value_offset), value);
            }

            if kind <= 2 {
                let new_flags = if source_flags & SKIP_FLAG != 0 {
                    target_flags | SKIP_FLAG
                } else {
                    target_flags & !SKIP_FLAG
                };
                *((destination.wrapping_add(TRACK_FLAGS)) as *mut u8) = new_flags;
            }
        }
        0
    }
});
