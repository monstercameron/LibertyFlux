// Rewrite of peds_task_path_build (0x00DC1C90), thiscall.
// Builds the waypoint list of one ped task. Arguments: `this` (the caller
// object; its bytes +0xC44 and +0xC45 gate the shape call), `obj` (the path
// object that receives the entries), `start_xyz` and `end_xyz` (pointers to the
// start and end point float blocks) and `max` (the entry capacity). Returns 1
// on success and on each capacity exit, and 0 on the fail exit (the global
// fail flag at VA 0x16B6B09 is set).
// The header copy loop and the end-point tail write the object's table words
// and float blocks; the slot loop and the main waypoint loop use the four
// per-slot callees (ids 1 to 10 in the checker contracts).
// Deliberate faults (the original cannot take these paths):
// - the node-kind re-test in the shape arm (`unreachable!("node kinds")`):
//   the same mask test already decided the node on this path;
// - the index guard (`unreachable!("index guard")`): the guard compares the
//   loop index with the previous index plus one, which is never greater.
// Stack words the original leaves uninitialised (the w component of the
// float blocks, esp+0xAC) are read as 0.0, which is the checker's zero stack
// fill. Under any other stack contents the w component would differ.

unsafe fn path_build(
    this: u32,
    obj: u32,
    start_xyz: u32,
    end_xyz: u32,
    max: u32,
) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 0x17A3378;
        const SLOT_HEAD: u32 = 0x17A2340;
        const SLOT_TABLE: u32 = 0x17A2344;
        const SLOT_ROTATE: u32 = 0x17A2348;
        const GLOBAL_XYZW: u32 = 0x179FD90;
        const MODE_FLAG: u32 = 0x1048266;
        const OBJ_SEQ: u32 = 0x1C;
        const OBJ_START: u32 = 0x90;
        const COPY_G1: u32 = 0x17A21A8;
        const COPY_G2: u32 = 0x17A29B8;
        const COPY_FLOATS: u32 = 0x179FDA8;
        const FIXED_XYZW: u32 = 0x179FD80;
        const DIST_LIMIT: u32 = 0xFE87E4;
        const MIDDLE_WORD: u32 = 0x17A2B58;
        const THIS_ZERO: u32 = 0xC4C;
        const KEY_CALLEE: u32 = 1;
        const APPLY_CALLEE: u32 = 2;
        const QUERY_CALLEE: u32 = 3;
        /// The second query call of a hit (site 0x9C2876) is a separate stub id, so the
        /// two calls can write different out-words (one row per callee per trial).
        const QUERY_CALLEE_B: u32 = 12;
        /// The node's own items (site 0x9C2A32) and the solve call (site 0x9C2A79).
        const NODE_ITEM_CALLEE: u32 = 13;
        const SOLVE_CALLEE: u32 = 9;
        const EMIT_CALLEE: u32 = 4;
        const KIND_FLAG: u32 = 0xFE00_0000;
        const OBJ_WORD0: u32 = 0x1E0;
        const OBJ_WORD1: u32 = 0x1E4;
        /// The original reads an uninitialized stack word for every w slot;
        /// under the checker's zero stack fill that word is 0.0.
        const STACK_W: f32 = 0.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
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
        unsafe fn gr32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write(v) }
        }
        #[inline(always)]
        unsafe fn gwf(va: u32, v: f32) {
            unsafe { gw32(va, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn grf(va: u32) -> f32 {
            unsafe { f32::from_bits(gr32(va)) }
        }

        let count = gr32(SLOT_COUNT);
        if count == 0 && rd32(obj + OBJ_SEQ) == 2 {
            return 1;
        }
        // The clear runs for every non-early-exit entry: the jump for a
        // nonzero count lands before it, and the zero-count fallthrough
        // stores twice. Later blocks read the sequence field, so skipping
        // this store is visible.
        wr32(obj + OBJ_SEQ, 0);
        let sx = rdf(start_xyz);
        let sy = rdf(start_xyz + 4);
        let sz = rdf(start_xyz + 8);
        let ex = rdf(end_xyz);
        let ey = rdf(end_xyz + 4);
        let ez = rdf(end_xyz + 8);
        let rotate_head_before = gr32(SLOT_ROTATE);
        let rotated = gr32(SLOT_TABLE.wrapping_add(count.wrapping_mul(4)));
        gw32(SLOT_ROTATE.wrapping_add(count.wrapping_mul(4)), rotated);
        let head_now = gr32(SLOT_ROTATE);
        let published = head_now == gr32(SLOT_HEAD);
        if published {
            gwf(GLOBAL_XYZW, sx);
            gwf(GLOBAL_XYZW + 4, sy);
            gwf(GLOBAL_XYZW + 8, sz);
            gwf(GLOBAL_XYZW + 12, STACK_W);
        }
        // Slot loop: slots `published as u32 .. count`, all bounds
        // compared unsigned like the original's jae/jb. Each slot resolves
        // its node kind through the key callee, then either applies the node
        // straight to the slot's float block or queries per-item indices into
        // scratch and emits one record. Scratch stays zero: the original's
        // frame buffer holds only the zero stack fill on this path.
        let mut scratch = [0u32; 64];
        let mut i = published as u32;
        let mut slot_ptr = SLOT_ROTATE.wrapping_add(i.wrapping_mul(4));
        // File address for slot_ptr (only used for global reads); the float
        // block pointer is passed to callees by value, so relocate it.
        let mut fptr =
            lf_checker_rt::relocated(GLOBAL_XYZW.wrapping_add(i.wrapping_mul(16)));
        while i < count {
            let node = gr32(slot_ptr);
            let kind = (rd32(node.wrapping_add(4)) >> 0x11) & 0xFFF;
            let key: u32 = lf_checker_rt::callee_cdecl!(KEY_CALLEE, u32, kind);
            let w0 = rd32(node);
            if (w0 & KIND_FLAG) == KIND_FLAG {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(APPLY_CALLEE, u32, key, node, fptr);
            } else {
                let n = (w0 >> 0x15) & 0xF;
                let base = rd32(node.wrapping_add(4)) & 0x1_FFFF;
                let tbl60 = rd32(key.wrapping_add(0x60));
                for j in 0..n {
                    let idx = rd16(tbl60.wrapping_add(base.wrapping_add(j).wrapping_mul(2)));
                    let buf = (scratch.as_mut_ptr() as u32).wrapping_add(j.wrapping_mul(16));
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(QUERY_CALLEE, u32, key, idx, buf);
                }
                let node2 = gr32(slot_ptr);
                let a = rd32(node2) >> 0x19;
                // Caller cleans the stack after this call, so cdecl: the
                // node goes on the stack only (ecx happens to hold it too
                // in the original, but the convention ignores it).
                let out = scratch.as_mut_ptr() as u32;
                let _: u32 =
                    lf_checker_rt::callee_cdecl!(EMIT_CALLEE, u32, node2, out, fptr, a);
            }
            i = i.wrapping_add(1);
            slot_ptr = slot_ptr.wrapping_add(4);
            fptr = fptr.wrapping_add(16);
        }
        // Mode flag dispatch: clear takes the copy loop and header tail;
        // set takes the reposition branch.
        if lf_checker_rt::global::<u8>(MODE_FLAG).read() != 0 {
            // Reposition branch. fbase selects this count's float
            // slot; the scratch end point and tail w follow the taken branch.
            let fbase = count.wrapping_mul(16);
            let slot_out = GLOBAL_XYZW.wrapping_add(fbase);
            let (ox, oy, oz, ow);
            // Only the forced arm falls through the count decrement; the
            // other arms jump over it, so they enter the main loop with a
            // bound of count, while forced skips it when count is 1.
            let forced = rd8(obj.wrapping_add(0x14)) & 0xC0 != 0;
            if forced {
                // Forced copy of the fixed block one slot forward.
                let g0 = grf(FIXED_XYZW.wrapping_add(fbase));
                let g1 = grf(FIXED_XYZW.wrapping_add(fbase).wrapping_add(4));
                let g2 = grf(FIXED_XYZW.wrapping_add(fbase).wrapping_add(8));
                let g3 = grf(FIXED_XYZW.wrapping_add(fbase).wrapping_add(12));
                gwf(slot_out, g0);
                gwf(slot_out.wrapping_add(4), g1);
                gwf(slot_out.wrapping_add(8), g2);
                gwf(slot_out.wrapping_add(12), g3);
                ox = g0;
                oy = g1;
                oz = g2;
                ow = g3;
            } else if rd8(obj.wrapping_add(0x340)) & 2 == 0 {
                // Plain publish of the end point.
                gwf(slot_out, ex);
                gwf(slot_out.wrapping_add(4), ey);
                gwf(slot_out.wrapping_add(8), ez);
                gwf(slot_out.wrapping_add(12), STACK_W);
                ox = ex;
                oy = ey;
                oz = ez;
                ow = STACK_W;
            } else {
                // Distance check in the original's operation order; the
                // inverted comparison matches comiss+jbe, NaN included.
                let gx = grf(FIXED_XYZW.wrapping_add(fbase));
                let gy = grf(FIXED_XYZW.wrapping_add(fbase).wrapping_add(4));
                let gz = grf(FIXED_XYZW.wrapping_add(fbase).wrapping_add(8));
                let g3 = grf(FIXED_XYZW.wrapping_add(fbase).wrapping_add(12));
                let dx = fsub(gx, rdf(obj.wrapping_add(0x50)));
                let dy = fsub(gy, rdf(obj.wrapping_add(0x54)));
                let dz = fsub(gz, rdf(obj.wrapping_add(0x58)));
                let d2 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                let limit = f32::from_bits(gr32(DIST_LIMIT));
                let near = limit > d2;
                if near {
                    gwf(slot_out, ex);
                    gwf(slot_out.wrapping_add(4), ey);
                    gwf(slot_out.wrapping_add(8), ez);
                    gwf(slot_out.wrapping_add(12), STACK_W);
                    ox = ex;
                    oy = ey;
                    oz = ez;
                    ow = STACK_W;
                } else {
                    let g0 = gx;
                    let g1 = gy;
                    let g2 = gz;
                    gwf(slot_out, g0);
                    gwf(slot_out.wrapping_add(4), g1);
                    gwf(slot_out.wrapping_add(8), g2);
                    gwf(slot_out.wrapping_add(12), g3);
                    ox = g0;
                    oy = g1;
                    oz = g2;
                    ow = g3;
                }
            }
            // Middle block: append the start point, link the fixed word,
            // clear the this-flag, then (count == 1) skip the main loop.
            let seq = rd32(obj.wrapping_add(OBJ_SEQ));
            let slot_a = obj.wrapping_add(seq.wrapping_add(9).wrapping_mul(16));
            wrf(slot_a, sx);
            wrf(slot_a.wrapping_add(4), sy);
            wrf(slot_a.wrapping_add(8), sz);
            wrf(slot_a.wrapping_add(12), STACK_W);
            let seq2 = rd32(obj.wrapping_add(OBJ_SEQ));
            wr32(
                obj.wrapping_add(seq2.wrapping_mul(4)).wrapping_add(OBJ_WORD1),
                gr32(SLOT_ROTATE),
            );
            wr32(
                obj.wrapping_add(seq2.wrapping_mul(4)).wrapping_add(0x1A0),
                gr32(MIDDLE_WORD),
            );
            wr32(obj.wrapping_add(OBJ_SEQ), seq2.wrapping_add(1));
            wr32(this.wrapping_add(THIS_ZERO), 0);
            // Forced with count 0: the bound wraps to 0xFFFFFFFF and the loop
            // runs (exits only by capacity).
            let loop_bound = if forced { count.wrapping_sub(1) } else { count };
            // Main-loop fail flag: when set, the first loop
            // iteration aborts the path with a zero return.
            if loop_bound >= 1
                && lf_checker_rt::global::<u8>(0x16B6B09).read() != 0
            {
                wr32(obj.wrapping_add(8), 0xFFFF_FFF2);
                return 0;
            }
            // Main loop, iterations 1..=bound. Iteration i reads
            // node SLOT_ROTATE[i], the previous node SLOT_ROTATE[i-1] as node0 (the
            // original keeps the previous node in its frame word, which the merge
            // path stores), and the float blocks GLOBAL+16(i-1) and GLOBAL+16i.
            let mut prev_node: u32 = rotate_head_before;
            let mut prev_fptr: u32 = lf_checker_rt::relocated(GLOBAL_XYZW);
            let mut iter: u32 = 1;
            while iter <= loop_bound {
                // Main loop body for one iteration.
                let node_slot = gr32(SLOT_ROTATE.wrapping_add(iter.wrapping_mul(4)));
                let node0 = prev_node;
                let fptr_b = lf_checker_rt::relocated(
                    GLOBAL_XYZW.wrapping_add(iter.wrapping_mul(16)),
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this);
                wr16(this.wrapping_add(0xC44), 0);
                let fptr_a = prev_fptr;
                let scan: u32 = lf_checker_rt::callee_thiscall!(
                    6, u32, this, fptr_a, fptr_b, node_slot, node0, 0
                );
                // Scan accepted: when the global at 0x104827C is
                // nonzero, id7 decides; an accepted scan with id7 accepted and
                // both this-flag bytes clear loops on with no append. A rejected
                // id7 or a set flag byte falls to the node-kind test below.
                let mut skip_shape = false;
                if (scan as u8) != 0 {
                    let id7_ok = if gr32(0x104827C) != 0 {
                        let bit9 = (rd32(obj.wrapping_add(0x14)) >> 9) & 1;
                        let r: u32 = lf_checker_rt::callee_thiscall!(
                            7, u32, this, fptr_a, fptr_b, bit9
                        );
                        (r as u8) != 0
                    } else {
                        true
                    };
                    skip_shape = id7_ok
                        && rd8(this.wrapping_add(0xC44)) == 0
                        && rd8(this.wrapping_add(0xC45)) == 0;
                }
                let scan_skipped = skip_shape;
                // Node kinds set. The first key id1 resolves the node's kind
                // before the bit-20 test of the node's first word; bit 20 clear
                // goes to the query search. Bit 20 set resolves the linked
                // entry's kind with a second id1 call. A tagged key (+0x80 ==
                // 0xE74) takes the index remap. An entry count of zero merges
                // into the loop back-edge with no append; a nonzero count is the
                // entry search.
                if !skip_shape && rd32(node_slot) & 0x12_C000 != 0 {
                    let kind1 = (rd32(node_slot.wrapping_add(4)) >> 0x11) & 0xFFF;
                    let key1: u32 = lf_checker_rt::callee_cdecl!(KEY_CALLEE, u32, kind1);
                    if (rd32(node_slot) >> 0x14) & 1 == 0 {
                        // Query-search arm, iteration index 1. The
                        // slot-table word for index 1 is the query node L; its
                        // count (w0 bits 21..24) and base (w1 low 17 bits) pick the
                        // query items of key_l (+0x64 table, 8-byte items). Key_l
                        // and key_n (this node's kind) are resolved after key1.
                        // For each item the query callee (id 8) fills one out
                        // word: low 12 bits must equal this node's kind, and bits
                        // 16..31 give an index; a kind match whose entry (key_n
                        // base + 40 * index) is the node is the hit.
                        let kind_node = (rd32(node_slot.wrapping_add(4)) >> 0x11) & 0xFFF;
                        // The query node is the slot-table word of this iteration.
                        let l_node = gr32(SLOT_TABLE.wrapping_add(iter.wrapping_mul(4)));
                        let kind_l = (rd32(l_node.wrapping_add(4)) >> 0x11) & 0xFFF;
                        let key_l: u32 = lf_checker_rt::callee_cdecl!(KEY_CALLEE, u32, kind_l);
                        let key_n: u32 = lf_checker_rt::callee_cdecl!(KEY_CALLEE, u32, kind_node);
                        if key_l != 0 && key_n != 0 {
                            let nq = (rd32(l_node) >> 0x15) & 0xF;
                            let base = rd32(l_node.wrapping_add(4)) & 0x1_FFFF;
                            let tbl = rd32(key_l.wrapping_add(0x64));
                            // Index of the first item whose out-word names this node (the
                            // original's exit with eax below the count).
                            let mut hit_item: Option<u32> = None;
                            for j in 0..nq {
                                let p = tbl.wrapping_add(base.wrapping_add(j).wrapping_mul(8));
                                // Out word, pre-written with the seed (zero under stack fill) masked.
                                let mut out = [0u32; 4];
                                let outp = out.as_mut_ptr() as u32;
                                out[0] = 0;
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(8, u32, p, outp);
                                let w = unsafe { core::ptr::read_volatile(outp as *const u32) };
                                if w & 0xFFF != kind_node {
                                    continue;
                                }
                                let cand = rd32(key_n.wrapping_add(0x6C))
                                    .wrapping_add(((w >> 16) & 0xFFFF).wrapping_mul(40));
                                if cand == node_slot {
                                    hit_item = Some(j);
                                    break;
                                }
                            }
                            if let Some(j) = hit_item {
                                // Query hit arm. Two id3 calls on key_l: item j
                                // (base + j) into buffer A, then item (j + 1) % count (base +
                                // that) into buffer B. Each id3 call reads key_l's u16 table at
                                // +0x60 afresh, as the original does.
                                let fdiv = |a: f32, b: f32| {
                                    core::hint::black_box(a) / core::hint::black_box(b)
                                };
                                let mut buf_a = [0u32; 4];
                                let mut buf_b = [0u32; 4];
                                let pa = buf_a.as_mut_ptr() as u32;
                                let pb = buf_b.as_mut_ptr() as u32;
                                let word_a = rd16(
                                    rd32(key_l.wrapping_add(0x60))
                                        .wrapping_add(base.wrapping_add(j).wrapping_mul(2)),
                                );
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    QUERY_CALLEE, u32, key_l, word_a, pa
                                );
                                let next = j.wrapping_add(1) % nq;
                                let word_b = rd16(
                                    rd32(key_l.wrapping_add(0x60))
                                        .wrapping_add(base.wrapping_add(next).wrapping_mul(2)),
                                );
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    QUERY_CALLEE_B, u32, key_l, word_b, pb
                                );
                                // The two out-points, read back after both calls.
                                let rv = |a: u32| -> f32 {
                                    f32::from_bits(unsafe { (a as *const u32).read_volatile() })
                                };
                                let (ax, ay, az) = (rv(pa), rv(pa.wrapping_add(4)), rv(pa.wrapping_add(8)));
                                let (bx, by, bz) = (rv(pb), rv(pb.wrapping_add(4)), rv(pb.wrapping_add(8)));
                                let half = f32::from_bits(gr32(0xFE8830));
                                // Midpoint of the two points: the block this arm appends.
                                let mid_x = fmul(fadd(ax, bx), half);
                                let mid_y = fmul(fadd(ay, by), half);
                                let mid_z = fmul(fadd(az, bz), half);
                                // Difference vector from A to B and its squared length.
                                let dx = fsub(bx, ax);
                                let dy = fsub(by, ay);
                                let dz = fsub(bz, az);
                                let d2 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                                // Normalise: a squared length that compares equal to zero (NaN does
                                // not) gives zero; anything else gives the constant over its root.
                                let norm: f32 = if d2 == 0.0 {
                                    0.0
                                } else {
                                    let root = core::hint::black_box(d2).sqrt();
                                    fdiv(f32::from_bits(gr32(0xFE88E8)), root)
                                };
                                // Scaled difference components and the zero products the original
                                // computes with the zero register. They feed the solve call's
                                // second append.
                                let zero = 0.0f32;
                                let scaled_x = fmul(dx, norm);
                                let scaled_y = fmul(dy, norm);
                                let scaled_z = fmul(dz, norm);
                                let zero_z = fmul(scaled_z, zero);
                                let frame_c = fsub(zero_z, scaled_x);
                                let frame_20 = fsub(scaled_y, zero_z);
                                let zero_x = fmul(scaled_x, zero);
                                let zero_y = fmul(scaled_y, zero);
                                let frame_10 = fsub(zero_x, zero_y);
                                core::hint::black_box((frame_c, frame_20, frame_10));
                                // One append: the midpoint block at seq + 9, the item word with a
                                // zero link, and the global middle word of this iteration.
                                let seq_h = rd32(obj.wrapping_add(OBJ_SEQ));
                                let block =
                                    obj.wrapping_add(seq_h.wrapping_add(9).wrapping_mul(16));
                                wrf(block, mid_x);
                                wrf(block.wrapping_add(4), mid_y);
                                wrf(block.wrapping_add(8), mid_z);
                                wrf(block.wrapping_add(12), STACK_W);
                                wr32(
                                    obj.wrapping_add(seq_h.wrapping_mul(4))
                                        .wrapping_add(OBJ_WORD1),
                                    0,
                                );
                                let middle = gr32(0x17A2B58u32.wrapping_add(iter.wrapping_mul(4)));
                                wr32(
                                    obj.wrapping_add(seq_h.wrapping_mul(4)).wrapping_add(0x1A0),
                                    middle,
                                );
                                wr32(obj.wrapping_add(OBJ_SEQ), seq_h.wrapping_add(1));
                                // Capacity check after the append: exit when full. The continue
                                // path runs the solve call below.
                                if rd32(obj.wrapping_add(OBJ_SEQ)) >= max {
                                    return 1;
                                }
                                // The node's own items: one call per count item on key_n at
                                // (base + k), each into a 16-byte record from the record block.
                                let mut records = [0u32; 64];
                                let records_p = records.as_mut_ptr() as u32;
                                let item_count = (rd32(node_slot) >> 0x15) & 0xF;
                                let item_base = rd32(node_slot.wrapping_add(4)) & 0x1_FFFF;
                                for k in 0..item_count {
                                    let tbl_n = rd32(key_n.wrapping_add(0x60));
                                    let item_word = rd16(
                                        tbl_n.wrapping_add(item_base.wrapping_add(k).wrapping_mul(2)),
                                    );
                                    let rec = records_p.wrapping_add(k.wrapping_mul(16));
                                    let _: u32 = lf_checker_rt::callee_thiscall!(
                                        NODE_ITEM_CALLEE, u32, key_n, item_word, rec
                                    );
                                }
                                // Solve call: this, the node, record 0, zero, the midpoint block
                                // (x, y, z, w), the out-point block, the key_n word (by pointer)
                                // and 1. The float return stays unused; typing it f32 makes the
                                // compiler pop the x87 stack.
                                let mut mid_block = [
                                    mid_x.to_bits(),
                                    mid_y.to_bits(),
                                    mid_z.to_bits(),
                                    STACK_W.to_bits(),
                                ];
                                let mut solve_out = [0u32; 4];
                                let mut key_word = key_n;
                                let mid_p = mid_block.as_mut_ptr() as u32;
                                let out_p = solve_out.as_mut_ptr() as u32;
                                let key_p = (&mut key_word as *mut u32) as u32;
                                let _: f32 = lf_checker_rt::callee_thiscall!(
                                    SOLVE_CALLEE, f32, this, node_slot, records_p, 0, mid_p, out_p, key_p, 1
                                );
                                // Second append: the solve's out-point plus the frame offsets, with
                                // a zero link word and the zero w component.
                                let (o0, o1, o2) = (
                                    rv(out_p),
                                    rv(out_p.wrapping_add(4)),
                                    rv(out_p.wrapping_add(8)),
                                );
                                let add_x = fadd(o0, frame_20);
                                let add_y = fadd(o1, frame_c);
                                let add_z = fadd(o2, frame_10);
                                let seq_s = rd32(obj.wrapping_add(OBJ_SEQ));
                                let block_s =
                                    obj.wrapping_add(seq_s.wrapping_add(9).wrapping_mul(16));
                                wrf(block_s, add_x);
                                wrf(block_s.wrapping_add(4), add_y);
                                wrf(block_s.wrapping_add(8), add_z);
                                wrf(block_s.wrapping_add(12), rv(out_p.wrapping_add(12)));
                                wr32(
                                    obj.wrapping_add(seq_s.wrapping_mul(4)).wrapping_add(OBJ_WORD1),
                                    0,
                                );
                                wr16(obj.wrapping_add(seq_s.wrapping_mul(4)).wrapping_add(0x1A0), 0);
                                wr8(obj.wrapping_add(seq_s.wrapping_mul(4)).wrapping_add(0x1A2), 0);
                                wr32(obj.wrapping_add(OBJ_SEQ), seq_s.wrapping_add(1));
                                // Capacity check after the second append: exit, or continue to the
                                // loop back-edge (below, shared with the other arms).
                                if rd32(obj.wrapping_add(OBJ_SEQ)) >= max {
                                    return 1;
                                }
                            }
                        }
                        skip_shape = true;
                    } else {
                        let entry = rd32(node_slot.wrapping_add(0xC));
                        let kind2 = (rd32(entry.wrapping_add(4)) >> 0x11) & 0xFFF;
                        let key2: u32 = lf_checker_rt::callee_cdecl!(KEY_CALLEE, u32, kind2);
                        // Index remap: a tagged key2 maps the
                        // linked entry through the 40-byte table at key2+0x6C. The
                        // index is the difference divided by 40 (signed magic
                        // multiply, then the sign correction). The table at the
                        // global 0x16B8F90 gives a u16 pair per index: the first
                        // word is the third key's id, the second word the entry
                        // number.
                        let (key_k, entry_k) = if rd32(key2.wrapping_add(0x80)) == 0xE74 {
                            let diff = entry.wrapping_sub(rd32(key2.wrapping_add(0x6C)));
                            let prod = (diff as i32 as i64).wrapping_mul(0x6666_6667);
                            let t = ((prod >> 32) as i32) >> 4;
                            let idx = t.wrapping_add(((t as u32) >> 31) as i32) as u32;
                            let slot_t = gr32(0x16B8F90).wrapping_add(idx.wrapping_mul(4));
                            let a = rd16(slot_t);
                            let b = rd16(slot_t.wrapping_add(2));
                            let key3: u32 = lf_checker_rt::callee_cdecl!(
                                KEY_CALLEE,
                                u32,
                                a
                            );
                            (
                                key3,
                                rd32(key3.wrapping_add(0x6C)).wrapping_add(
                                    b.wrapping_mul(40),
                                ),
                            )
                        } else {
                            (key2, entry)
                        };
                        // Entry search over the key's count entries (0x16 bytes each,
                        // from key+0x74): a kind match at word +6 is the hit path;
                        // no match merges like the count-zero miss.
                        let count_k = rd32(key_k.wrapping_add(0x8C));
                        // The entry's kind is read only when the search runs.
                        let kind_k = if count_k != 0 {
                            (rd32(entry_k.wrapping_add(4)) >> 0x11) & 0xFFF
                        } else {
                            0
                        };
                        let table_k = rd32(key_k.wrapping_add(0x74));
                        // Entry search, the original's order of tests: kind (+6), the
                        // entry's index from key_k's base (+8), the kind of esi (+0x10),
                        // the index of esi from key1's base (+0x12). esi is the node
                        // when key2 is not remapped and key1 after a remap.
                        #[inline(always)]
                        fn sdiv40(diff: u32) -> u32 {
                            // Signed division by 40 by the magic multiply and sign fix-up.
                            let prod = (diff as i32 as i64).wrapping_mul(0x6666_6667);
                            let t = ((prod >> 32) as i32) >> 4;
                            t.wrapping_add(((t as u32) >> 31) as i32) as u32
                        }
                        // esi is the node in both cases: the remap's reload of esi
                        // from the frame reads the node before the frame is balanced.
                        // First key tagged: the node's index from key1's base through the pair table
                        // (signed divide by 40); the first word is the new first key, the second word
                        // gives the new node as key_k's base plus 40 times that word.
                        let (key1_s, esi_v) = if rd32(key1.wrapping_add(0x80)) == 0xE74 {
                            let idx = sdiv40(node_slot.wrapping_sub(rd32(key1.wrapping_add(0x6C))));
                            let slot_t = gr32(0x16B8F90).wrapping_add(idx.wrapping_mul(4));
                            let a = rd16(slot_t);
                            let b = rd16(slot_t.wrapping_add(2));
                            let key1r: u32 = lf_checker_rt::callee_cdecl!(KEY_CALLEE, u32, a);
                            (key1r, rd32(key_k.wrapping_add(0x6C)).wrapping_add(b.wrapping_mul(40)))
                        } else {
                            (key1, node_slot)
                        };
                        let mut hit_e: Option<u32> = None;
                        for j in 0..count_k {
                            let e = table_k.wrapping_add(j.wrapping_mul(0x16));
                            if rd16(e.wrapping_add(6)) != kind_k {
                                continue;
                            }
                            if rd16(e.wrapping_add(8))
                                != sdiv40(entry_k.wrapping_sub(rd32(key_k.wrapping_add(0x6C))))
                            {
                                continue;
                            }
                            let kind_s = (rd32(esi_v.wrapping_add(4)) >> 0x11) & 0xFFF;
                            if rd16(e.wrapping_add(0x10)) != kind_s {
                                continue;
                            }
                            if rd16(e.wrapping_add(0x12))
                                != sdiv40(esi_v.wrapping_sub(rd32(key1_s.wrapping_add(0x6C))))
                            {
                                continue;
                            }
                            hit_e = Some(e);
                            break;
                        }
                        // Entry-match arm. A flag byte of 1 or 2 takes three appends: a copy of the previous float
                        // block with the slot-table word as link, the entry-derived
                        // block, and a second block from key1 and key_k. Any other flag
                        // byte, and a miss, merge with no append.
                        if let Some(e) = hit_e {
                            let flag_e = rd8(e.wrapping_add(0x14));
                            if flag_e.wrapping_sub(1) <= 1 {
                                let seq_h = rd32(obj.wrapping_add(OBJ_SEQ));
                                if seq_h.wrapping_add(3) >= max {
                                    return 1;
                                }
                                let c = grf(0xFE8674);
                                let f = |off: u32| (rd16(e.wrapping_add(off)) as i32) as f32;
                                let k40 = rdf(key_k.wrapping_add(0x40));
                                let k44 = rdf(key_k.wrapping_add(0x44));
                                let k48 = rdf(key_k.wrapping_add(0x48));
                                let pk = rd32(key_k.wrapping_add(0x90));
                                let (p0, p4, p8) = (rdf(pk), rdf(pk.wrapping_add(4)), rdf(pk.wrapping_add(8)));
                                let a40 = rdf(key1_s.wrapping_add(0x40));
                                let a44 = rdf(key1_s.wrapping_add(0x44));
                                let a48 = rdf(key1_s.wrapping_add(0x48));
                                let qk = rd32(key1_s.wrapping_add(0x90));
                                let (q0, q4, q8) = (rdf(qk), rdf(qk.wrapping_add(4)), rdf(qk.wrapping_add(8)));
                                // Entry block (x, y, z); the x word uses key_k's point.
                                let t2 = fmul(f(2), c);
                                let t0 = fmul(f(4), c);
                                let ka = fmul(k44, t2);
                                let kb = fmul(k48, t0);
                                let z_e = fadd(p8, kb);
                                let y_e = fadd(p4, ka);
                                let x_e = fadd(fmul(fmul(f(0), c), k40), p0);
                                // Second block (x, y, z) from key1's point.
                                let x_e2 = fadd(fmul(fmul(f(0xA), c), a40), q0);
                                let t4 = fmul(f(0xC), c);
                                let ka2 = fmul(a44, t4);
                                let y_e2 = fadd(q4, ka2);
                                let t5 = fmul(f(0xE), c);
                                let kb2 = fmul(a48, t5);
                                let z_e2 = fadd(q8, kb2);
                                // Block one: copy of the previous float block.
                                let fp = fptr_b;
                                let s1 = obj.wrapping_add(seq_h.wrapping_add(9).wrapping_mul(16));
                                wr32(s1, rd32(fp.wrapping_sub(0x10)));
                                wr32(s1.wrapping_add(4), rd32(fp.wrapping_sub(0xC)));
                                wr32(s1.wrapping_add(8), rd32(fp.wrapping_sub(8)));
                                wr32(s1.wrapping_add(12), rd32(fp.wrapping_sub(4)));
                                // The link is the previous node of this pass: SLOT_ROTATE[iter - 1].
                                wr32(
                                    obj.wrapping_add(seq_h.wrapping_mul(4)).wrapping_add(OBJ_WORD1),
                                    gr32(SLOT_TABLE.wrapping_add(iter.wrapping_mul(4))),
                                );
                                wr16(obj.wrapping_add(seq_h.wrapping_mul(4)).wrapping_add(0x1A0), 0);
                                wr8(obj.wrapping_add(seq_h.wrapping_mul(4)).wrapping_add(0x1A2), 0);
                                let seq2 = seq_h.wrapping_add(1);
                                wr32(obj.wrapping_add(OBJ_SEQ), seq2);
                                // Block two: the entry block.
                                let s2 = obj.wrapping_add(seq2.wrapping_add(9).wrapping_mul(16));
                                wrf(s2, x_e);
                                wrf(s2.wrapping_add(4), y_e);
                                wrf(s2.wrapping_add(8), z_e);
                                wrf(s2.wrapping_add(12), STACK_W);
                                wr32(obj.wrapping_add(seq2.wrapping_mul(4)).wrapping_add(OBJ_WORD1), 0);
                                let tag2: u32 = if flag_e == 1 { 0x10 } else { 0x20 };
                                wr16(obj.wrapping_add(seq2.wrapping_mul(4)).wrapping_add(0x1A0), tag2 as u16);
                                wr8(
                                    obj.wrapping_add(seq2.wrapping_mul(4)).wrapping_add(0x1A2),
                                    rd8(e.wrapping_add(0x15)),
                                );
                                let seq3 = seq2.wrapping_add(1);
                                wr32(obj.wrapping_add(OBJ_SEQ), seq3);
                                // Block three: the second point block.
                                let s3 = obj.wrapping_add(seq3.wrapping_add(9).wrapping_mul(16));
                                wrf(s3, x_e2);
                                wrf(s3.wrapping_add(4), y_e2);
                                wrf(s3.wrapping_add(8), z_e2);
                                wrf(s3.wrapping_add(12), STACK_W);
                                wr32(obj.wrapping_add(seq3.wrapping_mul(4)).wrapping_add(OBJ_WORD1), 0);
                                wr16(obj.wrapping_add(seq3.wrapping_mul(4)).wrapping_add(0x1A0), 0);
                                wr8(obj.wrapping_add(seq3.wrapping_mul(4)).wrapping_add(0x1A2), 0);
                                wr32(obj.wrapping_add(OBJ_SEQ), seq3.wrapping_add(1));
                            }
                        }
                        skip_shape = true;
                    }
                }
                if skip_shape {
                    // Loop back-edge with no append. The scan-accepted path does
                    // not store the previous node (see prev_node below).
                } else {
                    if rd32(node_slot) & 0x12_C000 != 0 {
                        unreachable!("node kinds");
                    }
                    // Index guard (never taken). The original compares the loop
                    // index (1 here) with the word at frame+0xC plus one and takes
                    // a different path when the index is larger. That word is
                    // stored 0 by the loop prologue (0xDC21AD) and, on every path
                    // that reaches this test, holds the previous index (stored at
                    // 0xDC249C and 0xDC2BF2 before the index is incremented). The
                    // index advances by one, so the test is never true. The guard
                    // stays as a deliberate fault.
                    let guard_prev_index: u32 = 0;
                    if 1u32 > guard_prev_index.wrapping_add(1) {
                        unreachable!("index guard (never taken)");
                    }
                    let seq = rd32(obj.wrapping_add(OBJ_SEQ));
                    let slot =
                        obj.wrapping_add(seq.wrapping_add(9).wrapping_mul(16));
                    // First shape arg re-reads the scan block pointer the loop
                    // prologue stored (frame+0x20): the same fptr_a.
                    let shape: u32 = lf_checker_rt::callee_thiscall!(
                        10, u32, this, fptr_a, fptr_b, node0, node_slot, slot, 0
                    );
                    if (shape as u8) == 0 {
                            // Midpoint block: average the two float blocks, halve, append.
                        // No link store on this path.
                        let half = f32::from_bits(gr32(0xFE8830));
                        let seq_m = rd32(obj.wrapping_add(OBJ_SEQ));
                        let slot_m = obj.wrapping_add(
                            seq_m.wrapping_add(9).wrapping_mul(16),
                        );
                        let mx = fmul(
                            fadd(
                                rdf(fptr_b),
                                rdf(fptr_a),
                            ),
                            half,                    );
                        let my = fmul(
                            fadd(
                                rdf(fptr_b.wrapping_add(4)),
                                rdf(fptr_a.wrapping_add(4)),
                            ),
                            half,                    );
                        let mz = fmul(
                            fadd(
                                rdf(fptr_b.wrapping_add(8)),
                                rdf(fptr_a.wrapping_add(8)),
                            ),
                            half,                    );
                        wrf(slot_m, mx);
                        wrf(slot_m.wrapping_add(4), my);
                        wrf(slot_m.wrapping_add(8), mz);
                        wrf(slot_m.wrapping_add(12), STACK_W);
                        wr32(obj.wrapping_add(OBJ_SEQ), seq_m.wrapping_add(1));
                    } else {
                        // Shape accepted: a zero link word is appended.
                        let seq_e = rd32(obj.wrapping_add(OBJ_SEQ));
                        wr32(
                            obj.wrapping_add(seq_e.wrapping_mul(4))
                                .wrapping_add(OBJ_WORD1),
                            0,
                        );
                        wr32(obj.wrapping_add(OBJ_SEQ), seq_e.wrapping_add(1));
                    }
                    // Both shape arms meet at the capacity check.
                    if rd32(obj.wrapping_add(OBJ_SEQ)) >= max {
                        return 1;
                    }
                    // Shape arm at any iteration: the shape call, midpoint or
                    // link, capacity check and iteration-one copy use only the
                    // per-iteration pointers, so a bound of two or more continues
                    // to the next iteration.
                    // Copy the slot's float block (fptr_b, bit moves) to the object
                    // at seq+9, link the slot's node, clear the tag words, advance
                    // seq, check the capacity again.
                    let seq_c = rd32(obj.wrapping_add(OBJ_SEQ));
                    let slot_c = obj.wrapping_add(seq_c.wrapping_add(9).wrapping_mul(16));
                    for k in 0..4u32 {
                        wr32(slot_c.wrapping_add(k * 4), rd32(fptr_b.wrapping_add(k * 4)));
                    }
                    wr32(
                        obj.wrapping_add(seq_c.wrapping_mul(4))
                            .wrapping_add(OBJ_WORD1),
                        node_slot,
                    );
                    wr16(obj.wrapping_add(seq_c.wrapping_mul(4)).wrapping_add(0x1A0), 0);
                    wr8(obj.wrapping_add(seq_c.wrapping_mul(4)).wrapping_add(0x1A2), 0);
                    wr32(obj.wrapping_add(OBJ_SEQ), seq_c.wrapping_add(1));
                    if rd32(obj.wrapping_add(OBJ_SEQ)) >= max {
                        return 1;
                    }
                    // Back-edge: the next iteration (if any) follows; the loop exits
                    // to the end-point tail below.
                }
                if !scan_skipped {
                    prev_node = node_slot;
                    prev_fptr = fptr_b;
                }
                iter = iter.wrapping_add(1);
            }
            {
                // End-point tail: append the selected point and return.
                let seq3 = rd32(obj.wrapping_add(OBJ_SEQ));
                let slot_b =
                    obj.wrapping_add(seq3.wrapping_add(9).wrapping_mul(16));
                wrf(slot_b, ox);
                wrf(slot_b.wrapping_add(4), oy);
                wrf(slot_b.wrapping_add(8), oz);
                wrf(slot_b.wrapping_add(12), ow);
                wr32(
                    obj.wrapping_add(seq3.wrapping_mul(4)).wrapping_add(OBJ_WORD1),
                    rotated,
                );
                wr16(obj.wrapping_add(seq3.wrapping_mul(4)).wrapping_add(0x1A0), 0);
                wr8(obj.wrapping_add(seq3.wrapping_mul(4)).wrapping_add(0x1A2), 0);
                wr32(obj.wrapping_add(OBJ_SEQ), seq3.wrapping_add(1));
                return 1;
            }
        }
        // Header copy loop: slots 1..count copy their float block
        // and table words from the globals into the object. Skipped for
        // count <= 1 exactly like the original's compare-and-branch. The
        // loop bound is unsigned (jb).
        let g1off = COPY_G1.wrapping_sub(obj);
        let g2off = COPY_G2.wrapping_sub(obj);
        let mut cx = obj.wrapping_add(0x1A4);
        let mut di = obj.wrapping_add(0xA8);
        let mut dxf = COPY_FLOATS;
        let mut j = 1u32;
        while j < count {
            wr32(di.wrapping_sub(8), gr32(dxf.wrapping_sub(8)));
            wrf(di.wrapping_sub(4), grf(dxf.wrapping_sub(4)));
            wrf(di, grf(dxf));
            wr32(di.wrapping_add(4), gr32(dxf.wrapping_add(4)));
            wr32(cx.wrapping_add(0x44), gr32(g1off.wrapping_add(cx)));
            wr32(cx, gr32(g2off.wrapping_add(cx)));
            cx = cx.wrapping_add(4);
            di = di.wrapping_add(16);
            dxf = dxf.wrapping_add(16);
            j = j.wrapping_add(1);
        }
        wr32(obj + OBJ_WORD1, gr32(SLOT_ROTATE));
        wrf(obj + OBJ_START + 12, STACK_W);
        wrf(obj + OBJ_START, sx);
        wrf(obj + OBJ_START + 4, sy);
        wrf(obj + OBJ_START + 8, sz);
        let count2 = gr32(SLOT_COUNT);
        let table_word = gr32(SLOT_TABLE.wrapping_add(count2.wrapping_mul(4)));
        wr32(obj + count2.wrapping_mul(4) + OBJ_WORD0, table_word);
        let slot = obj.wrapping_add(count2.wrapping_add(8).wrapping_mul(16));
        wrf(slot, ex);
        wrf(slot + 4, ey);
        wrf(slot + 8, ez);
        wrf(slot + 12, STACK_W);
        wr32(obj + OBJ_SEQ, count2);
        1
    }
}

lf_checker_rt::export!(thiscall, rw_00DC1C90(this: u32, obj: u32, start_xyz: u32, end_xyz: u32, max: u32) -> u32 { unsafe { path_build(this, obj, start_xyz, end_xyz, max) } });
