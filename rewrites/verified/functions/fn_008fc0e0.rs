// original: 0x008FC0E0 text_set_field_990
/// Copy a string into the field at `+0x990`, capped at 15 bytes.
///
/// A null source just clears the field's first byte. Otherwise the
/// copy routine fills the field and the byte at `+0x99f` is forced
/// to NUL. Thiscall, one stack argument; the null path returns
/// uninitialized eax, so the return value is not compared.
export!(thiscall, rw_008fc0e0(this: u32, s: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x990;
        const CAP: u32 = 0x99f;
        const LIMIT: u32 = 0xf;
        if s == 0 {
            ((this + FIELD) as *mut u8).write(0);
            return 0;
        }
        let d = this.wrapping_add(FIELD);
        let _: u32 = callee_cdecl!(1, u32, d, s, LIMIT);
        ((this + CAP) as *mut u8).write(0);
        0
    }
});
