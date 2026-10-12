// original: 0x00DF3CB0 file_entry_buffer_copy

/// Clear and populate the file-entry buffer and return the final view update.
///
/// The fourth stack word supplies a NUL-terminated string copied to the
/// 256-byte buffer at `+0x200`; the second supplies a second string copied to
/// `+0x300`. Three float bit patterns are copied unchanged to `+0x318` through
/// `+0x324`, with the middle word also retained at `+0x30c`. The buffer clear
/// is delegated to the intercepted C runtime call. Finally, vtable slots
/// `+0x1ec`, `+0x24` and `+0x13c` run in order, the latter two with argument 1;
/// the last result is returned.
lf_checker_rt::export!(thiscall, rw_df3cb0(
    this: u32,
    _opaque_word_1: u32,
    second_text: u32,
    _opaque_word_3: u32,
    first_text: u32,
    float_bits_0: u32,
    float_bits_1: u32,
    float_bits_2: u32,
) -> u32 {
    unsafe {
        const ENTRY_TEXT_BUFFER: u32 = 0x200;
        const SECOND_TEXT_FIELD: u32 = 0x300;
        const DISPLAY_VALUE_0: u32 = 0x318;
        const DISPLAY_VALUE_1: u32 = 0x31C;
        const DISPLAY_VALUE_2: u32 = 0x320;
        const DISPLAY_VALUE_3: u32 = 0x324;

        let entry_text_buffer = this.wrapping_add(ENTRY_TEXT_BUFFER);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, entry_text_buffer, 0u32, 0x100u32
        );

        ((this.wrapping_add(SECOND_TEXT_FIELD)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(SECOND_TEXT_FIELD + 4)) as *mut u8).write(0);
        ((this.wrapping_add(DISPLAY_VALUE_0)) as *mut u32).write_unaligned(float_bits_0);
        ((this.wrapping_add(DISPLAY_VALUE_1)) as *mut u32).write_unaligned(float_bits_1);
        ((this.wrapping_add(DISPLAY_VALUE_2)) as *mut u32).write_unaligned(float_bits_1);
        ((this.wrapping_add(0x30C)) as *mut u32).write_unaligned(float_bits_1);
        ((this.wrapping_add(DISPLAY_VALUE_3)) as *mut u32).write_unaligned(float_bits_2);

        let mut source = first_text as *const u8;
        let mut target = entry_text_buffer as *mut u8;
        loop {
            let byte = source.read();
            target.write(byte);
            source = source.add(1);
            target = target.add(1);
            if byte == 0 { break; }
        }

        let mut source = second_text as *const u8;
        let mut target = this.wrapping_add(SECOND_TEXT_FIELD) as *mut u8;
        loop {
            let byte = source.read();
            target.write(byte);
            source = source.add(1);
            target = target.add(1);
            if byte == 0 { break; }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, 1u32);
        lf_checker_rt::callee_thiscall!(4, u32, this, 1u32)
    }
});
