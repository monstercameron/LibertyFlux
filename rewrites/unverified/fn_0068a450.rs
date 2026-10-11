// original: 0x0068A450 track_key_partition_0068A450 (proposed)

/// Partition a pointer array around the key of a selected track.
///
/// The range is `[begin, end)` in ECX and EDX, and the stack argument points
/// to the pivot track. A track key is an unsigned 24-bit value made from the
/// byte at `+5` and the word at `+6`. The left cursor advances over keys
/// below the pivot; the right cursor retreats over keys above it. Outlying
/// pointers are swapped until the cursors meet. The function returns the
/// left cursor. The entry uses the 32-bit fastcall convention.
lf_checker_rt::export!(fastcall, rw_0068A450(begin: u32, end: u32, pivot_track: u32) -> u32 {
    unsafe {
        const POINTER_BYTES: u32 = 4;
        const KEY_HIGH_BYTE: u32 = 5;
        const KEY_LOW_WORD: u32 = 6;

        #[inline(always)]
        unsafe fn key(track: u32) -> u32 {
            let high = *((track.wrapping_add(KEY_HIGH_BYTE)) as *const u8) as u32;
            let low = *((track.wrapping_add(KEY_LOW_WORD)) as *const u16) as u32;
            (high << 16) | low
        }

        let pivot = key(pivot_track);
        let mut left = begin;
        let mut right = end;
        loop {
            let first_track = *((left) as *const u32);
            if key(first_track) < pivot {
                loop {
                    let next_slot = left.wrapping_add(POINTER_BYTES);
                    let next_track = *((next_slot) as *const u32);
                    left = next_slot;
                    if key(next_track) >= pivot {
                        break;
                    }
                }
            }

            let last_slot = right.wrapping_sub(POINTER_BYTES);
            let mut last_track = *((last_slot) as *const u32);
            right = last_slot;
            while key(last_track) > pivot {
                let previous_slot = right.wrapping_sub(POINTER_BYTES);
                last_track = *((previous_slot) as *const u32);
                right = previous_slot;
                if key(last_track) <= pivot {
                    break;
                }
            }

            if left >= right {
                return left;
            }
            let left_track = *((left) as *const u32);
            let right_track = *((right) as *const u32);
            *((left) as *mut u32) = right_track;
            *((right) as *mut u32) = left_track;
            left = left.wrapping_add(POINTER_BYTES);
        }
    }
});
