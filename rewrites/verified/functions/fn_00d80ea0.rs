// original: 0x00D80EA0 sweep_tile_window (proposed)

/// Sweep the tile window around a position, invoking the tile routine at
/// 0x00D81E30 once per tile, then set the done flag.
///
/// `obj` is the input object. A half-width `w` (44 or 11, chosen by the
/// flag object at `+FSEL`) widens the position (`+INNER`, two floats) into
/// a float box; scaled by `+SCALE` and shifted by `+BIAS`, its floored
/// corners clamp to `0..0x77` per axis, giving an inclusive tile window
/// (empty windows skip the sweep). A global tick word (`+TICK`) usually
/// increments; when it saturates, callee 1 (cdecl, no arguments) runs and
/// the tick resets to 1.
///
/// Each tile calls callee 2 (cdecl, ten arguments) with a table entry
/// (`+TABLE`, indexed by the low nibbles of both counters times 20 bytes),
/// the object, the four unscaled box edges, a slot holding the mode byte
/// (`+MODE`) as a float, that same float, the sub-object at `+SUB`, and the
/// word above this function's own argument slot (harness fill; passed
/// through opaquely). The low two bits of `+ENTRY` shifted form a scratch
/// bit kept in the incoming argument slot (hence the stack check is off
/// for this function); the tail sets bit 7 of `+DONE` and folds the bit
/// back into bit 2 of `+ENTRY`, which leaves it unchanged.
///
/// Original: 0x00D80EA0 (cdecl, one stack word, no return value).
export!(cdecl, rw_00D80EA0(obj: u32) -> u32 {
    unsafe {
        const FSEL: u32 = 0xf50;
        const FSEL_TEST: u32 = 0x219;
        const ENTRY: u32 = 0xef8;
        const INNER: u32 = 0x20;
        const MODE: u32 = 0xe6f;
        const SUB: u32 = 0xe48;
        const DONE: u32 = 0xf15;
        const WIDE: u32 = 0x00eec924;
        const NARROW: u32 = 0x00e9af9c;
        const SCALE: u32 = 0x00fe8734;
        const BIAS: u32 = 0x00fe8b80;
        const TICK: u32 = 0x011a8908;
        const TABLE: u32 = 0x011a891c;
        const DIM: i32 = 0x77;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Floor to i32 with convert-with-truncation edge semantics: NaN
        /// and out-of-range results give `i32::MIN`.
        #[inline(always)]
        fn f2i(x: f32) -> i32 {
            let f = x.floor();
            if f >= -2147483648.0 && f < 2147483648.0 {
                f as i32
            } else {
                i32::MIN
            }
        }

        let sel = rd32(obj + FSEL);
        let w = if sel != 0 && rd8(sel + FSEL_TEST) != 0 {
            lf_checker_rt::global::<u32>(WIDE).read_unaligned()
        } else {
            lf_checker_rt::global::<u32>(NARROW).read_unaligned()
        };
        let w = f32::from_bits(w);
        let bit = (rd8(obj + ENTRY) >> 2) & 1;
        let inner = rd32(obj + INNER);
        let p0 = rdf(inner + 0x30);
        let p1 = rdf(inner + 0x34);
        let sc = f32::from_bits(lf_checker_rt::global::<u32>(SCALE).read_unaligned());
        let bi = f32::from_bits(lf_checker_rt::global::<u32>(BIAS).read_unaligned());
        let lo0 = 0.max(f2i(add(mul(sub(p0, w), sc), bi)));
        let lo1 = 0.max(f2i(add(mul(sub(p1, w), sc), bi)));
        let hi0 = DIM.min(f2i(add(mul(add(p0, w), sc), bi)));
        let hi1 = DIM.min(f2i(add(mul(add(p1, w), sc), bi)));

        let tick = lf_checker_rt::global::<u16>(TICK).read_unaligned();
        if tick >= 0xffff {
            let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            lf_checker_rt::global::<u16>(TICK).write_unaligned(1);
        } else {
            lf_checker_rt::global::<u16>(TICK).write_unaligned(tick + 1);
        }

        let bf = rd8(obj + MODE) as f32;
        let mut slot = bf;
        let e0 = sub(p0, w);
        let e1 = sub(p1, w);
        let e2 = add(p0, w);
        let e3 = add(p1, w);
        let sub_obj = obj.wrapping_add(SUB);
        let mut outer = lo1;
        if lo1 <= hi1 {
            loop {
                if lo0 <= hi0 {
                    let ob = (outer & 0xf) << 4;
                    let mut si = lo0;
                    loop {
                        let idx = (((si & 0xf) + ob) * 20) as u32;
                        let entry = lf_checker_rt::global::<u8>(TABLE) as u32 + idx;
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            2, u32, entry, obj,
                            e0.to_bits(), e1.to_bits(),
                            e2.to_bits(), e3.to_bits(),
                            &mut slot as *mut f32 as u32,
                            bf.to_bits(), sub_obj, 0
                        );
                        si += 1;
                        if si > hi0 {
                            break;
                        }
                    }
                }
                outer += 1;
                if outer > hi1 {
                    break;
                }
            }
        }

        wr8(obj + DONE, rd8(obj + DONE) | 0x80);
        let m = rd8(obj + ENTRY);
        let mut al = (bit << 2) ^ m;
        al &= 4;
        (obj as *mut u8).add(ENTRY as usize).write(m ^ al);
        0
    }
});
