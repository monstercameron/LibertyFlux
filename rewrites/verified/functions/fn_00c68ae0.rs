// original: 0x00C68AE0 match_key_then_dispatch (proposed)

/// Look a 32-bit key up in two small word tables, else dispatch on three
/// sentinel values or a pointer table.
///
/// `this` is a controller object (a mode byte at `+0x712`, a code byte at
/// `+0x400`); `arg0` is the key. The function first scans a word table
/// (count at `COUNT_ADDR`, SIGNED, skipped when non-positive; words at
/// `WORDS_ADDR`) for the key; a match returns the matched word's high
/// bytes with bit 0 set. On a miss it asks a gate helper: when the gate
/// is set it reads a base `r` from a second helper and scans a remainder
/// table (`REM_WORDS_ADDR`) for indices `(k + r) % div` (SIGNED remainder
/// by the global `DIV_ADDR`) for `k` below a bound of 4 (mode byte clear)
/// or 2 (mode byte set), a match returning the word's high bytes with bit
/// 0 set and exhaustion returning the last word's high bytes (the base's
/// high bytes when the bound is not positive). When the gate is clear
/// the key is compared against three sentinel globals: the first needs an
/// enable call plus a level of at least 3 (SIGNED) for 1; the second needs
/// a first level below 3 to fail and a second level below 5 (SIGNED) for 1;
/// the third returns the level's high bytes with bit 0 set iff the SIGNED
/// level reaches 5. Any other key indexes a pointer table (`TABLE_ADDR`):
/// when the pointed struct's flag word at `+0x70` is 2 the function
/// returns the pointer's high bytes with the code byte; otherwise it asks
/// a probe helper (0 means run the teardown helper and return its value)
/// and a final helper, returning the final answer's high bytes with bit 0
/// set iff its low byte is nonzero. All loop bounds and thresholds are
/// SIGNED; the two helpers below 0x500000 run natively only through the
/// checker's patching (their answers are scripted).
///
/// Original: 0x00C68AE0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00C68AE0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const COUNT_ADDR: u32 = 0x0169_E3C8;
        const WORDS_ADDR: u32 = 0x0169_E0E4;
        const DIV_ADDR: u32 = 0x0169_E3CC;
        const REM_WORDS_ADDR: u32 = 0x0169_E12A;
        const SENT_A: u32 = 0x012F_9E34;
        const SENT_B: u32 = 0x012F_A428;
        const SENT_C: u32 = 0x012F_A56C;
        const TABLE_ADDR: u32 = 0x0129_5CD8;
        const FLAG_OFF: u32 = 0x70;
        const MODE_OFF: u32 = 0x712;
        const CODE_OFF: u32 = 0x400;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn g16(a: u32) -> u16 {
            unsafe {
                (lf_checker_rt::global::<u16>(a) as *const u16).read_unaligned()
            }
        }

        let key = arg0;
        let n = g32(COUNT_ADDR) as i32;
        if n > 0 {
            let mut j: i32 = 0;
            loop {
                let w = g16(WORDS_ADDR + (j as u32) * 2) as u32;
                if w == key {
                    return (w & 0xFFFF_FF00) | 1;
                }
                j += 1;
                if j >= n {
                    break;
                }
            }
        }
        let gate = lf_checker_rt::callee_thiscall!(1, u32, this);
        if (gate as u8) != 0 {
            let r = lf_checker_rt::callee_thiscall!(2, u32, this) as i32;
            let bound: i32 = if rd8(this + MODE_OFF) == 0 { 4 } else { 2 };
            let div = g32(DIV_ADDR) as i32;
            if bound > 0 {
                let mut k: i32 = 0;
                let mut w: u32 = 0;
                loop {
                    if k >= bound {
                        break;
                    }
                    let rem = (k + r) % div;
                    w = g16(REM_WORDS_ADDR + (rem as u32) * 2) as u32;
                    if w == key {
                        return (w & 0xFFFF_FF00) | 1;
                    }
                    k += 1;
                }
                return w & 0xFFFF_FF00;
            }
            return (r as u32) & 0xFFFF_FF00;
        }
        if key == g32(SENT_A) {
            let on = lf_checker_rt::callee_cdecl!(3, u32,);
            if (on as u8) == 0 {
                return 0;
            }
            let flow = lf_checker_rt::callee_cdecl!(4, u32,);
            let v = lf_checker_rt::callee_thiscall!(5, u32, flow);
            if (v as i32) >= 3 {
                return 1;
            }
            return 0;
        }
        if key == g32(SENT_B) {
            let flow = lf_checker_rt::callee_cdecl!(4, u32,);
            let v = lf_checker_rt::callee_thiscall!(5, u32, flow);
            if (v as i32) < 3 {
                return 0;
            }
            let flow2 = lf_checker_rt::callee_cdecl!(4, u32,);
            let v2 = lf_checker_rt::callee_thiscall!(5, u32, flow2);
            if (v2 as i32) >= 5 {
                return 0;
            }
            return 1;
        }
        if key == g32(SENT_C) {
            let flow = lf_checker_rt::callee_cdecl!(4, u32,);
            let v = lf_checker_rt::callee_thiscall!(5, u32, flow);
            return (v & 0xFFFF_FF00) | u32::from((v as i32) >= 5);
        }
        let ptr = g32(TABLE_ADDR.wrapping_add(key.wrapping_mul(4)));
        if rd32(ptr.wrapping_add(FLAG_OFF)) == 2 {
            return (ptr & 0xFFFF_FF00) | rd8(this.wrapping_add(CODE_OFF)) as u32;
        }
        let probe = lf_checker_rt::callee_cdecl!(6, u32, 0x18, key);
        if (probe as u8) == 0 {
            return lf_checker_rt::callee_stdcall!(8, u32, key);
        }
        let fin = lf_checker_rt::callee_cdecl!(7, u32,);
        (fin & 0xFFFF_FF00) | u32::from((fin as u8) != 0)
    }
});
