// original: 0x00622360 rage::rlFireAndForgetTask<rage::snDropGamersTask>::snDropGamersTask> (symbols)

/// Session drop-gamers task setup: when the session is live and the id
/// pair matches, check every gamer slot and dispatch the drop; otherwise
/// take the mismatch path (stage 2, not in this file yet).
///
/// `this` is the session manager, `arg1` the request (null ends
/// immediately), `arg2` the gamer list (count at `+0x10c`, entries of 8
/// bytes from `+8`). When `this+0x50` is 2 or 3 (compared SIGNED with
/// `jl`/`jg`; both miss directions land on the same path, so no wrong
/// version can distinguish the signedness here) and the id words at
/// `this+0xbf0`/`+0xbf4` equal those at `+0xc30`/`+0xc34`, each entry is
/// checked by callee 1 (`0x61f090`, thiscall/1 with `arg1`); the count of
/// successes must equal the entry count (compared for equality, no
/// signedness), else the function ends. The entry count gate itself is
/// SIGNED (`jle`): 0 skips the loop and continues, negatives end.
///
/// On success the frame scratch `[+0x38, +0x228)` is zeroed, callee 2
/// (`0x6211c0`, thiscall/4) copies address blocks into it, and callee 3
/// (`0x623880`, thiscall/7) dispatches. Both take the scratch at `+0x38`
/// through push-shifted `lea`s. Callee 3's fifth stack word is whatever
/// the callee-2 stub left in ecx (clobbered scratch, compared nowhere).
/// The security-cookie computation is frame scratch feeding only the
/// stubbed check (id 4) and is not modelled. Both direct callers ignore
/// eax, so the contract compares no return channel.
///
/// Otherwise (state outside 2/3, or the id words differing) the
/// request's own id pair (at `arg1+0x40`/`+0x44`) is compared against the
/// wanted pair; when it differs too, the mismatch tail (stage 2a) checks
/// entries until the first success and reports it through callee 19
/// (`0x620b80`, thiscall/2). When the request pair matches, the match
/// side (stage 2b) scans entries with callee 5 (`0x622d90`, thiscall/1)
/// for a find hit. A hit is vetted by callee 6 (thiscall/0, byte answer):
/// zero drops to chain B, nonzero collects address blocks (callee 7,
/// thiscall/2), runs a first-megabyte helper (callee 8, thiscall/2),
/// compacts the negatively-marked 16-byte blocks (callee 9 classifies,
/// SIGNED gate), walks node chain 2 (callee 13 per live node) and
/// dispatches when the kept count is nonzero. With no hit, chain A walks
/// the nodes (callee 11 per live node, byte answer) until a live answer
/// or the end; a live answer continues to chain B, which walks the list
/// again (callee 12), reads the allocator gate globals, allocates a task
/// (planted slot 8), constructs it (callee 15, thiscall/0), builds it
/// (callee 16, fastcall/6 without callee cleanup) and publishes through
/// callee 17 or 18 by the build's byte answer. The `+0x32d4` node lists,
/// the two gate globals and the task object are fabricated by the
/// contract.
///
/// Original: 0x00622360 (thiscall, ecx = this, two stack words).
lf_checker_rt::export!(thiscall, rw_00622360(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CB_CHECK: u32 = 1;
        const CB_COPY: u32 = 2;
        const CB_DISPATCH: u32 = 3;
        const CB_COOKIE: u32 = 4;
        const CB_FIND: u32 = 5;
        const CB_HITTEST: u32 = 6;
        const CB_COLLECT: u32 = 7;
        const CB_RUN: u32 = 8;
        const CB_CLASSIFY: u32 = 9;
        const CB_LIVE: u32 = 11;
        const CB_LIVE2: u32 = 12;
        const CB_TOUCH: u32 = 13;
        const CB_CTOR: u32 = 15;
        const CB_BUILD: u32 = 16;
        const CB_PUB_A: u32 = 17;
        const CB_PUB_B: u32 = 18;
        const CB_TAIL: u32 = 19;
        const VT_SLOT: u32 = 0x28;
        const MAGIC: u32 = 0x1bb66b8;

        const STATE_OFF: u32 = 0x50;
        const ID0_OFF: u32 = 0xbf0;
        const ID1_OFF: u32 = 0xbf4;
        const WANT0_OFF: u32 = 0xc30;
        const WANT1_OFF: u32 = 0xc34;
        const COUNT_OFF: u32 = 0x10c;
        const ENTRIES_OFF: u32 = 0x08;
        const ENTRY_STRIDE: u32 = 8;
        const EXTRA_OFF: u32 = 0x108;

        const F_ARG2: u32 = 0x14;
        const F_ARG1: u32 = 0x1c;
        const F_THIS: u32 = 0x20;
        const F_COUNT: u32 = 0x18;
        const F_SCRATCH: u32 = 0x38;
        const F_ZERO_BASE: u32 = 0x41;
        const F_ZERO_ITERS: i32 = 31;
        const F_FLAG: u32 = 0x13;
        const F_GATE: u32 = 0x18;
        const F_MARKER: u32 = 0x24;
        const F_BLKPTR: u32 = 0x28;
        const F_BLK0: u32 = 0x2c;
        const F_BLK1: u32 = 0x30;
        const F_SAVEDNB: u32 = 0x34;
        const CHAIN_OFF: u32 = 0x32d4;
        const CHAIN_FLAG: u32 = 0x0c;
        const CHAIN_NEXT: u32 = 0x64;
        const EVENT_RUN_OFF: u32 = 0x32c4;
        const GATE_FLAG: u32 = 0x18b8309;
        const GATE_PTR: u32 = 0x19f3a10;
        const TASK_SIZE: u32 = 0x3a8;
        const TASK_VT: u32 = 0xfe1f44;
        const TASK_REC: u32 = 0x1c;
        const TASK_BUF: u32 = 0x3a0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr64(a: u32, v: u64) {
            unsafe { (a as *mut u64).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn vcall28(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT)) as usize);
                f(obj)
            }
        }

        let mut frame = [0u8; 0x244];
        let fb = frame.as_mut_ptr() as u32;
        // Chain B: full node walk, then the allocator gate and task build.
        // Every path through it ends the function.
        let chain_b = || unsafe {
            let mut nd = rd32(this.wrapping_add(CHAIN_OFF));
            if nd != 0 {
                loop {
                    if rd32(nd.wrapping_add(CHAIN_FLAG)) != 2 {
                        if vcall28(nd) == MAGIC {
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                CB_LIVE2,
                                u32,
                                nd,
                                arg2.wrapping_add(ENTRIES_OFF),
                                rd32(arg2.wrapping_add(COUNT_OFF))
                            );
                        }
                    }
                    nd = rd32(nd.wrapping_add(CHAIN_NEXT));
                    if nd == 0 {
                        break;
                    }
                }
            }
            let flag = rd8(lf_checker_rt::global::<u8>(GATE_FLAG) as u32);
            let gate: u32 = if flag == 0 { 0 } else { GATE_PTR };
            let obj = rd32(gate);
            wr32(fb.wrapping_add(F_GATE), gate);
            let vt2 = rd32(obj);
            let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt2.wrapping_add(8)) as usize);
            let task = alloc(obj, TASK_SIZE, 0, 0);
            if task == 0 {
                return;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(CB_CTOR, u32, task);
            let saved = rd32(fb.wrapping_add(F_GATE));
            wr32(task, TASK_VT);
            wr32(task.wrapping_add(TASK_BUF), 0);
            wr32(task.wrapping_add(TASK_BUF).wrapping_add(4), 0);
            wr32(task.wrapping_add(TASK_REC), saved);
            let g: u32 = lf_checker_rt::callee_fastcall!(
                CB_BUILD,
                u32,
                task,
                this,
                rd32(arg1.wrapping_add(0x40)),
                rd32(arg1.wrapping_add(0x44)),
                arg2.wrapping_add(ENTRIES_OFF),
                rd32(arg2.wrapping_add(COUNT_OFF)),
                rd32(arg2.wrapping_add(EXTRA_OFF)),
                task.wrapping_add(TASK_BUF)
            );
            // The publish calls re-read the same gate flag.
            let flag2 = rd8(lf_checker_rt::global::<u8>(GATE_FLAG) as u32);
            let s2: u32 = if flag2 == 0 { 0 } else { GATE_PTR };
            if (g & 0xff) != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(CB_PUB_A, u32, s2, 0, task);
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(CB_PUB_B, u32, s2, task);
            }
        };
        wr32(fb.wrapping_add(F_THIS), this);
        wr32(fb.wrapping_add(F_ARG1), arg1);
        wr32(fb.wrapping_add(F_ARG2), arg2);
        if arg1 == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
            return 0;
        }
        // Signed state gate; both misses share the path-2 target.
        let state = rd32(this.wrapping_add(STATE_OFF)) as i32;
        if state < 2
            || state > 3
            || rd32(this.wrapping_add(ID0_OFF)) != rd32(this.wrapping_add(WANT0_OFF))
            || rd32(this.wrapping_add(ID1_OFF)) != rd32(this.wrapping_add(WANT1_OFF))
        {
            // Path 2: compare the request's id pair, then the mismatch tail
            // (stage 2a). The match side is stage 2b.
            let d = rd32(arg1.wrapping_add(0x40));
            let a = rd32(arg1.wrapping_add(0x44));
            wr32(fb.wrapping_add(F_ARG2), d);
            wr32(fb.wrapping_add(F_COUNT), a);
            if d != rd32(this.wrapping_add(WANT0_OFF))
                || a != rd32(this.wrapping_add(WANT1_OFF))
            {
                let n = rd32(arg2.wrapping_add(COUNT_OFF));
                if (n as i32) > 0 {
                    let mut e = arg2.wrapping_add(ENTRIES_OFF);
                    let mut i = 0u32;
                    loop {
                        let r: u32 =
                            lf_checker_rt::callee_thiscall!(CB_CHECK, u32, arg1, e);
                        if r != 0 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                CB_TAIL,
                                u32,
                                this,
                                rd32(fb.wrapping_add(F_ARG2)),
                                rd32(fb.wrapping_add(F_COUNT))
                            );
                            break;
                        }
                        i += 1;
                        if i >= n {
                            break;
                        }
                        e = e.wrapping_add(ENTRY_STRIDE);
                    }
                }
                let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                return 0;
            }
            // Match side (stage 2b): scan entries for a find hit.
            wr8(fb.wrapping_add(F_FLAG), 0);
            let mut hit = 0u32;
            if (rd32(arg2.wrapping_add(COUNT_OFF)) as i32) > 0 {
                let mut bp = arg2.wrapping_add(ENTRIES_OFF);
                let mut si = 0u32;
                loop {
                    let f: u32 = lf_checker_rt::callee_thiscall!(CB_FIND, u32, this, bp);
                    if f != 0 {
                        hit = f;
                        break;
                    }
                    si += 1;
                    // Signed bound, count re-read like the original.
                    if (si as i32) >= (rd32(arg2.wrapping_add(COUNT_OFF)) as i32) {
                        break;
                    }
                    bp = bp.wrapping_add(ENTRY_STRIDE);
                }
            }
            let mut al = rd8(fb.wrapping_add(F_FLAG));
            if hit != 0 {
                let h: u32 = lf_checker_rt::callee_thiscall!(CB_HITTEST, u32, hit);
                if (h & 0xff) == 0 {
                    chain_b();
                    let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                    return 0;
                }
                let mut z = F_ZERO_BASE;
                let mut cc = F_ZERO_ITERS;
                loop {
                    cc -= 1;
                    wr64(fb.wrapping_add(z).wrapping_sub(9), 0);
                    wr64(fb.wrapping_add(z).wrapping_sub(1), 0);
                    wr16(fb.wrapping_add(z).wrapping_sub(1), 0);
                    z = z.wrapping_add(0x10);
                    if cc < 0 {
                        break;
                    }
                }
                let nb: u32 = lf_checker_rt::callee_thiscall!(
                    CB_COLLECT,
                    u32,
                    this,
                    fb.wrapping_add(F_SCRATCH),
                    0xFFFF_FFFF
                );
                // Push-shifted block: the marker word, a self pointer and
                // two zero words around it, plus the saved block count.
                wr32(fb.wrapping_add(F_BLK0), 0);
                wr32(fb.wrapping_add(F_BLK1), 0);
                wr32(fb.wrapping_add(F_BLKPTR), fb.wrapping_add(F_MARKER));
                wr32(fb.wrapping_add(F_MARKER), 0xfe22f8);
                wr32(fb.wrapping_add(F_SAVEDNB), nb);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CB_RUN,
                    u32,
                    this.wrapping_add(EVENT_RUN_OFF),
                    this,
                    fb.wrapping_add(F_MARKER)
                );
                if (nb as i32) > 0 {
                    let mut di = fb.wrapping_add(F_SCRATCH);
                    let mut si2 = fb.wrapping_add(F_SCRATCH);
                    let mut kept = 0u32;
                    let mut left = nb;
                    loop {
                        let c: u32 =
                            lf_checker_rt::callee_thiscall!(CB_CLASSIFY, u32, si2);
                        // Signed: jns keeps negative-marked blocks.
                        if (c as i32) < 0 {
                            wr64(di, rd64(si2));
                            wr64(di.wrapping_add(8), rd64(si2.wrapping_add(8)));
                            kept += 1;
                            di = di.wrapping_add(0x10);
                        }
                        si2 = si2.wrapping_add(0x10);
                        left = left.wrapping_sub(1);
                        if left == 0 {
                            break;
                        }
                    }
                    wr32(fb.wrapping_add(F_ARG2), kept);
                }
                // Chain 2 over the same node list, then dispatch when the
                // kept count (or the stale id save) is nonzero.
                let mut nd2 = rd32(this.wrapping_add(CHAIN_OFF));
                if nd2 != 0 {
                    loop {
                        if rd32(nd2.wrapping_add(CHAIN_FLAG)) != 2 {
                            if vcall28(nd2) == MAGIC {
                                let _: u32 =
                                    lf_checker_rt::callee_thiscall!(CB_TOUCH, u32, nd2);
                            }
                        }
                        nd2 = rd32(nd2.wrapping_add(CHAIN_NEXT));
                        if nd2 == 0 {
                            break;
                        }
                    }
                }
                let kept2 = rd32(fb.wrapping_add(F_ARG2));
                if kept2 == 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                    return 0;
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CB_DISPATCH,
                    u32,
                    this,
                    arg1,
                    fb.wrapping_add(F_SCRATCH),
                    kept2,
                    0,
                    0,
                    0xFFFF_FFFF,
                    0
                );
                let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                return 0;
            }
            // Chain A: no find hit; walk nodes until a live answer.
            let mut nd = rd32(this.wrapping_add(CHAIN_OFF));
            if nd == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                return 0;
            }
            let mut to_b = false;
            loop {
                if al != 0 {
                    to_b = true;
                    break;
                }
                if rd32(nd.wrapping_add(CHAIN_FLAG)) != 2 {
                    if vcall28(nd) == MAGIC {
                        let r: u32 = lf_checker_rt::callee_thiscall!(
                            CB_LIVE,
                            u32,
                            nd,
                            arg2.wrapping_add(ENTRIES_OFF),
                            rd32(arg2.wrapping_add(COUNT_OFF))
                        );
                        al = (r & 0xff) as u8;
                        wr8(fb.wrapping_add(F_FLAG), al);
                    } else {
                        al = rd8(fb.wrapping_add(F_FLAG));
                    }
                }
                nd = rd32(nd.wrapping_add(CHAIN_NEXT));
                if nd == 0 {
                    break;
                }
            }
            if !to_b && al == 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
                return 0;
            }
            chain_b();
            let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
            return 0;
        }
        let count = rd32(arg2.wrapping_add(COUNT_OFF));
        wr32(fb.wrapping_add(F_COUNT), count);
        let mut ok = 0u32;
        // Signed count gate: jle skips the loop for 0 and negatives.
        if (count as i32) > 0 {
            let mut e = arg2.wrapping_add(ENTRIES_OFF);
            let mut left = count;
            loop {
                let r: u32 = lf_checker_rt::callee_thiscall!(CB_CHECK, u32, arg1, e);
                if r != 0 {
                    ok += 1;
                }
                e = e.wrapping_add(ENTRY_STRIDE);
                left = left.wrapping_sub(1);
                if left == 0 {
                    break;
                }
            }
        }
        if count != ok {
            let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
            return 0;
        }
        let mut z = F_ZERO_BASE;
        let mut cc = F_ZERO_ITERS;
        loop {
            cc -= 1;
            wr64(fb.wrapping_add(z).wrapping_sub(9), 0);
            wr64(fb.wrapping_add(z).wrapping_sub(1), 0);
            wr16(fb.wrapping_add(z).wrapping_sub(1), 0);
            z = z.wrapping_add(0x10);
            if cc < 0 {
                break;
            }
        }
        // idx0 is the last pushed word: entries, count, scratch, -1.
        let copied: u32 = lf_checker_rt::callee_thiscall!(
            CB_COPY,
            u32,
            this,
            arg2.wrapping_add(ENTRIES_OFF),
            count,
            fb.wrapping_add(F_SCRATCH),
            0xFFFF_FFFF
        );
        // idx0..idx6: arg1, scratch, copied, 0, ecx-residue, extra, 0.
        // idx4 is the copy call's clobbered ecx; the contract skips it.
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CB_DISPATCH,
            u32,
            this,
            arg1,
            fb.wrapping_add(F_SCRATCH),
            copied,
            0,
            0,
            rd32(arg2.wrapping_add(EXTRA_OFF)),
            0
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(CB_COOKIE, u32,);
        0
    }
});
