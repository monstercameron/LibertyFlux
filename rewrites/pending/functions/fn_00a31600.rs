// original: 0x00a31600 CEntity::vf26
/// Project the four model corners into an output bounds record.
///
/// `this` is the entity, `out` a caller record of four floats. The model
/// index word at +0x2E selects the parameter row; when the entity matrix at
/// +0x20 is present each corner is transformed inline, otherwise the shared
/// placement helper (callee 1) projects it through the placement at +0x10.
/// Returns `out`.
export!(thiscall, rw_00a31600(this: u32, out: u32) -> u32 {
    unsafe {
        const PARAM_TABLE: u32 = 0x0129_5cd8;
        const BIG: f32 = f32::from_bits(0x4974_2400);
        const NEG_BIG: f32 = f32::from_bits(0xc974_2400);

        /// Transform one corner through the entity matrix.
        ///
        /// `m` points at the matrix, `(a, b, c)` is the corner triple. Returns the
        /// two projected components. Operation order matches the original exactly:
        /// each result accumulates `base + offset + depth + translation` left to
        /// right so NaN payloads agree bit for bit.
        #[inline(always)]
        fn corner_pair(m: u32, a: f32, b: f32, c: f32) -> (f32, f32) {
            unsafe {
                let base = m as *const f32;
                let m0 = base.read();
                let m4 = base.add(1).read();
                let m10 = base.add(4).read();
                let m14 = base.add(5).read();
                let m20 = base.add(8).read();
                let m24 = base.add(9).read();
                let m30 = base.add(12).read();
                let m34 = base.add(13).read();
                let mut first = m10 * b;
                first += m0 * a;
                first += m20 * c;
                first += m30;
                let mut second = m14 * b;
                second += m4 * a;
                second += m24 * c;
                second += m34;
                (first, second)
            }
        }

        /// Fold one projected corner into the running bounds: slot 0 is the first
        /// component's minimum, slot 2 its maximum, slot 3 the second component's
        /// minimum and slot 1 its maximum.
        #[inline(always)]
        fn accumulate(out: u32, first: f32, second: f32) {
            unsafe {
                let o = out as *mut f32;
                if o.read() > first {
                    o.write(first);
                }
                if first > o.add(2).read() {
                    o.add(2).write(first);
                }
                if o.add(3).read() > second {
                    o.add(3).write(second);
                }
                if second > o.add(1).read() {
                    o.add(1).write(second);
                }
            }
        }
        let slot = ((this as *const u8).add(0x2e) as *const i16).read();
        let params = global::<u32>(PARAM_TABLE).offset(slot as isize).read();
        let mat = ((this as *const u32).add(8)).read();
        let p = params as *const f32;
        let p20 = p.add(8).read();
        let p24 = p.add(9).read();
        let p28 = p.add(10).read();
        let p30 = p.add(12).read();
        let p34 = p.add(13).read();
        let p38 = p.add(14).read();
        let o = out as *mut f32;
        o.write(BIG);
        o.add(1).write(NEG_BIG);
        o.add(2).write(NEG_BIG);
        o.add(3).write(BIG);
        let place = this.wrapping_add(0x10);
        let mut tmp = [0f32; 3];
        // Block 1: corner (p20, p24, p28).
        let (first, second) = if mat != 0 {
            corner_pair(mat, p20, p24, p28)
        } else {
            let corner = [p20, p24, p28];
            let _: u32 = callee_cdecl!(1, u32, tmp.as_mut_ptr() as u32, place, corner.as_ptr() as u32);
            (tmp[0], tmp[1])
        };
        accumulate(out, first, second);
        // Block 2: corner (p30, p34, p38).
        let (first, second) = if mat != 0 {
            corner_pair(mat, p30, p34, p38)
        } else {
            let corner = [p30, p34, p38];
            let _: u32 = callee_cdecl!(1, u32, tmp.as_mut_ptr() as u32, place, corner.as_ptr() as u32);
            (tmp[0], tmp[1])
        };
        accumulate(out, first, second);
        // Block 3: corner (p30, p24, p28).
        let (first, second) = if mat != 0 {
            corner_pair(mat, p30, p24, p28)
        } else {
            let corner = [p30, p24, p28];
            let _: u32 = callee_cdecl!(1, u32, tmp.as_mut_ptr() as u32, place, corner.as_ptr() as u32);
            (tmp[0], tmp[1])
        };
        accumulate(out, first, second);
        // Block 4: corner (p20, p34, p38).
        let (first, second) = if mat != 0 {
            corner_pair(mat, p20, p34, p38)
        } else {
            let corner = [p20, p34, p38];
            let _: u32 = callee_cdecl!(1, u32, tmp.as_mut_ptr() as u32, place, corner.as_ptr() as u32);
            (tmp[0], tmp[1])
        };
        accumulate(out, first, second);
        out
    }
});
