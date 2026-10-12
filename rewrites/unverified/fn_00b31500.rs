// original: 0x00B31500 shocking_response_weighted_pick

/// Select one of five responses from the entry for a response kind and
/// category. The cdecl arguments are the category index, the response kind,
/// and the paired interaction kind. The pair (kind 3 through 11, interaction
/// kind 14 or 28), and its reverse (kind 14 or 28, interaction kind 3 through
/// 11), return response 4 without consulting the table. Other inputs read a
/// six-byte table entry at `0x0166142C + kind * 0x34 + category * 6`: an
/// acceptance percentage followed by five unsigned response weights. A
/// 16-bit CRT random value is scaled by the image's `1/32768` and `100.0`
/// constants and truncated to signed i32; the entry is rejected only when
/// that nonnegative value is signed-greater than the unsigned percentage.
/// Accepted entries draw again, scale by the sum of the five weights, and
/// return the first index whose cumulative unsigned weight is signed-greater
/// than the nonnegative truncated draw. Empty or unmatched entries return
/// -1. The original's security-cookie check preserves the result in EAX.
/// The function returns i32 in EAX and uses caller-cleaned cdecl arguments.
lf_checker_rt::export!(cdecl, rw_00b31500(
    category_index: u32,
    response_kind: u32,
    interaction_kind: u32,
) -> i32 {
    unsafe {
        const RESPONSE_TABLE_VA: u32 = 0x0166_142C;
        const RESPONSE_KIND_STRIDE: u32 = 0x34;
        const CATEGORY_STRIDE: u32 = 6;
        const ACCEPTANCE_OFFSET: u32 = 0;
        const WEIGHT_COUNT: u32 = 5;
        const FIRST_WEIGHT_OFFSET: u32 = 1;
        const SPECIAL_KIND_MIN: u32 = 3;
        const SPECIAL_KIND_MAX: u32 = 11;
        const SPECIAL_KIND_A: u32 = 14;
        const SPECIAL_KIND_B: u32 = 28;
        const RANDOM_CALLEE: u32 = 0x00DF_C099;
        const SECURITY_COOKIE_CHECK: u32 = 0x00DF_8AAE;
        const SECURITY_COOKIE_VA: u32 = 0x0105_7FB4;
        const RANDOM_FRACTION_SCALE_VA: u32 = 0x00FE_8680;
        const RANDOM_PERCENT_SCALE_VA: u32 = 0x00FE_8BB0;

        #[inline(always)]
        unsafe fn read_image_f32(image_va: u32) -> f32 {
            let bits = unsafe { lf_checker_rt::global::<u32>(image_va).read() };
            f32::from_bits(bits)
        }

        #[inline(always)]
        unsafe fn read_image_u8(image_va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(image_va).read() }
        }

        #[inline(always)]
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn random_word() -> u32 {
            lf_checker_rt::callee_cdecl!(RANDOM_CALLEE, u32,) & 0xffff
        }

        #[inline(always)]
        unsafe fn check_security_cookie() {
            let cookie = unsafe {
                lf_checker_rt::global::<u32>(SECURITY_COOKIE_VA).read()
            };
            let _ = lf_checker_rt::callee_thiscall!(
                SECURITY_COOKIE_CHECK,
                u32,
                cookie
            );
        }

        let special_other_kind =
            interaction_kind == SPECIAL_KIND_A || interaction_kind == SPECIAL_KIND_B;
        let special_response_kind =
            response_kind == SPECIAL_KIND_A || response_kind == SPECIAL_KIND_B;
        let response_is_common_kind =
            (SPECIAL_KIND_MIN..=SPECIAL_KIND_MAX).contains(&response_kind);
        let interaction_is_common_kind =
            (SPECIAL_KIND_MIN..=SPECIAL_KIND_MAX).contains(&interaction_kind);

        if (response_is_common_kind && special_other_kind)
            || (special_response_kind && interaction_is_common_kind)
        {
            unsafe { check_security_cookie() };
            return 4;
        }

        let entry = RESPONSE_TABLE_VA
            .wrapping_add(response_kind.wrapping_mul(RESPONSE_KIND_STRIDE))
            .wrapping_add(category_index.wrapping_mul(CATEGORY_STRIDE));
        let acceptance_threshold = unsafe {
            read_image_u8(entry.wrapping_add(ACCEPTANCE_OFFSET))
        };

        let first_random = unsafe { random_word() };
        let first_fraction = ordered_mul(
            first_random as f32,
            unsafe { read_image_f32(RANDOM_FRACTION_SCALE_VA) },
        );
        let random_percent = ordered_mul(
            first_fraction,
            unsafe { read_image_f32(RANDOM_PERCENT_SCALE_VA) },
        );
        // `first_random <= 65,535`; with the image constants the result is
        // finite and below 200, matching the original CVTTSS2SI conversion.
        let random_percent_i32 = random_percent as i32;

        if random_percent_i32 > i32::from(acceptance_threshold) {
            unsafe { check_security_cookie() };
            return -1;
        }

        let mut weights = [0u8; WEIGHT_COUNT as usize];
        let mut total_weight = 0u32;
        let mut index = 0u32;
        while index < WEIGHT_COUNT {
            let weight = unsafe {
                read_image_u8(entry.wrapping_add(FIRST_WEIGHT_OFFSET + index))
            };
            weights[index as usize] = weight;
            total_weight += u32::from(weight);
            index += 1;
        }

        if total_weight == 0 {
            unsafe { check_security_cookie() };
            return -1;
        }

        let second_random = unsafe { random_word() };
        let second_fraction = ordered_mul(
            second_random as f32,
            unsafe { read_image_f32(RANDOM_FRACTION_SCALE_VA) },
        );
        let random_weight = ordered_mul(second_fraction, total_weight as f32);
        // The five byte weights sum to at most 1,275, so this conversion is
        // finite and in range; CVTTSS2SI therefore agrees with the cast.
        let random_weight_i32 = random_weight as i32;

        let mut cumulative_weight = 0i32;
        let mut response_index = 0i32;
        while response_index < WEIGHT_COUNT as i32 {
            cumulative_weight += i32::from(weights[response_index as usize]);
            if random_weight_i32 < cumulative_weight {
                unsafe { check_security_cookie() };
                return response_index;
            }
            response_index += 1;
        }

        unsafe { check_security_cookie() };
        -1
    }
});
