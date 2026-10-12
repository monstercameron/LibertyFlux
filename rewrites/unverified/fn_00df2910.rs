// original: 0x00DF2910 UIMontageText::format_numeric_field_label

/// Format the numeric field reached through `this + 0x1e4` and update the
/// label object at `this + 0x1ec`. The field accessor is called once to choose
/// a format and again to obtain the value that is formatted. Its first result
/// is compared as an unsigned integer against three: values below three use
/// the low-range string, while three and above use the high-range string and
/// subtract four from the second result with wrapping arithmetic. The output
/// is sent through the label object's virtual slot at byte offset `0x1e0`.
/// The helper and formatter calls are ordinary calls; the final call remains
/// virtual so the class's dispatch behavior is preserved.
lf_checker_rt::export!(thiscall, rw_00df2910(this: u32) -> u32 {
    unsafe {
        const FIELD_OBJECT: u32 = 0x1e4;
        const LABEL_OBJECT: u32 = 0x1ec;
        const LABEL_SLOT: u32 = 0x1e0;
        const FIELD_ACCESSOR: u32 = 1;
        const FORMATTER: u32 = 2;
        const LOW_RANGE_FORMAT: u32 = 0x00f0_1404;
        const HIGH_RANGE_FORMAT: u32 = 0x00f0_1408;
        const HIGH_RANGE_START: u32 = 3;
        const HIGH_RANGE_OFFSET: u32 = 4;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_field(object: u32) -> u32 {
            lf_checker_rt::callee_thiscall!(FIELD_ACCESSOR, u32, object)
        }

        let field_object = read_u32(this.wrapping_add(FIELD_OBJECT));
        let selection = read_field(field_object);
        let label_object = read_u32(this.wrapping_add(LABEL_OBJECT));
        let label_table = read_u32(label_object);

        let (format_string, displayed_value) = if selection < HIGH_RANGE_START {
            (LOW_RANGE_FORMAT, read_field(field_object))
        } else {
            (
                HIGH_RANGE_FORMAT,
                read_field(field_object).wrapping_sub(HIGH_RANGE_OFFSET),
            )
        };

        let formatted_label = lf_checker_rt::callee_cdecl!(
            FORMATTER,
            u32,
            lf_checker_rt::relocated(format_string),
            displayed_value,
        );

        let method_address = read_u32(label_table.wrapping_add(LABEL_SLOT));
        let method: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(method_address as usize);
        method(label_object, formatted_label, 0)
    }
});
