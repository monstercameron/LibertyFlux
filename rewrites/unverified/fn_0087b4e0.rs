// original: 0x0087B4E0 append_formatted_text

/// Format text into a logging buffer and advance its pointer and capacity.
///
/// The cdecl arguments are a logging context, a format pointer, then the
/// varargs words. The context stores the buffer pointer at `+0x08` and
/// remaining capacity at `+0x0c`. A null buffer formats into a local 256-byte
/// buffer and returns the formatter result. A non-null buffer with zero
/// capacity returns zero. Otherwise the function terminates the last byte,
/// calls the formatter with capacity minus one, then advances the context
/// pointer and decreases capacity for each output byte until the terminator.
/// All formatter calls are intercepted; varargs pointer addresses are skipped
/// and the supplied words are snapshotted.
///
/// Original: cdecl with a context, format pointer and two tested varargs words.
lf_checker_rt::export!(cdecl, rw_0087b4e0(
    context: u32,
    format: u32,
    argument0: u32,
    argument1: u32
) -> u32 {
    const BUFFER_OFFSET: u32 = 0x08;
    const CAPACITY_OFFSET: u32 = 0x0C;
    const STACK_BUFFER_SIZE: usize = 0x100;
    const STACK_BUFFER_SIZE_ARG: u32 = 0xFF;
    const TERMINATOR_OFFSET: usize = 0xFF;
    const NORMAL_FORMATTER: u32 = 1;
    const STACK_FORMATTER: u32 = 2;

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
        let buffer = read_u32(context.wrapping_add(BUFFER_OFFSET));
        if buffer == 0 {
            if format == 0 {
                return 0;
            }
            let mut local_buffer = [0u8; STACK_BUFFER_SIZE];
            let arguments = [argument0, argument1];
            return lf_checker_rt::callee_cdecl!(
                STACK_FORMATTER,
                u32,
                local_buffer.as_mut_ptr() as u32,
                STACK_BUFFER_SIZE_ARG,
                format,
                arguments.as_ptr() as u32
            );
        }

        let capacity = read_u32(context.wrapping_add(CAPACITY_OFFSET));
        if capacity == 0 {
            return 0;
        }

        write_u8(buffer.wrapping_add(capacity.wrapping_sub(1)), 0);
        let arguments = [argument0, argument1];
        let _ = lf_checker_rt::callee_cdecl!(
            NORMAL_FORMATTER,
            u32,
            buffer,
            capacity.wrapping_sub(1),
            format,
            arguments.as_ptr() as u32
        );

        if read_u8(buffer) == 0 {
            return buffer;
        }

        loop {
            let remaining = read_u32(context.wrapping_add(CAPACITY_OFFSET));
            if remaining == 0 {
                return 0;
            }
            let next = read_u32(context.wrapping_add(BUFFER_OFFSET)).wrapping_add(1);
            write_u32(context.wrapping_add(BUFFER_OFFSET), next);
            let after_character = remaining.wrapping_sub(1);
            write_u32(context.wrapping_add(CAPACITY_OFFSET), after_character);
            if read_u8(next) == 0 {
                return after_character;
            }
        }
    }
});
