// original: 0x00d90300 audio_box_query_best
/// Find the overlapped cell nearest to a query segment, through the child gate.
///
/// `this` is the audio geometry object: `+0x50` flag byte (bit 2 selects a
/// refresh through callee 1), `+0x60` the packed u16 corner-index array,
/// `+0x6c` the cell table (40 bytes per cell), `+0x7c` the cell count limit.
/// `a` and `b` are the segment endpoints (3 floats each); `out` receives the
/// winning plane-test point (4 floats). The fifth argument is ignored.
///
/// The segment's component-wise min/max box is scaled by 8 (the z range is
/// padded by 1 first) and truncated to 16-bit integers. Callee 2 resolves
/// the child gate for `a`; a null gate returns `0xFFFF`. Otherwise every
/// u16 index in the gate's list (`[gate+0x2c]+4`, count a u16 at `+0xc`)
/// below the cell limit is bounds-tested as signed 16-bit against its
/// cell's packed box (words at `+0x10..0x1a`). Each passing cell resolves
/// its corners into the shared plane slots through callee 3 (thiscall:
/// this, corner word, slot address; fills 3 words), then runs plane-pair
/// tests through callee 4 (cdecl: `a`, `b`, 12-word frame block, 4-word
/// frame out-buffer) while the masked flag word exceeds `0x400000`. A
/// passing test competes by squared distance from `a`; the winner's index
/// is returned and its point stored to `out`. NaN endpoints sort below
/// everything (`comiss` + `jbe` is `!(b > a)`); float operation order is
/// the original's, pinned so NaN payloads propagate bit-exactly.
///
/// Original: 0x00d90300 (thiscall, four stack words; the fourth is not read).
lf_checker_rt::export!(thiscall, rw_00d90300(this: u32, a: u32, b: u32, out: u32, _tag: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x50;
        const REFRESH_BIT: u8 = 4;
        const WORDS_OFF: u32 = 0x60;
        const CELLS_OFF: u32 = 0x6c;
        const LIMIT_OFF: u32 = 0x7c;
        const GATE_LINK: u32 = 0x2c;
        const CELL_STRIDE: u32 = 40;
        const COUNT_SHIFT: u32 = 0x15;
        const COUNT_MASK: u32 = 0xf;
        const BASE_MASK: u32 = 0x1ffff;
        const KIND_MASK: u32 = 0x1e00000;
        const KIND_MIN: u32 = 0x400000;
        const SCALE: f32 = 8.0;
        const PAD: f32 = 1.0;
        const BEST_INIT: f32 = f32::from_bits(0x7f7fffff);
        const NO_WINNER: u32 = 0xffff;
        const PLANES: u32 = 0x179fb20;
        const FLAG_WORD: u32 = 0x179fc20;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(p: u32) -> u16 {
            unsafe { (p as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(p: u32) -> u8 {
            unsafe { (p as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { (p as *mut u32).write_unaligned(v.to_bits()) }
        }
        /// Truncate toward zero exactly like cvttss2si: NaN and
        /// out-of-range values yield 0x80000000 instead of saturating.
        #[cfg(target_arch = "x86")]
        #[inline(always)]
        unsafe fn cvtt(x: f32) -> i32 {
            unsafe {
                core::arch::x86::_mm_cvtt_ss2si(core::arch::x86::_mm_set_ss(x))
            }
        }
        #[cfg(not(target_arch = "x86"))]
        #[inline(always)]
        unsafe fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        if rd8(this + FLAG_OFF) & REFRESH_BIT != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this);
        }
        let ax = rdf(a);
        let ay = rdf(a + 4);
        let az = rdf(a + 8);
        let bx = rdf(b);
        let by = rdf(b + 4);
        let bz = rdf(b + 8);
        let (lox, hix) = if !(bx > ax) { (bx, ax) } else { (ax, bx) };
        let (loy, hiy) = if !(by > ay) { (by, ay) } else { (ay, by) };
        let (loz, hiz) = if !(bz > az) { (bz, az) } else { (az, bz) };
        let lo0 = cvtt(fmul(lox, SCALE)) as i16;
        let lo1 = cvtt(fmul(loy, SCALE)) as i16;
        let lo2 = cvtt(fmul(fsub(loz, PAD), SCALE)) as i16;
        let hi0 = cvtt(fmul(hix, SCALE)) as i16;
        let hi1 = cvtt(fmul(hiy, SCALE)) as i16;
        let hi2 = cvtt(fmul(fadd(hiz, PAD), SCALE)) as i16;

        let gate = lf_checker_rt::callee_thiscall!(2, u32, this, a);
        if gate == 0 {
            return NO_WINNER;
        }
        let flag = lf_checker_rt::global::<u32>(FLAG_WORD);
        if unsafe { *flag } & 1 == 0 {
            unsafe {
                *flag |= 1;
            }
        }
        let inner = rd32(gate + GATE_LINK);
        let count = rd16(inner + 0xc) as u32;
        let index_list = rd32(inner + 4);
        let limit = rd32(this + LIMIT_OFF);
        let cells = rd32(this + CELLS_OFF);
        let words = rd32(this + WORDS_OFF);
        let mut best = BEST_INIT;
        let mut besti = NO_WINNER;
        let mut i = 0u32;
        while i < count {
            let idx = rd16(index_list + i * 2) as u32;
            if idx < limit {
                let e = cells.wrapping_add(idx.wrapping_mul(CELL_STRIDE));
                let e0 = rd32(e);
                let t0 = rd16(e + 0x10) as i16;
                let t1 = rd16(e + 0x12) as i16;
                let t2 = rd16(e + 0x14) as i16;
                let t3 = rd16(e + 0x16) as i16;
                let t4 = rd16(e + 0x18) as i16;
                let t5 = rd16(e + 0x1a) as i16;
                if lo0 <= t1 && lo1 <= t3 && lo2 <= t5
                    && hi0 >= t0 && hi1 >= t2 && hi2 >= t4
                {
                    let n = (e0 >> COUNT_SHIFT) & COUNT_MASK;
                    let base = rd32(e + 4) & BASE_MASK;
                    if n != 0 {
                        let mut k = 0u32;
                        while k < n {
                            let w = rd16(words + (base + k) * 2) as u32;
                            let slot = lf_checker_rt::relocated(PLANES + k * 0x10);
                            lf_checker_rt::callee_thiscall!(3, u32, this, w, slot);
                            k += 1;
                        }
                    }
                    if (e0 & KIND_MASK) > KIND_MIN {
                        let mut blk = [0.0f32; 12];
                        let mut w = 0usize;
                        while w < 4 {
                            blk[w] = unsafe {
                                *lf_checker_rt::global::<f32>(PLANES + (w as u32) * 4)
                            };
                            w += 1;
                        }
                        let mut j = 2u32;
                        let mut s_lo = 0x10u32;
                        let mut s_hi = 0x20u32;
                        while j < n {
                            let mut w = 0usize;
                            while w < 4 {
                                blk[4 + w] = unsafe {
                                    *lf_checker_rt::global::<f32>(PLANES + s_hi + (w as u32) * 4)
                                };
                                blk[8 + w] = unsafe {
                                    *lf_checker_rt::global::<f32>(PLANES + s_lo + (w as u32) * 4)
                                };
                                w += 1;
                            }
                            let mut ob = [0u32; 4];
                            let r = lf_checker_rt::callee_cdecl!(
                                4, u32, a, b,
                                blk.as_mut_ptr() as u32,
                                ob.as_mut_ptr() as u32
                            );
                            if r & 0xFF != 0 {
                                let ox = f32::from_bits(ob[0]);
                                let oy = f32::from_bits(ob[1]);
                                let oz = f32::from_bits(ob[2]);
                                let dx = fsub(ox, ax);
                                let dy = fsub(oy, ay);
                                let dz = fsub(oz, az);
                                let dist = fadd(
                                    fadd(fmul(dy, dy), fmul(dx, dx)),
                                    fmul(dz, dz),
                                );
                                if dist < best {
                                    best = dist;
                                    besti = idx;
                                    wrf(out, ox);
                                    wrf(out + 4, oy);
                                    wrf(out + 8, oz);
                                    wrf(out + 12, f32::from_bits(ob[3]));
                                }
                            }
                            j += 1;
                            s_lo = s_hi;
                            s_hi += 0x10;
                        }
                    }
                }
            }
            i += 1;
        }
        besti
    }
});
