// original: 0x00d28980 target_angle_score
/// Pick the best-scoring of 8 candidate slots by heading alignment.
///
/// Scores the reference `a0` (or the owner's matrix heading when null) with
/// a two-float angle helper, then scans the 8 slots at `this+0x34` (stride
/// 64, each entry's word 0 is the candidate): null entries, entries equal to
/// `a0`, and — when the low byte of `a1` is set — entries whose kind at
/// `+0x14 & 7` reads 2 are skipped. Each live candidate is angle-scored the
/// same way; the absolute answer must not exceed the `a4` limit, else the
/// candidate is skipped. Otherwise the candidate's score minus the reference
/// score feeds a one-float helper whose answer is normalized through a
/// sign-mask xor from the image: with the low byte of `a2` clear the result
/// is always the xored answer; with it set, positive answers subtract a
/// limit constant, negative answers pass through, and zero/NaN answers are
/// xored. A normalized score strictly above the running best (seeded from
/// the image) takes the lead with its candidate. Returns the winning
/// candidate, or 0. The float operation order is the original's.
///
/// Original: thiscall, ECX plus five stack words.
lf_checker_rt::export!(thiscall, rw_00d28980(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const ANGLE2: u32 = 1;
        const ANGLE1: u32 = 2;
        const SEED_VA: u32 = 0x00e988a8;
        const SIGNMASK_VA: u32 = 0x00fe8fa0;
        const LIMITC_VA: u32 = 0x00fe8c58;
        let owner = ((this + 0x24c) as *const u32).read_unaligned();
        let omat = ((owner + 0x20) as *const u32).read_unaligned();
        let mx = f32::from_bits(((omat + 0x30) as *const u32).read_unaligned());
        let my = f32::from_bits(((omat + 0x34) as *const u32).read_unaligned());
        let mask = lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
        let flip = |v: f32| f32::from_bits(v.to_bits() ^ mask);
        let mut best = f32::from_bits(lf_checker_rt::global::<u32>(SEED_VA).read_unaligned());
        let mut winner = 0u32;
        let (rx, ry) = if a0 != 0 {
            let dx = ((a0 + 0x20) as *const u32).read_unaligned();
            let p: u32 = if dx != 0 { dx + 0x30 } else { a0 + 0x10 };
            let px = f32::from_bits((p as *const u32).read_unaligned());
            let py = f32::from_bits(((p + 4) as *const u32).read_unaligned());
            let dxv = core::hint::black_box(px) - core::hint::black_box(mx);
            let dyv = core::hint::black_box(py) - core::hint::black_box(my);
            (dxv, dyv)
        } else {
            let hx = f32::from_bits(((omat + 0x10) as *const u32).read_unaligned());
            let hy = f32::from_bits(((omat + 0x14) as *const u32).read_unaligned());
            (hx, hy)
        };
        let r0: f32 = lf_checker_rt::callee_cdecl!(ANGLE2, f32, rx.to_bits(), ry.to_bits());
        let a3f = f32::from_bits(a3);
        let a4f = f32::from_bits(a4);
        let limc = f32::from_bits(lf_checker_rt::global::<u32>(LIMITC_VA).read_unaligned());
        let mut i = 0u32;
        while i < 8 {
            let entry = this + 0x34 + i * 0x40;
            let slot = (entry as *const u32).read_unaligned();
            let mut live = slot != 0 && slot != a0;
            if live && (a1 & 0xFF) != 0 {
                let kind = (((entry + 0x14) as *const u32).read_unaligned() & 7) as u8;
                if kind == 2 {
                    live = false;
                }
            }
            if live {
                let dx = ((slot + 0x20) as *const u32).read_unaligned();
                let p: u32 = if dx != 0 { dx + 0x30 } else { slot + 0x10 };
                let px = f32::from_bits((p as *const u32).read_unaligned());
                let py = f32::from_bits(((p + 4) as *const u32).read_unaligned());
                let dxv = core::hint::black_box(px) - core::hint::black_box(mx);
                let dyv = core::hint::black_box(py) - core::hint::black_box(my);
                let r: f32 = lf_checker_rt::callee_cdecl!(ANGLE2, f32, dxv.to_bits(), dyv.to_bits());
                let d = core::hint::black_box(r) - core::hint::black_box(a3f);
                let s: f32 = lf_checker_rt::callee_cdecl!(ANGLE1, f32, d.to_bits());
                let t = if s < 0.0 { flip(s) } else { s };
                if !(t > a4f) {
                    let u = core::hint::black_box(r) - core::hint::black_box(r0);
                    let v: f32 = lf_checker_rt::callee_cdecl!(ANGLE1, f32, u.to_bits());
                    let nv = if (a2 & 0xFF) == 0 {
                        flip(v)
                    } else if v > 0.0 {
                        core::hint::black_box(v) - core::hint::black_box(limc)
                    } else if v < 0.0 {
                        v
                    } else {
                        flip(v)
                    };
                    if nv > best {
                        best = nv;
                        winner = slot;
                    }
                }
            }
            i += 1;
        }
        winner
    }
});

/// Wrong version of rw_00d28980: winners are never recorded.
lf_checker_rt::export!(thiscall, mut_00d28980(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const ANGLE2: u32 = 1;
        const ANGLE1: u32 = 2;
        const SEED_VA: u32 = 0x00e988a8;
        const SIGNMASK_VA: u32 = 0x00fe8fa0;
        const LIMITC_VA: u32 = 0x00fe8c58;
        let owner = ((this + 0x24c) as *const u32).read_unaligned();
        let omat = ((owner + 0x20) as *const u32).read_unaligned();
        let mx = f32::from_bits(((omat + 0x30) as *const u32).read_unaligned());
        let my = f32::from_bits(((omat + 0x34) as *const u32).read_unaligned());
        let mask = lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
        let flip = |v: f32| f32::from_bits(v.to_bits() ^ mask);
        let mut best = f32::from_bits(lf_checker_rt::global::<u32>(SEED_VA).read_unaligned());
        let mut winner = 0u32;
        let (rx, ry) = if a0 != 0 {
            let dx = ((a0 + 0x20) as *const u32).read_unaligned();
            let p: u32 = if dx != 0 { dx + 0x30 } else { a0 + 0x10 };
            let px = f32::from_bits((p as *const u32).read_unaligned());
            let py = f32::from_bits(((p + 4) as *const u32).read_unaligned());
            let dxv = core::hint::black_box(px) - core::hint::black_box(mx);
            let dyv = core::hint::black_box(py) - core::hint::black_box(my);
            (dxv, dyv)
        } else {
            let hx = f32::from_bits(((omat + 0x10) as *const u32).read_unaligned());
            let hy = f32::from_bits(((omat + 0x14) as *const u32).read_unaligned());
            (hx, hy)
        };
        let r0: f32 = lf_checker_rt::callee_cdecl!(ANGLE2, f32, rx.to_bits(), ry.to_bits());
        let a3f = f32::from_bits(a3);
        let a4f = f32::from_bits(a4);
        let limc = f32::from_bits(lf_checker_rt::global::<u32>(LIMITC_VA).read_unaligned());
        let mut i = 0u32;
        while i < 8 {
            let entry = this + 0x34 + i * 0x40;
            let slot = (entry as *const u32).read_unaligned();
            let mut live = slot != 0 && slot != a0;
            if live && (a1 & 0xFF) != 0 {
                let kind = (((entry + 0x14) as *const u32).read_unaligned() & 7) as u8;
                if kind == 2 {
                    live = false;
                }
            }
            if live {
                let dx = ((slot + 0x20) as *const u32).read_unaligned();
                let p: u32 = if dx != 0 { dx + 0x30 } else { slot + 0x10 };
                let px = f32::from_bits((p as *const u32).read_unaligned());
                let py = f32::from_bits(((p + 4) as *const u32).read_unaligned());
                let dxv = core::hint::black_box(px) - core::hint::black_box(mx);
                let dyv = core::hint::black_box(py) - core::hint::black_box(my);
                let r: f32 = lf_checker_rt::callee_cdecl!(ANGLE2, f32, dxv.to_bits(), dyv.to_bits());
                let d = core::hint::black_box(r) - core::hint::black_box(a3f);
                let s: f32 = lf_checker_rt::callee_cdecl!(ANGLE1, f32, d.to_bits());
                let t = if s < 0.0 { flip(s) } else { s };
                if !(t > a4f) {
                    let u = core::hint::black_box(r) - core::hint::black_box(r0);
                    let v: f32 = lf_checker_rt::callee_cdecl!(ANGLE1, f32, u.to_bits());
                    let nv = if (a2 & 0xFF) == 0 {
                        flip(v)
                    } else if v > 0.0 {
                        core::hint::black_box(v) - core::hint::black_box(limc)
                    } else if v < 0.0 {
                        v
                    } else {
                        flip(v)
                    };
                    if nv > best {
                        best = nv;
                        // MUTANT: winner update omitted.
                    }
                }
            }
            i += 1;
        }
        winner
    }
});
