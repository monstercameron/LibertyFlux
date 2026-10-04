// original: 0x00CAB7F0 event_zone_construct (proposed)

/// Construct a zone event record in place and return its address.
///
/// Runs the sized base constructor on `this`, installs the record's two
/// virtual tables, zeroes `+0x20`..`+0x28` and `+0x40`..`+0x48`, copies three
/// words from the vector argument into `+0x30`..`+0x38`, stores the integer
/// argument at `+0x50`, the two byte arguments at `+0x54` and `+0x55`, and
/// the tag argument at `+0x58`. When the optional second vector is non-null,
/// its four words are copied into `+0x40`..`+0x4C`. All float movement is
/// bitwise (no arithmetic is performed).
///
/// Original: 0x00CAB7F0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00cab7f0(this: u32, tag0: u32, vec: u32, tag: u32, b0: u32, b1: u32, vec2: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00ED8B9C;
        const VTABLE2: u32 = 0x00ED8BF4;
        const BASE_SIZE_BITS: u32 = 0x3F800000;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, BASE_SIZE_BITS);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE2));
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(0);
        let mut i = 0u32;
        while i < 3 {
            let w = (vec.wrapping_add(i * 4) as *const u32).read_unaligned();
            (this.wrapping_add(0x30 + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        (this.wrapping_add(0x40) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x44) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x48) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x50) as *mut u32).write_unaligned(tag0);
        (this.wrapping_add(0x54) as *mut u8).write((b0 & 0xFF) as u8);
        (this.wrapping_add(0x55) as *mut u8).write((b1 & 0xFF) as u8);
        (this.wrapping_add(0x58) as *mut u32).write_unaligned(tag);
        if vec2 != 0 {
            let mut j = 0u32;
            while j < 4 {
                let w = (vec2.wrapping_add(j * 4) as *const u32).read_unaligned();
                (this.wrapping_add(0x40 + j * 4) as *mut u32).write_unaligned(w);
                j += 1;
            }
        }
        this
    }
});
