// original: 0x008900F0 audsound_apply_voice_random
/// Applies randomized voice parameters from `src` onto `this`.
///
/// `src` is the parameter block, `this` the voice. For each of five
/// width/answer pairs the jitter callee (cdecl; the width is pushed first,
/// so arg0 is its negation, arg1 the width) is called when the width is
/// nonzero and its answer used:
/// the float at `this+0x1c` becomes `(float)(int16)[src+0x0f] * K +
/// (float)answer * K` with `K` the float constant (in that operand order);
/// the word at `this+0x20` becomes `(uint16)([src+0x13] + answer)`; the word
/// at `this+0xa` becomes all-ones when `(int16)[src+0x17]` is -1, otherwise
/// the SIGNED remainder of `((int16)answer + (int16)[src+0x17] + 0x168) /
/// 0x168` (only the answer's low 16 bits are used, sign-extended);
/// the dword at `this+0x58` becomes `(uint16)[src+0x1b] + answer`; the dword
/// at `this+0x68` becomes `[src+0x1f] + answer` (or `+ 0` when the dword
/// width at `src+0x23` is zero). A zero width skips its call and contributes
/// 0 (the width register still holds the zero width at the use). The second
/// stack argument is never read. Returns the last jitter answer, or 0 when
/// the dword width was zero.
/// Original: 0x008900F0 (thiscall, two stack words: src, unused).
export!(thiscall, rw_008900F0(this: *mut u8, src: *const u8, _unused: u32) -> u32 {
    unsafe {
        const JITTER: u32 = 1;
        const K_G: u32 = 0x00fe870c;
        const MODULUS: i32 = 0x168;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let k = *global::<f32>(K_G);
        let w0 = *(src.add(0x11) as *const u16);
        // A zero width skips the call; EAX is then the zero width itself.
        let r0: u32 = if w0 != 0 {
            callee_cdecl!(JITTER, u32, (w0 as u32).wrapping_neg(), w0 as u32)
        } else {
            0
        };
        let base = *(src.add(0x0f) as *const i16) as f32;
        let j0 = (r0 as i32) as f32;
        *(this.add(0x1c) as *mut f32) = add(mul(base, k), mul(j0, k));
        let w1 = *(src.add(0x15) as *const u16);
        let r1: u32 = if w1 != 0 {
            callee_cdecl!(JITTER, u32, (w1 as u32).wrapping_neg(), w1 as u32)
        } else {
            0
        };
        let s13 = *(src.add(0x13) as *const u16);
        *(this.add(0x20) as *mut u16) = s13.wrapping_add(r1 as u16);
        if *(src.add(0x17) as *const i16) == -1 {
            *(this.add(0x0a) as *mut u16) = 0xffff;
        } else {
            let w2 = *(src.add(0x19) as *const u16);
            let r2: u32 = if w2 != 0 {
                callee_cdecl!(JITTER, u32, (w2 as u32).wrapping_neg(), w2 as u32)
            } else {
                0
            };
            // cwde: only the answer's low 16 bits survive, sign-extended.
            let v = ((r2 as u16) as i16 as i32)
                .wrapping_add(*(src.add(0x17) as *const i16) as i32)
                .wrapping_add(MODULUS);
            *(this.add(0x0a) as *mut u16) = v.wrapping_rem(MODULUS) as u16;
        }
        let w3 = *(src.add(0x1d) as *const u16);
        let r3: u32 = if w3 != 0 {
            callee_cdecl!(JITTER, u32, (w3 as u32).wrapping_neg(), w3 as u32)
        } else {
            0
        };
        let s1b = *(src.add(0x1b) as *const u16) as u32;
        *(this.add(0x58) as *mut u32) = s1b.wrapping_add(r3);
        let w4 = *(src.add(0x23) as *const u32);
        let r4: u32 = if w4 != 0 {
            callee_cdecl!(JITTER, u32, w4.wrapping_neg(), w4)
        } else {
            0
        };
        let s1f = *(src.add(0x1f) as *const u32);
        *(this.add(0x68) as *mut u32) = s1f.wrapping_add(r4);
        r4
    }
});
