// original: 0x00DFF29E wcsicmp_l
/// Compare two NUL-terminated wide strings using the supplied locale context.
///
/// Null string pointers set the thread error slot to EINVAL and return i32::MAX.
/// The ASCII locale path folds only A through Z; the locale mapping path
/// delegates each code unit to the locale character worker.
const LOCALE_CASE_FLAG_OFFSET: usize = 0xA8;
const ERROR_INVALID_ARGUMENT: u32 = 0x16;
const COMPARISON_ERROR: u32 = 0x7FFF_FFFF;

fn ascii_lowercase(code_unit: u16) -> u16 {
    if (0x41..=0x5A).contains(&code_unit) {
        code_unit + 0x20
    } else {
        code_unit
    }
}

unsafe fn compare_wide_strings(
    mut left: u32,
    mut right: u32,
    locale_info: u32,
    locale_context: u32,
) -> u32 {
    let uses_locale_mapping =
        ((locale_info as *const u8).add(LOCALE_CASE_FLAG_OFFSET) as *const u32).read() != 0;

    loop {
        let left_code = (left as *const u16).read();
        let right_code = (right as *const u16).read();

        let (left_folded, right_folded) = if uses_locale_mapping {
            let left_folded =
                callee_cdecl!(4, u32, u32::from(left_code), locale_context) as u16;
            let right_folded =
                callee_cdecl!(4, u32, u32::from(right_code), locale_context) as u16;
            (left_folded, right_folded)
        } else {
            (ascii_lowercase(left_code), ascii_lowercase(right_code))
        };

        left = left.wrapping_add(2);
        right = right.wrapping_add(2);

        if left_folded == 0 || left_folded != right_folded {
            return u32::from(left_folded).wrapping_sub(u32::from(right_folded));
        }
    }
}

export!(cdecl, rw_00dff29e(left: u32, right: u32, locale: u32) -> u32 {
    unsafe {
        let mut locale_state = [0u32; 4];
        let locale_info_slot = locale_state.as_mut_ptr() as u32;
        let _ = callee_thiscall!(1, u32, locale_info_slot, locale);

        if left == 0 || right == 0 {
            let error_slot = callee_cdecl!(2, u32,);
            (error_slot as *mut u32).write(ERROR_INVALID_ARGUMENT);
            let _ = callee_cdecl!(3, u32,);
            return COMPARISON_ERROR;
        }

        compare_wide_strings(left, right, locale_state[0], locale)
    }
});