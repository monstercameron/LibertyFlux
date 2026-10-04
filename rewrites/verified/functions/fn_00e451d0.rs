// original: 0x00e451d0 format_append_field_09
// format-append at field 9.
// Appends arg to the buffer at this+9 with the shared "%s" format, like
// 0x00e45160 but for the first field. Returns the appender's answer.
export!(thiscall, rw_00e451d0(this_obj: u32, arg: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 9;
        const FORMAT_STR: u32 = 0x00F1_5904;
        let field = this_obj.wrapping_add(FIELD_OFF);
        callee_cdecl!(1, u32, field, relocated(FORMAT_STR), arg)
    }
});
