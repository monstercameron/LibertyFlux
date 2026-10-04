// original: 0x008BA5A0 pad_state_hash_dispatch
/// Look up which pad action the current input state selects, run it, and
/// report the sticky flag.
///
/// Hashes a state-table row picked by a two-float range search and dispatches
/// on the hash: most arms mark entries in the pad object returned by callee 1,
/// twelve arms forward a small id to callee 5, and four arms set a flag byte
/// in the pad object. Returns the sticky flag byte.
///
/// Only the low byte of the return value is behaviour: the original builds it
/// with byte moves that leave the upper three bytes as register leftovers, so
/// the contract compares `al` and this rewrite returns `u8`.
export!(cdecl, rw_008BA5A0() -> u8 {
    const G_FLAG: u32 = 0x011609F4;
    const G_GATE1: u32 = 0x011609F5;
    const G_GATE2: u32 = 0x01030B9D;
    const G_GATE2B: u32 = 0x01160C24;
    const G_I1: u32 = 0x018B7A80;
    const G_M1: u32 = 0x017ACCE8;
    const G_I2: u32 = 0x018B7A8C;
    const G_M2: u32 = 0x017ACCF0;
    const G_COUNT: u32 = 0x011609E8;
    const G_TABLE: u32 = 0x01161740;
    const G_CALLEE2_ARG: u32 = 0x0116182C;
    const G_HASH_INPUT: u32 = 0x011609F8;
    const G_KEYA: u32 = 0x018B7A88;
    const G_KEYB: u32 = 0x018B7A84;
    const G_SEL: u32 = 0x0117E6DA;
    const G_C5THIS: u32 = 0x0118D110;

    /// Hash slots in the order the original compares them. There is no slot
    /// at 0x1160BE4: the chain skips from 0x1160BE0 to 0x1160BE8.
    const SLOTS: [u32; 31] = [
        0x01160B88, 0x01160B8C, 0x01160B90, 0x01160B94, 0x01160B98, 0x01160B9C,
        0x01160BA0, 0x01160BA4, 0x01160BA8, 0x01160BAC, 0x01160BB0, 0x01160BB4,
        0x01160BB8, 0x01160BBC, 0x01160BC0, 0x01160BC4, 0x01160BC8, 0x01160BCC,
        0x01160BD0, 0x01160BD4, 0x01160BD8, 0x01160BDC, 0x01160BE0, 0x01160BE8,
        0x01160BEC, 0x01160BF0, 0x01160BF4, 0x01160BF8, 0x01160BFC, 0x01160C00,
        0x01160C04,
    ];

    /// (set-byte, link-word, index-byte) pad offsets for the first eight arms.
    const PADS8: [(u32, u32, u32); 8] = [
        (0x2B6E, 0x2B74, 0x2B70),
        (0x2B7E, 0x2B84, 0x2B80),
        (0x2CAE, 0x2CB4, 0x2CB0),
        (0x2CBE, 0x2CC4, 0x2CC0),
        (0x2BBE, 0x2BC4, 0x2BC0),
        (0x2BAE, 0x2BB4, 0x2BB0),
        (0x2B8E, 0x2B94, 0x2B90),
        (0x2B9E, 0x2BA4, 0x2BA0),
    ];

    #[inline(always)]
    unsafe fn rd8(va: u32) -> u8 {
        unsafe { *(global::<u8>(va) as *const u8) }
    }

    #[inline(always)]
    unsafe fn rd32(va: u32) -> u32 {
        unsafe { *(global::<u32>(va) as *const u32) }
    }

    #[inline(always)]
    unsafe fn rd32f(va: u32) -> f32 {
        f32::from_bits(unsafe { rd32(va) })
    }

    #[inline(always)]
    unsafe fn wr8(va: u32, v: u8) {
        unsafe {
            *global::<u8>(va) = v;
        }
    }

    #[inline(always)]
    unsafe fn prd8(p: u32) -> u8 {
        unsafe { *(p as *const u8) }
    }

    #[inline(always)]
    unsafe fn prd32(p: u32) -> u32 {
        unsafe { *(p as *const u32) }
    }

    #[inline(always)]
    unsafe fn pwr8(p: u32, v: u8) {
        unsafe {
            *(p as *mut u8) = v;
        }
    }

    /// Arm prefix: combine the two key words, raise the sticky flag, and test
    /// bit 0. Returns false when the arm falls through to the epilogue.
    #[inline(always)]
    unsafe fn prefix(variant_b: bool) -> bool {
        unsafe {
            let a = rd32(G_KEYA);
            let b = rd32(G_KEYB);
            let c = if variant_b { (a ^ b) & b } else { (a ^ b) & a };
            wr8(G_FLAG, 1);
            c as u8 & 1 != 0
        }
    }

    /// Shared pad arm body: mark the pad slot, follow its link, and mark the
    /// linked entry picked by the index byte.
    #[inline(always)]
    unsafe fn pad_body(pad: u32, set_off: u32, link_off: u32, idx_off: u32) {
        unsafe {
            pwr8(pad + set_off, 0x80);
            let target = prd32(pad + link_off);
            if target != 0 {
                let ix = prd8(pad + idx_off) as u32;
                pwr8(target + ix * 8, 0x80);
            }
        }
    }

    #[inline(always)]
    unsafe fn call5(imm: u32) {
        unsafe {
            let _: u32 = callee_thiscall!(5, u32, relocated(G_C5THIS), imm, 1, 0);
        }
    }

    #[inline(always)]
    unsafe fn epilogue(scratch: u32) -> u8 {
        unsafe {
            let f = rd8(G_FLAG);
            let _: u32 = callee_thiscall!(6, u32, scratch);
            f
        }
    }

    unsafe {
        if rd8(G_GATE1) != 0 {
            return 0;
        }
        if rd8(G_GATE2) == 0 && rd32(G_GATE2B) != 0x25 {
            return 0;
        }
        let pad: u32 = callee_cdecl!(1, u32, 0);
        let f1: f32 = (rd32(G_I1) as i32) as f32 * rd32f(G_M1);
        let f2: f32 = (rd32(G_I2) as i32) as f32 * rd32f(G_M2);
        let mut scratch = [0u32; 2];
        let sp = scratch.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(2, u32, sp, relocated(G_CALLEE2_ARG));
        let count = rd32(G_COUNT) as i32;
        let mut edi: i32 = 0;
        let mut found = false;
        if count > 0 {
            let mut i = 0;
            while i < count {
                let e = G_TABLE + (i as u32) * 16;
                let lo1 = rd32f(e);
                let lo2 = rd32f(e + 4);
                let hi1 = rd32f(e + 8);
                let hi2 = rd32f(e + 12);
                if f1 >= lo1 && hi1 >= f1 && f2 >= lo2 && hi2 >= f2 {
                    found = true;
                    edi = i;
                    break;
                }
                i += 1;
            }
            if !found {
                edi = count;
            }
        }
        let _: u32 = callee_thiscall!(3, u32, sp);
        let count2 = rd32(G_COUNT) as i32;
        if edi >= count2 || !found {
            return epilogue(sp);
        }
        let arg = G_HASH_INPUT.wrapping_add((edi as u32).wrapping_mul(33));
        let h: u32 = callee_cdecl!(4, u32, relocated(arg), 0);
        let mut idx: usize = 31;
        let mut k: usize = 0;
        while k < 31 {
            if h == rd32(SLOTS[k]) {
                idx = k;
                break;
            }
            k += 1;
        }
        match idx {
            0..=7 => {
                if prefix(false) {
                    let (a, b, c) = PADS8[idx];
                    pad_body(pad, a, b, c);
                }
            }
            8 => {
                if prefix(true) {
                    pad_body(pad, 0x2B5E, 0x2B64, 0x2B60);
                }
            }
            9 => {
                if prefix(false) {
                    call5(0xD3);
                }
            }
            10 => {
                if prefix(false) {
                    if rd8(G_SEL) == 0 {
                        pad_body(pad, 0x2B6E, 0x2B74, 0x2B70);
                    } else {
                        call5(0x1C);
                    }
                }
            }
            11 => {
                if prefix(false) {
                    if rd8(G_SEL) == 0 {
                        pad_body(pad, 0x2B5E, 0x2B64, 0x2B60);
                    } else {
                        pad_body(pad, 0x2E5E, 0x2E64, 0x2E60);
                    }
                }
            }
            12 => {
                if prefix(false) {
                    call5(0x3F);
                }
            }
            13 => {
                if prefix(false) {
                    pad_body(pad, 0x2B5E, 0x2B64, 0x2B60);
                }
            }
            14 => {
                if prefix(false) {
                    call5(0x32);
                }
            }
            15 => {
                if prefix(false) {
                    call5(0x19);
                }
            }
            16 => {
                if prefix(false) {
                    call5(0x0F);
                }
            }
            17 => {
                if prefix(false) {
                    call5(0x39);
                }
            }
            18 => {
                if prefix(false) {
                    call5(0x2A);
                }
            }
            19 | 20 => {
                if prefix(false) {
                    call5(0x2E);
                }
            }
            21 => {
                if prefix(false) {
                    call5(0x15);
                }
            }
            22 => {
                if prefix(false) {
                    call5(0x16);
                }
            }
            23 => {
                if prefix(false) {
                    call5(0x2F);
                }
            }
            24 => {
                if prefix(false) {
                    pad_body(pad, 0x314E, 0x3154, 0x3150);
                }
            }
            25 => {
                if prefix(false) {
                    pad_body(pad, 0x317E, 0x3184, 0x3180);
                }
            }
            26 => {
                if prefix(false) {
                    pwr8(pad + 0x3A80, 1);
                }
            }
            27 => {
                if prefix(false) {
                    pwr8(pad + 0x3A81, 1);
                }
            }
            28 => {
                if prefix(false) {
                    pwr8(pad + 0x3A82, 1);
                }
            }
            29 => {
                if prefix(false) {
                    pwr8(pad + 0x3A83, 1);
                }
            }
            30 => {
                if prefix(false) {
                    pad_body(pad, 0x2FEE, 0x2FF4, 0x2FF0);
                }
            }
            _ => {}
        }
        epilogue(sp)
    }
});
