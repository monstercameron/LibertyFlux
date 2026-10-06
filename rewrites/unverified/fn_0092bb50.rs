// original: 0x0092BB50 cascade_shadow_map_update (proposed)

/// Recompute the cascade shadow maps for the current frame: pick the source
/// row by thread state, copy it into this frame's matrix row, derive the
/// five cascade splits by comparing a measured depth against scripted
/// bounds, fit one projection per cascade, and finish the destination
/// matrix block.
///
/// Takes no arguments (cdecl, nothing on the stack, no register inputs);
/// everything comes from globals, fabricates, and callee answers. Returns
/// the last dword copied into the destination block.
///
/// Behaviour:
/// - Select a table index from one of two globals by thread-state bits
///   (bit 1 or bit 3 set selects the first), exactly like the caller.
/// - Run the row-source callee, then copy four dwords from the selected
///   table row and four floats from the source-float globals into this
///   frame's matrix row (`MATRIX_BASE + idx2*0x270`).
/// - Run the row followers (bound check, two keyed builds, two keyed
///   transforms), accumulate one float, and run the row finish callee.
/// - Form a four-term dot product of paired globals, negate it with the
///   image sign mask, and run the picker callee whose answer selects the
///   cascade object (or null for the direct path).
/// - Clamp the cascade count from its global to [0, 4] (the decrement is
///   tested signed for below-zero, the upper bound is a signed greater
///   compare against 4), then choose the measured depth: zero when the
///   override flag is set, otherwise the row's stored depth.
/// - Five cascade passes over the split tables: when the low bound
///   compares below the depth, or the depth below the high bound (either
///   `comiss` jumps on unordered too, so NaN takes the copying branch),
///   copy the pass's split words from the matching table; otherwise, when
///   a cascade object was picked, consult its state flags and virtual
///   slots, then interpolate the split between the tables by
///   `depth/high` in the original's operation order.
/// - Convert two integer viewport globals to float, divide the reference
///   float by each, and run the viewport object's virtual slot; store the
///   results and two more floats into the row, then emit one projection
///   per cascade (four passes): keyed calls, a min/max scratch fill, the
///   fitting callee whose four out-words are scripted, and the fitted
///   values stored back into the row with the original's operation order.
/// - Finish the destination block (`row+0x1a0`): integer-to-float scales,
///   a chain of subtractions and divisions in the original's order, then
///   three fixup sweeps over the row's four float quads with a zero factor
///   (`x*+0.0` keeps NaN and the sign of zero, exactly as the original's
///   `mulss`), and copy sixteen dwords from `row+0x60` to the destination.
///
/// Float operation order is the original's throughout, pinned with
/// `black_box` so NaN signs and payloads match bit for bit. Integer
/// viewport conversion is the signed 32-bit value converted exactly. The
/// original's stack frame is mirrored byte for byte (`Frame`, offsets from
/// the balanced stack pointer); slots the original never writes stay zero
/// on both sides (the contract fills uninitialized stack with zero).
///
/// Original: 0x0092BB50 (cdecl, no stack arguments, returns a u32).
lf_checker_rt::export!(cdecl, rw_0092bb50() -> u32 {
    unsafe {
        const TLS_INDEX: u32 = 0x17ABA14;
        const TLS_STATE_OFF: u32 = 0x8D0;
        const TABLE_IDX_A: u32 = 0x1174790;
        const TABLE_IDX_B: u32 = 0x1174794;
        const TABLE_STRIDE: u32 = 128;
        const ROW_TABLE: u32 = 0x154CBC0;
        const SRC_TABLE: u32 = 0x11A2500;
        const MATRIX_BASE: u32 = 0x11A1C50;
        const MATRIX_STRIDE: u32 = 0x270;
        const SRC_F0: u32 = 0x11A2570;
        const SRC_F1: u32 = 0x11A2574;
        const SRC_F2: u32 = 0x11A2578;
        const SRC_F3: u32 = 0x11A257C;
        const ACC_ADDEND: u32 = 0x10369B4;
        const DOT_A0: u32 = 0x11A2660;
        const DOT_A1: u32 = 0x11A2664;
        const DOT_A2: u32 = 0x11A2668;
        const DOT_A3: u32 = 0x11A266C;
        const DOT_B0: u32 = 0x11A2670;
        const DOT_B1: u32 = 0x11A2674;
        const DOT_B2: u32 = 0x11A2678;
        const DOT_B3: u32 = 0x11A267C;
        const SIGN_MASK: u32 = 0xFE8FA0;
        const CLAMP_SRC: u32 = 0x1160EAC;
        const OVERRIDE_FLAG: u32 = 0x1036ADC;
        const DEPTH_OUT: u32 = 0x1036930;
        const BOUND_LO: u32 = 0x1036934;
        const BOUND_HI: u32 = 0x1036938;
        const SPLIT_OUT: u32 = 0x103693C;
        const SPLIT_B_OUT: u32 = 0x1036A1C;
        const SPLIT_A_TAB: u32 = 0x1036950;
        const SPLIT_B_TAB: u32 = 0x10369B8;
        const SPLIT2_A_TAB: u32 = 0x1036A30;
        const SPLIT2_B_TAB: u32 = 0x1036A80;
        const VIEW_INT0: u32 = 0x11A1B80;
        const VIEW_INT1: u32 = 0x11A1B84;
        const VIEW_OBJ: u32 = 0x11A1B88;
        const VIEW_REF: u32 = 0xFE88E8;
        const FIT_SCALE: u32 = 0x11A1BD4;
        const FIT_BASE: u32 = 0xFE87E4;
        const FIT_K: u32 = 0xFE8A24;
        const ROW_F3: u32 = 0x103694C;
        const EMIT_TAB: u32 = 0x1036940;
        const QUARTER: u32 = 0x3E800000;
        const HALF: u32 = 0x3F000000;
        const THREE_Q: u32 = 0x3F400000;
        const FMAX: u32 = 0x7F7FFFFF;
        const FNEGMAX: u32 = 0xFF7FFFFF;

        const ROW_SOURCE: u32 = 1;
        const ROW_BOUND: u32 = 2;
        const ROW_BUILD: u32 = 3;
        const ROW_XF: u32 = 4;
        const ROW_KEYS: u32 = 5;
        const ROW_TRIO: u32 = 6;
        const PICK: u32 = 7;
        const VT_GATE: u32 = 8;
        const VT_KIND: u32 = 9;
        const EMIT_OPEN: u32 = 10;
        const EMIT_A: u32 = 11;
        const EMIT_B: u32 = 12;
        const EMIT_SRC: u32 = 13;
        const EMIT_FIT: u32 = 14;
        const EMIT_DONE: u32 = 15;
        const VT_VIEW: u32 = 16;

        /// Mirror of the original's stack frame, offsets from the balanced
        /// stack pointer (verified with an esp tracker, not by hand).
        #[repr(C)]
        struct Frame {
            _p00: [u8; 8],
            slot08: u32,       // +0x08 cascade count, then floats/pointers
            slot0c: u32,       // +0x0c picked object, then floats/pointers
            slot10: u32,       // +0x10 floats, then the destination pointer
            _p14: [u8; 0x14],
            slot28: u32,       // +0x28 emit temporaries
            slot2c: u32,       // +0x2c
            _p30: [u8; 4],
            slot34: u32,       // +0x34 negated dot, then a tail float
            slot38: u32,       // +0x38 row pointer + 0x60
            slot3c: u32,       // +0x3c row pointer
            blk40: [u32; 8],   // +0x40 min/max scratch, then fit out-words
            _p60: [u8; 8],
            slot68: u32,       // +0x68 emit temporaries
            slot6c: u32,       // +0x6c
            blk70: [u32; 8],   // +0x70 emit object (never written)
            _pad: [u8; 0x3D0],
            blk460: [u32; 8],  // +0x460 row constants [0,0,.25,0,.5,0,.75,0]
            _p480: [u8; 0x20],
            blk4a0: [u32; 8],  // +0x4a0 keyed-build scratch (never written)
            _tail: [u8; 0x68],
        }
        const _: () = assert!(core::mem::offset_of!(Frame, slot08) == 0x08);
        const _: () = assert!(core::mem::offset_of!(Frame, slot0c) == 0x0C);
        const _: () = assert!(core::mem::offset_of!(Frame, slot10) == 0x10);
        const _: () = assert!(core::mem::offset_of!(Frame, slot28) == 0x28);
        const _: () = assert!(core::mem::offset_of!(Frame, slot2c) == 0x2C);
        const _: () = assert!(core::mem::offset_of!(Frame, slot34) == 0x34);
        const _: () = assert!(core::mem::offset_of!(Frame, slot38) == 0x38);
        const _: () = assert!(core::mem::offset_of!(Frame, slot3c) == 0x3C);
        const _: () = assert!(core::mem::offset_of!(Frame, blk40) == 0x40);
        const _: () = assert!(core::mem::offset_of!(Frame, slot68) == 0x68);
        const _: () = assert!(core::mem::offset_of!(Frame, slot6c) == 0x6C);
        const _: () = assert!(core::mem::offset_of!(Frame, blk70) == 0x70);
        const _: () = assert!(core::mem::offset_of!(Frame, blk460) == 0x460);
        const _: () = assert!(core::mem::offset_of!(Frame, blk4a0) == 0x4A0);
        const _: () = assert!(core::mem::size_of::<Frame>() == 0x528);

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// `comiss`+`jb`: strictly below, or unordered (NaN either side).
        #[inline(always)]
        fn below(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) >= core::hint::black_box(b))
        }
        /// `movd`+`cvtdq2ps`: the dword as a signed integer, converted exactly.
        #[inline(always)]
        fn int_to_float(bits: u32) -> f32 {
            core::hint::black_box(bits as i32) as f32
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let target: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                target(obj)
            }
        }

        let slot = rd32(lf_checker_rt::relocated(TLS_INDEX));
        let state = rd32(lf_checker_rt::tls_slot(slot as usize).wrapping_add(TLS_STATE_OFF));
        let idx = if (state >> 1) & 1 != 0 {
            rd32(lf_checker_rt::relocated(TABLE_IDX_A))
        } else if (state >> 3) & 1 != 0 {
            rd32(lf_checker_rt::relocated(TABLE_IDX_A))
        } else {
            rd32(lf_checker_rt::relocated(TABLE_IDX_B))
        };
        let tab = lf_checker_rt::relocated(ROW_TABLE)
            .wrapping_add(idx.wrapping_mul(TABLE_STRIDE));

        let mut frame: Frame = core::mem::zeroed();
        let base = core::ptr::addr_of!(frame) as u32;
        let at = |off: u32| base.wrapping_add(off);

        let _: u32 = lf_checker_rt::callee_cdecl!(
            ROW_SOURCE, u32,
            lf_checker_rt::relocated(SRC_TABLE),
            at(0x4A0)
        );
        let idx2 = rd32(lf_checker_rt::relocated(TABLE_IDX_A));
        let row = lf_checker_rt::relocated(MATRIX_BASE)
            .wrapping_add(idx2.wrapping_mul(MATRIX_STRIDE));
        wr32(row.wrapping_add(0x250), rd32(tab));
        wr32(row.wrapping_add(0x254), rd32(tab.wrapping_add(4)));
        wr32(row.wrapping_add(0x258), rd32(tab.wrapping_add(8)));
        wr32(row.wrapping_add(0x25C), rd32(tab.wrapping_add(12)));
        wrf(row.wrapping_add(0x260), rdf(lf_checker_rt::relocated(SRC_F0)));
        wrf(row.wrapping_add(0x264), rdf(lf_checker_rt::relocated(SRC_F1)));
        wrf(row.wrapping_add(0x268), rdf(lf_checker_rt::relocated(SRC_F2)));
        wrf(row.wrapping_add(0x26C), rdf(lf_checker_rt::relocated(SRC_F3)));
        frame.slot3c = row;
        frame.slot38 = row.wrapping_add(0x60);
        let _: u32 = lf_checker_rt::callee_thiscall!(ROW_BOUND, u32, row.wrapping_add(0x60));

        let _: u32 = lf_checker_rt::callee_cdecl!(
            ROW_BUILD, u32,
            lf_checker_rt::relocated(SRC_TABLE),
            tab,
            at(0x4A0),
            at(0x460)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(ROW_XF, u32, row.wrapping_add(0x60), at(0x460));
        let _: u32 = lf_checker_rt::callee_cdecl!(
            ROW_KEYS, u32,
            at(0x4A0),
            row.wrapping_add(0x60),
            at(0x50),
            at(0x40)
        );
        let acc = add(
            rdf(lf_checker_rt::relocated(ACC_ADDEND)),
            rdf(at(0x48)),
        );
        wrf(at(0x48), acc);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            ROW_TRIO, u32,
            at(0x50),
            at(0x40),
            at(0x460)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(ROW_XF, u32, row.wrapping_add(0x60), at(0x460));

        frame.blk460[0] = 0;
        frame.blk460[1] = 0;
        frame.blk460[2] = QUARTER;
        frame.blk460[3] = 0;
        frame.blk460[4] = HALF;
        frame.blk460[5] = 0;
        frame.blk460[6] = THREE_Q;
        frame.blk460[7] = 0;
        let dot_hi = mul(
            rdf(lf_checker_rt::relocated(DOT_A1)),
            rdf(lf_checker_rt::relocated(DOT_B1)),
        );
        let dot_lo = mul(
            rdf(lf_checker_rt::relocated(DOT_A0)),
            rdf(lf_checker_rt::relocated(DOT_B0)),
        );
        let mut dot = add(dot_lo, dot_hi);
        dot = add(
            dot,
            mul(
                rdf(lf_checker_rt::relocated(DOT_A2)),
                rdf(lf_checker_rt::relocated(DOT_B2)),
            ),
        );
        dot = add(
            dot,
            mul(
                rdf(lf_checker_rt::relocated(DOT_A3)),
                rdf(lf_checker_rt::relocated(DOT_B3)),
            ),
        );
        let neg = f32::from_bits(dot.to_bits() ^ rd32(lf_checker_rt::relocated(SIGN_MASK)));
        wrf(at(0x34), neg);
        frame.slot34 = neg.to_bits();

        let picked: u32 = lf_checker_rt::callee_cdecl!(PICK, u32,);
        frame.slot0c = picked;
        let raw = rd32(lf_checker_rt::relocated(CLAMP_SRC));
        let dec = raw.wrapping_sub(1);
        let count = if (dec as i32) < 0 {
            0u32
        } else if (dec as i32) > 4 {
            4u32
        } else {
            dec
        };
        frame.slot08 = count;
        let depth: f32;
        if rd8(lf_checker_rt::relocated(OVERRIDE_FLAG)) != 0 {
            wrf(lf_checker_rt::relocated(DEPTH_OUT), 0.0);
            depth = 0.0;
        } else {
            depth = rdf(row.wrapping_add(0x268));
            wrf(lf_checker_rt::relocated(DEPTH_OUT), depth);
        }
        let hi = rdf(lf_checker_rt::relocated(BOUND_HI));
        let mut tab_off = count.wrapping_mul(20);
        let mut cascade = 0u32;
        loop {
            let lo = rdf(lf_checker_rt::relocated(BOUND_LO));
            if below(lo, depth) {
                wr32(
                    lf_checker_rt::relocated(SPLIT_OUT).wrapping_add(cascade.wrapping_mul(4)),
                    rd32(lf_checker_rt::relocated(SPLIT_A_TAB).wrapping_add(tab_off)),
                );
                if cascade < 4 {
                    let t = cascade.wrapping_add(count.wrapping_mul(4));
                    wr32(
                        lf_checker_rt::relocated(SPLIT_B_OUT).wrapping_add(cascade.wrapping_mul(4)),
                        rd32(lf_checker_rt::relocated(SPLIT2_A_TAB).wrapping_add(t.wrapping_mul(4))),
                    );
                }
            } else if below(depth, hi) {
                wr32(
                    lf_checker_rt::relocated(SPLIT_OUT).wrapping_add(cascade.wrapping_mul(4)),
                    rd32(lf_checker_rt::relocated(SPLIT_B_TAB).wrapping_add(tab_off)),
                );
                if cascade < 4 {
                    let t = cascade.wrapping_add(count.wrapping_mul(4));
                    wr32(
                        lf_checker_rt::relocated(SPLIT_B_OUT).wrapping_add(cascade.wrapping_mul(4)),
                        rd32(lf_checker_rt::relocated(SPLIT2_B_TAB).wrapping_add(t.wrapping_mul(4))),
                    );
                }
            } else {
                // Branch C: consult the picked object. A null object, a
                // set state bit, or a refused gate all take the
                // interpolated path; otherwise the split is copied
                // directly. Both tails run the second-half blend below.
                let obj = frame.slot0c;
                let mut dw = depth;
                let mut hw = hi;
                let to_lerp: bool;
                if obj == 0 {
                    to_lerp = true;
                } else if rd32(obj.wrapping_add(0x28)) & 0x3C0 == 0xC0 {
                    to_lerp = rd32(obj.wrapping_add(0x26C)) & 0xA000 != 0;
                } else {
                    let ans = vcall0(obj, 0x24);
                    if ans & 0xFF != 0 {
                        hw = rdf(lf_checker_rt::relocated(BOUND_HI));
                        dw = rdf(lf_checker_rt::relocated(DEPTH_OUT));
                        to_lerp = rd32(obj.wrapping_add(0x26C)) & 0xA000 != 0;
                    } else if rd32(obj.wrapping_add(0x28)) & 0x3C0 != 0x80 {
                        hw = rdf(lf_checker_rt::relocated(BOUND_HI));
                        dw = rdf(lf_checker_rt::relocated(DEPTH_OUT));
                        to_lerp = true;
                    } else {
                        let kind = rd32(obj.wrapping_add(0x1304));
                        if kind == 4 || kind == 5 {
                            hw = rdf(lf_checker_rt::relocated(BOUND_HI));
                            dw = rdf(lf_checker_rt::relocated(DEPTH_OUT));
                            to_lerp = true;
                        } else {
                            let ans2 = vcall0(obj, 0x164);
                            hw = rdf(lf_checker_rt::relocated(BOUND_HI));
                            dw = rdf(lf_checker_rt::relocated(DEPTH_OUT));
                            // This path tests the gate answer, not flags.
                            to_lerp = ans2 & 0xFF != 0;
                        }
                    }
                }
                if !to_lerp {
                    wr32(
                        lf_checker_rt::relocated(SPLIT_OUT)
                            .wrapping_add(cascade.wrapping_mul(4)),
                        rd32(lf_checker_rt::relocated(SPLIT_A_TAB).wrapping_add(tab_off)),
                    );
                } else {
                    let a = rdf(lf_checker_rt::relocated(SPLIT_A_TAB).wrapping_add(tab_off));
                    let b = rdf(lf_checker_rt::relocated(SPLIT_B_TAB).wrapping_add(tab_off));
                    let t = div(dw, hw);
                    wrf(
                        lf_checker_rt::relocated(SPLIT_OUT)
                            .wrapping_add(cascade.wrapping_mul(4)),
                        add(mul(sub(b, a), t), a),
                    );
                }
                if cascade < 4 {
                    let t2 = cascade.wrapping_add(count.wrapping_mul(4));
                    let a = rdf(
                        lf_checker_rt::relocated(SPLIT2_A_TAB)
                            .wrapping_add(t2.wrapping_mul(4)),
                    );
                    let b = rdf(
                        lf_checker_rt::relocated(SPLIT2_B_TAB)
                            .wrapping_add(t2.wrapping_mul(4)),
                    );
                    let tt = div(dw, hw);
                    wrf(
                        lf_checker_rt::relocated(SPLIT_B_OUT)
                            .wrapping_add(cascade.wrapping_mul(4)),
                        add(mul(sub(b, a), tt), a),
                    );
                }
            }
            cascade = cascade.wrapping_add(1);
            tab_off = tab_off.wrapping_add(4);
            if cascade >= 5 {
                break;
            }
        }

        // ---- viewport scales and the viewport object call ----
        let vw0 = div(
            rdf(lf_checker_rt::relocated(VIEW_REF)),
            int_to_float(rd32(lf_checker_rt::relocated(VIEW_INT0))),
        );
        wrf(at(0x10), vw0);
        frame.slot10 = vw0.to_bits();
        let vw1 = div(
            rdf(lf_checker_rt::relocated(VIEW_REF)),
            int_to_float(rd32(lf_checker_rt::relocated(VIEW_INT1))),
        );
        wrf(at(0x08), vw1);
        frame.slot08 = vw1.to_bits();
        let view_obj = rd32(lf_checker_rt::relocated(VIEW_OBJ));
        let view_ans = vcall0(view_obj, 0x20);
        let vw2 = div(
            rdf(lf_checker_rt::relocated(VIEW_REF)),
            int_to_float(view_ans),
        );
        let row2 = frame.slot3c;
        wrf(row2.wrapping_add(0x50), rdf(at(0x10)));
        frame.slot10 = row2.wrapping_add(0x1A0);
        wrf(row2.wrapping_add(0x54), rdf(at(0x08)));
        let row_f3 = rdf(lf_checker_rt::relocated(ROW_F3));
        frame.slot0c = row2.wrapping_add(0x20);
        wrf(row2.wrapping_add(0x58), vw2);
        wrf(row2.wrapping_add(0x5C), row_f3);
        frame.slot08 = row2.wrapping_add(0xA0);
        let mut emit_row = frame.slot0c;

        // ---- four emit passes ----
        let mut emit = 0u32;
        loop {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                EMIT_OPEN, u32,
                at(0x70),
                lf_checker_rt::relocated(SRC_TABLE)
            );
            wrf(at(0x0C), rdf(
                lf_checker_rt::relocated(EMIT_TAB).wrapping_add(emit.wrapping_mul(4)),
            ));
            frame.slot0c = rd32(at(0x0C));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                EMIT_A, u32,
                at(0x70),
                rd32(
                    lf_checker_rt::relocated(SPLIT_OUT).wrapping_add(emit.wrapping_mul(4)),
                )
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(
                EMIT_B, u32,
                at(0x70),
                frame.slot0c
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(
                EMIT_SRC, u32,
                at(0x70),
                at(0x4A0)
            );
            frame.blk40[0] = FMAX;
            frame.blk40[1] = FMAX;
            frame.blk40[2] = FMAX;
            frame.blk40[3] = FNEGMAX;
            frame.blk40[4] = FNEGMAX;
            frame.blk40[5] = FNEGMAX;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                ROW_KEYS, u32,
                at(0x4A0),
                frame.slot38,
                at(0x40),
                at(0x50)
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(
                ROW_TRIO, u32,
                at(0x40),
                at(0x50),
                at(0x4A0)
            );
            let fit_ctx = frame.slot10;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                EMIT_FIT, u32,
                fit_ctx,
                emit,
                at(0x40),
                at(0x50)
            );
            // Fit results (the callee's four out-words) drive this pass.
            let fz1 = rdf(at(0x54));
            let fz0 = rdf(at(0x44));
            let fz3 = rdf(at(0x50));
            let fz2 = rdf(at(0x40));
            let mut q0 = sub(fz1, fz0);
            let mut q1 = sub(fz3, fz2);
            let mut q7 = div(rdf(lf_checker_rt::relocated(VIEW_REF)), q0);
            let mut q3 = div(
                rdf(lf_checker_rt::relocated(FIT_SCALE)),
                int_to_float(rd32(lf_checker_rt::relocated(VIEW_INT0))),
            );
            let mut q2 = div(rdf(lf_checker_rt::relocated(VIEW_REF)), q1);
            let n5 = f32::from_bits(fz2.to_bits() ^ rd32(lf_checker_rt::relocated(SIGN_MASK)));
            let n6 = f32::from_bits(fz0.to_bits() ^ rd32(lf_checker_rt::relocated(SIGN_MASK)));
            q1 = rdf(at(0x460).wrapping_add(emit.wrapping_mul(8)));
            q0 = q3;
            q0 = mul(q0, rdf(lf_checker_rt::relocated(FIT_K)));
            q1 = add(q1, q3);
            let mut q4 = sub(rdf(lf_checker_rt::relocated(FIT_BASE)), q0);
            q0 = q7;
            q0 = mul(q0, n6);
            q4 = mul(q4, q2);
            wrf(at(0x2C), q0);
            frame.slot2c = q0.to_bits();
            q2 = q4;
            q2 = mul(q2, n5);
            wrf(at(0x28), q2);
            frame.slot28 = q2.to_bits();
            frame.slot68 = frame.slot28;
            frame.slot6c = frame.slot2c;
            q0 = rdf(at(0x464).wrapping_add(emit.wrapping_mul(8)));
            q0 = add(q0, rdf(at(0x6C)));
            q1 = add(q1, q2);
            wrf(emit_row.wrapping_sub(0x20), q4);
            wrf(emit_row.wrapping_sub(0x10), q7);
            wrf(emit_row, q1);
            wrf(emit_row.wrapping_add(0x10), q0);
            q0 = add(rdf(at(0x0C)), rdf(at(0x34)));
            wrf(emit_row.wrapping_add(0x20), q0);
            let _: u32 = lf_checker_rt::callee_thiscall!(EMIT_DONE, u32, at(0x70));
            frame.slot08 = frame.slot08.wrapping_add(0x40);
            emit = emit.wrapping_add(1);
            emit_row = emit_row.wrapping_add(4);
            if emit >= 4 {
                break;
            }
        }

        // ---- destination block finish ----
        let dst = frame.slot10;
        let sc0 = mul(
            int_to_float(rd32(lf_checker_rt::relocated(VIEW_INT0))),
            rdf(lf_checker_rt::relocated(FIT_BASE)),
        );
        wrf(at(0x28), sc0);
        frame.slot28 = sc0.to_bits();
        wr32(dst.wrapping_add(0x40), frame.slot28);
        let sc1 = int_to_float(rd32(lf_checker_rt::relocated(VIEW_INT1)));
        wrf(at(0x2C), sc1);
        frame.slot2c = sc1.to_bits();
        wr32(dst.wrapping_add(0x44), frame.slot2c);
        let base3 = rdf(dst.wrapping_add(0x40));
        let d1 = sub(rdf(dst.wrapping_add(0x84)), rdf(dst.wrapping_add(0x64)));
        let d0 = sub(rdf(dst.wrapping_add(0x70)), rdf(dst.wrapping_add(0x50)));
        let d6 = sub(rdf(dst.wrapping_add(0x7C)), rdf(dst.wrapping_add(0x5C)));
        let d4 = sub(rdf(dst.wrapping_add(0x74)), rdf(dst.wrapping_add(0x54)));
        let d5 = sub(rdf(dst.wrapping_add(0x78)), rdf(dst.wrapping_add(0x58)));
        wrf(at(0x38), d1);
        frame.slot38 = d1.to_bits();
        let mut e1 = sub(rdf(dst.wrapping_add(0x88)), rdf(dst.wrapping_add(0x68)));
        let mut e7 = rdf(dst.wrapping_add(0x80));
        let k2 = div(base3, d0);
        wrf(at(0x10), e1);
        frame.slot10 = e1.to_bits();
        e1 = sub(rdf(dst.wrapping_add(0x8C)), rdf(dst.wrapping_add(0x6C)));
        e7 = sub(e7, rdf(dst.wrapping_add(0x60)));
        let k0 = div(base3, d5);
        wrf(at(0x08), e1);
        frame.slot08 = e1.to_bits();
        let m1 = rdf(dst.wrapping_add(0x44));
        wrf(at(0x34), m1);
        frame.slot34 = m1.to_bits();
        let k1 = div(base3, d4);
        let k3 = div(base3, d6);
        wrf(dst.wrapping_add(0x94), k1);
        wrf(dst.wrapping_add(0x98), k0);
        wrf(dst.wrapping_add(0x9C), k3);
        let m3 = rdf(at(0x34));
        wrf(dst.wrapping_add(0x90), k2);
        let kk2 = div(m3, e7);
        let kk1 = div(m3, rdf(at(0x38)));
        let kk0 = div(m3, rdf(at(0x10)));
        let kk3 = div(m3, rdf(at(0x08)));
        wrf(dst.wrapping_add(0xA4), kk1);
        wrf(dst.wrapping_add(0xA8), kk0);
        wrf(dst.wrapping_add(0xA0), kk2);
        wrf(dst.wrapping_add(0xAC), kk3);

        // ---- row fixup sweeps (zero factor) ----
        // Each block subtracts an accumulated sum from one lane; the
        // factor is +0.0, so products keep NaN and signed zero exactly
        // like the original's mulss. Later blocks read updated lanes.
        let fix = frame.slot3c;
        let z = 0.0f32;
        // Sweep 1.
        let t0 = rdf(fix.wrapping_add(4));
        let p0 = mul(rdf(fix.wrapping_add(8)), z);
        let mut acc = mul(t0, z);
        acc = add(acc, rdf(fix));
        acc = add(acc, p0);
        acc = add(acc, mul(rdf(fix.wrapping_add(12)), z));
        wrf(fix.wrapping_add(4), sub(t0, acc));
        let t0 = rdf(fix.wrapping_add(0x14));
        let p0 = mul(rdf(fix.wrapping_add(0x18)), z);
        let mut acc = mul(t0, z);
        acc = add(acc, rdf(fix.wrapping_add(0x10)));
        acc = add(acc, p0);
        acc = add(acc, mul(rdf(fix.wrapping_add(0x1C)), z));
        wrf(fix.wrapping_add(0x14), sub(t0, acc));
        let t0 = rdf(fix.wrapping_add(0x24));
        let p0 = mul(rdf(fix.wrapping_add(0x28)), z);
        let mut acc = mul(t0, z);
        acc = add(acc, rdf(fix.wrapping_add(0x20)));
        acc = add(acc, p0);
        acc = add(acc, mul(rdf(fix.wrapping_add(0x2C)), z));
        wrf(fix.wrapping_add(0x24), sub(t0, acc));
        let t0 = rdf(fix.wrapping_add(0x34));
        let p0 = mul(rdf(fix.wrapping_add(0x38)), z);
        let mut acc = mul(t0, z);
        acc = add(acc, rdf(fix.wrapping_add(0x30)));
        acc = add(acc, p0);
        acc = add(acc, mul(rdf(fix.wrapping_add(0x3C)), z));
        wrf(fix.wrapping_add(0x34), sub(t0, acc));
        // Sweep 2.
        let mut acc = rdf(fix);
        acc = add(acc, rdf(fix.wrapping_add(4)));
        acc = add(acc, mul(rdf(fix.wrapping_add(8)), z));
        acc = add(acc, mul(rdf(fix.wrapping_add(12)), z));
        wrf(fix.wrapping_add(8), sub(rdf(fix.wrapping_add(8)), acc));
        let mut acc = rdf(fix.wrapping_add(0x14));
        acc = add(acc, rdf(fix.wrapping_add(0x10)));
        acc = add(acc, mul(rdf(fix.wrapping_add(0x18)), z));
        acc = add(acc, mul(rdf(fix.wrapping_add(0x1C)), z));
        wrf(fix.wrapping_add(0x18), sub(rdf(fix.wrapping_add(0x18)), acc));
        let mut acc = rdf(fix.wrapping_add(0x24));
        acc = add(acc, rdf(fix.wrapping_add(0x20)));
        acc = add(acc, mul(rdf(fix.wrapping_add(0x28)), z));
        acc = add(acc, mul(rdf(fix.wrapping_add(0x2C)), z));
        wrf(fix.wrapping_add(0x28), sub(rdf(fix.wrapping_add(0x28)), acc));
        let mut acc = rdf(fix.wrapping_add(0x30));
        acc = add(acc, rdf(fix.wrapping_add(0x34)));
        acc = add(acc, mul(rdf(fix.wrapping_add(0x38)), z));
        acc = add(acc, mul(rdf(fix.wrapping_add(0x3C)), z));
        wrf(fix.wrapping_add(0x38), sub(rdf(fix.wrapping_add(0x38)), acc));
        // Sweep 3.
        let t0 = rdf(fix.wrapping_add(0x0C));
        let mut acc = rdf(fix);
        acc = add(acc, rdf(fix.wrapping_add(4)));
        let p0 = mul(t0, z);
        acc = add(acc, rdf(fix.wrapping_add(8)));
        acc = add(acc, p0);
        wrf(fix.wrapping_add(0x0C), sub(t0, acc));
        let t0 = rdf(fix.wrapping_add(0x1C));
        let mut acc = rdf(fix.wrapping_add(0x14));
        acc = add(acc, rdf(fix.wrapping_add(0x10)));
        acc = add(acc, rdf(fix.wrapping_add(0x18)));
        acc = add(acc, mul(t0, z));
        wrf(fix.wrapping_add(0x1C), sub(t0, acc));
        let t0 = rdf(fix.wrapping_add(0x2C));
        let mut acc = rdf(fix.wrapping_add(0x24));
        acc = add(acc, rdf(fix.wrapping_add(0x20)));
        let p0 = mul(t0, z);
        acc = add(acc, rdf(fix.wrapping_add(0x28)));
        acc = add(acc, p0);
        wrf(fix.wrapping_add(0x2C), sub(t0, acc));
        let t0 = rdf(fix.wrapping_add(0x3C));
        let mut acc = rdf(fix.wrapping_add(0x30));
        acc = add(acc, rdf(fix.wrapping_add(0x34)));
        let p0 = mul(t0, z);
        acc = add(acc, rdf(fix.wrapping_add(0x38)));
        acc = add(acc, p0);
        wrf(fix.wrapping_add(0x3C), sub(t0, acc));

        // ---- destination block copy; the return value is the last word ----
        let mut i = 0u32;
        while i < 16 {
            wr32(
                dst.wrapping_add(i.wrapping_mul(4)),
                rd32(fix.wrapping_add(0x60).wrapping_add(i.wrapping_mul(4))),
            );
            i = i.wrapping_add(1);
        }
        rd32(fix.wrapping_add(0x9C))
    }
});
