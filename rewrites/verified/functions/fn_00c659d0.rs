// original: 0x00c659d0 CCutsceneObject::vf26

/// Bounding rectangle of a cutscene object over four corner transforms.
///
/// `this` points to the cutscene object. Six floats are read from it (two
/// corner triples at `+0x2f0` and `+0x300`); `+0x20` holds either null or a
/// pointer to a 3x4 matrix with translation. `out` receives four words:
/// min-x at `+0`, max-y at `+4`, max-x at `+8`, min-y at `+0xc`, seeded to
/// +1e6/-1e6/+... (-1e6 for the maxima, +1e6 for the minima).
///
/// Four rounds run, one per corner of the box. The corners pair one triple's
/// x with the other's y/z in turn: (ax,ay,az), (bx,by,bz), (bx,ay,az),
/// (ax,by,bz). When the matrix is present each corner is pushed through it
/// inline (`rx = ((m10*vy + m0*vx) + m20*vz) + m30` and likewise for y, with
/// a z row the last round skips); when it is null callee 1 computes the
/// corner instead (cdecl: out buffer, `this + 0x10`, corner triple). Each
/// round folds its x into the min/max pair at `out+0`/`out+8` and its y
/// into `out+0xc`/`out+4` with ordered strict comparisons, so a NaN corner
/// never replaces a bound.
///
/// The matrix pointer is reloaded every round but nothing in the function
/// stores to it, so all four rounds always take the same path. The third
/// word the callee writes and the inline z row are never read back.
///
/// Original: thiscall, one stack word (the out pointer), returns it in EAX.
/// Float operation order is the original's, pinned through `black_box`.
lf_checker_rt::export!(thiscall, rw_00c659d0(this: u32, out: u32) -> u32 {
    unsafe {
        const MATRIX_PTR: u32 = 0x20;
        const CALLEE_OBJ_OFF: u32 = 0x10;
        const CORNER_AX: u32 = 0x2f0;
        const CORNER_AY: u32 = 0x2f4;
        const CORNER_AZ: u32 = 0x2f8;
        const CORNER_BX: u32 = 0x300;
        const CORNER_BY: u32 = 0x304;
        const CORNER_BZ: u32 = 0x308;
        const CORNER_CALLEE: u32 = 1;
        const POS_INF_SEED: u32 = 0x4974_2400; // 1e6
        const NEG_INF_SEED: u32 = 0xc974_2400; // -1e6

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// One corner through the matrix: ((row1*vy + row0*vx) + row2*vz)
        /// + translation, per output row. Matches every round's inline path.
        #[inline(always)]
        unsafe fn xform(mat: u32, vx: f32, vy: f32, vz: f32) -> (f32, f32, f32) {
            unsafe {
                let m0 = rdf(mat);
                let m4 = rdf(mat + 4);
                let m8 = rdf(mat + 8);
                let m10 = rdf(mat + 0x10);
                let m14 = rdf(mat + 0x14);
                let m18 = rdf(mat + 0x18);
                let m20 = rdf(mat + 0x20);
                let m24 = rdf(mat + 0x24);
                let m28 = rdf(mat + 0x28);
                let rx = add(add(add(mul(m10, vy), mul(m0, vx)), mul(m20, vz)), rdf(mat + 0x30));
                let ry = add(add(add(mul(m14, vy), mul(m4, vx)), mul(m24, vz)), rdf(mat + 0x34));
                let rz = add(add(add(mul(m18, vy), mul(m8, vx)), mul(m28, vz)), rdf(mat + 0x38));
                (rx, ry, rz)
            }
        }
        /// One corner through the matrix, x/y rows only (fourth round).
        #[inline(always)]
        unsafe fn xform2(mat: u32, vx: f32, vy: f32, vz: f32) -> (f32, f32) {
            unsafe {
                let m0 = rdf(mat);
                let m4 = rdf(mat + 4);
                let m10 = rdf(mat + 0x10);
                let m14 = rdf(mat + 0x14);
                let m20 = rdf(mat + 0x20);
                let m24 = rdf(mat + 0x24);
                let rx = add(add(add(mul(m10, vy), mul(m0, vx)), mul(m20, vz)), rdf(mat + 0x30));
                let ry = add(add(add(mul(m14, vy), mul(m4, vx)), mul(m24, vz)), rdf(mat + 0x34));
                (rx, ry)
            }
        }
        /// One corner through the callee; only the first two words are read.
        #[inline(always)]
        unsafe fn corner_call(this: u32, vx: f32, vy: f32, vz: f32) -> (f32, f32) {
            unsafe {
                let vec = [vx.to_bits(), vy.to_bits(), vz.to_bits()];
                let mut corner = [0u32; 3];
                lf_checker_rt::callee_cdecl!(
                    CORNER_CALLEE,
                    u32,
                    core::ptr::addr_of_mut!(corner) as u32,
                    this.wrapping_add(CALLEE_OBJ_OFF),
                    core::ptr::addr_of!(vec) as u32
                );
                (f32::from_bits(corner[0]), f32::from_bits(corner[1]))
            }
        }
        /// Fold one corner into the bounds. Each comparison stores only on
        /// ordered strict less-than, matching the original's comiss/jbe, so
        /// NaN corners (and ties) leave the bounds alone.
        #[inline(always)]
        unsafe fn fold(out: u32, rx: f32, ry: f32) {
            unsafe {
                if rx < rdf(out) {
                    wrf(out, rx);
                }
                if rdf(out + 8) < rx {
                    wrf(out + 8, rx);
                }
                if ry < rdf(out + 0x0c) {
                    wrf(out + 0x0c, ry);
                }
                if rdf(out + 4) < ry {
                    wrf(out + 4, ry);
                }
            }
        }

        let ax = rdf(this + CORNER_AX);
        let ay = rdf(this + CORNER_AY);
        let az = rdf(this + CORNER_AZ);
        let bx = rdf(this + CORNER_BX);
        let by = rdf(this + CORNER_BY);
        let bz = rdf(this + CORNER_BZ);

        wr32(out, POS_INF_SEED);
        wr32(out + 0x0c, POS_INF_SEED);
        wr32(out + 8, NEG_INF_SEED);
        wr32(out + 4, NEG_INF_SEED);

        // Round 1: corner (ax, ay, az).
        let mat = rd32(this + MATRIX_PTR);
        if mat == 0 {
            let (rx, ry) = corner_call(this, ax, ay, az);
            fold(out, rx, ry);
        } else {
            let (rx, ry, _rz) = xform(mat, ax, ay, az);
            fold(out, rx, ry);
        }
        // Round 2: corner (bx, by, bz).
        let mat = rd32(this + MATRIX_PTR);
        if mat == 0 {
            let (rx, ry) = corner_call(this, bx, by, bz);
            fold(out, rx, ry);
        } else {
            let (rx, ry, _rz) = xform(mat, bx, by, bz);
            fold(out, rx, ry);
        }
        // Round 3: corner (bx, ay, az).
        let mat = rd32(this + MATRIX_PTR);
        if mat == 0 {
            let (rx, ry) = corner_call(this, bx, ay, az);
            fold(out, rx, ry);
        } else {
            let (rx, ry, _rz) = xform(mat, bx, ay, az);
            fold(out, rx, ry);
        }
        // Round 4: corner (ax, by, bz); inline path skips the z row.
        let mat = rd32(this + MATRIX_PTR);
        if mat == 0 {
            let (rx, ry) = corner_call(this, ax, by, bz);
            fold(out, rx, ry);
        } else {
            let (rx, ry) = xform2(mat, ax, by, bz);
            fold(out, rx, ry);
        }
        out
    }
});
