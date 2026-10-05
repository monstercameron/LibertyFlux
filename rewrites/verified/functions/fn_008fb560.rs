// original: 0x008FB560 Text_FormatWithString
/// Format a text with two resolved parts, by id or directly.
///
/// A null receiver ends the call at once. With the flag byte set,
/// both signed index arguments must be non-negative (else the call
/// ends or skips ahead); each non-negative index is resolved through
/// the id lookup and the pair is composed. With the flag clear, the
/// two pointer arguments pass through when non-null. The composed
/// triple always runs through the compose routine. Cdecl, seven
/// stack arguments; the null-receiver path returns uninitialized
/// eax, so the return value is not compared (see the contract).
export!(cdecl, rw_008fb560(p0: u32, p1: u32, q: u32, flag: u32, x: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 0x11db280;
        if p0 == 0 {
            return 0;
        }
        if (flag as u8) != 0 {
            if (a as i32) < 0 {
                return a;
            }
        } else if p1 == 0 {
            return a;
        }
        let mut esi: u32 = 0;
        let mut edi: u32 = 0;
        if (flag as u8) != 0 {
            if (a as i32) >= 0 {
                esi = callee_thiscall!(1, u32, relocated(LOOKUP), a, x);
            }
            if (b as i32) >= 0 {
                edi = callee_thiscall!(2, u32, relocated(LOOKUP), b, x);
                let ans: u32 = callee_cdecl!(3, u32, p0, esi, edi);
                return ans;
            }
            let ans: u32 = callee_cdecl!(4, u32, p0, esi, edi);
            return ans;
        }
        if p1 != 0 {
            esi = p1;
        }
        if q != 0 {
            edi = q;
        }
        let ans: u32 = callee_cdecl!(4, u32, p0, esi, edi);
        ans
    }
});
