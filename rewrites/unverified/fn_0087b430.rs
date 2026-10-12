// original: 0x0087B430 append_indented_format_line

/// Append the requested indentation lines and one formatted text line.
///
/// `this` is the logging context, whose signed line count is at `+0x10` and
/// indentation flag at `+0x14`. The first stack word is an optional format
/// pointer; the second points to a slot containing the varargs pointer. The
/// function emits `count - 1` prefix lines when indentation is set, otherwise
/// `count` lines, emits the extra indentation line when flagged, clears that
/// flag, and formats into a 256-byte local buffer when the format pointer is
/// non-null. It then appends the local buffer, clears the caller's varargs
/// slot, and returns the final formatter's EAX result. Prefix counts are
/// signed; loop calls are compared individually.
///
/// Original: thiscall with two stack words, callee cleanup of eight bytes.
/// Formatter calls are intercepted. Their local-buffer pointer arguments are
/// skipped and the pointed-to 64-word buffer is snapshotted at each call.
lf_checker_rt::export!(thiscall, rw_0087b430(
    context: u32,
    format: u32,
    varargs_slot: u32
) -> u32 {
    const LINE_COUNT_OFFSET: u32 = 0x10;
    const INDENT_FLAG_OFFSET: u32 = 0x14;
    const PREFIX_FORMAT: u32 = 0x00FC831C;
    const INDENT_FORMAT: u32 = 0x00FC82C8;
    const OUTPUT_FORMAT: u32 = 0x00FC82CC;
    const OUTPUT_SIZE: u32 = 0xFF;
    const BUFFER_SIZE: usize = 0x100;
    const VSNPRINTF_BUFFER_TERMINATOR: usize = 0xFB;
    const APPEND_PREFIX: u32 = 1;
    const APPEND_INDENT: u32 = 2;
    const FORMAT_TEXT: u32 = 3;
    const APPEND_OUTPUT: u32 = 4;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        let has_indent = read_u8(context.wrapping_add(INDENT_FLAG_OFFSET)) != 0;
        let mut prefix_count = read_u32(context.wrapping_add(LINE_COUNT_OFFSET)) as i32;
        if has_indent {
            prefix_count = prefix_count.wrapping_sub(1);
        }
        while prefix_count > 0 {
            let _ = lf_checker_rt::callee_cdecl!(
                APPEND_PREFIX,
                u32,
                context,
                lf_checker_rt::relocated(PREFIX_FORMAT)
            );
            prefix_count = prefix_count.wrapping_sub(1);
        }

        if has_indent {
            let _ = lf_checker_rt::callee_cdecl!(
                APPEND_INDENT,
                u32,
                context,
                lf_checker_rt::relocated(INDENT_FORMAT)
            );
            write_u8(context.wrapping_add(INDENT_FLAG_OFFSET), 0);
        }

        let mut local_buffer = [0u8; BUFFER_SIZE];
        if format != 0 {
            local_buffer[VSNPRINTF_BUFFER_TERMINATOR] = 0;
            let varargs = read_u32(varargs_slot);
            let buffer_address = local_buffer.as_mut_ptr() as u32;
            let _ = lf_checker_rt::callee_cdecl!(
                FORMAT_TEXT,
                u32,
                buffer_address,
                OUTPUT_SIZE,
                format,
                varargs
            );
        } else {
            local_buffer[0] = 0;
        }

        let buffer_address = local_buffer.as_mut_ptr() as u32;
        let result = lf_checker_rt::callee_cdecl!(
            APPEND_OUTPUT,
            u32,
            context,
            lf_checker_rt::relocated(OUTPUT_FORMAT),
            buffer_address
        );
        write_u32(varargs_slot, 0);
        result
    }
});
