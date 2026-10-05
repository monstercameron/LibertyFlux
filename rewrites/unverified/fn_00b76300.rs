// original: 0x00b76300 ped_task_params_configure (proposed)

/// Fill a ped task parameter block from 25 argument words.
///
/// `this` points at the block. Words 0-2 are stored as the first three
/// floats; words 3-5 feed a normalisation: with `G0 = 1.0` and the low
/// lane of a sign-mask global (`0x80000000`, so word 3 is negated),
/// `n = G0 / sqrt(neg(w3)^2 + w4^2)`, then `n * w4 * w5` and
/// `n * neg(w3) * w5` are stored as the next two floats, in the original's
/// operation order.
///
/// A lookup callee takes word 6 plus an out-pointer; its answer word
/// becomes the block's id halfword, and a flag bit is set when the answer
/// matches one of three known ids or the answer object's kind word equals
/// 4. Three flag words may then each overwrite the id with a preset. Words
/// 7-10 map to four selector bytes (`0xff` when the word is -1, else its
/// low byte); words 11-24 are packed bit by bit into four flag bytes
/// together with two input bytes already in the block, and the id's
/// neighbour halfword is set to `0xffff`. A tail callee finishes the
/// block. No defined return value.
///
/// Original: 0x00b76300 (thiscall, 25 stack words).
unsafe fn run_00b76300(
    this: u32,
    w0: u32, w1: u32, w2: u32, w3: u32, w4: u32, w5: u32, w6: u32,
    w7: u32, w8: u32, w9: u32, w10: u32, w11: u32, w12: u32, w13: u32,
    w14: u32, w15: u32, w16: u32, w17: u32, w18: u32, w19: u32,
    w20: u32, w21: u32, w22: u32, w23: u32, w24: u32,
    skip_store_1e: bool,
) -> u32 {
    unsafe {
        const UNIT: u32 = 0x00fe88e8;
        const SIGN_MASK: u32 = 0x00fe8fa0;
        const KNOWN_A: u32 = 0x012fa5c0;
        const KNOWN_B: u32 = 0x012fa5b4;
        const KNOWN_C: u32 = 0x012fa3b0;
        const PRESET_A: u32 = 0x012f9fb4;
        const PRESET_B: u32 = 0x012fa47c;
        const PRESET_C: u32 = 0x012fa308;
        const ID_LOOKUP: u32 = 1;
        const ID_FINISH: u32 = 2;
        const KIND_OFF: u32 = 0x6c;
        const KIND_WANT: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let g0 = rdf(lf_checker_rt::relocated(UNIT));
        let mask = rd32(lf_checker_rt::relocated(SIGN_MASK));
        let f3 = f32::from_bits(w3);
        let f4 = f32::from_bits(w4);
        let f3n = f32::from_bits(w3 ^ mask);
        wr32(this, w0);
        wr32(this.wrapping_add(4), w1);
        wr32(this.wrapping_add(8), w2);
        wr32(this.wrapping_add(0x0c), w3);
        let t = add(mul(f3n, f3n), mul(f4, f4));
        ((this.wrapping_add(0x2a)) as *mut u8).write(((this.wrapping_add(0x2a)) as *const u8).read() & 0xdf);
        wr32(this.wrapping_add(0x10), w4);
        let n = div(g0, t.sqrt());
        let out0 = mul(mul(n, f4), f32::from_bits(w5));
        let out1 = mul(mul(n, f3n), f32::from_bits(w5));
        wr32(this.wrapping_add(0x14), out0.to_bits());
        wr32(this.wrapping_add(0x18), out1.to_bits());
        let mut answer = 0u32;
        let found = lf_checker_rt::callee_cdecl!(ID_LOOKUP, u32, w6, (&mut answer as *mut u32) as u32);
        if found == 0 {
            wr16(this.wrapping_add(0x1c), 0xffff);
        } else {
            wr16(this.wrapping_add(0x1c), (answer & 0xffff) as u16);
            let known = answer == rd32(lf_checker_rt::relocated(KNOWN_A))
                || answer == rd32(lf_checker_rt::relocated(KNOWN_B))
                || answer == rd32(lf_checker_rt::relocated(KNOWN_C))
                || rd32((found as u32).wrapping_add(KIND_OFF)) == KIND_WANT;
            if known {
                ((this.wrapping_add(0x2a)) as *mut u8)
                    .write(((this.wrapping_add(0x2a)) as *const u8).read() | 0x20);
            }
        }
        if ((w15 & 0xff) as u8) != 0 {
            wr16(this.wrapping_add(0x1c), rd16(lf_checker_rt::relocated(PRESET_A)));
        }
        if ((w16 & 0xff) as u8) != 0 {
            wr16(this.wrapping_add(0x1c), rd16(lf_checker_rt::relocated(PRESET_B)));
        }
        if ((w17 & 0xff) as u8) != 0 {
            wr16(this.wrapping_add(0x1c), rd16(lf_checker_rt::relocated(PRESET_C)));
        }
        let sel = |w: u32| if w == 0xffffffff { 0xffu8 } else { (w & 0xff) as u8 };
        let d0 = sel(w7);
        let d1 = sel(w8);
        let d2 = sel(w9);
        let a10 = sel(w10);
        ((this.wrapping_add(0x22)) as *mut u8).write(d0);
        ((this.wrapping_add(0x23)) as *mut u8).write(d1);
        let mut cl = (w23 & 0xff) as u8;
        ((this.wrapping_add(0x25)) as *mut u8).write(a10);
        ((this.wrapping_add(0x26)) as *mut u8).write((w12 & 0xff) as u8);
        ((this.wrapping_add(0x27)) as *mut u8).write((w13 & 0xff) as u8);
        cl <<= 4;
        cl |= ((w22 & 0xff) as u8) & 0x0f;
        if !skip_store_1e {
            wr16(this.wrapping_add(0x1e), 0xffff);
        }
        ((this.wrapping_add(0x24)) as *mut u8).write(d2);
        let mut dl = ((w16 & 0xff) as u8) & 1 | (((w17 & 0xff) as u8) << 1);
        dl <<= 1;
        dl |= ((w15 & 0xff) as u8) & 1;
        dl <<= 4;
        dl |= ((w11 & 0xff) as u8) & 1;
        let keep29 = ((this.wrapping_add(0x29)) as *const u8).read() & 0x18;
        ((this.wrapping_add(0x28)) as *mut u8).write(cl);
        cl = ((w24 & 0xff) as u8) & 1;
        dl <<= 1;
        dl |= keep29;
        cl <<= 1;
        cl |= ((w21 & 0xff) as u8) & 1;
        cl <<= 1;
        cl |= ((w20 & 0xff) as u8) & 1;
        let keep2a = ((this.wrapping_add(0x2a)) as *const u8).read() & 0xe3;
        cl <<= 2;
        cl |= keep2a;
        let w19b = (w19 & 0xff) as u8;
        let w18b = (w18 & 0xff) as u8;
        dl |= 1;
        ((this.wrapping_add(0x2b)) as *mut u8).write(0x65);
        ((this.wrapping_add(0x29)) as *mut u8).write(dl);
        ((this.wrapping_add(0x2a)) as *mut u8).write(cl);
        if w18b != 0 || w19b != 0 {
            let mut al = ((w19b & 1) << 1) | (cl & 0xfc);
            al |= w18b & 1;
            ((this.wrapping_add(0x2a)) as *mut u8).write(al);
        } else {
            ((this.wrapping_add(0x2a)) as *mut u8).write(cl | 3);
        }
        wr16(this.wrapping_add(0x20), (w14 & 0xffff) as u16);
        dl |= 0x10;
        ((this.wrapping_add(0x29)) as *mut u8).write(dl);
        lf_checker_rt::callee_thiscall!(ID_FINISH, u32, this);
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00b76300(
    this: u32,
    w0: u32, w1: u32, w2: u32, w3: u32, w4: u32, w5: u32, w6: u32,
    w7: u32, w8: u32, w9: u32, w10: u32, w11: u32, w12: u32, w13: u32,
    w14: u32, w15: u32, w16: u32, w17: u32, w18: u32, w19: u32,
    w20: u32, w21: u32, w22: u32, w23: u32, w24: u32
) -> u32 {
    unsafe {
        run_00b76300(
            this,
            w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13,
            w14, w15, w16, w17, w18, w19, w20, w21, w22, w23, w24,
            false,
        )
    }
});
