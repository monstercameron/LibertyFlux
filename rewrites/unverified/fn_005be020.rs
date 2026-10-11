// original: 0x005BE020 copy_streaming_record_fields
/// Copy selected 32-bit fields from a 0x78-byte record. Offsets 0 and 4 and
/// every dword from 0x10 through 0x74 are copied; bytes 8 through 0x0f are
/// left untouched. The destination pointer is returned.
///
/// The original is thiscall: ECX is the destination and one stack argument
/// is the source pointer. All copied data is bitwise, including float fields.
lf_checker_rt::export!(thiscall, rw_005be020(dst: u32, src: u32) -> u32 {
    const FIRST_FIELDS: [u32; 2] = [0, 4];
    unsafe {
        for offset in FIRST_FIELDS.into_iter().chain((0x10..=0x74).step_by(4)) {
            let value = (src.wrapping_add(offset) as *const u32).read_unaligned();
            (dst.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
    }
    dst
});
