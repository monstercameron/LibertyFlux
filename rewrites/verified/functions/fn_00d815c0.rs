// original: 0x00D815C0 scan_box_commit_matches (proposed)

/// Scan a node list for entities inside a float box, scoring each
/// candidate through a six-float helper and committing matches through the
/// routine at 0x00D817A0.
///
/// `head` points to the list head word; each node holds an entity pointer
/// and the next node. `base` is the reference entity (a node pointing at it
/// is skipped). The box is four bound words (`f10`..`f1c`); `gate` points
/// to a float that must stay positive or the scan stops; `flagobj`
/// (`+FLAGTEST`) selects negated helper inputs; `tag` is carried to the
/// commit call untouched.
///
/// Per node: entities already stamped with the global word (`+STAMP`) or
/// without the ready bit (`+READY`) are skipped; otherwise the entity is
/// stamped and its position (`+POS`, three floats) must fall inside the box
/// and within `LIMIT1` of the base depth in the third axis. Survivors call
/// callee 1 (cdecl, six floats, float result) with the base direction
/// (possibly negated), the base depth pair and the entity's first two
/// position floats; the answer scales the direction and shifts by the base
/// depth, and when the result lands within `LIMIT2` of the entity depth,
/// callee 2 commits `(entity, base, gate, f24, flagobj, tag)`.
///
/// Original: 0x00D815C0 (cdecl, ten stack words, no return value).
export!(cdecl, rw_00D815C0(head: u32, base: u32, f10: u32, f14: u32, f18: u32, f1c: u32, gate: u32, f24: u32, flagobj: u32, tag: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x20;
        const STAMP: u32 = 0x3c;
        const READY: u32 = 0x24;
        const FLAGTEST: u32 = 0x2b;
        const GLOBAL: u32 = 0x011a8908;
        const LIM1: f32 = f32::from_bits(0x41200000); // 10.0
        const LIM2: f32 = f32::from_bits(0x40400000); // 3.0

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ 0x8000_0000)
        }

        let inner_b = rd32(base + INNER);
        let dir0 = rdf(inner_b + 0x10);
        let dir1 = rdf(inner_b + 0x14);
        let dir2 = rdf(inner_b + 0x18);
        let dep0 = rdf(inner_b + 0x30);
        let dep1 = rdf(inner_b + 0x34);
        let dep2 = rdf(inner_b + 0x38);
        let b10 = f32::from_bits(f10);
        let b14 = f32::from_bits(f14);
        let b18 = f32::from_bits(f18);
        let b1c = f32::from_bits(f1c);

        let mut node = rd32(head);
        if node == 0 {
            return 0;
        }
        loop {
            if !(rdf(gate) > 0.0) {
                return 0;
            }
            let ent = rd32(node);
            node = rd32(node + 4);
            if ent != base {
                let stamp = lf_checker_rt::global::<u16>(GLOBAL).read_unaligned() as u32;
                if rd32(ent + STAMP) != stamp && rd8(ent + READY) & 1 != 0 {
                    wr32(ent + STAMP, stamp);
                    let ie = rd32(ent + INNER);
                    let x = rdf(ie + 0x30);
                    let y = rdf(ie + 0x34);
                    let z = rdf(ie + 0x38);
                    if x > b10 && b18 > x && y > b14 && b1c > y {
                        let mut dx = sub(z, dep2);
                        if dx < 0.0 {
                            dx = neg(dx);
                        }
                        if LIM1 > dx {
                            let (d0, d1, d2) = if rd8(flagobj + FLAGTEST) & 1 != 0 {
                                (neg(dir0), neg(dir1), neg(dir2))
                            } else {
                                (dir0, dir1, dir2)
                            };
                            let rbits: u32 = lf_checker_rt::callee_cdecl!(
                                1, u32,
                                dep0.to_bits(), dep1.to_bits(),
                                d0.to_bits(), d1.to_bits(),
                                x.to_bits(), y.to_bits()
                            );
                            let t = add(mul(f32::from_bits(rbits), d2), dep2);
                            let mut u = sub(z, t);
                            if u < 0.0 {
                                u = neg(u);
                            }
                            if LIM2 > u {
                                let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, ent, base, gate, f24, flagobj, tag);
                            }
                        }
                    }
                }
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
