// original: 0x00c2c660 audEntityRadioEmitter::vf9
/// Stores station bytes, or a global fallback.
///
/// On the radio path stores the argument's low byte (twice unless it is
/// 0xFE or more). With a null inner object, or the alternate path flag,
/// stores the byte in a global unless the argument is exactly 0xFF.
/// Returns nothing meaningful.
export!(thiscall, rw_00c2c660(this: *const u8, arg: u32) -> u32 {
    unsafe {
        let inner = *(this.add(4) as *const u32);
        if inner != 0 {
            let flags = (*(inner.wrapping_add(0x28) as *const u32)) & 0x3C0;
            if flags == 0x80 {
                *((inner.wrapping_add(0xD10)) as *mut u8) = arg as u8;
                if (arg as u8) < 0xFE {
                    *((inner.wrapping_add(0xD11)) as *mut u8) = arg as u8;
                }
                return 0;
            }
            if flags != 0xC0 {
                return 0;
            }
        }
        if arg != 0xFF {
            *global::<u32>(0x1284644) = arg & 0xFF;
        }
        0
    }
});
