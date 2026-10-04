// original: 0x0091C6C0 scan_token_end
/// Scan a wide string for the end of the current token.
///
/// Returns null for a null input. Otherwise classifies the first character
/// and remembers its class byte (kept in the incoming arg0 stack slot by the
/// original, in a local here); a leading space ends the scan at once. Then it
/// advances two bytes at a time while the character is none of NUL, `0x7E`,
/// `0x807E` or space: each step queries the index helper and re-classifies,
/// leaving the loop early through the per-index flag table at `0x0119BF5C`
/// when the class changes in the way the original tests. Returns the pointer
/// where the scan stopped.
export!(cdecl, rw_0091c6c0(a0: u32) -> u32 {
    unsafe {
        if a0 == 0 {
            return 0;
        }
        let mut edi = a0;
        let mut si = *(edi as *const u16);
        let first: u32 = callee_cdecl!(1, u32, si as u32);
        let saved = (first & 0xFF) as u8;
        if si == 0x20 {
            return edi;
        }
        loop {
            if si == 0 || si == 0x7E || si == 0x807E {
                break;
            }
            edi = edi.wrapping_add(2);
            let bx: u32 = callee_cdecl!(2, u32,);
            si = *(edi as *const u16);
            let a: u32 = callee_cdecl!(1, u32, si as u32);
            let al = (a & 0xFF) as u8;
            if al != 0 {
                let off = (bx & 0xFFFFFFFF).wrapping_mul(9).wrapping_mul(8);
                if *((relocated(0x119BF5C) + off) as *const u8) != 0 {
                    break;
                }
            } else if al == saved {
                // fall through to the space check below
            } else {
                let off = (bx & 0xFFFFFFFF).wrapping_mul(9).wrapping_mul(8);
                if *((relocated(0x119BF5C) + off) as *const u8) != 0 {
                    break;
                }
            }
            if si == 0x20 {
                break;
            }
        }
        edi
    }
});
