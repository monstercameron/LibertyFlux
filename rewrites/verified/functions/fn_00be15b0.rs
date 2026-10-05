// original: 0x00be15b0 CTaskComplexReact::vf20

/// Poll a reaction task: refresh geometry, dispatch on the owner state.
///
/// `this` is the task object and `arg0` the subject record. Returns eax:
/// the owner pointer, or 0 on the short accept path.
///
/// Behaviour: the entry geometry (four words selected by the anchor word)
/// is copied into the task. A virtual poll of the owner dispatches: an
/// out-of-range answer exits; the middle answer runs the local branch and
/// the dispatch answer the remote branch. The local branch with no helper
/// either keeps or probes-and-marks the owner flag and then accepts
/// through the notify callee; with a helper it ticks a countdown, gates on
/// the helper and subject flags, scales the probe counter against its
/// constant and releases through the finish callee. The remote branch
/// notifies, then either retires early through a countdown comparison or
/// marks the owner, ticks the countdown to a computed value and sets the
/// done bit. The countdown subtracts the global step; a positive
/// remainder skips the compare section (local) or exits (remote), while
///
/// a spent (non-positive) remainder runs it, through unsigned jumps the
/// rewrite mirrors exactly.
/// Original: 0x00be15b0 (thiscall, one stack word; returns eax).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00be15b0(this: u32, arg0: u32, mutate_short: bool) -> u32 {
    unsafe {
        const POLL_CALLEE: u32 = 1;
        const PROBE_CALLEE: u32 = 2;
        const NOTIFY_CALLEE: u32 = 3;
        const COUNTER_CALLEE: u32 = 4;
        const FINISH_CALLEE: u32 = 5;
        const REMOTE_CALLEE: u32 = 6;
        const ENABLE_CALLEE: u32 = 7;
        const TICK_CALLEE: u32 = 8;
        const G_DT: u32 = 0x11735bc;
        const G_CMP: u32 = 0xe9d785;
        const G_ADD: u32 = 0xfe870c;
        const G_GATE: u32 = 0x10475d0;
        const G_K: u32 = 0xfe8684;
        const G_K2: u32 = 0xfe8a24;
        const G_ONE: u32 = 0xfe88e8;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
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
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let tgt: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                tgt(obj, a0, a1, a2)
            }
        }

        let owner = rd32(this.wrapping_add(8));
        let anch = rd32(this.wrapping_add(0x3c));
        if anch != 0 {
            let g = rd32(anch.wrapping_add(0x20));
            let p = if g != 0 {
                g.wrapping_add(0x30)
            } else {
                anch.wrapping_add(0x10)
            };
            wr32(this.wrapping_add(0x20), rd32(p));
            wr32(this.wrapping_add(0x24), rd32(p.wrapping_add(4)));
            wr32(this.wrapping_add(0x28), rd32(p.wrapping_add(8)));
            wr32(this.wrapping_add(0x2c), rd32(p.wrapping_add(0xc)));
        }
        let ans = vcall0(owner, 0xc);
        if ans.wrapping_sub(0xde) == 0 {
            // Remote branch.
            let _: u32 = lf_checker_rt::callee_thiscall!(
                REMOTE_CALLEE,
                u32,
                arg0,
                2.0f32.to_bits()
            );
            let q = rd32(owner.wrapping_add(0x14));
            if q == 0 {
                return rd32(this.wrapping_add(8));
            }
            let x0 = rdf(this.wrapping_add(0x40));
            if x0 > 0.0 {
                let x0 = sub(x0, gf(G_DT));
                wr32(this.wrapping_add(0x40), x0.to_bits());
                // `jb` after comparing 0 against the remainder: only a
                // non-positive remainder continues; positive exits.
                if !(x0 <= 0.0) {
                    return rd32(this.wrapping_add(8));
                }
                let w = rd32(arg0.wrapping_add(0x224));
                let al = rd8(w.wrapping_add(0x2c8));
                if (al as i8) <= (g8(G_CMP) as i8) {
                    let qw = rd32(q.wrapping_add(4));
                    if (qw >> 6) & 1 == 0 {
                        return rd32(this.wrapping_add(8));
                    }
                    wr32(q.wrapping_add(4), qw & 0xffffffbf);
                } else {
                    wr32(this.wrapping_add(0x40), 0x3c23d70a);
                }
                return rd32(this.wrapping_add(8));
            }
            if rd8(this.wrapping_add(0x44)) != 0 {
                return rd32(this.wrapping_add(8));
            }
            wr8(this.wrapping_add(0x44), 1);
            let en: u32 = lf_checker_rt::callee_thiscall!(
                ENABLE_CALLEE,
                u32,
                q,
                0x80u32
            );
            if en as u8 == 0 {
                return rd32(this.wrapping_add(8));
            }
            // The [0, 1.0] pair below the frame is two more stack words
            // of this call, which cleans all four itself.
            // (&arg0 stands in for the incoming slot address; the pointed-to
            // word is what the contract compares.)
            let tick: u32 = lf_checker_rt::callee_thiscall!(
                TICK_CALLEE,
                u32,
                q,
                1u32,
                &arg0 as *const u32 as u32,
                0u32,
                1.0f32.to_bits()
            );
            if tick as u8 == 0 {
                return rd32(this.wrapping_add(8));
            }
            let n: u32 = lf_checker_rt::callee_cdecl!(COUNTER_CALLEE, u32,);
            let mut x0 = (n as i32) as f32;
            x0 = mul(x0, gf(G_K));
            x0 = mul(x0, gf(G_K2));
            x0 = add(x0, gf(G_ONE));
            wr32(this.wrapping_add(0x40), x0.to_bits());
            wr32(q.wrapping_add(4), rd32(q.wrapping_add(4)) | 0x40);
            return rd32(this.wrapping_add(8));
        }
        if ans.wrapping_sub(0xde).wrapping_sub(0x3b) != 0 {
            return rd32(this.wrapping_add(8));
        }
        // Local branch.
        let h = rd32(this.wrapping_add(0x38));
        if h == 0 {
            // MUTANT (mut_00be15b0): the short path returns the owner.
            if rd8(owner.wrapping_add(0xc)) & 1 == 0 {
                let pb: u32 = vcall3(owner, 0x14, arg0, 2, 0);
                if pb as u8 == 0 {
                    return rd32(this.wrapping_add(8));
                }
                wr8(owner.wrapping_add(0xc), rd8(owner.wrapping_add(0xc)) | 2);
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, arg0);
            return if mutate_short { rd32(this.wrapping_add(8)) } else { 0 };
        }
        if (rd32(h.wrapping_add(4)) >> 6) & 1 != 0 {
            let x0 = rdf(this.wrapping_add(0x40));
            if x0 > 0.0 {
                let x0 = sub(x0, gf(G_DT));
                wr32(this.wrapping_add(0x40), x0.to_bits());
                // Only a spent remainder reaches the compare section.
                if x0 <= 0.0 {
                    let w = rd32(arg0.wrapping_add(0x224));
                    let al = rd8(w.wrapping_add(0x2c8));
                    if (al as i8) <= (g8(G_CMP) as i8) {
                        let hw = rd32(h.wrapping_add(4));
                        if (hw >> 6) & 1 != 0 {
                            wr32(h.wrapping_add(4), hw & 0xffffffbf);
                        }
                    } else {
                        let x0 = add(x0, gf(G_ADD));
                        wr32(this.wrapping_add(0x40), x0.to_bits());
                    }
                }
            }
        }
        if rd32(h.wrapping_add(0x74)) & 0x4000 == 0 {
            return rd32(this.wrapping_add(8));
        }
        if rd8(arg0.wrapping_add(0x26c)) & 4 != 0 {
            return rd32(this.wrapping_add(8));
        }
        let n: u32 = lf_checker_rt::callee_cdecl!(COUNTER_CALLEE, u32,);
        let mut x1 = (n as i32) as f32;
        x1 = mul(x1, gf(G_K));
        if !(gf(G_GATE) > x1) {
            return rd32(this.wrapping_add(8));
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            FINISH_CALLEE,
            u32,
            h,
            (-4.0f32).to_bits()
        );
        rd32(this.wrapping_add(8))
    }
}

lf_checker_rt::export!(thiscall, rw_00be15b0(this: u32, arg0: u32) -> u32 {
    unsafe { run_00be15b0(this, arg0, false) }
});

lf_checker_rt::export!(thiscall, mut_00be15b0(this: u32, arg0: u32) -> u32 {
    unsafe { run_00be15b0(this, arg0, true) }
});
