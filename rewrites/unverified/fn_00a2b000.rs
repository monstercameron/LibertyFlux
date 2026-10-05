// original: 0x00a2b000 pedtask_followup_dispatch

/// Look up a follow-up action for a task and either submit or measure it.
///
/// `this` is the task object; the low byte of `arg0` selects between two
/// request codes (0x26e3 / 0x22e3). Returns al: 1 when the task advances,
/// 0 when it is released or the submitter refuses it.
///
/// Behaviour: an index word is fetched from a 12-byte-stride table at
/// `[this+0x2b0]`, indexed by `[this+0x2b0]+3`, and passed to the lookup
/// callee; the float at answer `+0x14` seeds `f`, scaled by
/// `[[this+0x228]+0x5b4] * f` when the global mode word is 2, the gate
/// callee agrees and `[this+0x228]` is non-null. If `[this+0x398]` is null,
/// has the wrong kind bits, or lacks its ready bit, the answer goes to the
/// submitter callee (with the request code chosen by `[answer+0xc] == 1`)
/// and a non-zero reply marks `[obj+0x70]+0x415` before returning 1.
/// Otherwise the transform callee fills an output block from the anchor
/// geometry and `f` is accepted when ordered-above-or-equal to the length of
/// the block's two reported components; below that the task is released and
/// 0 returned.
///
/// Note: the `test code, 0x1000` branch never fires (neither request code
/// has bit 12 set), so the kind/ready checks always run on that path.
/// Note: the transform callee writes words 0, 1 and 3 of the output
/// block (word 2 overlays the return address and is preserved).
///
/// Original: 0x00a2b000 (thiscall, one stack word; returns al, upper bytes
/// are stale).
lf_checker_rt::export!(thiscall, rw_00a2b000(this: u32, arg0: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x2b0;
        const OWNER: u32 = 0x398;
        const HANDLE: u32 = 0xe6c;
        const ENT_ANCHOR: u32 = 0x20;
        const ANCHOR_POS: u32 = 0x30;
        const ENT_KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_SUB: u32 = 0x0c0;
        const CODE_A: u32 = 0x26e3;
        const CODE_B: u32 = 0x22e3;
        const CODE_SPECIAL: u32 = 0x36f3;
        const LOOKUP_CALLEE: u32 = 1;
        const GATE_CALLEE: u32 = 2;
        const TRANSFORM_CALLEE: u32 = 3;
        const RELEASE_CALLEE: u32 = 4;
        const SUBMIT_CALLEE: u32 = 5;
        const G_MODE: u32 = 0x11d6fd4;
        const G_READY: u32 = 0x1160c68;
        const G_FLAG: u32 = 0x105c646;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        // Table index: (base + 3) * 3, scaled by 4 into a byte offset, so a
        // 12-byte stride from TABLE.
        let base = rd32(this.wrapping_add(TABLE));
        let idx = base.wrapping_add(3).wrapping_mul(3);
        let key = rd32(this.wrapping_add(TABLE).wrapping_add(idx.wrapping_mul(4)));
        let ans: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, key);
        let mut f = rdf(ans.wrapping_add(0x14));

        let gmode: u32 = lf_checker_rt::global::<u32>(G_MODE).read();
        if gmode == 2 {
            let gate: u32 =
                lf_checker_rt::callee_cdecl!(GATE_CALLEE, u32,);
            if gate != 0 {
                let m = rd32(this.wrapping_add(0x228));
                if m != 0 {
                    // Operand order is the original's: factor * f.
                    f = mul(rdf(m.wrapping_add(0x5b4)), f);
                }
            }
        }

        let code = if (arg0 as u8) != 0 { CODE_B } else { CODE_A };
        let owner = rd32(this.wrapping_add(OWNER));
        let mut take_measure_path = false;
        if owner != 0 {
            let h = rd32(this.wrapping_add(HANDLE));
            let blocked = rd32(h.wrapping_add(0x264)) & 0x800000 != 0;
            let gready: u32 = lf_checker_rt::global::<u32>(G_READY).read();
            // A blocked handle or an unready global skips the gate checks
            // below but still runs the kind check; only a refused gate or
            // a set flag forces the measure path outright.
            let mut force_measure = false;
            if !(blocked || gready == 0) {
                let gate2: u32 =
                    lf_checker_rt::callee_cdecl!(GATE_CALLEE, u32,);
                if gate2 == 0 {
                    force_measure = true;
                } else {
                    let gflag: u8 =
                        lf_checker_rt::global::<u8>(G_FLAG).read();
                    if gflag != 0 {
                        force_measure = true;
                    }
                }
            }
            if force_measure {
                take_measure_path = true;
            } else {
                // Dead branch in the original (neither code has bit 12);
                // written as the original tests it.
                if code & 0x1000 == 0 {
                    let kind_ok = rd32(owner.wrapping_add(ENT_KIND)) & KIND_MASK
                        == KIND_SUB;
                    let bit =
                        (rd32(owner.wrapping_add(0x270)) >> 4) & 1 != 0;
                    if kind_ok && bit {
                        take_measure_path = true;
                    }
                } else {
                    take_measure_path = true;
                }
            }
        }
        if !take_measure_path {
            // Submit path.
            let sel = if rd32(ans.wrapping_add(0x0c)) == 1 {
                CODE_SPECIAL
            } else {
                code
            };
            let r: u32 = lf_checker_rt::callee_thiscall!(
                SUBMIT_CALLEE, u32,
                this.wrapping_add(HANDLE), sel, f.to_bits(), 0u32, 0u32, 0u32);
            if r == 0 {
                return 0;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(
                RELEASE_CALLEE, u32, this.wrapping_add(TABLE), r);
            let m = rd32(this.wrapping_add(0x228));
            if m != 0 {
                ((m.wrapping_add(0x70).wrapping_add(0x415)) as *mut u8)
                    .write(1);
            } else {
                // Same fault the original takes through [0+0x415].
                ((0x415u32) as *mut u8).write(1);
            }
            return 1;
        }

        // Measure path.
        let c = rd32(owner.wrapping_add(ENT_ANCHOR));
        let p = if c != 0 {
            c.wrapping_add(ANCHOR_POS)
        } else {
            owner.wrapping_add(0x10)
        };
        let d = rd32(this.wrapping_add(ENT_ANCHOR));
        let dx = sub(rdf(p), rdf(d.wrapping_add(0x30)));
        let c2 = rd32(owner.wrapping_add(ENT_ANCHOR));
        let p2 = if c2 != 0 {
            c2.wrapping_add(ANCHOR_POS)
        } else {
            owner.wrapping_add(0x10)
        };
        let dy = sub(rdf(p2.wrapping_add(4)), rdf(d.wrapping_add(0x34)));
        // Output block shared with the transform callee; words 0, 1 and 3
        // are reread afterwards (word 2 overlays the return address slot
        // in the original and is preserved by the callee).
        let mut blk = [0u32; 4];
        blk[0] = f.to_bits();
        blk[1] = dx.to_bits();
        blk[3] = dy.to_bits();
        let blk_ptr = blk.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            TRANSFORM_CALLEE, u32, owner, this, blk_ptr);
        let fout = f32::from_bits(blk[0]);
        let ry = f32::from_bits(blk[1]);
        let rx = f32::from_bits(blk[3]);
        // Length of the two reported components; the original's sqrtps
        // also transforms two stale upper lanes that the scalar compare
        // then ignores.
        let len2 = add(mul(rx, rx), mul(ry, ry));
        let len = len2.sqrt();
        // comiss+jae: accept when ordered-above-or-equal.
        if fout >= len {
            return 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RELEASE_CALLEE, u32, this.wrapping_add(TABLE), 0u32);
        0
    }
});
