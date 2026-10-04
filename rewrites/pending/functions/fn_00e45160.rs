// original: 0x00e45160 format_append_field_1d
// format-append at field 0x1d.
// Appends arg to the buffer at this+0x1d with the shared "%s" format.
// The format address carries a relocation entry, so it is resolved for the
// loaded image base. Returns the appender's answer.
export!(thiscall, rw_00e45160(this_obj: u32, arg: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x1d;
        const FORMAT_STR: u32 = 0x00F1_5908;
        let field = this_obj.wrapping_add(FIELD_OFF);
        callee_cdecl!(1, u32, field, relocated(FORMAT_STR), arg)
    }
});
