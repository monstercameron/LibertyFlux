// original: 0x009630f0 name_field_set_60
/// Copy a name string into the fixed field at `+0x60`.
///
/// Thiscall: object in ECX, source pointer as the stack argument. A null
/// source or an empty string leaves the object untouched; otherwise the
/// bounded copy helper (cdecl/3: dst, src, `0x3F`) fills the field and the
/// terminator byte at `+0x9F` is forced to zero. Returns the source on the
/// early paths and the helper's answer otherwise.
lf_checker_rt::export!(thiscall, rw_009630f0(obj: u32, src: u32) -> u32 {
    unsafe {
        const DST_OFF: u32 = 0x60;
        const CAP: u32 = 0x3f;
        const TERM_OFF: u32 = 0x9f;
        if src == 0 {
            return src;
        }
        if (src as *const u8).read() == 0 {
            return src;
        }
        let dst = obj.wrapping_add(DST_OFF);
        let r: u32 = lf_checker_rt::callee_cdecl!(1, u32, dst, src, CAP);
        (obj.wrapping_add(TERM_OFF) as *mut u8).write(0);
        r
    }
});
