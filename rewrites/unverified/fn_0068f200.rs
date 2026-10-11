// original: 0x0068F200 track_key_pointer_insertion_sort_0068F200 (proposed)

/// Stably insertion-sort a pointer window by each track's unsigned 24-bit key.
///
/// ECX points to the first pointer to sort and EDX is the one-past-end cursor.
/// Each key combines the byte at track offset `+5` as its high byte with the
/// word at `+6` as its low word. The pointer immediately before the window is
/// also read while inserting; inputs must provide a preceding value no greater
/// than the current value or the search continues before the window. Equal
/// keys retain their order. An empty window returns EDX; otherwise EAX returns
/// the last track pointer loaded from the final slot.
lf_checker_rt::export!(fastcall, rw_0068F200(begin: u32, end: u32) -> u32 {
    unsafe {
        const POINTER_BYTES: u32 = 4;
        #[inline(always)]
        unsafe fn track_key(track: u32) -> u32 {
            const KEY_HIGH_BYTE_OFFSET: u32 = 5;
            const KEY_LOW_WORD_OFFSET: u32 = 6;
            let high = *((track.wrapping_add(KEY_HIGH_BYTE_OFFSET)) as *const u8) as u32;
            let low = *((track.wrapping_add(KEY_LOW_WORD_OFFSET)) as *const u16) as u32;
            (high << 16) | low
        }

        let mut cursor = begin;
        let mut last_track = end;
        while cursor != end {
            let track = *((cursor) as *const u32);
            last_track = track;
            let current_key = track_key(track);
            let mut insertion_slot = cursor;
            let mut previous_slot = cursor.wrapping_sub(POINTER_BYTES);

            loop {
                let previous_track = *((previous_slot) as *const u32);
                if current_key >= track_key(previous_track) {
                    break;
                }
                *((insertion_slot) as *mut u32) = previous_track;
                insertion_slot = previous_slot;
                previous_slot = previous_slot.wrapping_sub(POINTER_BYTES);
            }

            *((insertion_slot) as *mut u32) = track;
            cursor = cursor.wrapping_add(POINTER_BYTES);
        }
        last_track
    }
});
