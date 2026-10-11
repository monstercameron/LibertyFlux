// original: 0x00DFD4CF strnicmp_l
/// Compare at most `count` bytes using the locale-aware, case-insensitive CRT rules.
///
/// A zero count returns zero. Invalid pointers or an out-of-range count set EINVAL
/// and return the CRT comparison error value. The ASCII path folds A through Z;
/// the locale path uses the locale character worker.
const LOCALE_CASE_FLAG_OFFSET: usize = 0xA8;
const ERROR_INVALID_ARGUMENT: u32 = 0x16;
const COMPARISON_ERROR: u32 = 0x7FFF_FFFF;

fn ascii_lowercase(byte: u8) -> u32 {
    if (b'A'..=b'Z').contains(&byte) {
        u32::from(byte + 0x20)
    } else {
        u32::from(byte)
    }
}

unsafe fn compare_bytes(
    left: u32,
    right: u32,
    count: u32,
    locale_state: u32,
    locale_context: u32,
) -> i32 {
    let locale_info = (locale_state as *const u32).read();
    let uses_locale_mapping =
        ((locale_info as *const u8).add(LOCALE_CASE_FLAG_OFFSET) as *const u32).read() != 0;

    for index in 0..count {
        let left_byte = (left as *const u8).add(index as usize).read();
        let right_byte = (right as *const u8).add(index as usize).read();
        let (left_folded, right_folded) = if uses_locale_mapping {
            let state_pointer = locale_state;
            let left_folded = callee_cdecl!(4, u32, u32::from(left_byte), state_pointer);
            let right_folded = callee_cdecl!(4, u32, u32::from(right_byte), state_pointer);
            (left_folded, right_folded)
        } else {
            let _ = locale_context;
            (ascii_lowercase(left_byte), ascii_lowercase(right_byte))
        };

        if left_folded == 0 || left_folded != right_folded {
            return left_folded.wrapping_sub(right_folded) as i32;
        }
    }

    0
}

export!(cdecl, rw_00dfd4cf(left: u32, right: u32, count: u32, locale: u32) -> i32 {
    unsafe {
        if count == 0 {
            return 0;
        }

        let mut locale_state = [0u32; 4];
        let locale_state_pointer = locale_state.as_mut_ptr() as u32;
        let _ = callee_thiscall!(1, u32, locale_state_pointer, locale);

        if left == 0 || right == 0 || count > 0x7FFF_FFFF {
            let error_slot = callee_cdecl!(2, u32,);
            (error_slot as *mut u32).write(ERROR_INVALID_ARGUMENT);
            let _ = callee_cdecl!(3, u32,);
            return COMPARISON_ERROR as i32;
        }

        compare_bytes(left, right, count, locale_state_pointer, locale)
    }
});
