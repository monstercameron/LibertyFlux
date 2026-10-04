// original: 0x00DDE260 is_char_allowed_in_field
/// Decide whether a character may be entered in the field (1) or not (0).
///
/// A character is tentatively accepted when it is in the punctuation table,
/// is alphanumeric, or passes the language set selected by the configured
/// input mode (each mode delegates to one of the sibling predicates);
/// when the field requires filtering, the final say belongs to the typable
/// predicate. Only the low byte of the result is significant; the upper
/// bytes repeat the last value the original computed, as there.
const CHARSET_DDE260: [u8; 32] = [0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0];
lf_checker_rt::export!(thiscall, rw_dde260(this: u32, ch: u32) -> u32 {
    unsafe {
        let early = ch.wrapping_sub(0x20);
        let mut accepted = if early > 0x1F {
            0u32
        } else {
            1 - CHARSET_DDE260[early as usize] as u32
        };
        if ch.wrapping_sub(0x2B) <= 0x11 {
            accepted = 1;
        }
        if ch.wrapping_sub(0x41) <= 0x19 {
            accepted = 1;
        }
        let mut ebx = accepted & 0xFF;
        if ch.wrapping_sub(0x61) <= 0x19 {
            ebx = 1;
        }
        let mode: u32 = *lf_checker_rt::global::<u32>(0x01160CC8);
        let mut eax = mode.wrapping_sub(1);
        if eax <= 4 {
            if eax == 0 {
                let a = lf_checker_rt::callee_thiscall!(1, u32, this, ch);
                eax = a;
                if a & 0xFF != 0 {
                    ebx = 1;
                }
            } else if eax == 1 {
                let a1 = lf_checker_rt::callee_thiscall!(3, u32, this, ch);
                eax = a1;
                if a1 & 0xFF != 0 {
                    ebx = 1;
                }
                let a2 = lf_checker_rt::callee_thiscall!(4, u32, this, ch);
                eax = a2;
                if a2 & 0xFF != 0 {
                    ebx = 1;
                }
            } else if eax == 2 {
                let a = lf_checker_rt::callee_thiscall!(2, u32, this, ch);
                eax = a;
                if a & 0xFF != 0 {
                    ebx = 1;
                }
            } else if eax == 3 {
                let a2 = lf_checker_rt::callee_thiscall!(4, u32, this, ch);
                eax = a2;
                if a2 & 0xFF != 0 {
                    ebx = 1;
                }
            } else {
                let a = lf_checker_rt::callee_thiscall!(5, u32, this, ch);
                eax = a;
                if a & 0xFF != 0 {
                    ebx = 1;
                }
            }
        }
        let filtered = *((this + 0x20B) as *const u8);
        if filtered == 0 {
            (eax & 0xFFFFFF00) | (ebx & 0xFF)
        } else if ebx & 0xFF == 0 {
            eax & 0xFFFFFF00
        } else {
            let b = lf_checker_rt::callee_thiscall!(6, u32, this, ch);
            if b & 0xFF == 0 {
                b & 0xFFFFFF00
            } else {
                (b & 0xFFFFFF00) | 1
            }
        }
    }
});
