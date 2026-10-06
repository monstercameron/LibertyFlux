//! Differential cases, part 1: the task pose volume.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (written bytes and callee call logs). Each method has a deliberately
//! wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_peds_tasks::ped_task::{
        AngleTuning, ConeSolvers, FLAG_ACTIVE, FLAG_COPY, FLAG_SPHERE, FLAG_TRANSFORM, Link,
        LinkMatrix, Matrix, NormSlot, PoseFill, PoseSample, PoseVolume,
    };
    use lf_pedtaskdiff::rewrites::*;
    use lf_pedtaskdiff::{set_callee, set_relocated};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        ABS_VA, HALF_PI_VA, HALF_VA, NEG_VA, ONE_VA, Rng, TAU_VA, get_u32, heap_addr, lock, put_u32,
    };

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_peds_tasks::ped_task::{
            AngleTuning, Blended, ConeSolvers, FLAG_ACTIVE, FLAG_COPY, FLAG_SPHERE, LinkMatrix,
            Matrix, NORM_COUNT, NormSlot, PoseFill, PoseVolume, Transformed,
        };

        /// Reads the matrix row-major (transposed) instead of column-major.
        pub fn transform_row_major<L: LinkMatrix>(
            vol: &mut PoseVolume,
            links: &mut L,
        ) -> Option<Transformed> {
            let mut twisted = vol.clone();
            if let Some(link) = twisted.link.as_mut() {
                if let Some(m) = link.matrix.as_ref() {
                    let c = m.cols;
                    link.matrix = Some(Matrix {
                        cols: [
                            [c[0][0], c[1][0], c[2][0]],
                            [c[0][1], c[1][1], c[2][1]],
                            [c[0][2], c[1][2], c[2][2]],
                        ],
                        trans: m.trans,
                    });
                }
            }
            twisted.transform(links)
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        /// The blend with the halved-tag square dropped from the sum.
        pub fn blend_no_tag2<F: PoseFill>(vol: &PoseVolume, filler: &mut F, half: f32) -> Blended {
            let sample = filler.fill_pose(vol);
            if vol.flags & FLAG_COPY != 0 {
                let cf = f32::from_bits(sample.c);
                Blended {
                    out1: sample.a,
                    out2: mul(cf, cf),
                }
            } else {
                let (a0, a1, a2) = (sample.a[0], sample.a[1], sample.a[2]);
                let (b0, b1, b2) = (sample.b[0], sample.b[1], sample.b[2]);
                let cf = f32::from_bits(sample.c);
                let mut s3 = add(b1, a1);
                let mut s4 = add(b0, a0);
                let mut s2 = add(b2, a2);
                let mut s0 = cf;
                s3 = mul(s3, half);
                s4 = mul(s4, half);
                let mut d7 = sub(a1, s3);
                s2 = mul(s2, half);
                let mut d6 = sub(a0, s4);
                s0 = mul(s0, half);
                d7 = mul(d7, d7);
                let mut d5 = sub(a2, s2);
                d6 = mul(d6, d6);
                s0 = mul(s0, s0);
                d7 = add(d7, d6);
                d5 = mul(d5, d5);
                d7 = add(d7, d5);
                // Wrong: the `+ s0` term is missing.
                let _ = s0;
                Blended {
                    out1: [s4, s3, s2, sample.b[3]],
                    out2: d7,
                }
            }
        }

        /// The volume test with the sphere gate widened to `>=` and the
        /// cone height gates swapped.
        pub fn contains_flipped<P: PoseFill, S: ConeSolvers>(
            vol: &PoseVolume,
            filler: &mut P,
            solvers: &mut S,
            tuning: &AngleTuning,
            pt: [f32; 3],
        ) -> bool {
            if vol.flags & FLAG_ACTIVE == 0 {
                return false;
            }
            let sample = filler.fill_pose(vol);
            let fa = [sample.a[0], sample.a[1], sample.a[2]];
            let fb = [sample.b[0], sample.b[1], sample.b[2]];
            let fc = f32::from_bits(sample.c);
            if vol.flags & FLAG_SPHERE != 0 {
                let d4 = sub(fa[1], pt[1]);
                let d0 = sub(fa[0], pt[0]);
                let d8 = sub(fa[2], pt[2]);
                let dd = add(add(mul(d0, d0), mul(d4, d4)), mul(d8, d8));
                let cc = mul(fc, fc);
                // Wrong: `>=` instead of `>`.
                return cc >= dd;
            }
            let c1 = solvers.base_angle(sample.a[0], sample.a[1], sample.b[0], sample.b[1]);
            let mut ang = add(c1, tuning.half_pi);
            while ang < 0.0 {
                ang = add(ang, tuning.tau);
            }
            while ang > tuning.tau {
                ang = sub(ang, tuning.tau);
            }
            let radius = mul(fc, tuning.half);
            let cosv = solvers.cos_factor(ang);
            let px = add(mul(cosv, radius), fa[0]);
            let sinv = solvers.sin_factor(ang);
            let py = add(
                f32::from_bits(mul(sinv, radius).to_bits() ^ tuning.neg_mask),
                fa[1],
            );
            let (cx, cy) = (fa[0], fa[1]);
            let dx1 = sub(fb[0], cx);
            let dy1 = sub(fb[1], cy);
            let ex = sub(cx, px);
            let ey = sub(cy, py);
            let len1sq = add(mul(dy1, dy1), mul(dx1, dx1));
            let len2sq = add(mul(ey, ey), mul(ex, ex));
            let len1 = len1sq.sqrt();
            let len2 = len2sq.sqrt();
            let qx = sub(pt[0], cx);
            let qy = sub(pt[1], cy);
            let s1 = if len1sq == 0.0 {
                0.0
            } else {
                div(tuning.one, len1sq.sqrt())
            };
            let nx1 = mul(dx1, s1);
            let ny1 = mul(dy1, s1);
            let mut d1 = [0u32; 3];
            let mut d2 = [0u32; 3];
            let src1 = [nx1.to_bits(), ny1.to_bits(), sample.a[0].to_bits()];
            solvers.normalise(NormSlot::First, &mut d1, &src1, NORM_COUNT);
            let src2 = [qx.to_bits(), qy.to_bits(), nx1.to_bits()];
            solvers.normalise(NormSlot::Second, &mut d2, &src2, NORM_COUNT);
            let fd1 = [
                f32::from_bits(d1[0]),
                f32::from_bits(d1[1]),
                f32::from_bits(d1[2]),
            ];
            let fd2 = [
                f32::from_bits(d2[0]),
                f32::from_bits(d2[1]),
                f32::from_bits(d2[2]),
            ];
            let dot1 = add(
                add(mul(fd2[0], fd1[0]), mul(fd2[1], fd1[1])),
                mul(fd2[2], fd1[2]),
            );
            if !(dot1 >= 0.0) {
                return false;
            }
            if !(len1 >= dot1) {
                return false;
            }
            let len3sq = add(mul(ey, ey), mul(ex, ex));
            let s3 = if len3sq == 0.0 {
                0.0
            } else {
                div(tuning.one, len3sq.sqrt())
            };
            let mx = mul(ex, s3);
            let my = mul(ey, s3);
            let src3 = [mx.to_bits(), my.to_bits(), qx.to_bits()];
            solvers.normalise(NormSlot::Third, &mut d2, &src3, NORM_COUNT);
            let src4 = [qx.to_bits(), qy.to_bits(), nx1.to_bits()];
            solvers.normalise(NormSlot::Fourth, &mut d1, &src4, NORM_COUNT);
            let fd1 = [
                f32::from_bits(d1[0]),
                f32::from_bits(d1[1]),
                f32::from_bits(d1[2]),
            ];
            let fd2 = [
                f32::from_bits(d2[0]),
                f32::from_bits(d2[1]),
                f32::from_bits(d2[2]),
            ];
            let dot2 = add(
                add(mul(fd1[0], fd2[0]), mul(fd1[1], fd2[1])),
                mul(fd1[2], fd2[2]),
            );
            let absdot = f32::from_bits(dot2.to_bits() & tuning.abs_mask);
            if !(len2 >= absdot) {
                return false;
            }
            // Wrong: the height gates are swapped.
            if !(pt[2] >= fb[2]) {
                return false;
            }
            if !(fa[2] >= pt[2]) {
                return false;
            }
            true
        }
    }

    // Recording stubs for the rewrite side. Each test plants the stubs
    // its rewrites call; the serial lock keeps the logs exact.

    /// Expected link/matrix addresses for the build/fetch stubs.
    static LINK_ADDR: Mutex<u32> = Mutex::new(0);
    /// Matrix addresses the build stub publishes, in order.
    static BUILD_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static BUILD_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "thiscall" fn build_stub(link: u32) -> u32 {
        assert_eq!(link, *LINK_ADDR.lock().unwrap(), "build this");
        BUILD_LOG.lock().unwrap().push(link);
        let mat = BUILD_SCRIPT.lock().unwrap().pop_front().unwrap();
        unsafe { ((link + 0x20) as *mut u32).write_unaligned(mat) };
        0
    }

    static FETCH_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    extern "thiscall" fn fetch_stub(base: u32, mat: u32) -> u32 {
        let link = *LINK_ADDR.lock().unwrap();
        assert_eq!(base, link.wrapping_add(0x10), "fetch base");
        let cur = unsafe { ((link + 0x20) as *const u32).read_unaligned() };
        assert_eq!(mat, cur, "fetch matrix");
        FETCH_LOG.lock().unwrap().push((base, mat));
        0
    }

    /// Scripted fill words: (a words, b words, c word).
    static FILL_SCRIPT: Mutex<VecDeque<([u32; 4], [u32; 4], u32)>> = Mutex::new(VecDeque::new());
    static FILL_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static FILL_THIS: Mutex<u32> = Mutex::new(0);
    extern "thiscall" fn fill_stub(this: u32, a: u32, b: u32, c: u32) -> u32 {
        assert_eq!(this, *FILL_THIS.lock().unwrap(), "fill this");
        FILL_LOG.lock().unwrap().push(this);
        let (wa, wb, wc) = FILL_SCRIPT.lock().unwrap().pop_front().unwrap();
        unsafe {
            for (i, w) in wa.iter().enumerate() {
                ((a + i as u32 * 4) as *mut u32).write_unaligned(*w);
            }
            for (i, w) in wb.iter().enumerate() {
                ((b + i as u32 * 4) as *mut u32).write_unaligned(*w);
            }
            (c as *mut u32).write_unaligned(wc);
        }
        0
    }

    static ANGLE_LOG: Mutex<Vec<[u32; 4]>> = Mutex::new(Vec::new());
    static ANGLE_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn angle_stub(a0: u32, a1: u32, b0: u32, b1: u32) -> f32 {
        ANGLE_LOG.lock().unwrap().push([a0, a1, b0, b1]);
        f32::from_bits(ANGLE_SCRIPT.lock().unwrap().pop_front().unwrap())
    }

    static COS_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static COS_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn cos_stub(ang: u32) -> u32 {
        COS_LOG.lock().unwrap().push(ang);
        COS_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    static SIN_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static SIN_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn sin_stub(ang: u32) -> u32 {
        SIN_LOG.lock().unwrap().push(ang);
        SIN_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    /// Normaliser logs per callee id: (count, source words, written words).
    static NORM_LOG: Mutex<Vec<(u32, u32, [u32; 3], [u32; 3])>> = Mutex::new(Vec::new());
    static NORM_SCRIPT: Mutex<VecDeque<[u32; 3]>> = Mutex::new(VecDeque::new());
    macro_rules! norm_stub {
        ($name:ident, $id:expr) => {
            extern "thiscall" fn $name(dst: u32, src: u32, count: u32) -> u32 {
                assert_eq!(count, 1, "normaliser count");
                let s = unsafe {
                    [
                        ((src) as *const u32).read_unaligned(),
                        ((src + 4) as *const u32).read_unaligned(),
                        ((src + 8) as *const u32).read_unaligned(),
                    ]
                };
                let w = NORM_SCRIPT.lock().unwrap().pop_front().unwrap();
                unsafe {
                    for (i, word) in w.iter().enumerate() {
                        ((dst + i as u32 * 4) as *mut u32).write_unaligned(*word);
                    }
                }
                NORM_LOG.lock().unwrap().push(($id, count, s, w));
                0
            }
        };
    }
    norm_stub!(norm1_stub, 4);
    norm_stub!(norm2_stub, 5);
    norm_stub!(norm3_stub, 6);
    norm_stub!(norm4_stub, 7);

    fn clear_logs() {
        BUILD_LOG.lock().unwrap().clear();
        BUILD_SCRIPT.lock().unwrap().clear();
        FETCH_LOG.lock().unwrap().clear();
        FILL_LOG.lock().unwrap().clear();
        FILL_SCRIPT.lock().unwrap().clear();
        ANGLE_LOG.lock().unwrap().clear();
        ANGLE_SCRIPT.lock().unwrap().clear();
        COS_LOG.lock().unwrap().clear();
        COS_SCRIPT.lock().unwrap().clear();
        SIN_LOG.lock().unwrap().clear();
        SIN_SCRIPT.lock().unwrap().clear();
        NORM_LOG.lock().unwrap().clear();
        NORM_SCRIPT.lock().unwrap().clear();
    }

    // Lift-side fakes: scripted like the stubs, logging the same calls.

    struct FakeLinks {
        builds: u32,
        fetches: u32,
        install: VecDeque<Matrix>,
    }

    impl LinkMatrix for FakeLinks {
        fn build_matrix(&mut self, link: &mut Link) {
            self.builds += 1;
            if let Some(m) = self.install.pop_front() {
                link.matrix = Some(m);
            }
        }
        fn fetch_matrix(&mut self, _link: &Link) {
            self.fetches += 1;
        }
    }

    struct FakeFill {
        calls: u32,
        script: VecDeque<PoseSample>,
    }

    impl PoseFill for FakeFill {
        fn fill_pose(&mut self, _vol: &PoseVolume) -> PoseSample {
            self.calls += 1;
            self.script.pop_front().unwrap()
        }
    }

    #[derive(Debug, PartialEq)]
    enum SolverCall {
        Angle([u32; 4]),
        Cos(u32),
        Sin(u32),
        Norm(NormSlot, [u32; 3], u32),
    }

    struct FakeSolvers {
        calls: Vec<SolverCall>,
        angle: VecDeque<u32>,
        cos: VecDeque<u32>,
        sin: VecDeque<u32>,
        norm: VecDeque<[u32; 3]>,
    }

    impl ConeSolvers for FakeSolvers {
        fn base_angle(&mut self, a0: f32, a1: f32, b0: f32, b1: f32) -> f32 {
            self.calls.push(SolverCall::Angle([
                a0.to_bits(),
                a1.to_bits(),
                b0.to_bits(),
                b1.to_bits(),
            ]));
            f32::from_bits(self.angle.pop_front().unwrap())
        }
        fn cos_factor(&mut self, ang: f32) -> f32 {
            self.calls.push(SolverCall::Cos(ang.to_bits()));
            f32::from_bits(self.cos.pop_front().unwrap())
        }
        fn sin_factor(&mut self, ang: f32) -> f32 {
            self.calls.push(SolverCall::Sin(ang.to_bits()));
            f32::from_bits(self.sin.pop_front().unwrap())
        }
        fn normalise(&mut self, slot: NormSlot, dst: &mut [u32; 3], src: &[u32; 3], count: u32) {
            self.calls.push(SolverCall::Norm(slot, *src, count));
            *dst = self.norm.pop_front().unwrap();
        }
    }

    fn sample_of(a: [u32; 4], b: [u32; 4], c: u32) -> PoseSample {
        PoseSample {
            a: a.map(f32::from_bits),
            b: b.map(f32::from_bits),
            c,
        }
    }

    fn matrix_of(cols: [[u32; 3]; 3], trans: [u32; 3]) -> Matrix {
        Matrix {
            cols: cols.map(|col| col.map(f32::from_bits)),
            trans: trans.map(f32::from_bits),
        }
    }

    /// Plants a link image: fallback triple at +0x10, matrix word +0x20.
    fn plant_link(fallback: [u32; 3], mat: u32) -> Box<[u8; 0x30]> {
        let mut img = Box::new([0u8; 0x30]);
        for (i, w) in fallback.iter().enumerate() {
            put_u32(&mut img[..], 0x10 + i * 4, *w);
        }
        put_u32(&mut img[..], 0x20, mat);
        img
    }

    /// Plants a matrix image: columns at strides of 0x10, trans +0x30.
    fn plant_matrix(cols: [[u32; 3]; 3], trans: [u32; 3]) -> Box<[u8; 0x40]> {
        let mut img = Box::new([0u8; 0x40]);
        for (c, col) in cols.iter().enumerate() {
            for (r, w) in col.iter().enumerate() {
                put_u32(&mut img[..], c * 0x10 + r * 4, *w);
            }
        }
        for (i, w) in trans.iter().enumerate() {
            put_u32(&mut img[..], 0x30 + i * 4, *w);
        }
        img
    }

    /// Plants a volume image: vectors, tag, link word, flag byte.
    fn plant_vol(a: [u32; 4], b: [u32; 4], tag: u32, link: u32, flags: u8) -> Box<[u8; 0x30]> {
        let mut img = Box::new([0u8; 0x30]);
        for (i, w) in a.iter().enumerate() {
            put_u32(&mut img[..], i * 4, *w);
        }
        for (i, w) in b.iter().enumerate() {
            put_u32(&mut img[..], 0x10 + i * 4, *w);
        }
        put_u32(&mut img[..], 0x20, tag);
        put_u32(&mut img[..], 0x24, link);
        img[0x28] = flags;
        img
    }

    fn words4(buf: &[u8], off: usize) -> [u32; 4] {
        [
            get_u32(buf, off),
            get_u32(buf, off + 4),
            get_u32(buf, off + 8),
            get_u32(buf, off + 12),
        ]
    }

    #[test]
    fn transform_matches_rewrite() {
        let _guard = lock();
        set_callee(0, build_stub as usize as u32);
        set_callee(1, fetch_stub as usize as u32);
        let mut rng = Rng(0x40E9_11D1);
        let mut compared = 0u32;
        let mut caught = 0u32;

        // Case shape: flags low bits, linked, matrix starts present.
        // Covered: inactive, linkless copy, copy-with-tail over fallback
        // and over a prebuilt matrix, full transform with and without a
        // build. (The second build site is unreachable: a null after the
        // first build faults at the first vector, so nothing re-runs.)
        for case in 0..240 {
            clear_logs();
            let shape = case % 8;
            let flags: u8 = match shape {
                0 => (rng.u32() & 0xFC) as u8,
                1 => FLAG_ACTIVE | (rng.u32() as u8 & 0xFC),
                2 => FLAG_ACTIVE | (rng.u32() as u8 & 0xFC),
                3 => FLAG_ACTIVE | FLAG_TRANSFORM | (rng.u32() as u8 & 0xF8),
                4 => FLAG_ACTIVE | FLAG_TRANSFORM | (rng.u32() as u8 & 0xF8),
                5 => FLAG_ACTIVE | FLAG_TRANSFORM | FLAG_COPY | (rng.u32() as u8 & 0xF8),
                6 => (rng.u32() & 0xFE) as u8,
                _ => FLAG_ACTIVE | FLAG_TRANSFORM | (rng.u32() as u8 & 0xF8),
            };
            let linked = !matches!(shape, 0 | 1 | 6) || rng.below(2) == 0;
            let prebuilt = matches!(shape, 3 | 4 | 7) && rng.below(2) == 0;

            let mut aw = [0u32; 4];
            let mut bw = [0u32; 4];
            for w in aw.iter_mut().chain(bw.iter_mut()) {
                *w = rng.float_bits();
            }
            // Targeted asymmetric values on some transform-path cases so
            // the row-major mutant cannot hide behind symmetry.
            if shape == 4 && case % 3 == 0 {
                aw = [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits);
                bw = [5.0f32, 6.0, 7.0, 8.0].map(f32::to_bits);
            }
            let tag = rng.edge_word();
            let fallback = [rng.float_bits(), rng.float_bits(), rng.float_bits()];
            let cols_a = [
                [rng.float_bits(), rng.float_bits(), rng.float_bits()],
                [rng.float_bits(), rng.float_bits(), rng.float_bits()],
                [rng.float_bits(), rng.float_bits(), rng.float_bits()],
            ];
            let trans_a = [rng.float_bits(), rng.float_bits(), rng.float_bits()];

            let mat_a_img = plant_matrix(cols_a, trans_a);
            let mat_a = heap_addr(&mat_a_img);
            let start_mat = if prebuilt { mat_a } else { 0 };
            let link_img = plant_link(fallback, start_mat);
            let link_addr = heap_addr(&link_img);
            *LINK_ADDR.lock().unwrap() = link_addr;
            let vol_img = plant_vol(aw, bw, tag, if linked { link_addr } else { 0 }, flags);
            let vol_addr = heap_addr(&vol_img);

            // Scripts: the build publishes A when none is prebuilt.
            let will_build = linked && flags & FLAG_ACTIVE != 0 && flags & FLAG_TRANSFORM != 0;
            if will_build && !prebuilt {
                BUILD_SCRIPT.lock().unwrap().push_back(mat_a);
            }
            let mut install = VecDeque::new();
            if will_build && !prebuilt {
                install.push_back(matrix_of(cols_a, trans_a));
            }

            let out_a = Box::new([0x1111_1111u32; 4]);
            let out_b = Box::new([0x2222_2222u32; 4]);
            let out_c = Box::new(0x3333_3333u32);
            let (oa, ob, oc) = (heap_addr(&out_a), heap_addr(&out_b), heap_addr(&out_c));
            let rw_answer = unsafe {
                fn_00A62A10::rw_00a62a10(
                    vol_addr,
                    heap_addr(&out_a),
                    heap_addr(&out_b),
                    heap_addr(&out_c),
                )
            };
            let rw_a = words4(
                unsafe { core::slice::from_raw_parts(oa as *const u8, 16) },
                0,
            );
            let rw_b = words4(
                unsafe { core::slice::from_raw_parts(ob as *const u8, 16) },
                0,
            );
            let rw_c = unsafe { (oc as *const u32).read_unaligned() };

            let link = linked.then(|| Link {
                matrix: prebuilt.then(|| matrix_of(cols_a, trans_a)),
                fallback: fallback.map(f32::from_bits),
            });
            let mut vol = PoseVolume::new(
                aw.map(f32::from_bits),
                bw.map(f32::from_bits),
                tag,
                link,
                flags,
            );
            let mut fake = FakeLinks {
                builds: 0,
                fetches: 0,
                install,
            };
            let got = vol.transform(&mut fake);

            match &got {
                None => {
                    assert_eq!(rw_answer, 0, "case {case}: answer");
                    assert_eq!(rw_a, [0x1111_1111; 4], "case {case}: out_a untouched");
                    assert_eq!(rw_b, [0x2222_2222; 4], "case {case}: out_b untouched");
                    assert_eq!(rw_c, 0x3333_3333, "case {case}: out_c untouched");
                }
                Some(t) => {
                    assert_eq!(rw_answer, 1, "case {case}: answer");
                    assert_eq!(
                        [
                            t.a[0].to_bits(),
                            t.a[1].to_bits(),
                            t.a[2].to_bits(),
                            t.a[3].to_bits()
                        ],
                        rw_a,
                        "case {case}: out_a",
                    );
                    assert_eq!(
                        [
                            t.b[0].to_bits(),
                            t.b[1].to_bits(),
                            t.b[2].to_bits(),
                            t.b[3].to_bits()
                        ],
                        rw_b,
                        "case {case}: out_b",
                    );
                    assert_eq!(t.tag, rw_c, "case {case}: out_c");
                }
            }
            assert_eq!(
                BUILD_LOG.lock().unwrap().len() as u32,
                fake.builds,
                "case {case}: build count"
            );
            assert_eq!(
                FETCH_LOG.lock().unwrap().len() as u32,
                fake.fetches,
                "case {case}: fetch count"
            );
            compared += 1;

            // The wrong lift: row-major must differ on some transform path.
            if got.is_some() {
                let mut install = VecDeque::new();
                if will_build && !prebuilt {
                    install.push_back(matrix_of(cols_a, trans_a));
                }
                let link = linked.then(|| Link {
                    matrix: prebuilt.then(|| matrix_of(cols_a, trans_a)),
                    fallback: fallback.map(f32::from_bits),
                });
                let mut vol2 = PoseVolume::new(
                    aw.map(f32::from_bits),
                    bw.map(f32::from_bits),
                    tag,
                    link,
                    flags,
                );
                let mut fake2 = FakeLinks {
                    builds: 0,
                    fetches: 0,
                    install,
                };
                let bad = wrong::transform_row_major(&mut vol2, &mut fake2);
                let differs = match (&got, &bad) {
                    (Some(g), Some(b)) => g != b,
                    _ => got.is_some() != bad.is_some(),
                };
                if differs {
                    caught += 1;
                }
            }
        }
        assert_eq!(compared, 240);
        assert!(caught > 0, "row-major mutant was never caught");
    }

    #[test]
    fn blend_matches_rewrite() {
        let _guard = lock();
        set_callee(0, fill_stub as usize as u32);
        let half_word = Box::new(0u32);
        set_relocated(HALF_VA, heap_addr(&half_word));
        let mut rng = Rng(0xB1E4_D005);
        let mut compared = 0u32;
        let mut caught = 0u32;
        // Halves probed: the real one-half plus hostile values.
        let halves: [u32; 6] = [
            0x3F00_0000, // 0.5
            0x3F80_0000, // 1.0
            0x0000_0000, // 0.0
            0xC000_0000, // -2.0
            0x7F80_0000, // +inf
            0x7FC0_0000, // qNaN
        ];

        for case in 0..180 {
            clear_logs();
            let copy = case % 2 == 0;
            let flags: u8 =
                (if copy { FLAG_COPY } else { 0 }) | (rng.u32() as u8 & (0xFF ^ FLAG_COPY));
            let mut aw = [0u32; 4];
            let mut bw = [0u32; 4];
            for w in aw.iter_mut().chain(bw.iter_mut()) {
                *w = rng.float_bits();
            }
            let c = rng.float_bits();
            // Targeted zero-fill case: the dropped tag term then decides
            // the scalar alone (1.0 vs 0.0 with c = 2, half = 1/2).
            let (aw, bw, c, half_bits) = if case == 100 {
                ([0, 0, 0, 0], [0, 0, 0, 0], 0x4000_0000, 0x3F00_0000)
            } else {
                (aw, bw, c, halves[case % halves.len()])
            };
            unsafe { (heap_addr(&half_word) as *mut u32).write_unaligned(half_bits) };
            let half = f32::from_bits(half_bits);

            let vol_img = plant_vol([0, 0, 0, 0], [0, 0, 0, 0], 0, 0, flags);
            let vol_addr = heap_addr(&vol_img);
            *FILL_THIS.lock().unwrap() = vol_addr;
            FILL_SCRIPT.lock().unwrap().push_back((aw, bw, c));

            let out1 = Box::new([0xAAAA_AAAAu32; 4]);
            let out2 = Box::new([0xBBBB_BBBBu32; 1]);
            let (o1, o2) = (heap_addr(&out1), heap_addr(&out2));
            let rw_answer =
                unsafe { fn_00A62D50::rw_00a62d50(vol_addr, heap_addr(&out1), heap_addr(&out2)) };
            let rw_o1 = words4(
                unsafe { core::slice::from_raw_parts(o1 as *const u8, 16) },
                0,
            );
            let rw_o2 = unsafe { (o2 as *const u32).read_unaligned() };

            let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, flags);
            let mut fake = FakeFill {
                calls: 0,
                script: VecDeque::from([sample_of(aw, bw, c)]),
            };
            let got = vol.blend(&mut fake, half);

            assert_eq!(rw_answer, o2, "case {case}: out2 answer");
            assert_eq!(
                [
                    got.out1[0].to_bits(),
                    got.out1[1].to_bits(),
                    got.out1[2].to_bits(),
                    got.out1[3].to_bits()
                ],
                rw_o1,
                "case {case}: out1",
            );
            assert_eq!(got.out2.to_bits(), rw_o2, "case {case}: out2");
            assert_eq!(
                FILL_LOG.lock().unwrap().len() as u32,
                fake.calls,
                "case {case}: fills"
            );
            compared += 1;

            let mut fake2 = FakeFill {
                calls: 0,
                script: VecDeque::from([sample_of(aw, bw, c)]),
            };
            let bad = wrong::blend_no_tag2(&vol, &mut fake2, half);
            if bad.out1.map(|x| x.to_bits()) != rw_o1 || bad.out2.to_bits() != rw_o2 {
                caught += 1;
            }
        }
        assert_eq!(compared, 180);
        assert!(caught > 0, "dropped-tag mutant was never caught");
    }

    /// A cone case: fills, point, tuning, solver answers.
    struct ConeCase {
        flags: u8,
        aw: [u32; 4],
        bw: [u32; 4],
        c: u32,
        pt: [u32; 3],
        tuning: AngleTuning,
        angle: u32,
        cos: u32,
        sin: u32,
        norms: Vec<[u32; 3]>,
    }

    fn sane_tuning() -> AngleTuning {
        AngleTuning {
            half: 0.5,
            one: 1.0,
            half_pi: 1.5707964,
            tau: 6.2831855,
            neg_mask: 0x8000_0000,
            abs_mask: 0x7FFF_FFFF,
        }
    }

    fn run_cone(cc: &ConeCase, tune_img: &[Box<u32>; 6], case: u32) -> (u32, Vec<SolverCall>, u32) {
        // Returns (rewrite answer, lift solver calls, rewrite norm count).
        clear_logs();
        unsafe {
            (heap_addr(&tune_img[0]) as *mut u32).write_unaligned(cc.tuning.half.to_bits());
            (heap_addr(&tune_img[1]) as *mut u32).write_unaligned(cc.tuning.one.to_bits());
            (heap_addr(&tune_img[2]) as *mut u32).write_unaligned(cc.tuning.half_pi.to_bits());
            (heap_addr(&tune_img[3]) as *mut u32).write_unaligned(cc.tuning.tau.to_bits());
            (heap_addr(&tune_img[4]) as *mut u32).write_unaligned(cc.tuning.neg_mask);
            (heap_addr(&tune_img[5]) as *mut u32).write_unaligned(cc.tuning.abs_mask);
        }
        let vol_img = plant_vol([0, 0, 0, 0], [0, 0, 0, 0], 0, 0, cc.flags);
        let vol_addr = heap_addr(&vol_img);
        *FILL_THIS.lock().unwrap() = vol_addr;
        FILL_SCRIPT.lock().unwrap().push_back((cc.aw, cc.bw, cc.c));
        ANGLE_SCRIPT.lock().unwrap().push_back(cc.angle);
        COS_SCRIPT.lock().unwrap().push_back(cc.cos);
        SIN_SCRIPT.lock().unwrap().push_back(cc.sin);
        for n in cc.norms.iter() {
            NORM_SCRIPT.lock().unwrap().push_back(*n);
        }
        let pt_img = Box::new([0u8; 12]);
        for (i, w) in cc.pt.iter().enumerate() {
            put_u32(
                unsafe { core::slice::from_raw_parts_mut(heap_addr(&pt_img) as *mut u8, 12) },
                i * 4,
                *w,
            );
        }
        let rw_answer = unsafe { fn_00A63990::rw_00a63990(vol_addr, heap_addr(&pt_img), 0xDEAD) };

        let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, cc.flags);
        let mut fake_fill = FakeFill {
            calls: 0,
            script: VecDeque::from([sample_of(cc.aw, cc.bw, cc.c)]),
        };
        let mut fake = FakeSolvers {
            calls: Vec::new(),
            angle: VecDeque::from([cc.angle]),
            cos: VecDeque::from([cc.cos]),
            sin: VecDeque::from([cc.sin]),
            norm: VecDeque::from(cc.norms.clone()),
        };
        let pt = cc.pt.map(f32::from_bits);
        let got = vol.contains(&mut fake_fill, &mut fake, &cc.tuning, pt);
        assert_eq!(rw_answer, u32::from(got), "case {case}: answer");
        assert_eq!(
            FILL_LOG.lock().unwrap().len() as u32,
            fake_fill.calls,
            "case {case}: fills"
        );
        // The solver call logs must agree in order and arguments.
        let mut want_calls = Vec::new();
        for a in ANGLE_LOG.lock().unwrap().iter() {
            want_calls.push(SolverCall::Angle(*a));
        }
        for c in COS_LOG.lock().unwrap().iter() {
            want_calls.push(SolverCall::Cos(*c));
        }
        for s in SIN_LOG.lock().unwrap().iter() {
            want_calls.push(SolverCall::Sin(*s));
        }
        let slot_of = |id: u32| match id {
            4 => NormSlot::First,
            5 => NormSlot::Second,
            6 => NormSlot::Third,
            _ => NormSlot::Fourth,
        };
        for (id, count, src, _w) in NORM_LOG.lock().unwrap().iter() {
            want_calls.push(SolverCall::Norm(slot_of(*id), *src, *count));
        }
        // Rewrite logs group by stub while the lift logs in order; both
        // orders are fixed, so compare the merged sequences per kind.
        let mut lift_angles = Vec::new();
        let mut lift_cos = Vec::new();
        let mut lift_sin = Vec::new();
        let mut lift_norms = Vec::new();
        for c in fake.calls.iter() {
            match c {
                SolverCall::Angle(a) => lift_angles.push(*a),
                SolverCall::Cos(c) => lift_cos.push(*c),
                SolverCall::Sin(s) => lift_sin.push(*s),
                SolverCall::Norm(slot, src, count) => lift_norms.push((*slot, *src, *count)),
            }
        }
        let mut rw_angles = Vec::new();
        let mut rw_cos = Vec::new();
        let mut rw_sin = Vec::new();
        let mut rw_norms = Vec::new();
        for c in want_calls.iter() {
            match c {
                SolverCall::Angle(a) => rw_angles.push(*a),
                SolverCall::Cos(c) => rw_cos.push(*c),
                SolverCall::Sin(s) => rw_sin.push(*s),
                SolverCall::Norm(slot, src, count) => rw_norms.push((*slot, *src, *count)),
            }
        }
        assert_eq!(lift_angles, rw_angles, "case {case}: angle calls");
        assert_eq!(lift_cos, rw_cos, "case {case}: cos calls");
        assert_eq!(lift_sin, rw_sin, "case {case}: sin calls");
        assert_eq!(lift_norms, rw_norms, "case {case}: norm calls");
        // Normaliser ids run 4,5,6,7 in order when all four run.
        let ids: Vec<u32> = NORM_LOG.lock().unwrap().iter().map(|x| x.0).collect();
        for w in ids.windows(2) {
            assert!(w[0] < w[1], "case {case}: norm order");
        }
        (rw_answer, fake.calls, NORM_LOG.lock().unwrap().len() as u32)
    }

    #[test]
    fn cone_matches_rewrite() {
        let _guard = lock();
        set_callee(0, fill_stub as usize as u32);
        set_callee(1, angle_stub as usize as u32);
        set_callee(2, cos_stub as usize as u32);
        set_callee(3, sin_stub as usize as u32);
        set_callee(4, norm1_stub as usize as u32);
        set_callee(5, norm2_stub as usize as u32);
        set_callee(6, norm3_stub as usize as u32);
        set_callee(7, norm4_stub as usize as u32);
        let tune_img: [Box<u32>; 6] = Default::default();
        set_relocated(HALF_VA, heap_addr(&tune_img[0]));
        set_relocated(ONE_VA, heap_addr(&tune_img[1]));
        set_relocated(HALF_PI_VA, heap_addr(&tune_img[2]));
        set_relocated(TAU_VA, heap_addr(&tune_img[3]));
        set_relocated(NEG_VA, heap_addr(&tune_img[4]));
        set_relocated(ABS_VA, heap_addr(&tune_img[5]));

        let one = 1.0f32.to_bits();
        let zero = 0.0f32.to_bits();
        // Targeted cases first.
        let targeted = vec![
            // Inactive: no calls at all.
            ConeCase {
                flags: 0,
                aw: [one, one, one, one],
                bw: [one, one, one, one],
                c: one,
                pt: [zero, zero, zero],
                tuning: sane_tuning(),
                angle: zero,
                cos: one,
                sin: zero,
                norms: vec![],
            },
            // Sphere hit: centre at origin, tag 2, point at distance 1.
            ConeCase {
                flags: FLAG_ACTIVE | FLAG_SPHERE,
                aw: [zero, zero, zero, zero],
                bw: [zero, zero, zero, zero],
                c: 0x4000_0000,
                pt: [one, zero, zero],
                tuning: sane_tuning(),
                angle: zero,
                cos: one,
                sin: zero,
                norms: vec![],
            },
            // Sphere boundary: cc == dd answers false (strict).
            ConeCase {
                flags: FLAG_ACTIVE | FLAG_SPHERE,
                aw: [zero, zero, zero, zero],
                bw: [zero, zero, zero, zero],
                c: 0x4000_0000,
                pt: [0x4000_0000, zero, zero],
                tuning: sane_tuning(),
                angle: zero,
                cos: one,
                sin: zero,
                norms: vec![],
            },
            // Sphere NaN: unordered comparisons fail.
            ConeCase {
                flags: FLAG_ACTIVE | FLAG_SPHERE,
                aw: [0x7FC0_0000, zero, zero, zero],
                bw: [zero, zero, zero, zero],
                c: 0x4000_0000,
                pt: [one, zero, zero],
                tuning: sane_tuning(),
                angle: zero,
                cos: one,
                sin: zero,
                norms: vec![],
            },
            // Full cone pass (numbers worked out by hand: angle 0,
            // radius 1, dots 2 and 0.5 inside lengths 4 and 1).
            ConeCase {
                flags: FLAG_ACTIVE,
                aw: [zero, zero, zero, zero],
                bw: [0x4080_0000, zero, 0x4120_0000, zero],
                c: 0x4000_0000,
                pt: [0x4000_0000, zero, 0x40A0_0000],
                tuning: AngleTuning {
                    half: 0.5,
                    one: 1.0,
                    half_pi: 0.0,
                    tau: 6.2831855,
                    neg_mask: 0,
                    abs_mask: 0x7FFF_FFFF,
                },
                angle: zero,
                cos: one,
                sin: zero,
                norms: vec![
                    [one, zero, zero],
                    [0x4000_0000, zero, zero],
                    [0x3F00_0000, zero, zero],
                    [one, zero, zero],
                ],
            },
            // Cone early exit: negative first dot.
            ConeCase {
                flags: FLAG_ACTIVE,
                aw: [zero, zero, zero, zero],
                bw: [0x4080_0000, zero, 0x4120_0000, zero],
                c: 0x4000_0000,
                pt: [0x4000_0000, zero, 0x40A0_0000],
                tuning: AngleTuning {
                    half: 0.5,
                    one: 1.0,
                    half_pi: 0.0,
                    tau: 6.2831855,
                    neg_mask: 0,
                    abs_mask: 0x7FFF_FFFF,
                },
                angle: zero,
                cos: one,
                sin: zero,
                norms: vec![[one, zero, zero], [0xBF80_0000, zero, zero]],
            },
            // Cone wrap-up: the angle starts below zero.
            ConeCase {
                flags: FLAG_ACTIVE,
                aw: [zero, zero, zero, zero],
                bw: [0x4080_0000, zero, 0x4120_0000, zero],
                c: 0x4000_0000,
                pt: [0x4000_0000, zero, 0x40A0_0000],
                tuning: AngleTuning {
                    half: 0.5,
                    one: 1.0,
                    half_pi: 0.0,
                    tau: 6.2831855,
                    neg_mask: 0,
                    abs_mask: 0x7FFF_FFFF,
                },
                angle: 0xC120_0000,
                cos: one,
                sin: zero,
                norms: vec![
                    [one, zero, zero],
                    [0x4000_0000, zero, zero],
                    [0x3F00_0000, zero, zero],
                    [one, zero, zero],
                ],
            },
            // Cone wrap-down twice: the angle starts far above tau.
            ConeCase {
                flags: FLAG_ACTIVE,
                aw: [zero, zero, zero, zero],
                bw: [0x4080_0000, zero, 0x4120_0000, zero],
                c: 0x4000_0000,
                pt: [0x4000_0000, zero, 0x40A0_0000],
                tuning: AngleTuning {
                    half: 0.5,
                    one: 1.0,
                    half_pi: 0.0,
                    tau: 1.0,
                    neg_mask: 0,
                    abs_mask: 0x7FFF_FFFF,
                },
                angle: 0x4020_0000,
                cos: one,
                sin: zero,
                norms: vec![
                    [one, zero, zero],
                    [0x4000_0000, zero, zero],
                    [0x3F00_0000, zero, zero],
                    [one, zero, zero],
                ],
            },
        ];
        let mut compared = 0u32;
        let mut caught_sphere = 0u32;
        let mut caught_cone = 0u32;
        for (i, cc) in targeted.iter().enumerate() {
            let (rw_answer, _, _) = run_cone(cc, &tune_img, i as u32);
            compared += 1;
            // The wrong lift runs on fresh fakes with the same scripts.
            let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, cc.flags);
            let mut fake_fill = FakeFill {
                calls: 0,
                script: VecDeque::from([sample_of(cc.aw, cc.bw, cc.c)]),
            };
            let mut fake = FakeSolvers {
                calls: Vec::new(),
                angle: VecDeque::from([cc.angle]),
                cos: VecDeque::from([cc.cos]),
                sin: VecDeque::from([cc.sin]),
                norm: VecDeque::from(cc.norms.clone()),
            };
            let bad = wrong::contains_flipped(
                &vol,
                &mut fake_fill,
                &mut fake,
                &cc.tuning,
                cc.pt.map(f32::from_bits),
            );
            if u32::from(bad) != rw_answer {
                if cc.flags & FLAG_SPHERE != 0 {
                    caught_sphere += 1;
                } else {
                    caught_cone += 1;
                }
            }
        }

        // Fuzz: sane tuning, hostile fills and points.
        let mut rng = Rng(0xC0E5_ED);
        for case in 0..160 {
            let sphere = case % 3 == 0;
            let mut flags = FLAG_ACTIVE | (rng.u32() as u8 & 0xF0);
            if sphere {
                flags |= FLAG_SPHERE;
            } else {
                flags &= !FLAG_SPHERE;
            }
            if case % 11 == 0 {
                flags &= !FLAG_ACTIVE;
            }
            let mut aw = [0u32; 4];
            let mut bw = [0u32; 4];
            for w in aw.iter_mut().chain(bw.iter_mut()) {
                *w = rng.float_bits();
            }
            let pt = [rng.float_bits(), rng.float_bits(), rng.float_bits()];
            // Sane angles only: the wrap loops need a finite angle and
            // a positive tau to terminate, as in the original.
            let angle = f32::from_bits(rng.float_bits());
            let angle = if angle.is_finite() {
                (angle % 20.0).to_bits()
            } else {
                0.0f32.to_bits()
            };
            let mut norms = Vec::new();
            for _ in 0..4 {
                norms.push([rng.float_bits(), rng.float_bits(), rng.float_bits()]);
            }
            let cc = ConeCase {
                flags,
                aw,
                bw,
                c: rng.float_bits(),
                pt,
                tuning: sane_tuning(),
                angle,
                cos: rng.float_bits(),
                sin: rng.float_bits(),
                norms,
            };
            let (rw_answer, _, _) = run_cone(&cc, &tune_img, 1000 + case);
            compared += 1;
            let vol = PoseVolume::new([0.0; 4], [0.0; 4], 0, None, cc.flags);
            let mut fake_fill = FakeFill {
                calls: 0,
                script: VecDeque::from([sample_of(cc.aw, cc.bw, cc.c)]),
            };
            let mut fake = FakeSolvers {
                calls: Vec::new(),
                angle: VecDeque::from([cc.angle]),
                cos: VecDeque::from([cc.cos]),
                sin: VecDeque::from([cc.sin]),
                norm: VecDeque::from(cc.norms.clone()),
            };
            let bad = wrong::contains_flipped(
                &vol,
                &mut fake_fill,
                &mut fake,
                &cc.tuning,
                cc.pt.map(f32::from_bits),
            );
            if u32::from(bad) != rw_answer {
                if cc.flags & FLAG_SPHERE != 0 {
                    caught_sphere += 1;
                } else {
                    caught_cone += 1;
                }
            }
        }
        assert_eq!(compared, 8 + 160);
        assert!(caught_sphere > 0, "sphere mutant was never caught");
        assert!(caught_cone > 0, "cone mutant was never caught");
    }
}
