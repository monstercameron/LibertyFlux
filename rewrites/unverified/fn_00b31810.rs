// original: 0x00B31810 shocking_events_dat_parse_line

/// Parse up to 30 delimited numeric fields into a 0x34-byte event row. ECX
/// points to the row; the two caller-provided stack arguments are the input
/// text pointer and the value copied to row offset 0. The delimiter set is
/// loaded from the image. Each nonempty following token is converted by the
/// intercepted one-argument decimal parser. Fields 1, 3, and 4 are stored as
/// raw dwords; fields 2, 5, and 6 are multiplied by 1,000 with 32-bit wrapping
/// before storage; fields 7 through 30 store only the low byte. The parser
/// asks for a token after each processed field and stops on a null token or
/// once field 30 has been stored. The return register is the final tokenizer
/// result, and the thiscall callee removes its two stack arguments.
lf_checker_rt::export!(thiscall, rw_00b31810(
    record_ptr: u32,
    input_text_ptr: u32,
    initial_field_value: u32,
) -> u32 {
    unsafe {
        const TOKENIZER_CALLEE_ID: u32 = 1;
        const FIRST_NUMBER_CALLEE_ID: u32 = 2;
        const DELIMITER_SET_VA: u32 = 0x00EA_CDAC;
        const RECORD_LINE_VALUE: u32 = 0x00;
        const RAW_FIELD_1: u32 = 0x04;
        const SCALED_FIELD_2: u32 = 0x08;
        const RAW_FIELD_3: u32 = 0x0C;
        const RAW_FIELD_4: u32 = 0x10;
        const SCALED_FIELD_5: u32 = 0x14;
        const SCALED_FIELD_6: u32 = 0x18;
        const BYTE_FIELDS_START_INDEX: u32 = 7;
        const BYTE_FIELDS_START_OFFSET: u32 = 0x1C;
        const LAST_FIELD_INDEX: u32 = 30;
        const TIME_SCALE: u32 = 1_000;

        #[inline(always)]
        unsafe fn read_image_u8(image_va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(image_va).read() }
        }

        #[inline(always)]
        unsafe fn write_record_u32(record_ptr: u32, offset: u32, value: u32) {
            unsafe { ((record_ptr.wrapping_add(offset)) as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_record_u8(record_ptr: u32, offset: u32, value: u8) {
            unsafe { ((record_ptr.wrapping_add(offset)) as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn next_token(input_ptr: u32, separators_ptr: u32) -> u32 {
            lf_checker_rt::callee_cdecl!(
                TOKENIZER_CALLEE_ID,
                u32,
                input_ptr,
                separators_ptr
            )
        }

        #[inline(always)]
        unsafe fn parse_decimal(token_ptr: u32, field_index: u32) -> u32 {
            lf_checker_rt::callee_cdecl!(
                FIRST_NUMBER_CALLEE_ID + field_index - 1,
                u32,
                token_ptr
            )
        }

        let mut separators = [
            unsafe { read_image_u8(DELIMITER_SET_VA) },
            unsafe { read_image_u8(DELIMITER_SET_VA + 1) },
            unsafe { read_image_u8(DELIMITER_SET_VA + 2) },
            (record_ptr >> 24) as u8,
        ];
        let separators_ptr = separators.as_mut_ptr() as u32;

        let _first_token = unsafe { next_token(input_text_ptr, separators_ptr) };
        unsafe { write_record_u32(record_ptr, RECORD_LINE_VALUE, initial_field_value) };

        let mut field_index = 1u32;
        let mut token_result = 0u32;
        loop {
            token_result = unsafe { next_token(0, separators_ptr) };
            if token_result == 0 || field_index > LAST_FIELD_INDEX {
                break;
            }

            let parsed_value = unsafe { parse_decimal(token_result, field_index) };
            match field_index {
                1 => unsafe { write_record_u32(record_ptr, RAW_FIELD_1, parsed_value) },
                2 => unsafe {
                    write_record_u32(
                        record_ptr,
                        SCALED_FIELD_2,
                        parsed_value.wrapping_mul(TIME_SCALE),
                    )
                },
                3 => unsafe { write_record_u32(record_ptr, RAW_FIELD_3, parsed_value) },
                4 => unsafe { write_record_u32(record_ptr, RAW_FIELD_4, parsed_value) },
                5 => unsafe {
                    write_record_u32(
                        record_ptr,
                        SCALED_FIELD_5,
                        parsed_value.wrapping_mul(TIME_SCALE),
                    )
                },
                6 => unsafe {
                    write_record_u32(
                        record_ptr,
                        SCALED_FIELD_6,
                        parsed_value.wrapping_mul(TIME_SCALE),
                    )
                },
                _ => {
                    let byte_offset = BYTE_FIELDS_START_OFFSET
                        .wrapping_add(field_index - BYTE_FIELDS_START_INDEX);
                    unsafe { write_record_u8(record_ptr, byte_offset, parsed_value as u8) };
                }
            }

            field_index += 1;
        }

        token_result
    }
});
