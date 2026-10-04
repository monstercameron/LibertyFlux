// original: 0x00C8E960 ped_task_rebuild_cover (proposed)

/// Rebuild the ped-task cover table from the live entity set.
///
/// Takes no arguments and returns nothing. It resets the slot counter at
/// `SLOT_COUNT`, fetches the entity-set head through callee 0 (cdecl) and,
/// when non-null, seeds slot zero through callee 1 (thiscall) with the
/// head's word at `+0x20` plus `0x30`. It then walks the entity table at
/// `TABLE_PTR` backwards: `TABLE_COUNT` entries, each tried unless its flag
/// byte (base `TABLE_FLAGS`) has bit `0x80` set.
///
/// Each tried row sits at `TABLE_BASE + index * TABLE_STRIDE` and is
/// skipped when null, when its byte at `ROW_ACTIVE` is set, or when neither
/// bit 28 of the dword at `ROW_STATE` nor bit 0 of the byte at `ROW_MISC`
/// is set. Otherwise the row's position block (`ROW_POS`, three floats at
/// `+0x30`) is staged, and the row's middle object (`ROW_MID`) is scanned:
/// a chain from `MID_CHAIN` is walked through `+0x0c` while each link's
/// dword at `+0x04` differs from `WANT_TAG`. (Two identical tag
/// derivations are compared on the way; the comparison always finds them
/// equal, so the alternate branch is dead and omitted here.) On a match,
/// callee 2 (thiscall) probes the middle object and, on a non-zero answer,
/// callee 3 (thiscall) observes the staged position.
///
/// Three running values start from the globals `DEF0/DEF1/DEF2`. When bit 0
/// of the middle byte at `MID_FLAGS` is set, callee 4 (thiscall) measures
/// the staged position into a scratch word, whose square root clamps the
/// first two running values from below, and callee 5 (thiscall) fills two
/// more scratch triples. The third running value is then the maximum of
/// `DEF2` and the absolute value of either the scratch difference (bit 2 of
/// `MID_FLAGS` clear) or twice the root (bit 2 set).
///
/// Finally the row is offered to the existing slots: callee 6 (thiscall)
/// sees the staged position and the running triple for each slot in turn
/// and stops the scan on a non-zero low byte, and unless the table already
/// holds 16 rows callee 7 (thiscall) appends the row and the counter grows.
/// Returns with the counter holding the number of rows stored plus one.
///
/// All float order matches the original; the two maxima and the absolute
/// value take the unordered-safe branch on NaN, exactly like the original's
/// conditional jumps.
///
/// Original: 0x00C8E960 (cdecl, no stack words, no defined result).
lf_checker_rt::export!(cdecl, rw_00C8E960() -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 0x16FC674;
        const DEF0: u32 = 0x16FC680;
        const DEF1: u32 = 0x16FC684;
        const DEF2: u32 = 0x16FC688;
        const TABLE_PTR: u32 = 0x18B6F1C;
        const SLOT_BASE: u32 = 0x171B7B0;
        const SLOT_ARG: u32 = 0x171B9B0;
        const SCAN_ARG: u32 = 0x1050AD0;
        const SIGN_MASK: u32 = 0xFE8FA0;
        const TWO: u32 = 0xFE8A24;
        const WANT_TAG: u32 = 0x779;
        const SLOT_STRIDE: u32 = 0x20;
        const MAX_SLOTS: u32 = 0x10;
        const ROW_POS: u32 = 0x20;
        const ROW_ACTIVE: u32 = 0x211;
        const ROW_MID: u32 = 0x224;
        const ROW_MISC: u32 = 0x273;
        const ROW_STATE: u32 = 0x29C;
        const MID_FLAGS: u32 = 0x38;
        const MID_CHAIN: u32 = 0x2E0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn above(p: f32, q: f32) -> bool {
            p > q
        }

        let cntp = lf_checker_rt::relocated(SLOT_COUNT);
        wr32(cntp, 0);
        let head = lf_checker_rt::callee_cdecl!(0, u32,);
        if head == 0 {
            return 0;
        }
        let seed = rd32(head.wrapping_add(ROW_POS)).wrapping_add(0x30);
        lf_checker_rt::callee_thiscall!(
            1, u32, lf_checker_rt::relocated(SLOT_BASE), seed, lf_checker_rt::relocated(SLOT_ARG)
        );
        wr32(cntp, 1);
        let tab = rd32(lf_checker_rt::relocated(TABLE_PTR));
        let total = rd32(tab.wrapping_add(8));
        if total == 0 {
            return 0;
        }
        let sign = rd32(lf_checker_rt::relocated(SIGN_MASK));
        let two = rdf(lf_checker_rt::relocated(TWO));
        // Scratch slots, reused across rows like the original's frame slots.
        let mut pos = [0u32; 3];
        let mut out_a = [0u32; 3];
        let mut out_b = [0u32; 3];
        let mut measure = 0u32;
        let flags_base = rd32(tab.wrapping_add(4));
        let stride = rd32(tab.wrapping_add(0xc));
        let rows_base = rd32(tab);
        let mut b = total;
        while b > 0 {
            b = b.wrapping_sub(1);
            if rd8(flags_base.wrapping_add(b)) & 0x80 != 0 {
                continue;
            }
            let row = stride.wrapping_mul(b).wrapping_add(rows_base);
            if row == 0 {
                continue;
            }
            if rd8(row.wrapping_add(ROW_ACTIVE)) != 0 {
                continue;
            }
            if rd32(row.wrapping_add(ROW_STATE)) & 0x1000_0000 == 0
                && rd8(row.wrapping_add(ROW_MISC)) & 1 == 0
            {
                continue;
            }
            let pb = rd32(row.wrapping_add(ROW_POS));
            let mid = rd32(row.wrapping_add(ROW_MID));
            pos[0] = rd32(pb.wrapping_add(0x30));
            pos[1] = rd32(pb.wrapping_add(0x34));
            pos[2] = rd32(pb.wrapping_add(0x38));
            let pos_ptr = (&mut pos as *mut u32) as u32;
            let mut link = rd32(mid.wrapping_add(MID_CHAIN));
            while link != 0 {
                if rd32(link.wrapping_add(4)) == WANT_TAG {
                    let probe = lf_checker_rt::callee_thiscall!(
                        2, u32, mid.wrapping_add(0x44), WANT_TAG
                    );
                    if probe != 0 {
                        lf_checker_rt::callee_thiscall!(3, u32, probe, pos_ptr);
                    }
                    break;
                }
                link = rd32(link.wrapping_add(0xc));
            }
            let def0 = rdf(lf_checker_rt::relocated(DEF0));
            let def1 = rdf(lf_checker_rt::relocated(DEF1));
            let def2 = rdf(lf_checker_rt::relocated(DEF2));
            let mut v0 = def0;
            let mut v1 = def1;
            let mut v2 = def2;
            let mflags = rd8(mid.wrapping_add(MID_FLAGS));
            if mflags & 1 != 0 {
                lf_checker_rt::callee_thiscall!(
                    4, u32, mid.wrapping_add(0x10), pos_ptr, (&mut measure as *mut u32) as u32
                );
                let root = f32::from_bits(measure).sqrt();
                measure = root.to_bits();
                if above(def0, root) {
                    v0 = def0;
                } else {
                    v0 = root;
                }
                if above(def1, root) {
                    v1 = def1;
                } else {
                    v1 = root;
                }
                lf_checker_rt::callee_thiscall!(
                    5, u32, mid.wrapping_add(0x10), (&mut out_a as *mut u32) as u32,
                    (&mut out_b as *mut u32) as u32, (&mut measure as *mut u32) as u32
                );
                let x = if mflags & 4 == 0 {
                    let a = f32::from_bits(out_a[2]);
                    let c = f32::from_bits(out_b[2]);
                    core::hint::black_box(a) - core::hint::black_box(c)
                } else {
                    core::hint::black_box(root) * core::hint::black_box(two)
                };
                let fx = if above(0.0, x) {
                    f32::from_bits(x.to_bits() ^ sign)
                } else {
                    x
                };
                if above(def2, fx) {
                    v2 = def2;
                } else {
                    v2 = fx;
                }
            }
            let mut vals = [v0.to_bits(), v1.to_bits(), v2.to_bits()];
            let vals_ptr = (&mut vals as *mut u32) as u32;
            let n = rd32(cntp);
            let mut j = 0u32;
            let mut stopped = false;
            while j < n {
                let ans = lf_checker_rt::callee_thiscall!(
                    6, u32, lf_checker_rt::relocated(SLOT_BASE).wrapping_add(j.wrapping_mul(SLOT_STRIDE)),
                    pos_ptr, vals_ptr, lf_checker_rt::relocated(SCAN_ARG)
                );
                if (ans as u8) != 0 {
                    stopped = true;
                    break;
                }
                j = j.wrapping_add(1);
            }
            if !stopped {
                let m = rd32(cntp);
                if m != MAX_SLOTS {
                    lf_checker_rt::callee_thiscall!(
                        7, u32,
                        lf_checker_rt::relocated(SLOT_BASE).wrapping_add(m.wrapping_mul(SLOT_STRIDE)),
                        pos_ptr, vals_ptr
                    );
                    wr32(cntp, m.wrapping_add(1));
                }
            }
        }
        0
    }
});
