// original: 0x00B02D30 box_planes_visibility_test (proposed)

/// Tests whether an axis-aligned box survives a set of plane tests.
///
/// `this` points to an object whose first word is the base of an element
/// array with a row stride of 0x5580 bytes. `idx` selects the row. `a` and
/// `b` point to two 3-float corners (the ends of the box diagonal); `m`, when
/// non-null, points to a 3x3 matrix with translation (rows at +0x00, +0x10,
/// +0x20, translation at +0x30; the words at +0x0c, +0x1c, +0x2c are never
/// read) applied to all eight box corners first. The last two stack arguments
/// are never read. Original convention: thiscall, six stack words, callee
/// cleans 0x18, returns the answer in AL.
///
/// Behaviour: reads the entry count N at row+0x4000 and returns 0 at once
/// when it is 0. Otherwise builds the eight corners (raw box corners when `m`
/// is null, matrix-transformed ones when not; the float operation order below
/// is the original's) and returns 0 when N is negative. Then, for each entry
/// j (flag at row+0xe0+j*0x100, plane count at row+0xec+j*0x100, plane data
/// at row+0x08+j*0x100, 16 bytes per plane): an entry whose flag is non-zero
/// is skipped; an entry with a plane count at or below zero returns 1 at
/// once; otherwise every plane is tested against all eight corners as
/// d = ((x*py + y*px) + z*pz) - w with the plane words (px, py, pz, w) at
/// offsets (-4, -8, 0, +4) of the plane slot, and a single corner with d
/// above the threshold global (an ordered float compare against -0.0 by
/// default) moves on to the next entry. The first entry passing every plane
/// returns 1; when no entry passes, returns 0. Reads one float global and no
/// other shared state; writes nothing outside its own stack frame.
///
/// Edge cases: N = 0 returns 0 before touching A or B; N < 0 runs the corner
/// build (so wild A/B/M still fault) and then returns 0. A null `m` skips the
/// transform. NaN corner or plane values never compare above, so they pass.
/// The original also copies one uninitialized stack word into the unused w
/// lane of the corners; it is unobservable and not reproduced here.
lf_checker_rt::export!(thiscall, rw_00B02D30(this: u32, idx: u32, a: u32, b: u32, m: u32, _u1: u32, _u2: u32) -> u8 {
    unsafe {
        const ROW_STRIDE: u32 = 0x5580;
        const COUNT_OFF: u32 = 0x4000;
        const FIRST_ENTRY: u32 = 0xec;
        const ENTRY_STRIDE: u32 = 0x100;
        const PLANE_BACK: u32 = 0xe4;
        const PLANE_STRIDE: u32 = 0x10;
        const THRESH_VA: u32 = 0x00FE8D1C;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }

        let base = rd32(this);
        let row = base.wrapping_add(idx.wrapping_mul(ROW_STRIDE));
        let n = rd32(row.wrapping_add(COUNT_OFF));
        if n == 0 {
            return 0;
        }
        // Original load order (fault parity): A.x, B.xyz, A.y, A.z.
        let ax = rdf(a);
        let bx = rdf(b);
        let by = rdf(b.wrapping_add(4));
        let bz = rdf(b.wrapping_add(8));
        let ay = rdf(a.wrapping_add(4));
        let az = rdf(a.wrapping_add(8));
        // Eight corners in the order the plane test visits them.
        let corners: [[f32; 3]; 8];
        if m == 0 {
            corners = [
                [ax, ay, az],
                [bx, by, bz],
                [ax, ay, bz],
                [ax, by, az],
                [ax, by, bz],
                [bx, by, az],
                [bx, ay, bz],
                [bx, ay, az],
            ];
        } else {
            let m00 = rdf(m);
            let m04 = rdf(m.wrapping_add(0x04));
            let m08 = rdf(m.wrapping_add(0x08));
            let m10 = rdf(m.wrapping_add(0x10));
            let m14 = rdf(m.wrapping_add(0x14));
            let m18 = rdf(m.wrapping_add(0x18));
            let m20 = rdf(m.wrapping_add(0x20));
            let m24 = rdf(m.wrapping_add(0x24));
            let m28 = rdf(m.wrapping_add(0x28));
            let m30 = rdf(m.wrapping_add(0x30));
            let m34 = rdf(m.wrapping_add(0x34));
            let m38 = rdf(m.wrapping_add(0x38));
        let t1 = mul(m20, az);
        let t2 = mul(m10, ay);
        let t3 = mul(m00, ax);
        let t4 = mul(m14, ay);
        let t5 = mul(m04, ax);
        let t6 = add(t3, t2);
        let t7 = add(t6, t1);
        let t8 = mul(m24, az);
        let t9 = add(t5, t4);
        let t10 = add(t7, m30);
        let t11 = mul(m08, ax);
        let t12 = add(t9, t8);
        let t13 = mul(m18, ay);
        let t14 = add(t12, m34);
        let t15 = mul(m28, az);
        let t16 = mul(m28, bz);
        let t17 = add(t11, t13);
        let t18 = add(t3, t2);
        let t19 = add(t17, t15);
        let t20 = add(t5, t4);
        let t21 = add(t19, m38);
        let t22 = mul(m20, bz);
        let t23 = add(t18, t22);
        let t24 = mul(m24, bz);
        let t25 = add(t23, m30);
        let t26 = add(t20, t24);
        let t27 = add(t11, t13);
        let t28 = add(t26, m34);
        let t29 = add(t27, t16);
        let t30 = add(t29, m38);
        let t31 = mul(m10, by);
        let t32 = mul(m14, by);
        let t33 = add(t31, t3);
        let t34 = add(t33, t1);
        let t35 = add(t32, t5);
        let t36 = add(t34, m30);
        let t37 = add(t35, t8);
        let t38 = add(t37, m34);
        let t39 = mul(m18, by);
        let t40 = add(t31, t3);
        let t41 = add(t32, t5);
        let t42 = add(t40, t22);
        let t43 = add(t39, t11);
        let t44 = add(t41, t24);
        let t45 = add(t42, m30);
        let t46 = add(t43, t15);
        let t47 = add(t44, m34);
        let t48 = add(t46, m38);
        let t49 = add(t39, t11);
        let t50 = add(t49, t16);
        let t51 = add(t50, m38);
        let t52 = mul(m00, bx);
        let t53 = mul(m04, bx);
        let t54 = add(t52, t31);
        let t55 = add(t54, t22);
        let t56 = mul(m08, bx);
        let t57 = add(t55, m30);
        let t58 = add(t53, t32);
        let t59 = add(t56, t39);
        let t60 = add(t52, t31);
        let t61 = add(t58, t24);
        let t62 = add(t59, t16);
        let t63 = add(t60, t1);
        let t64 = add(t61, m34);
        let t65 = add(t62, m38);
        let t66 = add(t63, m30);
        let t67 = add(t53, t32);
        let t68 = add(t67, t8);
        let t69 = add(t56, t39);
        let t70 = add(t52, t2);
        let t71 = add(t68, m34);
        let t72 = add(t69, t15);
        let t73 = add(t70, t22);
        let t74 = add(t53, t4);
        let t75 = add(t72, m38);
        let t76 = add(t73, m30);
        let t77 = add(t74, t24);
        let t78 = add(t77, m34);
        let t79 = add(t56, t13);
        let t80 = add(t52, t2);
        let t81 = add(t53, t4);
        let t82 = add(t79, t16);
        let t83 = add(t80, t1);
        let t84 = add(t81, t8);
        let t85 = add(t82, m38);
        let t86 = add(t83, m30);
        let t87 = add(t84, m34);
        let t88 = add(t56, t13);
        let t89 = add(t88, t15);
        let t90 = add(t89, m38);
            corners = [
                [t10, t14, t21], // c50
                [t57, t64, t65], // c40
                [t25, t28, t30], // c80
                [t36, t38, t48], // cc0
                [t45, t47, t51], // cb0
                [t66, t71, t75], // c90
                [t76, t78, t85], // ca0
                [t86, t87, t90], // cd0
            ];
        }
        if (n as i32) <= 0 {
            return 0;
        }
        let thresh = f32::from_bits(rd32(lf_checker_rt::relocated(THRESH_VA)));
        let mut e = row.wrapping_add(FIRST_ENTRY);
        let mut j: u32 = 0;
        'outer: while j < n {
            let flag = rd32(e.wrapping_sub(0x0c));
            if flag != 0 {
                e = e.wrapping_add(ENTRY_STRIDE);
                j += 1;
                continue;
            }
            let mc = rd32(e) as i32;
            if mc <= 0 {
                return 1;
            }
            let mut p = e.wrapping_sub(PLANE_BACK);
            let mut q: i32 = 0;
            while q < mc {
                let px = rdf(p.wrapping_sub(4));
                let py = rdf(p.wrapping_sub(8));
                let pz = rdf(p);
                let pw = rdf(p.wrapping_add(4));
                for k in corners {
                    let mut d = mul(k[0], py);
                    d = add(d, mul(k[1], px));
                    d = add(d, mul(k[2], pz));
                    d = sub(d, pw);
                    if d > thresh {
                        e = e.wrapping_add(ENTRY_STRIDE);
                        j += 1;
                        continue 'outer;
                    }
                }
                p = p.wrapping_add(PLANE_STRIDE);
                q += 1;
            }
            return 1;
        }
        0
    }
});
