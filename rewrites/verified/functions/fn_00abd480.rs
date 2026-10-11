// original: 0x00ABD480 setup_probe_kernel

/// Configure a probe object, fold two pairs of scripted virtual-call values
/// into object tags, then send the selected input-vector differences through
/// a final helper chain.
///
/// The first allocation and constructor provide the first virtual-call
/// object. A second allocation receives the counter-derived tag and two
/// copied global fields before its virtual calls. Both folds use signed
/// remainder and signed division by sixteen. A single-precision quotient is
/// formed from a fixed numerator and a global denominator; its intermediate
/// helper return is not used. The final branch tests the global byte and a
/// selected float for inequality with zero, including unordered input, then
/// returns the last helper's value.
///
/// The argument points to four input words. The only stack argument is read
/// in the two final branches; the intercepted allocation and helper calls
/// preserve all other observable effects.
lf_checker_rt::export!(cdecl, rw_00abd480(arg0: u32) -> u32 {
    unsafe {
        const CONTROL_BYTE: u32 = 0x0103_6AD0;
        const VARIANT: u32 = 0x0117_4790;
        const MAIN_FLOATS: u32 = 0x0154_CC00;
        const COLOR_WORDS: u32 = 0x015E_8918;
        const COLOR_FLOATS: u32 = 0x015E_891C;
        const FLOAT_BLOCK: u32 = 0x0154_CBC0;
        const COLOR_SCALE: u32 = 0x00FE_86E8;
        const QUOTIENT_NUMERATOR: u32 = 0x00FE_88E8;
        const GLOBAL_COUNTER: u32 = 0x0103_27A0;
        const FIELD_SOURCE_A: u32 = 0x0154_E278;
        const FIELD_SOURCE_B: u32 = 0x0154_E27C;
        const QUOTIENT_DENOMINATOR: u32 = 0x0103_F430;
        const ID6_INPUT_WORD: u32 = 0x0172_0FD0;
        const ID6_ARGUMENT: u32 = 0x00AD_2F20;
        const PLACEHOLDER_TABLE: u32 = 0x00E7_E048;
        const REAL_TABLE: u32 = 0x00EA_5B40;
        const FLAG_MASK: u32 = 0x01FF_C000;

        #[inline(always)]
        unsafe fn g32(address: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(address).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(address: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(address).read() }
        }
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        fn signed_mod16(value: u32) -> u32 {
            (value as i32 % 16) as u32
        }
        #[inline(always)]
        fn fold_tag(tag: u32, first: u32, second: u32) -> u32 {
            let remainder = signed_mod16(first);
            let bias = signed_mod16(16u32.wrapping_sub(remainder));
            let combined = second.wrapping_add(bias);
            let shifted = ((combined as i32 / 16).wrapping_shl(14)) as u32;
            tag ^ ((shifted ^ tag) & FLAG_MASK)
        }

        // The flag selects the special tail when the global byte is set and
        // the indexed sample does not compare equal to zero; NaN is unequal.
        let variant = g32(VARIANT);
        let control_is_set = g8(CONTROL_BYTE) != 0;
        let indexed_sample = f32::from_bits(g32(
            MAIN_FLOATS.wrapping_add(variant.wrapping_mul(128)),
        ));
        let special_branch = control_is_set && indexed_sample != 0.0;

        // First object: the first two allocator results are non-null in the
        // contract, so both virtual calls execute on a valid object.
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let first_object: u32 =
            lf_checker_rt::callee_cdecl!(2, u32, 0x70u32, 0u32);
        let edi = if first_object == 0 {
            0
        } else {
            let color = g32(COLOR_WORDS.wrapping_add(variant.wrapping_mul(0x210)));
            let scale = f32::from_bits(g32(COLOR_SCALE));
            let red = ((color >> 16) as u8 as i32) as f32;
            let green = ((color >> 8) as u8 as i32) as f32;
            let blue = (color as u8 as i32) as f32;
            let _scaled_blue = core::hint::black_box(blue) * core::hint::black_box(scale);

            let mut argument_one = [0u32; 4];
            let mut argument_four = [0u32; 4];
            let mut argument_seven = [
                (core::hint::black_box(red) * core::hint::black_box(scale)).to_bits(),
                (core::hint::black_box(green) * core::hint::black_box(scale)).to_bits(),
                (core::hint::black_box(blue) * core::hint::black_box(scale)).to_bits(),
                g32(COLOR_FLOATS.wrapping_add(variant.wrapping_mul(0x210))),
            ];
            let table_rows = lf_checker_rt::relocated(FLOAT_BLOCK)
                .wrapping_add(variant.wrapping_mul(128));
            lf_checker_rt::callee_thiscall!(
                3,
                u32,
                first_object,
                1u32,
                argument_one.as_mut_ptr() as u32,
                0u32,
                table_rows,
                argument_four.as_mut_ptr() as u32,
                0u32,
                0u32,
                argument_seven.as_mut_ptr() as u32,
                g32(0x017E_D954),
            )
        };

        let first_answer = lf_checker_rt::callee_thiscall!(4, u32, edi);
        let first_second = lf_checker_rt::callee_thiscall!(4, u32, edi);
        let first_tag = rd32(edi.wrapping_add(4));
        wr32(
            edi.wrapping_add(4),
            fold_tag(first_tag, first_answer, first_second),
        );

        // Second object: stamp the temporary and real tables, combine the
        // current global counter with its tag, then advance that counter.
        let esi: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0x10u32, 0u32);
        if esi != 0 {
            let tag = rd32(esi.wrapping_add(4));
            wr32(esi, lf_checker_rt::relocated(PLACEHOLDER_TABLE));
            let counter = g32(GLOBAL_COUNTER);
            let tag_mask = (tag ^ counter) & 0x3FFF;
            wr32(esi.wrapping_add(4), tag ^ tag_mask);
            let counter_ptr = lf_checker_rt::global::<u32>(GLOBAL_COUNTER);
            counter_ptr.write_unaligned(counter.wrapping_add(1));
            wr32(esi, lf_checker_rt::relocated(REAL_TABLE));
            wr32(esi.wrapping_add(8), g32(FIELD_SOURCE_A));
            wr32(esi.wrapping_add(0x0C), g32(FIELD_SOURCE_B));
        }

        let second_answer = lf_checker_rt::callee_thiscall!(4, u32, esi);
        let second_first = lf_checker_rt::callee_thiscall!(4, u32, esi);
        let second_tag = rd32(esi.wrapping_add(4));
        wr32(
            esi.wrapping_add(4),
            fold_tag(second_tag, second_answer, second_first),
        );

        // The virtual-call folds update object tags. The tail quotient uses
        // the fixed numerator loaded for this path and its global divisor.
        let numerator = f32::from_bits(g32(QUOTIENT_NUMERATOR));
        let denominator = f32::from_bits(g32(QUOTIENT_DENOMINATOR));
        let quotient = core::hint::black_box(numerator) / core::hint::black_box(denominator);
        let _unused_float = lf_checker_rt::callee_cdecl!(5, f32,);
        let mut input_word = g32(ID6_INPUT_WORD);
        let mut quotient_word = quotient.to_bits();
        let _ = lf_checker_rt::callee_cdecl!(
            6,
            u32,
            lf_checker_rt::relocated(ID6_ARGUMENT),
            &mut input_word as *mut u32 as u32,
            &mut quotient_word as *mut u32 as u32,
        );

        let input_vector = arg0;
        if special_branch {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        }
        let final_object: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0x20u32, 0u32);
        if final_object == 0 {
            return lf_checker_rt::callee_cdecl!(9, u32, 0u32);
        }

        let x0 = f32::from_bits(rd32(input_vector));
        let x1 = f32::from_bits(rd32(input_vector.wrapping_add(4)));
        let x2 = f32::from_bits(rd32(input_vector.wrapping_add(8)));
        let x3 = f32::from_bits(rd32(input_vector.wrapping_add(12)));
        let difference_one = (core::hint::black_box(x3) - core::hint::black_box(x1)).to_bits();
        let difference_two = (core::hint::black_box(x2) - core::hint::black_box(x0)).to_bits();

        let mut zero_a = 0u32;
        let mut zero_b = 0u32;
        let mut one = 1u32;
        let mut difference_two_cell = difference_two;
        let mut difference_one_cell = difference_one;
        let answer = if special_branch {
            lf_checker_rt::callee_thiscall!(
                8,
                u32,
                final_object,
                &mut one as *mut u32 as u32,
                &mut zero_a as *mut u32 as u32,
                input_vector,
                input_vector.wrapping_add(4),
                &mut difference_two_cell as *mut u32 as u32,
                &mut difference_one_cell as *mut u32 as u32,
            )
        } else {
            lf_checker_rt::callee_thiscall!(
                8,
                u32,
                final_object,
                &mut zero_a as *mut u32 as u32,
                &mut zero_b as *mut u32 as u32,
                input_vector,
                input_vector.wrapping_add(4),
                &mut difference_two_cell as *mut u32 as u32,
                &mut difference_one_cell as *mut u32 as u32,
            )
        };
        lf_checker_rt::callee_cdecl!(9, u32, answer)
    }
});
