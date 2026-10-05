// original: 0x00DC4350 path_distance_to_marked_node (proposed)

/// Distance along a node path to the marked node, or whether the marked
/// node is already reached.
///
/// `this` points to an object whose first dword is the marked node id.
/// `path` points to a route object: dword at `+0x20` (an opaque value the
/// helper callee receives, plus `0x30`), the candidate id at `+0xDDC`, and
/// a node list starting at `+0xDD0` (first hop at `+0xDE0`/`+0xDE4`, further
/// nodes from `+0xDE8` while scanning). `out_id` receives a node id on
/// success; `out_dist` always receives a float distance.
///
/// Each node id packs a table index in its low word and a record selector
/// in its high word: the base array at `NODE_TABLE` maps the index to a
/// record block, and the record sits `(high << 5)` bytes into it, holding a
/// scaled (× 0.125) x/y point as two int16 at `+0x14`/`+0x16`. Index
/// `0xFFFF` and a null table entry both mean "no such node" and fail.
///
/// Behaviour: if the marked id already equals the first hop and the
/// candidate id is valid, store the candidate id and return 1. Otherwise
/// both hops must resolve; the helper (callee 0, thiscall, scratch pointer
/// in ecx, two stack words) reports a reference point, and the distance
/// starts as the projection of (second hop minus reference) onto the
/// normalised hop direction, clamped at zero from below (a NaN stays NaN).
/// If the marked id equals the second hop, store the first hop and return
/// 1. Otherwise up to nine further nodes are scanned, adding each segment
/// length to the distance; the first node equal to the marked id stores the
/// node `edi` slots into the list (current index) and returns 1. Any
/// invalid node, or no match after the ninth, returns 0 with the distance
/// so far. The distance slot is zeroed on entry; the id slot is written
/// only on success.
///
/// Original: 0x00DC4350 (thiscall, three stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00DC4350(this: u32, path: u32, out_id: u32, out_dist: u32) -> u8 {
    unsafe {
        const NODE_TABLE: u32 = 0x01178284;
        const SCALE: f32 = f32::from_bits(0x3E00_0000); // 0.125
        const ONE: f32 = 1.0;
        const INVALID: u32 = 0xFFFF;
        const PATH_OPAQUE: u32 = 0x20;
        const PATH_CALLEE_BIAS: u32 = 0x30;
        const PATH_CANDIDATE: u32 = 0xDDC;
        const PATH_HOP_A: u32 = 0xDE0;
        const PATH_HOP_B: u32 = 0xDE4;
        const PATH_SCAN: u32 = 0xDE8;
        const PATH_LIST: u32 = 0xDD0;
        const REC_X: u32 = 0x14;
        const REC_Y: u32 = 0x16;
        const SCAN_FIRST: u32 = 5;
        const SCAN_END: u32 = 0x0E;
        const CALLEE: u32 = 0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        /// Scaled coordinate: int16 loaded, widened, converted, × 0.125.
        #[inline(always)]
        unsafe fn coord(a: u32) -> f32 {
            unsafe {
                let v = (a as *const i16).read_unaligned() as i32 as f32;
                mul(v, SCALE)
            }
        }
        /// Record address for a node id within its table block.
        #[inline(always)]
        unsafe fn record(table: *const u32, node: u32) -> u32 {
            unsafe {
                let block = table.add((node & 0xFFFF) as usize).read_unaligned();
                block.wrapping_add((node >> 16) << 5)
            }
        }

        let table = lf_checker_rt::relocated(NODE_TABLE) as *const u32;
        wr32(out_dist, 0);
        let marked = rd32(this);
        let hop_a = rd32(path.wrapping_add(PATH_HOP_A));
        if marked == hop_a {
            let cand = rd32(path.wrapping_add(PATH_CANDIDATE));
            if cand & 0xFFFF != INVALID {
                wr32(out_id, cand);
                return 1;
            }
        }
        if hop_a & 0xFFFF == INVALID {
            return 0;
        }
        let hop_b = rd32(path.wrapping_add(PATH_HOP_B));
        if hop_b & 0xFFFF == INVALID {
            return 0;
        }
        if table.add((hop_a & 0xFFFF) as usize).read_unaligned() == 0 {
            return 0;
        }
        if table.add((hop_b & 0xFFFF) as usize).read_unaligned() == 0 {
            return 0;
        }
        let rec_a = record(table, hop_a);
        let rec_b = record(table, hop_b);
        let xas = coord(rec_a.wrapping_add(REC_X));
        let yas = coord(rec_a.wrapping_add(REC_Y));
        let xbs = coord(rec_b.wrapping_add(REC_X));
        let ybs = coord(rec_b.wrapping_add(REC_Y));
        let dx = sub(xbs, xas);
        let dy = sub(ybs, yas);
        let dist2 = add(mul(dy, dy), mul(dx, dx));
        // Zero length keeps a zero normaliser (skips 1/sqrt); NaN goes through.
        let mut norm = 0.0f32;
        if dist2 != 0.0 {
            norm = div(ONE, dist2.sqrt());
        }
        let ndx = mul(dx, norm);
        let ndy = mul(dy, norm);
        let mut probe = [0u32; 2];
        let arg0 = rd32(path.wrapping_add(PATH_OPAQUE)).wrapping_add(PATH_CALLEE_BIAS);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, probe.as_mut_ptr() as u32, arg0, 0);
        let ref_x = f32::from_bits(probe[0]);
        let ref_y = f32::from_bits(probe[1]);
        let along_x = mul(sub(xbs, ref_x), ndx);
        let along_y = mul(sub(ybs, ref_y), ndy);
        let proj = add(along_y, along_x);
        wrf(out_dist, proj);
        // max(proj, 0.0) with NaN passing through, as comiss/ja does.
        wrf(out_dist, if 0.0 > proj { 0.0 } else { proj });
        if rd32(this) == hop_b {
            wr32(out_id, hop_a);
            return 1;
        }
        let mut idx = SCAN_FIRST;
        let mut slot = path.wrapping_add(PATH_SCAN);
        loop {
            let cur = rd32(slot);
            if cur & 0xFFFF == INVALID {
                return 0;
            }
            if table.add((cur & 0xFFFF) as usize).read_unaligned() == 0 {
                return 0;
            }
            // Previous node: already validated on its own iteration, read as-is.
            let prev = rd32(slot.wrapping_sub(4));
            let rec_p = record(table, prev);
            let rec_c = record(table, cur);
            let seg_x = sub(coord(rec_p.wrapping_add(REC_X)), coord(rec_c.wrapping_add(REC_X)));
            let seg_y = sub(coord(rec_p.wrapping_add(REC_Y)), coord(rec_c.wrapping_add(REC_Y)));
            let seg = add(mul(seg_y, seg_y), mul(seg_x, seg_x)).sqrt();
            wrf(out_dist, add(seg, rdf(out_dist)));
            if rd32(this) == cur {
                wr32(out_id, rd32(path.wrapping_add(idx * 4).wrapping_add(PATH_LIST)));
                return 1;
            }
            idx += 1;
            slot = slot.wrapping_add(4);
            if idx >= SCAN_END {
                return 0;
            }
        }
    }
});

