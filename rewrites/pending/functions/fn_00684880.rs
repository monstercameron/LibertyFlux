// original: 0x00684880 cr_frame_dof_init_by_kind
/// Degree-of-freedom kind initializer.
///
/// Tags the object for one of four kinds (0-3): writes the kind's vtable
/// pointer at +0 and the tag byte 0x10+kind at +4. A null object or a kind
/// above 3 leaves memory untouched. Returns the low byte of the kind.
/// The vtable immediates carry loader fixups, so they are relocated.
export!(thiscall, rw_00684880(this_: *mut u8, kind: u32) -> u32 {
    unsafe {
        let k = (kind & 0xFF) as u8;
        if k <= 3 && !this_.is_null() {
            let (tag, vfptr) = match k {
                0 => (0x10u8, 0x00E9_5580u32),
                1 => (0x11u8, 0x00E9_55BCu32),
                2 => (0x12u8, 0x00EC_B95Cu32),
                _ => (0x13u8, 0x00FE_386Cu32),
            };
            *this_.add(TAG_OFF) = tag;
            *(this_ as *mut u32) = relocated(vfptr);
        }
        kind & 0xFF
    }
});
