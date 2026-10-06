// original: 0x00d191e0 ped_task_tree_dispatch (proposed)

/// Dispatch a ped task-tree node by its type byte, recursing into children.
///
/// `this` is the dispatcher object, `task` the task node, `a2` an auxiliary
/// object (one case reads a word at `a2+8`). The type byte at `task+4` is
/// zero-extended and compared unsigned against 12 (`ja` over the range
/// 0..=255, so signedness is unobservable here); it selects one of five case
/// blocks through an index map, while values 2, 9, 11 and anything above 12
/// run only the tail. Every path ends by returning what callee 3 answers.
///
/// Case A (0, 1, 3): forward to callee 1 and return the tail.
/// Case B (4, 5): reserve N 16-byte slots (N = `task+0xC8`, native alloca),
/// fill each from signed words under `task+0xB0` as
/// `slot = cvt(word) * scale + bias` with scales at `task+0x90..` and biases
/// at `task+0xA0..`, run callee 2 over `task+0xCC` sources stamping a counter
/// word at `this+0x3C`, and for type 5 only continue through a callee-3 probe
/// and a global flag byte to callee 1. Skipped cleanly for N = 0 or count <= 0
/// (both bounds signed).
/// Case C (6, 7, 10): resolve a child through callee 4, poll it through a
/// planted callback and callee 6 until either settles, then convert index
/// words under the child with the same scale/bias table as case B, splice a
/// small header into a copy of the child's first 32 bytes (native memcpy),
/// hand both buffers to callee 8, and repeat the poll/convert round until
/// callee 6 answers null.
/// Case D (8): walk an index list (callees 9/16, ends at -1); per index ask
/// callee 10 for a node, callee 11 for two float triples, write a
/// min/max box pair at `this+0..0x18` offset by global constants, and recurse
/// into the node, updating through callees 13/14/15. With no index, store a
/// negated/positive global box pair instead. Then the long tail.
/// Case E (12): scan `task+0x92` child slots (skipped for count <= 0, signed),
/// testing per-slot masks, probing each child through callees 17/18, recursing
/// in, and updating through callees 19/20/21; a nonzero answer from callee 20
/// ends the scan early. Then the long tail.
///
/// The long tail (cases D, E) runs callee 13 before the common tail.
/// Called thiscall with two stack words; returns the tail answer as u32.
/// Answers used as conditions are tested by low byte (`(an instruction of the original)`) except
/// the -1 sentinels and the child pointers, which compare full words.
lf_checker_rt::export!(thiscall, rw_00d191e0(this: u32, task: u32, a2: u32) -> u32 {
    unsafe {
        const C_FWD: u32 = 1;
        const C_SRC: u32 = 2;
        const C_TAIL: u32 = 3;
        const C_RESOLVE: u32 = 4;
        const C_NEXT: u32 = 6;
        const C_PREP: u32 = 7;
        const C_CONSUME: u32 = 8;
        const C_FIRST: u32 = 9;
        const C_NODE: u32 = 10;
        const C_TRIPLE: u32 = 11;
        const C_SELF: u32 = 12;
        const C_POST: u32 = 13;
        const C_IDX: u32 = 14;
        const C_FLAG: u32 = 15;
        const C_STEP: u32 = 16;
        const C_PROBE: u32 = 17;
        const C_TEST: u32 = 18;
        const C_UPDATE: u32 = 19;
        const C_MAYBE: u32 = 20;
        const C_FIN: u32 = 21;
        const C_CLOSE: u32 = 22;

        const TASK_TYPE: u32 = 0x04;
        const TASK_SCALE: u32 = 0x90;
        const TASK_BIAS: u32 = 0xa0;
        const CHILD_TAB: u32 = 0xb0;
        const CASEB_N: u32 = 0xc8;
        const CASEB_COUNT: u32 = 0xcc;
        const CASEB_SRC: u32 = 0x8c;
        const CASEE_KIDS: u32 = 0x80;
        const CASEE_SRC: u32 = 0x84;
        const CASEE_COUNT: u32 = 0x92;
        const CASEE_MASK: u32 = 0x94;
        const CB_FLAG: u32 = 0x1d30;
        const CB_PTR: u32 = 0x1d34;
        const CB_GATE: u32 = 0x1d38;
        const MASK_LO: u32 = 0x1d40;
        const MASK_HI: u32 = 0x1d44;
        const SCRATCH: u32 = 0x1450;
        const WORD_SEL: u32 = 0x1d24;
        const STAMP: u32 = 0x3c;

        const G_FLAG: u32 = 0x018b99d1;
        const G_BOXC: u32 = 0x0110dad0;
        const G_BOXD: u32 = 0x0110db20;
        const G_NEG: u32 = 0x00fe8fa0;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        /// One scaled lane: float(signed word) * scale + bias, original order.
        #[inline(always)]
        unsafe fn lane(tab: u32, scale: f32, bias: f32) -> f32 {
            unsafe {
                let w = (rd16(tab) as u16) as i16 as f32;
                add(mul(w, scale), bias)
            }
        }
        #[inline(always)]
        fn lo_is_zero(ans: u32) -> bool {
            (ans as u8) == 0
        }

        // Planted callback shared by case C's two poll sites.
        let callback = rd32(this.wrapping_add(CB_PTR));
        let poll = |arg_a2: u32, child: u32| -> u32 {
            let hook: extern "stdcall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(callback as usize) };
            hook(arg_a2, child)
        };

        let tail = || -> u32 {
            lf_checker_rt::callee_thiscall!(C_TAIL, u32, this)
        };

        match rd8(task.wrapping_add(TASK_TYPE)) {
            0 | 1 | 3 => {
                lf_checker_rt::callee_thiscall!(C_FWD, u32, this, task, a2);
                tail()
            }
            4 | 5 => {
                let n = rd32(task.wrapping_add(CASEB_N));
                let mut slots = vec![0u32; (n as usize).wrapping_mul(4)];
                if (n.wrapping_sub(1) as i32) >= 0 {
                    let tab = rd32(task.wrapping_add(CHILD_TAB));
                    let s0 = rdf(task.wrapping_add(TASK_SCALE));
                    let s1 = rdf(task.wrapping_add(TASK_SCALE).wrapping_add(4));
                    let s2 = rdf(task.wrapping_add(TASK_SCALE).wrapping_add(8));
                    let b0 = rdf(task.wrapping_add(TASK_BIAS));
                    let b1 = rdf(task.wrapping_add(TASK_BIAS).wrapping_add(4));
                    let b2 = rdf(task.wrapping_add(TASK_BIAS).wrapping_add(8));
                    for edx in 0..n {
                        let off = edx.wrapping_mul(6);
                        let v2 = lane(tab.wrapping_add(off), s0, b0);
                        let v1 = lane(tab.wrapping_add(off).wrapping_add(2), s1, b1);
                        let v0 = lane(tab.wrapping_add(off).wrapping_add(4), s2, b2);
                        let o = (edx as usize).wrapping_mul(4);
                        slots[o] = v2.to_bits();
                        slots[o + 1] = v1.to_bits();
                        slots[o + 2] = v0.to_bits();
                    }
                }
                let count = rd32(task.wrapping_add(CASEB_COUNT));
                if (count as i32) > 0 {
                    let base = rd32(task.wrapping_add(CASEB_SRC));
                    let mut i = 0u32;
                    let mut off = 0u32;
                    while (i as i32) < (count as i32) {
                        wr16(this.wrapping_add(STAMP), i as u16);
                        lf_checker_rt::callee_thiscall!(
                            C_SRC, u32, this,
                            base.wrapping_add(off),
                            slots.as_ptr() as u32,
                            a2, task
                        );
                        i = i.wrapping_add(1);
                        off = off.wrapping_add(0x20);
                    }
                }
                if rd8(task.wrapping_add(TASK_TYPE)) != 5 {
                    return tail();
                }
                let probe: u32 = lf_checker_rt::callee_thiscall!(C_TAIL, u32, this);
                if !lo_is_zero(probe) {
                    return tail();
                }
                let flag = rd8(lf_checker_rt::relocated(G_FLAG));
                if flag == (probe as u8) {
                    return tail();
                }
                lf_checker_rt::callee_thiscall!(C_FWD, u32, this, task, a2);
                tail()
            }
            6 | 7 | 10 => {
                if rd32(task.wrapping_add(CHILD_TAB)) == 0 {
                    return tail();
                }
                let w = if rd8(this.wrapping_add(WORD_SEL)) != 0 {
                    rd16(a2.wrapping_add(8))
                } else {
                    0xffff_ffff
                };
                let scratch = this.wrapping_add(SCRATCH);
                let mut child: u32 =
                    lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, this, task, scratch, w);
                if child != 0 {
                    loop {
                        if rd8(this.wrapping_add(CB_GATE)) == 0 {
                            break;
                        }
                        if !lo_is_zero(poll(a2, child)) {
                            break;
                        }
                        child = lf_checker_rt::callee_thiscall!(C_NEXT, u32, this, task, scratch);
                        if child == 0 {
                            break;
                        }
                    }
                }
                let mut head = [0u32; 8];
                head[0] = 0x0001_0000;
                head[1] = 0x0003_0002u32;
                let mut frame = [0u32; 8];
                lf_checker_rt::callee_thiscall!(C_PREP, u32, frame.as_mut_ptr() as u32);
                if child == 0 {
                    return tail();
                }
                loop {
                    let tab = rd32(task.wrapping_add(CHILD_TAB));
                    let n =
                        if rd16(child.wrapping_add(0x16)) == 0 { 3u32 } else { 4u32 };
                    let mut out = [0u32; 16];
                    let s0 = rdf(task.wrapping_add(TASK_SCALE));
                    let s1 = rdf(task.wrapping_add(TASK_SCALE).wrapping_add(4));
                    let s2 = rdf(task.wrapping_add(TASK_SCALE).wrapping_add(8));
                    let b0 = rdf(task.wrapping_add(TASK_BIAS));
                    let b1 = rdf(task.wrapping_add(TASK_BIAS).wrapping_add(4));
                    let b2 = rdf(task.wrapping_add(TASK_BIAS).wrapping_add(8));
                    for k in 0..n {
                        let idx = rd16(child.wrapping_add(0x10).wrapping_add(k * 2));
                        let t = idx.wrapping_mul(3);
                        let v2 = lane(tab.wrapping_add(t * 2), s0, b0);
                        let v1 =
                            lane(tab.wrapping_add(t * 2).wrapping_add(2), s1, b1);
                        let v0 =
                            lane(tab.wrapping_add(t * 2).wrapping_add(4), s2, b2);
                        out[(k as usize) * 4] = v2.to_bits();
                        out[(k as usize) * 4 + 1] = v1.to_bits();
                        out[(k as usize) * 4 + 2] = v0.to_bits();
                    }
                    // First 32 bytes of the child, spliced with `2 * n` header
                    // bytes at offset 16 (the original's native memcpy).
                    let mut block = [0u8; 32];
                    for i in 0..32u32 {
                        block[i as usize] = rd8(child.wrapping_add(i));
                    }
                    let hbytes: [u8; 8] = [
                        (head[0] & 0xff) as u8,
                        ((head[0] >> 8) & 0xff) as u8,
                        ((head[0] >> 16) & 0xff) as u8,
                        ((head[0] >> 24) & 0xff) as u8,
                        (head[1] & 0xff) as u8,
                        ((head[1] >> 8) & 0xff) as u8,
                        ((head[1] >> 16) & 0xff) as u8,
                        ((head[1] >> 24) & 0xff) as u8,
                    ];
                    let len = (n as usize) * 2;
                    for i in 0..len {
                        block[16 + i] = hbytes[i];
                    }
                    lf_checker_rt::callee_thiscall!(
                        C_CONSUME, u32, this,
                        block.as_ptr() as u32,
                        out.as_ptr() as u32,
                        a2, task
                    );
                    loop {
                        child = lf_checker_rt::callee_thiscall!(C_NEXT, u32, this, task, scratch);
                        if child == 0 {
                            return tail();
                        }
                        if rd8(this.wrapping_add(CB_GATE)) == 0 {
                            break;
                        }
                        if !lo_is_zero(poll(a2, child)) {
                            break;
                        }
                    }
                }
            }
            8 => {
                let mut idx: u32 = lf_checker_rt::callee_thiscall!(C_FIRST, u32, this, task);
                if idx != 0xffff_ffff {
                    loop {
                        let node: u32 =
                            lf_checker_rt::callee_thiscall!(C_NODE, u32, task, idx);
                        if node != 0 {
                            let mut mins = [0u32; 3];
                            let mut maxs = [0u32; 3];
                            lf_checker_rt::callee_thiscall!(
                                C_TRIPLE, u32, task, idx,
                                mins.as_mut_ptr() as u32,
                                maxs.as_mut_ptr() as u32
                            );
                            let c0 = rdf(lf_checker_rt::relocated(G_BOXC));
                            let c1 = rdf(lf_checker_rt::relocated(G_BOXC).wrapping_add(4));
                            let c2 = rdf(lf_checker_rt::relocated(G_BOXC).wrapping_add(8));
                            let f6 = sub(f32::from_bits(mins[0]), c0);
                            let f5 = sub(f32::from_bits(mins[1]), c1);
                            let f4 = sub(f32::from_bits(mins[2]), c2);
                            let f3 = add(f32::from_bits(maxs[0]), c0);
                            let f2 = add(f32::from_bits(maxs[1]), c1);
                            let f1 = add(f32::from_bits(maxs[2]), c2);
                            wrf(this, f6);
                            wrf(this.wrapping_add(4), f5);
                            wrf(this.wrapping_add(8), f4);
                            mins[0] = f6.to_bits();
                            mins[1] = f5.to_bits();
                            mins[2] = f4.to_bits();
                            maxs[0] = f3.to_bits();
                            maxs[1] = f2.to_bits();
                            maxs[2] = f1.to_bits();
                            wrf(this.wrapping_add(0x10), f3);
                            wrf(this.wrapping_add(0x14), f2);
                            wrf(this.wrapping_add(0x18), f1);
                            let r: u32 = lf_checker_rt::callee_thiscall!(C_SELF, u32, this, node, a2);
                            if !lo_is_zero(r) {
                                lf_checker_rt::callee_thiscall!(C_POST, u32, this);
                                lf_checker_rt::callee_thiscall!(C_IDX, u32, this, idx & 0xffff);
                                lf_checker_rt::callee_thiscall!(C_FLAG, u32, this, 1);
                            }
                        }
                        idx = lf_checker_rt::callee_thiscall!(C_STEP, u32, this, task, idx);
                        if idx == 0xffff_ffff {
                            break;
                        }
                    }
                }
                let mask = rd32(lf_checker_rt::relocated(G_NEG));
                let d0 = rd32(lf_checker_rt::relocated(G_BOXD));
                let d1 = rd32(lf_checker_rt::relocated(G_BOXD).wrapping_add(4));
                let d2 = rd32(lf_checker_rt::relocated(G_BOXD).wrapping_add(8));
                wr32(this, d0 ^ mask);
                wr32(this.wrapping_add(4), d1 ^ mask);
                wr32(this.wrapping_add(8), d2 ^ mask);
                wr32(this.wrapping_add(0x10), d0);
                wr32(this.wrapping_add(0x14), d1);
                wr32(this.wrapping_add(0x18), d2);
                lf_checker_rt::callee_thiscall!(C_CLOSE, u32, this);
                tail()
            }
            12 => {
                let count = rd16(task.wrapping_add(CASEE_COUNT));
                if (count as i32) <= 0 {
                    lf_checker_rt::callee_thiscall!(C_CLOSE, u32, this);
                    return tail();
                }
                let kids = rd32(task.wrapping_add(CASEE_KIDS));
                let src = rd32(task.wrapping_add(CASEE_SRC));
                let masks = rd32(task.wrapping_add(CASEE_MASK));
                let mut k = 0u32;
                let mut step = 0u32;
                let mut acc = count;
                loop {
                    let child = rd32(kids.wrapping_add(k.wrapping_mul(4)));
                    if child != 0 {
                        let mut ok = true;
                        if masks != 0 {
                            let mhi = rd32(masks.wrapping_add(k.wrapping_mul(8)).wrapping_add(4));
                            if mhi & rd32(this.wrapping_add(MASK_HI)) == 0 {
                                ok = false;
                            } else {
                                let mlo = rd32(masks.wrapping_add(k.wrapping_mul(8)));
                                if mlo & rd32(this.wrapping_add(MASK_LO)) == 0 {
                                    ok = false;
                                }
                            }
                        }
                        if ok {
                            let v = src.wrapping_add(step);
                            let t: u32 =
                                lf_checker_rt::callee_thiscall!(C_PROBE, u32, this, v);
                            let _ = t;
                            let p: u32 =
                                lf_checker_rt::callee_thiscall!(C_TEST, u32, this, child);
                            if lo_is_zero(p) {
                                let r: u32 = lf_checker_rt::callee_thiscall!(
                                    C_SELF, u32, this, child, a2
                                );
                                if !lo_is_zero(r) {
                                    lf_checker_rt::callee_thiscall!(C_POST, u32, this);
                                    lf_checker_rt::callee_thiscall!(
                                        C_UPDATE, u32, this, task, k, acc
                                    );
                                    let m: u32 = lf_checker_rt::callee_thiscall!(
                                        C_MAYBE, u32, this, v, 1
                                    );
                                    if !lo_is_zero(m) {
                                        lf_checker_rt::callee_thiscall!(C_CLOSE, u32, this);
                                        return tail();
                                    }
                                }
                            }
                            let _: u32 =
                                lf_checker_rt::callee_thiscall!(C_FIN, u32, this, v);
                            if rd8(child.wrapping_add(TASK_TYPE)) == 0x0c {
                                acc = acc.wrapping_add(rd16(child.wrapping_add(CASEE_COUNT)));
                            }
                        }
                    }
                    step = step.wrapping_add(0x40);
                    k = k.wrapping_add(1);
                    if !((k as i32) < (count as i32)) {
                        break;
                    }
                }
                lf_checker_rt::callee_thiscall!(C_CLOSE, u32, this);
                tail()
            }
            _ => tail(),
        }
    }
});
