// original: 0x00D01D30 CTaskComplexBeArrestedAndDrivenAway::vf20

/// Advance one tick of the be-arrested-and-driven-away task.
///
/// `this` is the task, `ped` the arresting ped (one stack word). Entry marks
/// the ped busy (`+0xA70` = 1) and adds the frame-time global to the timer
/// at `+0x1C`. It then walks the subject's sub-task list (from
/// `[subject+0x224]+0x2E0`, links at `+0x0C`) looking for a node tagged
/// `0x2E8`:
///
/// * Not found: unless flag bit 0 of `+0x0C` is set, the task's own virtual
///   slot 5 (`[[this]+0x14]`, called with `(ped, 1, 0)`) is asked whether
///   to proceed; false returns the stash at `+0x08`, true sets flag bit 1.
///   A null subject then clears the busy mark and returns 0, otherwise a
///   three-callee scratch sequence runs (fill a 4-word local off the
///   subject, consume it, finish it) before clearing the mark and
///   returning 0.
/// * Found, state `+0x18` 1: after a release callee, past 5 seconds a poll
///   callee is consulted (non-zero returns 0 with the mark still set);
///   otherwise the timer resets, a data callee fills two scratch words off
///   a manager object, and two check callees plus an emit callee consume
///   them; any check failing skips the rest.
/// * Found, state 2: the poll callee again (non-zero returns 0); past 10
///   seconds and a predicate callee, three pool allocations build a new
///   task (a lookup feeds a float temporary the original keeps in its own
///   incoming argument slot, a 6-word constructor, an x87-float measure
///   plus pi, a wrap, an attach) whose result is returned. A failed first
///   allocation returns 0.
/// * Found, state 3 with a poll result: two predicate flavours are tried
///   (either true returns the poll result); both false runs virtual slot 0
///   of the poll result with argument 1. Without a poll result, past 35
///   seconds and the predicate, a non-null subject runs the scratch
///   sequence again and returns 0 with the mark cleared.
/// * Tail: states 3 and 4 play a speech line and every state returns the
///   stash at `+0x08`.
///
/// All threshold comparisons are strict. The scratch words passed to
/// callees live in the original's frame (and its dead argument slot); the
/// rewrite keeps them in locals, so the stack comparison is off for this
/// function while every value flowing through the scratch is still compared
/// at the calls. Float operation order is the original's.
///
/// Original: 0x00D01D30 (thiscall, one stack word, u32 return).
lf_checker_rt::export!(thiscall, rw_00d01d30(this: u32, ped: u32) -> u32 {
    unsafe {
        const STASH: u32 = 0x08;
        const FLAGS: u32 = 0x0c;
        const SUBJECT: u32 = 0x14;
        const STATE: u32 = 0x18;
        const TIMER: u32 = 0x1c;
        const VSLOT: u32 = 0x14;
        const PED_BUSY: u32 = 0xa70;
        const PED_SLOT20: u32 = 0x20;
        const PED_LOOKUP_BASE: u32 = 0x78;
        const PED_USE_BASE: u32 = 0x224;
        const SUB_INNER: u32 = 0x224;
        const INNER_HEAD: u32 = 0x2e0;
        const NODE_KIND: u32 = 0x04;
        const NODE_FLAGS: u32 = 0x08;
        const NODE_NEXT: u32 = 0x0c;
        const KIND_WANTED: u32 = 0x2e8;
        const USE_ADJ: u32 = 0x84;
        const G_FRAME_TIME: u32 = 0x0117_35bc;
        const G_DWORD: u32 = 0x012b_4138;
        const G_TASK_MGR: u32 = 0x0167_e2a0;
        const MGR2: u32 = 0x016d_ceb8;
        const LINE_NOTHING: u32 = 0x000e_df570;
        const SAY_ONE: u32 = 0x3d4c_cccd; // 0.05
        const MEAS_ONE: u32 = 0x3dcc_cccd; // 0.1
        const BUILD_W: u32 = 0x447a_0000; // 1000.0
        const ONE_F: u32 = 0x3f80_0000;
        const LOOKUP_ID: u32 = 0x126;
        const BUILD_ID: u32 = 0x16;
        const FIVE_S: f32 = 5.0;
        const TEN_S: f32 = 10.0;
        const LONG_S: f32 = 35.0;
        const PI_F: f32 = f32::from_bits(0x4049_0fdb);
        const D_VPROCEED: u32 = 1;
        const D_FILL: u32 = 2;
        const D_USE: u32 = 3;
        const D_FINISH: u32 = 4;
        const D_RELEASE2: u32 = 5;
        const D_POLL1: u32 = 6;
        const D_MKDATA: u32 = 7;
        const D_CHECK1: u32 = 8;
        const D_CHECK2: u32 = 9;
        const D_EMIT: u32 = 10;
        const D_SAY: u32 = 11;
        const D_POLL2: u32 = 12;
        const D_TRY2: u32 = 13;
        const D_LOOKUP: u32 = 14;
        const D_ALLOC1: u32 = 15;
        const D_ALLOC2: u32 = 16;
        const D_BUILD: u32 = 17;
        const D_ALLOC3: u32 = 18;
        const D_MEASURE: u32 = 19;
        const D_WRAP: u32 = 20;
        const D_ATTACH: u32 = 21;
        const D_POLL3: u32 = 22;
        const D_TRY3A: u32 = 23;
        const D_TRY3B: u32 = 24;
        const D_VFINISH: u32 = 25;
        const D_TRY3C: u32 = 26;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// The three-callee scratch sequence over one 4-word scratch area.
        #[inline(always)]
        unsafe fn scratch_seq(subject: u32, ped: u32) {
            unsafe {
                let mut loc = [0u32; 4];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    D_FILL, u32, loc.as_mut_ptr() as u32, subject
                );
                let base = rd32(ped.wrapping_add(PED_USE_BASE));
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    D_USE, u32, base.wrapping_add(USE_ADJ),
                    loc.as_mut_ptr() as u32, 0, 1
                );
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(D_FINISH, u32, loc.as_mut_ptr() as u32);
            }
        }
        #[inline(always)]
        unsafe fn tail(this: u32) -> u32 {
            unsafe {
                let st = rd32(this.wrapping_add(STATE));
                if st == 3 || st == 4 {
                    let s = rd32(this.wrapping_add(SUBJECT));
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        D_SAY, u32, s, lf_checker_rt::relocated(LINE_NOTHING),
                        SAY_ONE, 0, 0
                    );
                }
                rd32(this.wrapping_add(STASH))
            }
        }

        wr32(ped.wrapping_add(PED_BUSY), 1);
        let g = f32::from_bits(
            (lf_checker_rt::global::<u32>(G_FRAME_TIME) as *const u32).read_unaligned(),
        );
        wrf(
            this.wrapping_add(TIMER),
            add(rdf(this.wrapping_add(TIMER)), g),
        );

        let subject = rd32(this.wrapping_add(SUBJECT));
        let mut found = false;
        if subject != 0 {
            let inner = rd32(subject.wrapping_add(SUB_INNER));
            let mut node = rd32(inner.wrapping_add(INNER_HEAD));
            while node != 0 {
                // The original loads the flag word twice for a comparison
                // whose result is dead; the reads still happen, so keep them.
                let f1 = rd32(node.wrapping_add(NODE_FLAGS));
                let f2 = rd32(node.wrapping_add(NODE_FLAGS));
                core::hint::black_box((f1, f2));
                if rd32(node.wrapping_add(NODE_KIND)) == KIND_WANTED {
                    found = true;
                    break;
                }
                node = rd32(node.wrapping_add(NODE_NEXT));
            }
        }

        if !found {
            if rd8(this.wrapping_add(FLAGS)) & 1 == 0 {
                let slot: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(VSLOT)) as usize);
                if (slot(this, ped, 1, 0) & 0xff) == 0 {
                    return rd32(this.wrapping_add(STASH));
                }
                wr32(
                    this.wrapping_add(FLAGS),
                    rd32(this.wrapping_add(FLAGS)) | 2,
                );
            }
            let s2 = rd32(this.wrapping_add(SUBJECT));
            if s2 == 0 {
                wr32(ped.wrapping_add(PED_BUSY), 0);
                return 0;
            }
            scratch_seq(s2, ped);
            wr32(ped.wrapping_add(PED_BUSY), 0);
            return 0;
        }

        match rd32(this.wrapping_add(STATE)) {
            1 => {
                let arg = rd32(subject.wrapping_add(PED_SLOT20)).wrapping_add(0x30);
                let _: u32 = lf_checker_rt::callee_thiscall!(D_RELEASE2, u32, ped, arg);
                if rdf(this.wrapping_add(TIMER)) > FIVE_S {
                    let r: u32 = lf_checker_rt::callee_thiscall!(D_POLL1, u32, this, ped);
                    if r != 0 {
                        return 0;
                    }
                    wrf(this.wrapping_add(TIMER), 0.0);
                    wr32(this.wrapping_add(STATE), 2);
                    let mut la = 0u32;
                    let mut lb = 0u32;
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        D_MKDATA, u32, lf_checker_rt::relocated(MGR2),
                        (&mut la as *mut u32) as u32, (&mut lb as *mut u32) as u32
                    );
                    let gd = (lf_checker_rt::global::<u32>(G_DWORD) as *const u32)
                        .read_unaligned();
                    let r1: u32 = lf_checker_rt::callee_cdecl!(D_CHECK1, u32, la, gd,);
                    if (r1 & 0xff) == 0 {
                        return tail(this);
                    }
                    let r2: u32 = lf_checker_rt::callee_cdecl!(D_CHECK2, u32, lb, gd,);
                    if (r2 & 0xff) == 0 {
                        return tail(this);
                    }
                    let e2 = rd32(ped.wrapping_add(PED_SLOT20)).wrapping_add(0x30);
                    let _: u32 = lf_checker_rt::callee_cdecl!(D_EMIT, u32, lb, e2, 0,);
                }
            }
            2 => {
                let r: u32 = lf_checker_rt::callee_thiscall!(D_POLL2, u32, this, ped);
                if r != 0 {
                    return 0;
                }
                if rdf(this.wrapping_add(TIMER)) > TEN_S {
                    let ok: u32 =
                        lf_checker_rt::callee_thiscall!(D_TRY2, u32, this, ped, 1, 0);
                    if (ok & 0xff) != 0 {
                        wr32(ped.wrapping_add(PED_BUSY), 1);
                        wr32(this.wrapping_add(STATE), 3);
                        let base78 = rd32(ped.wrapping_add(PED_LOOKUP_BASE));
                        let p: u32 =
                            lf_checker_rt::callee_thiscall!(D_LOOKUP, u32, base78, LOOKUP_ID);
                        // The original keeps this temporary in its incoming
                        // argument slot; a local holds the same value.
                        let mut temp = 0.0f32;
                        if p != 0 {
                            temp = f32::from_bits(rd32(p.wrapping_add(0x4c)));
                        }
                        let mgr = (lf_checker_rt::global::<u32>(G_TASK_MGR) as *const u32)
                            .read_unaligned();
                        let pool1: u32 =
                            lf_checker_rt::callee_thiscall!(D_ALLOC1, u32, mgr);
                        if pool1 == 0 {
                            return 0;
                        }
                        let pool2: u32 =
                            lf_checker_rt::callee_thiscall!(D_ALLOC2, u32, mgr);
                        let built = if pool2 == 0 {
                            0
                        } else {
                            lf_checker_rt::callee_thiscall!(
                                D_BUILD, u32, pool2, BUILD_ID, LOOKUP_ID, BUILD_W, 1,
                                ONE_F, temp.to_bits(),
                            )
                        };
                        let pool3: u32 =
                            lf_checker_rt::callee_thiscall!(D_ALLOC3, u32, mgr);
                        let wrapped = if pool3 == 0 {
                            0
                        } else {
                            let f: f32 = lf_checker_rt::callee_thiscall!(
                                D_MEASURE, f32, ped, 1, MEAS_ONE, 1, 0
                            );
                            lf_checker_rt::callee_thiscall!(
                                D_WRAP, u32, pool3, 2, add(f, PI_F).to_bits(),
                            )
                        };
                        let out: u32 = lf_checker_rt::callee_thiscall!(
                            D_ATTACH, u32, pool1, wrapped, built, 0, 0
                        );
                        return out;
                    }
                }
            }
            3 => {
                let b: u32 = lf_checker_rt::callee_thiscall!(D_POLL3, u32, this, ped);
                if b != 0 {
                    let o1: u32 =
                        lf_checker_rt::callee_thiscall!(D_TRY3A, u32, this, ped, 1, 0);
                    if (o1 & 0xff) != 0 {
                        return b;
                    }
                    let o2: u32 =
                        lf_checker_rt::callee_thiscall!(D_TRY3B, u32, this, ped, 2, 0);
                    if (o2 & 0xff) != 0 {
                        return b;
                    }
                    // Virtual slot 0 of the poll result: [[b]+0].
                    let fin: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rd32(rd32(b)) as usize);
                    let _: u32 = fin(b, 1);
                } else if rdf(this.wrapping_add(TIMER)) > LONG_S {
                    let o3: u32 =
                        lf_checker_rt::callee_thiscall!(D_TRY3C, u32, this, ped, 1, 0);
                    if (o3 & 0xff) != 0 {
                        let s3 = rd32(this.wrapping_add(SUBJECT));
                        if s3 != 0 {
                            scratch_seq(s3, ped);
                            wr32(ped.wrapping_add(PED_BUSY), 0);
                            return 0;
                        }
                    }
                }
            }
            _ => {}
        }
        tail(this)
    }
});
