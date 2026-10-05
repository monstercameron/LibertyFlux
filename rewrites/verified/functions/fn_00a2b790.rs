// original: 0x00a2b790 ped_candidate_scan

/// Scan nearby candidate peds and tag each one by range and facing.
///
/// `obj` (ECX) carries a selector at `+0x26c`: when bit 2 is set and the
/// record at `+0xb30` exists, the anchor position comes from that record's
/// matrix (`+0x20`, position at `+0x30`), otherwise from `obj`'s own matrix.
/// Callee 0 enumerates candidates around the base position (radius 15.0)
/// into a stack array with a count word. Each candidate is skipped when its
/// state byte at `+0x10b8` is 2 or its kind bits (`([+0x28] >> 10) & 0x1f`)
/// are 0 or 4. Of the rest, one beyond the squared-range constant from the
/// game data at file `0xfe8b40` is tagged 1 with the table pointer from file
/// `0x11735b4` plus `0x1388`; one inside range is tagged 3 or 9 by a facing
/// dot-product against the constant at file `0xfe8628`, with the same table
/// pointer plus `0x7d0`. The tag goes to `+0x2a` and the table pointer to
/// `+0x10` of callee 1's answer for that candidate.
///
/// The enumeration callee takes eleven arguments; the tag callee takes the
/// candidate. The returned value is whatever sits in the return slot last:
/// the enumeration's answer when the list is empty, a tagged callee answer,
/// or a skipped candidate pointer (the list load itself sets the slot).
///
/// The original reads its constants and table pointer through unrelocated
/// absolute addresses; the rewrite reads the relocated copies.
///
/// Original: 0x00a2b790 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a2b790(obj: u32) -> u32 {
    unsafe {
        const SEL: u32 = 0x26c;
        const SEL_BIT: u8 = 4;
        const ALT: u32 = 0xb30;
        const MATRIX: u32 = 0x20;
        const RADIUS_BITS: u32 = 0x41700000; // 15.0
        const STATE: u32 = 0x10b8;
        const STATE_SKIP: u8 = 2;
        const KIND_SHIFT: u32 = 10;
        const KIND_MASK: u32 = 0x1f;
        const TAG_FAR: u8 = 1;
        const TAG_FACING: u8 = 3;
        const TAG_AWAY: u8 = 9;
        const TAG_SLOT: u32 = 0x2a;
        const TABLE_SLOT: u32 = 0x10;
        const TABLE_FAR_OFF: u32 = 0x1388;
        const TABLE_NEAR_OFF: u32 = 0x7d0;
        const K_RANGE2: u32 = 0xfe8b40;
        const K_FACING: u32 = 0xfe8628;
        const G_TABLE: u32 = 0x11735b4;
        // Frame replica (offsets from the frame base): anchor xyz split as
        // the original lays them out, the callee-written count, the base
        // xyz vector passed to the callee, and the callee-written candidate
        // array.
        const F_AX: usize = 0x10;
        const F_AY: usize = 0x2c;
        const F_AZ: usize = 0x30;
        const F_COUNT: usize = 0x3c;
        const F_BASE: usize = 0x40;
        const F_ARRAY: usize = 0x50;
        const F_LEN: usize = 0x80;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn g32(ph: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(ph) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn gf(ph: u32) -> f32 {
            unsafe { f32::from_bits(g32(ph)) }
        }
        #[inline(always)]
        fn putf(f: &mut [u8], at: usize, v: f32) {
            f[at..at + 4].copy_from_slice(&v.to_bits().to_le_bytes());
        }
        #[inline(always)]
        fn getf(f: &[u8], at: usize) -> f32 {
            f32::from_bits(u32::from_le_bytes([f[at], f[at + 1], f[at + 2], f[at + 3]]))
        }

        let anchor = if rd8(obj.wrapping_add(SEL)) & SEL_BIT != 0 {
            let alt = rd32(obj.wrapping_add(ALT));
            if alt != 0 { rd32(alt.wrapping_add(MATRIX)) } else { rd32(obj.wrapping_add(MATRIX)) }
        } else {
            rd32(obj.wrapping_add(MATRIX))
        };
        let base = rd32(obj.wrapping_add(MATRIX));
        let mut frame = [0u8; F_LEN];
        putf(&mut frame, F_AX, rdf(anchor.wrapping_add(0x30)));
        putf(&mut frame, F_AY, rdf(anchor.wrapping_add(0x34)));
        putf(&mut frame, F_AZ, rdf(anchor.wrapping_add(0x38)));
        putf(&mut frame, F_BASE, rdf(base.wrapping_add(0x30)));
        putf(&mut frame, F_BASE + 4, rdf(base.wrapping_add(0x34)));
        putf(&mut frame, F_BASE + 8, rdf(base.wrapping_add(0x38)));
        let fbase = frame.as_mut_ptr() as u32;
        let first = lf_checker_rt::callee_cdecl!(
            0, u32,
            fbase.wrapping_add(F_BASE as u32),
            RADIUS_BITS, 1,
            fbase.wrapping_add(F_COUNT as u32),
            6,
            fbase.wrapping_add(F_ARRAY as u32),
            0, 1, 0, 0, 0
        );
        let count = u32::from_le_bytes([
            frame[F_COUNT],
            frame[F_COUNT + 1],
            frame[F_COUNT + 2],
            frame[F_COUNT + 3],
        ]) as i32;
        if count <= 0 {
            return first;
        }
        let (ax, ay, az) = (getf(&frame, F_AX), getf(&frame, F_AY), getf(&frame, F_AZ));
        let mut last = first;
        let mut i = 0i32;
        while i < count {
            let p = u32::from_le_bytes([
                frame[F_ARRAY + (i as usize) * 4],
                frame[F_ARRAY + (i as usize) * 4 + 1],
                frame[F_ARRAY + (i as usize) * 4 + 2],
                frame[F_ARRAY + (i as usize) * 4 + 3],
            ]);
            i += 1;
            // The candidate load itself sets the return slot; skipped
            // candidates are returned as-is.
            last = p;
            if rd8(p.wrapping_add(STATE)) == STATE_SKIP {
                continue;
            }
            let kind = (rd32(p.wrapping_add(0x28)) >> KIND_SHIFT) & KIND_MASK;
            if kind == 0 || kind == 4 {
                continue;
            }
            let q = rd32(p.wrapping_add(MATRIX));
            let dx = sub(rdf(q.wrapping_add(0x30)), ax);
            let dy = sub(rdf(q.wrapping_add(0x34)), ay);
            let dz = sub(rdf(q.wrapping_add(0x38)), az);
            let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            if dist2 > gf(K_RANGE2) {
                let r = lf_checker_rt::callee_cdecl!(1, u32, p);
                last = r;
                if r == 0 {
                    continue;
                }
                wr8(r.wrapping_add(TAG_SLOT), TAG_FAR);
                wr32(r.wrapping_add(TABLE_SLOT), g32(G_TABLE).wrapping_add(TABLE_FAR_OFF));
            } else {
                let fx = sub(ay, rdf(q.wrapping_add(0x34)));
                let fy = sub(ax, rdf(q.wrapping_add(0x30)));
                let facing = add(mul(fx, rdf(q.wrapping_add(0x14))), mul(fy, rdf(q.wrapping_add(0x10))));
                let r = lf_checker_rt::callee_cdecl!(1, u32, p);
                last = r;
                if r == 0 {
                    continue;
                }
                wr8(r.wrapping_add(TAG_SLOT), if facing > gf(K_FACING) { TAG_FACING } else { TAG_AWAY });
                wr32(r.wrapping_add(TABLE_SLOT), g32(G_TABLE).wrapping_add(TABLE_NEAR_OFF));
            }
        }
        last
    }
});
